//! 游戏模块：打砖块核心玩法（球/拍/砖块/生命/胜负）。
//!
//! 学习重点：
//! - 嵌套 GameState(Ready/Play/Pause/GameOver/GameWin) 子状态机
//! - #[derive(EntityEvent)] + commands.trigger() + .observe() + On<T> 即时事件（Observer 当帧执行）
//! - BoundingCircle 与 Aabb2d 碰撞检测 + closest_point 计算法线/穿透量
//! - 拍面反弹角度：按击中位置偏移计算反弹角，速度逐次递增
//! - 生命 Lives 资源 + resource_changed 触发 UI 刷新
//! - DespawnOnExit 退出游戏/状态时自动清理实体与覆盖层

use super::{GameSettings, GlobalGameState};
use bevy::math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume};
// FontSourceTemplate：bsn! 宏中 TextFont 的 font 字段类型
//   FontSourceTemplate::Handle("路径") 让 bsn 内部自动加载字体资产，无需手动 AssetServer
use bevy::{prelude::*, text::FontSourceTemplate};

// 中文字体路径：所有 UI 文本共享，bsn! 宏内部会缓存加载的资产
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// 游戏物理常量：集中定义便于调参
const PADDLE_SPEED: f32 = 600.0; // 拍子移动速度（像素/秒）
const PADDLE_WIDTH: f32 = 100.0; // 拍子宽度（也是碰撞体宽度）
const BALL_RADIUS: f32 = 10.0; // 球半径（碰撞圆半径）
const BALL_SPEED: f32 = 300.0; // 球初始速度分量（x/y 各 300，合速度约 424）

// 游戏内子状态机：仅在 GlobalGameState::Game 下活跃
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Ready, // 等待玩家按 Space 开始（球停在拍子上方）
    Play,     // 球运动中，物理系统运行
    Pause,    // 暂停（ESC 切换），物理系统停
    GameOver, // 生命耗尽，失败
    GameWin,  // 砖块全清，胜利
}

// 生命资源：掉球 -1，归零则 GameOver
#[derive(Resource)]
struct Lives(u32);

// 已击碎砖块数：球每撞碎一块砖 +1，用于日志统计（本游戏无独立计分 UI）
#[derive(Resource, Default)]
struct Score(u32);

// Marker 组件：标记「显示生命数的那行文字」，update_lives_ui 据此定位并刷新
#[derive(Component, Clone, Default)]
struct LivesText;

// 以下都是 Marker 组件：给实体打标签，Query 用 With<Paddle> 等筛选特定实体
#[derive(Component)]
struct Paddle;
#[derive(Component)]
struct Ball;
#[derive(Component)]
struct Brick;

// EntityEvent：即时事件，commands.trigger 后当帧由 Observer 处理（区别于缓冲式 Message）
// entity 字段决定事件路由：挂在该 entity 上的观察者（.observe）会收到，全局观察者（add_observer）也会收到所有事件
#[derive(EntityEvent)]
struct CollisionEvent {
    pub entity: Entity, // 被撞的碰撞体：砖块/拍子实体（决定事件路由到哪个 .observe 观察者）
    pub nudge: Vec2,    // 沿法线推出的位移量
    // 球击中时的世界坐标，用于日志记录
    pub ball_pos: Vec2,
}

// 速度组件：Newtype 包装 Vec2，derive Deref 后可用 velocity.x 直接访问内层 Vec2 字段
#[derive(Component, Deref, DerefMut, Debug)]
struct Velocity(Vec2);

impl Velocity {
    fn accelerate(&mut self) {
        // 每次碰撞速度 +10%，封顶 700 防止球过快失控
        self.0 = (self.0 * 1.10).clamp_length_max(700.0)
    }
}

// Collider：标记「可被球碰撞的实体」（拍子+砖块都挂），碰撞检测遍历它
#[derive(Component)]
struct Collider;

pub fn game_plugin(app: &mut App) {
    app // 进入顶层 Game 态时初始化关卡（生成拍/球/砖块）
        .add_systems(OnEnter(GlobalGameState::Game), game_setup)
        // 进入各子状态时弹出对应覆盖层
        .add_systems(OnEnter(GameState::Pause), pause_overlay)
        .add_systems(OnEnter(GameState::GameOver), game_over)
        .add_systems(OnEnter(GameState::GameWin), game_win)
        .add_systems(
            OnEnter(GameState::Ready),
            (ready_overlay, reset_ball_on_ready),
        )
        .insert_resource(Lives(3))
        .insert_resource(Score::default())
        .init_state::<GameState>()
        // Update 系统集合：每个用 run_if 限定在特定状态下运行
        .add_systems(
            Update,
            (
                back_to_main_menu
                    .run_if(in_state(GameState::GameOver).or_else(in_state(GameState::GameWin))),
                toggle_pause.run_if(in_state(GameState::Play).or_else(in_state(GameState::Pause))),
                start_game.run_if(in_state(GameState::Ready)),
                // resource_changed::<Lives>：仅当 Lives 资源被修改的那帧才跑，刷新 UI
                update_lives_ui.run_if(resource_changed::<Lives>),
                check_win_condition.run_if(in_state(GameState::Play)),
            ),
        )
        // FixedUpdate 在固定步长（见 main.rs 的 Time::<Fixed>::from_hz）下推进，物理与帧率解耦；
        // .chain() 强制按声明顺序执行：移拍 → 移球 → 检测碰撞，避免同帧顺序不定导致穿模
        .add_systems(
            FixedUpdate,
            (move_paddle, apply_velocity, check_collision)
                .chain()
                .run_if(in_state(GameState::Play)),
        )
        // 两个全局观察者：on_collision（加速）和 on_ball_collision（推出穿透量）。
        // 两者无数据依赖（一个改速度、一个改位置），即便执行顺序不保证也安全。
        .add_observer(on_collision)
        .add_observer(on_ball_collision);
}

// 关卡初始化：重置资源 → 生成 UI → 生成拍/球/砖块 → 切到 Ready 等玩家开始
// 参数较多是整合了资源重置与场景生成；新手示例为聚焦整体流程，不额外拆 SystemParam
#[allow(clippy::too_many_arguments)]
fn game_setup(
    mut commands: Commands,
    // Assets<T> 是资产仓库：meshes.add 返回 Handle<Mesh>，实体持有句柄即可引用资产
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut lives: ResMut<Lives>,
    mut score: ResMut<Score>,
    window: Single<&Window>,
    settings: Res<GameSettings>,
) {
    // 重置生命与分数：玩家从 GameOver 返回菜单再进 Game 时需恢复初始值
    lives.0 = 3;
    score.0 = 0;
    info!(
        "[状态] 进入 Game → Ready（关卡初始化：{}行 × {}列砖块，生命=3）",
        settings.brick_rows, settings.brick_columns
    );
    next_state.set(GameState::Ready);

    // 生命数 UI 文本
    commands.spawn_scene(bsn! {
        DespawnOnExit::<GlobalGameState>(GlobalGameState::Game)
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(15.0),
            right: Val::Px(15.0),
        }
        Children [
            (
                LivesText
                Text::new("生命：3")
                TextColor(Color::srgb(0.15, 0.15, 0.15))
                TextFont {
                    font: FontSourceTemplate::Handle(FONT_PATH),
                    font_size: FontSize::Px(30.0),
                }
            )
        ]
    });

    // 生成拍子
    // 模式：Mesh2d 用单位矩形（1×1），实际尺寸由 Transform.scale 控制，多个砖块可共享同一 mesh 资产
    commands.spawn((
        DespawnOnExit(GlobalGameState::Game),
        Paddle,
        Collider,
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::srgb(0.6, 0.2, 0.2))),
        Transform {
            translation: Vec3::new(0.0, -window.height() / 2.0 + 50.0, 0.0),
            scale: Vec3::new(PADDLE_WIDTH, 22.0, 1.0),
            ..default()
        },
    ));

    // 生成球
    commands.spawn((
        DespawnOnExit(GlobalGameState::Game),
        Ball,
        Velocity(Vec2::new(BALL_SPEED, BALL_SPEED)),
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(Color::srgb(0.6, 0.1, 0.5))),
        Transform {
            translation: Vec3::new(0.0, -window.height() / 2.0 + 70.0, 0.0),
            scale: Vec2::splat(BALL_RADIUS * 2.0).extend(1.0),
            ..default()
        },
    ));

    // 生成砖块：按行列网格排布砖块，整体居中填满窗口顶部
    let brick_area_gutter = 10.0; // 砖块区与窗口边缘的留白
    let brick_gap = 5.0; // 相邻砖块间距
    let brick_height = 20.0;
    // 可用宽度 = 窗口宽 - 两侧留白 - 所有间隙总和
    let brick_area_width = window.width()
        - (brick_area_gutter * 2.0)
        - (brick_gap * (settings.brick_columns as f32 - 1.0));
    let brick_width = brick_area_width / settings.brick_columns as f32;
    // 最左一列的中心 x：从窗口左缘推进「留白 + 半块宽」
    let column_start = -window.width() / 2.0 + brick_area_gutter + brick_width / 2.0;
    // 最上一行的中心 y：从窗口顶缘下移「留白 + 半块高」
    let row_start = window.height() / 2.0 - brick_area_gutter - brick_height / 2.0;

    // 颜色生成在外层循环：每行共享一个颜色，不同行颜色不同（经典 Breakout 视觉风格）
    for row in 0..settings.brick_rows {
        // random_range(0.0..1.0) 返回 [0.0, 1.0) 内的 f32：Rust Range 左闭右开，含 0.0 不含 1.0
        // 对 RGB 分量无影响（1.0 与接近 1.0 视觉等价）
        let r = rand::random_range(0.0..1.0);
        let g = rand::random_range(0.0..1.0);
        let b = rand::random_range(0.0..1.0);

        for column in 0..settings.brick_columns {
            let brick_x = column_start + column as f32 * (brick_width + brick_gap);
            let brick_y = row_start - row as f32 * (brick_height + brick_gap);
            commands
                .spawn((
                    DespawnOnExit(GlobalGameState::Game),
                    Brick,
                    Collider,
                    Mesh2d(meshes.add(Rectangle::default())),
                    MeshMaterial2d(materials.add(Color::srgb(r, g, b))),
                    Transform {
                        translation: Vec3::new(brick_x, brick_y, 0.0),
                        scale: Vec3::new(brick_width, brick_height, 1.0),
                        ..default()
                    },
                ))
                .observe(on_brick_collision);
        }
    }
}

fn start_game(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    // just_pressed 按下那帧才 true（边沿触发），pressed 按住期间每帧 true（电平触发）；切态用前者避免连切
    if keyboard_input.just_pressed(KeyCode::Space) {
        info!("[状态] Ready → Play（按 Space 开始）");
        next_state.set(GameState::Play);
    }
}

// ESC 暂停切换：Play↔Pause 互转
// Res<State> 只读当前态（.get()），ResMut<NextState> 写入下帧态（.set()）
fn toggle_pause(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match state.get() {
            GameState::Play => {
                info!("[状态] Play → Pause（ESC 暂停）");
                next_state.set(GameState::Pause);
            }
            GameState::Pause => {
                info!("[状态] Pause → Play（ESC 恢复）");
                next_state.set(GameState::Play);
            }
            _ => {}
        }
    }
}

fn back_to_main_menu(
    mut next_global_state: ResMut<NextState<GlobalGameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        info!("[状态] GameOver/GameWin → Menu（ESC 返回主菜单）");
        next_global_state.set(GlobalGameState::Menu);
    }
}

fn pause_overlay(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        DespawnOnExit::<GameState>(GameState::Pause)
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5))
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
        }
        Children [
            (
                Text::new("已暂停")
                TextColor(Color::WHITE)
                TextFont {
                    font: FontSourceTemplate::Handle(FONT_PATH),
                    font_size: FontSize::Px(80.0),
                }
            )
        ]
    });
}

fn game_over(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        DespawnOnExit::<GlobalGameState>(GlobalGameState::Game)
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5))
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
        }
        Children [
            (
                Text::new("游戏结束")
                TextColor(Color::WHITE)
                TextFont {
                    font: FontSourceTemplate::Handle(FONT_PATH),
                    font_size: FontSize::Px(40.0),
                }
            ),
            (
                Text::new("按 ESC 返回主菜单")
                TextColor(Color::WHITE)
                TextFont {
                    font: FontSourceTemplate::Handle(FONT_PATH),
                    font_size: FontSize::Px(40.0),
                }
            )
        ]
    });
}

fn game_win(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        DespawnOnExit::<GlobalGameState>(GlobalGameState::Game)
        BackgroundColor(Color::srgba(0.0, 0.4, 0.0, 0.6)) // 绿色覆盖层表示胜利
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
        }
        Children [
            (
                Text::new("你赢了！")
                TextColor(Color::WHITE)
                TextFont {
                    font: FontSourceTemplate::Handle(FONT_PATH),
                    font_size: FontSize::Px(60.0),
                }
            ),
            (
                Text::new("按 ESC 返回主菜单")
                TextColor(Color::WHITE)
                TextFont {
                    font: FontSourceTemplate::Handle(FONT_PATH),
                    font_size: FontSize::Px(30.0),
                }
            )
        ]
    });
}

fn ready_overlay(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        DespawnOnExit::<GameState>(GameState::Ready)
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5))
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
        }
        Children [
            (
                Text::new("准备好请按空格键")
                TextColor(Color::WHITE)
                TextFont {
                    font: FontSourceTemplate::Handle(FONT_PATH),
                    font_size: FontSize::Px(50.0),
                }
            )
        ]
    });
}

// Ready 态把球和拍复位到初始位置，速度重置，等待玩家按 Space
// Single 元组查询：同时拿多个组件，With/Without 做实体过滤
// Without<Ball> 防止拍子查询误拿到球（两者都有 Transform）
fn reset_ball_on_ready(
    ball_query: Single<(&mut Transform, &mut Velocity), With<Ball>>,
    paddle_query: Single<&mut Transform, (With<Paddle>, Without<Ball>)>,
    window: Single<&Window>,
) {
    let (mut ball_transform, mut ball_velocity) = ball_query.into_inner();
    ball_transform.translation = Vec3::new(0.0, -window.height() / 2.0 + 70.0, 0.0);
    ball_velocity.0 = Vec2::new(BALL_SPEED, BALL_SPEED);

    let mut paddle_transform = paddle_query.into_inner();
    paddle_transform.translation = Vec3::new(0.0, -window.height() / 2.0 + 50.0, 0.0);
}

// 生命数变化时刷新 UI 文字（由 resource_changed::<Lives> 触发）
fn update_lives_ui(lives: Res<Lives>, mut text: Single<&mut Text, With<LivesText>>) {
    // text.0 访问 Text 内部的 String 字段（Text 是 newtype Text(String)）
    text.0 = format!("生命：{}", lives.0);
}

// 胜利判定：场上没有 Brick 实体即获胜（每帧检查，Play 态运行）
fn check_win_condition(brick_query: Query<&Brick>, mut next_state: ResMut<NextState<GameState>>) {
    if brick_query.is_empty() {
        info!("[状态] Play → GameWin（所有砖块已击碎）");
        next_state.set(GameState::GameWin);
    }
}

fn move_paddle(
    mut paddle_transform: Single<&mut Transform, With<Paddle>>,
    window: Single<&Window>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let paddle_half_width = PADDLE_WIDTH / 2.0;
    let window_half_width = window.width() / 2.0;

    let mut direction = 0.0;
    // 同时支持方向键和 WASD：any_pressed 任一按下即触发（无障碍输入约定）
    if keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
        direction -= 1.0;
    }
    if keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
        direction += 1.0;
    }

    // delta_secs() 返回上一帧到现在的秒数（f32）；delta() 返回 Duration，前者便于直接参与数值运算
    let paddle_new_position =
        paddle_transform.translation.x + direction * PADDLE_SPEED * time.delta_secs();
    // clamp 限制拍子不越出窗口：左右各留半个拍宽，避免拍子完全移出屏幕
    paddle_transform.translation.x = paddle_new_position.clamp(
        -window_half_width + paddle_half_width,
        window_half_width - paddle_half_width,
    );
}

// 球位移：位置 += 速度 × 帧时长（.x/.y 来自 Velocity 对 Vec2 的 Deref）
fn apply_velocity(ball_query: Single<(&mut Transform, &Velocity), With<Ball>>, time: Res<Time>) {
    let (mut ball_transform, ball_velocity) = ball_query.into_inner();

    ball_transform.translation +=
        Vec3::new(ball_velocity.x, ball_velocity.y, 0.0) * time.delta_secs();
}

// 碰撞检测主系统：先处理墙壁反弹/掉球，再遍历所有 Collider 做球-体碰撞
fn check_collision(
    mut commands: Commands,
    window: Single<&Window>,
    ball_query: Single<(&Transform, &mut Velocity), With<Ball>>,
    paddle_query: Query<&Paddle>,
    collider_query: Query<(Entity, &Transform), With<Collider>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut lives: ResMut<Lives>,
) {
    let (ball_transform, mut ball_velocity) = ball_query.into_inner();
    let window_half_width = window.width() / 2.0;
    let window_half_height = window.height() / 2.0;

    // 左右墙：球缘越界则反向（用 abs 强制方向，避免连续穿透时方向抖动）
    if ball_transform.translation.x + BALL_RADIUS >= window_half_width {
        ball_velocity.x = -ball_velocity.x.abs();
        info!(
            "[碰撞] 撞右墙 @ x={:.1}，vx 反向 → {:.0}",
            ball_transform.translation.x, ball_velocity.x
        );
    } else if ball_transform.translation.x - BALL_RADIUS <= -window_half_width {
        ball_velocity.x = ball_velocity.x.abs();
        info!(
            "[碰撞] 撞左墙 @ x={:.1}，vx 反向 → {:.0}",
            ball_transform.translation.x, ball_velocity.x
        );
    }

    // 上墙反弹；下墙则掉球：还有命回 Ready 重置，没命则 GameOver
    if ball_transform.translation.y + BALL_RADIUS >= window_half_height {
        ball_velocity.y = -ball_velocity.y.abs();
        info!(
            "[碰撞] 撞上墙 @ y={:.1}，vy 反向 → {:.0}",
            ball_transform.translation.y, ball_velocity.y
        );
    } else if ball_transform.translation.y - BALL_RADIUS <= -window_half_height {
        // 掉球后立刻 return：球已出界，本帧不再做其他碰撞检测，避免对已掉出的球误触发砖块事件
        if lives.0 > 1 {
            lives.0 -= 1;
            info!("[状态] Play → Ready（掉球！剩余生命 {}）", lives.0);
            next_state.set(GameState::Ready);
            return;
        } else {
            lives.0 = 0;
            info!("[状态] Play → GameOver（生命耗尽）");
            next_state.set(GameState::GameOver);
            return;
        }
    }

    // 球的碰撞体积：圆心=球位置，半径=BALL_RADIUS
    // .xy() 把 Vec3 降为 Vec2（碰撞计算只需 xy 平面，丢弃 z）
    let ball_bounding_circle = BoundingCircle::new(ball_transform.translation.xy(), BALL_RADIUS);

    // 循环无 break：允许球角部一帧同时撞多块砖（连击），速度会反射多次但视觉上无感知
    for (entity, transform) in &collider_query {
        // 砖块/拍子的碰撞体积：轴对齐包围盒（AABB），中心=位置，半尺寸=scale/2
        // 因为 mesh 是单位矩形，scale 即实际宽高，所以 scale/2 是半宽半高
        let collision_entity_bounding_box =
            Aabb2d::new(transform.translation.xy(), transform.scale.xy() / 2.0);

        if ball_bounding_circle.intersects(&collision_entity_bounding_box) {
            // 球心到 AABB 边界的最近点，用于判断球从哪个方向撞入
            let closest =
                collision_entity_bounding_box.closest_point(ball_bounding_circle.center());
            // 法线方向：从最近点指向球心，归一化后即为碰撞法线
            let offset = ball_bounding_circle.center() - closest;
            let distance = offset.length();
            let normal = if offset == Vec2::ZERO {
                Vec2::Y // 球心恰好在边界内，默认向上弹
            } else {
                offset / distance
            };

            // 穿透量 = 球半径 − 球心到边界距离，正值表示球已嵌入碰撞体
            let overlap = BALL_RADIUS - distance;
            // nudge：沿法线把球推出穿透量，消除重叠
            let nudge = normal * overlap;

            // 按法线主轴反射速度：水平法线翻转 x 分量，垂直法线翻转 y 分量
            // 用 abs()*signum() 而非 *=-1：强制方向与法线一致，避免高速穿透时方向抖动
            if normal.x.abs() > normal.y.abs() {
                ball_velocity.x = ball_velocity.x.abs() * normal.x.signum()
            } else {
                ball_velocity.y = ball_velocity.y.abs() * normal.y.signum()
            }

            // 拍子特殊处理：按击中位置改变反弹角度（不是简单反射）
            if paddle_query.get(entity).is_ok() {
                // impact：击中点相对拍中心的偏移，归一化到 [-1, 1]（-1=左缘, 0=中心, 1=右缘）
                let mut impact = (ball_bounding_circle.center().x
                    - collision_entity_bounding_box.center().x)
                    / (PADDLE_WIDTH / 2.0);
                impact = impact.clamp(-1.0, 1.0);

                // 击中拍边最大反弹 60°（拍中心为 0°），越靠中心反弹越直，经典 Breakout 反弹机制
                const MAX_BOUNCE_ANGLE: f32 = 60.0_f32.to_radians();
                let angle = impact * MAX_BOUNCE_ANGLE;

                // 新方向：x=sin(角度) 决定左右偏移，y=cos(角度) 始终向上
                let new_direction = Vec2::new(angle.sin(), angle.cos()).normalize();

                // 保持原速度大小，只改方向
                let speed = ball_velocity.length();
                ball_velocity.0 = new_direction * speed;

                info!(
                    "[碰撞] 击中拍子 @ ({:.1}, {:.1})，偏移={:.2}，反弹角={:.0}°，速度={:.0}",
                    ball_transform.translation.x,
                    ball_transform.translation.y,
                    impact,
                    angle.to_degrees(),
                    speed
                );
            }

            // 触发碰撞事件：entity 字段让事件路由到该碰撞体的观察者
            // 全局观察者（on_collision/on_ball_collision）也会收到
            commands.trigger(CollisionEvent {
                entity,
                nudge,
                ball_pos: ball_transform.translation.xy(),
            });
        }
    }
}

// 砖块碰撞观察者：注册在每个 Brick 实体上（.observe），EntityEvent 按 entity 字段路由，
// 只有被撞的那块砖会收到事件
fn on_brick_collision(
    collision: On<CollisionEvent>,
    mut commands: Commands,
    mut score: ResMut<Score>,
) {
    let entity = collision.entity;
    commands.entity(entity).despawn();
    score.0 += 1;
    info!(
        "[碰撞] 击中砖块 @ ({:.1}, {:.1})，nudge=({:.1},{:.1}) | 已击碎: {}",
        collision.ball_pos.x, collision.ball_pos.y, collision.nudge.x, collision.nudge.y, score.0
    );
}

// 全局观察者（add_observer）：所有碰撞都触发，负责沿法线把球推出穿透量消除重叠
fn on_ball_collision(
    collision: On<CollisionEvent>,
    mut ball_transform: Single<&mut Transform, With<Ball>>,
) {
    ball_transform.translation += collision.nudge.extend(0.0)
}

// 全局观察者：所有碰撞（拍/砖）触发，每次都给球加速
fn on_collision(
    _collision: On<CollisionEvent>,
    mut ball_velocity: Single<&mut Velocity, With<Ball>>,
) {
    let old_speed = ball_velocity.length();
    ball_velocity.accelerate();
    info!(
        "[碰撞] 球加速：{:.0} → {:.0}",
        old_speed,
        ball_velocity.length()
    );
}
