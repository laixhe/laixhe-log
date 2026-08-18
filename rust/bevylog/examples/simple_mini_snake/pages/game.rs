use bevy::{prelude::*, text::FontSourceTemplate};
use rand::RngExt;

use crate::pages::router::GameState;

// 中文字体路径（与其它示例共用同一资产）
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// 蛇头标记
#[derive(Component)]
pub struct SnakeHead;
// 蛇身标记
#[derive(Component)]
pub struct SnakeBody;
// 蛇尾标记（用于吃食物时定位增长位置）
#[derive(Component)]
pub struct Tail;
// 玩家控制标记,例如有多个蛇时，由玩家控制的蛇头
#[derive(Component)]
pub struct PlayerControl;
// 块标记，可代表蛇的身体块，或者食物块，或墙体块
#[derive(Component)]
pub struct Block {
    pub color: Color,
}
// 食物标记
#[derive(Component)]
pub struct Food;
// 世界坐标（格数）
#[derive(Component, Clone)]
pub struct Position(pub i32, pub i32);
// 上一步的位置（用于蛇身跟随）
#[derive(Component)]
pub struct PreviousPosition(pub i32, pub i32);
// 跟随目标（蛇身用：Position = 目标.PreviousPosition）
#[derive(Component)]
pub struct Follow(pub Entity);
// 移动的方向（上、下、左、右）
#[derive(Component, Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
// 速度（每帧移动的格数）
#[derive(Component)]
pub struct Speed(u32);
// 无限世界标记：穿墙后从另一侧出现
#[derive(Component)]
pub struct WrapWorld;

#[derive(Resource)]
pub struct Score(pub u32);

#[derive(Component, Clone, Default)]
pub struct ScoreText;

#[derive(Component)]
pub struct GameArea;

// ==================== 移动定时器 ====================
/// 控制蛇的移动频率
#[derive(Resource)]
pub struct MoveTimer(pub Timer);

/// 每帧由 tick_move_timer 设置，供 run_if 只读检查
#[derive(Resource, Default)]
pub struct MoveTick(pub bool);

// ==================== 游戏常量配置 ====================
const CELL_SIZE: f32 = 25.0; // 每个格子的像素大小
const GRID_SIZE: u32 = 2; // 网格线宽度
const ARENA_WIDTH: u32 = 30; // 游戏区域宽度（格子数）
const ARENA_HEIGHT: u32 = 20; // 游戏区域高度（格子数）

/// 网格坐标 → 世界坐标（像素位置）
pub fn grid_to_world(gx: i32, gy: i32) -> Vec3 {
    let x = (gx as f32 - ARENA_WIDTH as f32 / 2.0) * CELL_SIZE + CELL_SIZE / 2.0;
    let y = (gy as f32 - ARENA_HEIGHT as f32 / 2.0) * CELL_SIZE + CELL_SIZE / 2.0;
    Vec3::new(x, y, 0.0)
}

pub fn setup_game(mut commands: Commands, mut score: ResMut<Score>) {
    score.0 = 0;

    // 蛇头实体 — 数据层只放逻辑组件，渲染由 block_render_system 自动创建
    let head_pos = Position(ARENA_WIDTH as i32 / 2, ARENA_HEIGHT as i32 / 2);
    info!(
        "[游戏] 关卡初始化：网格 {}x{}，蛇头出生 ({}, {})，初始方向向右",
        ARENA_WIDTH, ARENA_HEIGHT, head_pos.0, head_pos.1
    );
    let head_entity = commands
        .spawn((
            SnakeHead,
            PlayerControl, // 玩家控制标记
            // 无 WrapWorld → 有限世界，撞墙判负
            Block {
                color: Color::srgb(0.2, 0.8, 0.2),
            }, // 绿色蛇头
            Direction::Right,
            head_pos.clone(),
            PreviousPosition(head_pos.0, head_pos.1), // 初始与 Position 一致
            Speed(1),                                 // 每 tick 移动 1 格
        ))
        .id();

    // 初始蛇身（3 节）
    let body1 = commands
        .spawn((
            SnakeBody,
            Block {
                color: Color::srgb(0.15, 0.7, 0.15),
            },
            Position(head_pos.0 - 1, head_pos.1),
            PreviousPosition(head_pos.0 - 1, head_pos.1),
            Follow(head_entity),
        ))
        .id();

    let body2 = commands
        .spawn((
            SnakeBody,
            Block {
                color: Color::srgb(0.15, 0.7, 0.15),
            },
            Position(head_pos.0 - 2, head_pos.1),
            PreviousPosition(head_pos.0 - 2, head_pos.1),
            Follow(body1),
        ))
        .id();

    commands.spawn((
        SnakeBody,
        Tail, // 标记为蛇尾，吃食物时在其后方增长
        Block {
            color: Color::srgb(0.15, 0.7, 0.15),
        },
        Position(head_pos.0 - 3, head_pos.1),
        PreviousPosition(head_pos.0 - 3, head_pos.1),
        Follow(body2),
    ));
    // 得分文本实体渲染（bsn! 声明式构建，与项目其它 UI 一致）
    commands.spawn_scene(bsn! {
        Node {
            width: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(10.0)),
        }
        Children [
            (
                ScoreText
                Text::new("分数: 0")
                TextFont {
                    font: FontSourceTemplate::Handle(FONT_PATH),
                    font_size: FontSize::Px(30.0),
                }
                TextColor(Color::WHITE)
            )
        ]
    });

    // 游戏区域实体
    let width = ARENA_WIDTH as f32 * CELL_SIZE + GRID_SIZE as f32 / 2.0;
    let height = ARENA_HEIGHT as f32 * CELL_SIZE + GRID_SIZE as f32 / 2.0;
    //游戏区底色
    commands.spawn((
        GameArea,
        Sprite {
            color: Color::srgb(0.3, 0.3, 0.3),
            custom_size: Some(Vec2::new(width, height)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -1.0),
        Visibility::Visible,
    ));
    // 绘制网格
    create_grid(&mut commands);
}
fn create_grid(commands: &mut Commands) {
    let game_area_width = ARENA_WIDTH as f32 * CELL_SIZE;
    let game_area_height = ARENA_HEIGHT as f32 * CELL_SIZE;

    // 绘制垂直线
    for x in 0..=ARENA_WIDTH {
        //默认出生点在中心,所以要减width / 2.0
        let x_pos = x as f32 * CELL_SIZE - game_area_width / 2.0;

        commands.spawn((
            Sprite {
                color: Color::srgb(0.5, 0.5, 0.5), // 灰色网格线
                custom_size: Some(Vec2::new(
                    GRID_SIZE as f32,
                    game_area_height + GRID_SIZE as f32,
                )),
                ..default()
            },
            Transform::from_xyz(x_pos, 0.0, -0.9),
            Visibility::Visible,
        ));
    }

    // 绘制水平线
    for y in 0..=ARENA_HEIGHT {
        let y_pos = y as f32 * CELL_SIZE - game_area_height / 2.0;
        commands.spawn((
            Sprite {
                color: Color::srgb(0.5, 0.5, 0.5), // 灰色网格线
                custom_size: Some(Vec2::new(
                    game_area_width + GRID_SIZE as f32,
                    GRID_SIZE as f32,
                )),
                ..default()
            },
            Transform::from_xyz(0.0, y_pos, -0.9),
            Visibility::Visible,
        ));
    }
}

pub fn cleanup_game(
    mut commands: Commands,
    query: Query<
        Entity,
        Or<(
            With<SnakeHead>,
            With<SnakeBody>,
            With<Block>,
            With<Food>,
            With<ScoreText>,
            With<GameArea>,
        )>,
    >,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

// ==================== 数据驱动渲染系统 ====================

/// 系统1：Block 组件 → Sprite 自动渲染
/// Added<Block>:   自动插入 Sprite + Transform + Visibility
/// Changed<Block>: 自动更新 Sprite.color
/// Block 随实体一起 despawn 时，渲染组件会被引擎自动清理，无需额外处理
pub fn block_render_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Block, Option<&Position>), Or<(Added<Block>, Changed<Block>)>>,
) {
    // Block 被添加或修改 → 创建或更新渲染组件
    for (entity, block, position) in &mut query {
        let translation = position
            .map(|p| grid_to_world(p.0, p.1))
            .unwrap_or(Vec3::ZERO);

        commands.entity(entity).insert((
            Sprite {
                color: block.color,
                custom_size: Some(Vec2::new(
                    CELL_SIZE - GRID_SIZE as f32 / 2.0,
                    CELL_SIZE - GRID_SIZE as f32 / 2.0,
                )),
                ..default()
            },
            Transform::from_translation(translation),
            Visibility::Visible,
        ));
    }
}

/// 系统2：Position → Transform 同步
/// 当 Position 数据变化时，自动更新对应的 Transform.translation
pub fn sync_positions(mut query: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in &mut query {
        transform.translation = grid_to_world(pos.0, pos.1);
    }
}

// ==================== 移动系统 ====================

/// 每帧：tick 定时器，设置 MoveTick 标志
pub fn tick_move_timer(time: Res<Time>, mut timer: ResMut<MoveTimer>, mut tick: ResMut<MoveTick>) {
    tick.0 = timer.0.tick(time.delta()).just_finished();
}

/// run_if 条件：只读检查 MoveTick 标志
pub fn should_move(tick: Res<MoveTick>) -> bool {
    tick.0
}

/// 系统3：玩家方向控制
/// 查询被标记为 PlayerControl 且有 Direction 的实体
/// 根据键盘输入修改其 Direction，不关心移动
pub fn input_direction_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Direction, (With<PlayerControl>, With<SnakeHead>)>,
) {
    for mut dir in &mut query {
        let new_dir = if keyboard.just_pressed(KeyCode::ArrowUp)
            || keyboard.just_pressed(KeyCode::KeyW)
        {
            Some(Direction::Up)
        } else if keyboard.just_pressed(KeyCode::ArrowDown) || keyboard.just_pressed(KeyCode::KeyS)
        {
            Some(Direction::Down)
        } else if keyboard.just_pressed(KeyCode::ArrowLeft) || keyboard.just_pressed(KeyCode::KeyA)
        {
            Some(Direction::Left)
        } else if keyboard.just_pressed(KeyCode::ArrowRight) || keyboard.just_pressed(KeyCode::KeyD)
        {
            Some(Direction::Right)
        } else {
            None
        };

        // 不能 180° 掉头
        if let Some(new_dir) = new_dir {
            if !is_opposite(&dir, &new_dir) {
                info!("[输入] 方向切换为 {:?}", new_dir);
                *dir = new_dir;
            }
        }
    }
}

fn is_opposite(a: &Direction, b: &Direction) -> bool {
    matches!(
        (a, b),
        (Direction::Up, Direction::Down)
            | (Direction::Down, Direction::Up)
            | (Direction::Left, Direction::Right)
            | (Direction::Right, Direction::Left)
    )
}

/// 系统4：通用移动规则
/// 任何同时有 Position + Direction + Speed 的实体，都会按方向和速度移动
/// 如果还有 WrapWorld 标记，在边界处穿墙（从另一侧出现）
/// 如果没有 WrapWorld，则不处理边界——由未来的碰撞系统决定行为
pub fn move_system(
    mut query: Query<(Entity, &Direction, &Speed, &mut Position)>,
    wrap_query: Query<(), With<WrapWorld>>,
) {
    for (entity, dir, speed, mut pos) in &mut query {
        let step = speed.0 as i32;
        match dir {
            Direction::Right => pos.0 += step,
            Direction::Left => pos.0 -= step,
            Direction::Up => pos.1 += step,
            Direction::Down => pos.1 -= step,
        }

        // 无限世界：穿墙后从另一侧出现
        if wrap_query.contains(entity) {
            pos.0 = pos.0.rem_euclid(ARENA_WIDTH as i32);
            pos.1 = pos.1.rem_euclid(ARENA_HEIGHT as i32);
        }
        // 有限世界：不做任何事，由未来的碰撞系统处理撞墙结束游戏

        // 逐 tick 打印蛇头位置（频率较高，用 debug! 级别，RUST_LOG=debug 时可见）
        debug!("[移动] 蛇头 → ({}, {})", pos.0, pos.1);
    }
}

/// 系统5：位置快照
/// 将当前 Position 保存到 PreviousPosition
/// 用于蛇身跟随——Follow 实体读取前一段的快照
pub fn snapshot_positions(mut query: Query<(&Position, &mut PreviousPosition)>) {
    for (pos, mut prev) in &mut query {
        prev.0 = pos.0;
        prev.1 = pos.1;
    }
}

/// 系统6：蛇身跟随
/// 任何有 Follow 的实体，其 Position 被设置为其目标的 PreviousPosition
/// 这样蛇身会跟随前一段的轨迹，形成完美的链条
pub fn snake_follow_system(
    mut bodies: Query<(&mut Position, &Follow)>,
    prev_positions: Query<&PreviousPosition>,
) {
    for (mut pos, follow) in &mut bodies {
        if let Ok(prev) = prev_positions.get(follow.0) {
            pos.0 = prev.0;
            pos.1 = prev.1;
        }
    }
}

// ==================== 碰撞检测系统 ====================

/// 系统7：碰撞检测（撞墙 + 撞自己 → GameOver）
/// 在 snake_follow_system 之后运行
/// 此时蛇头已移动，身体段已跟随到位，检查碰撞
pub fn collision_system(
    head_query: Query<&Position, With<SnakeHead>>,
    body_query: Query<&Position, (With<SnakeBody>, Without<SnakeHead>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok(head_pos) = head_query.single() else {
        return;
    };

    // 撞墙检测：蛇头超出边界
    if head_pos.0 < 0
        || head_pos.0 >= ARENA_WIDTH as i32
        || head_pos.1 < 0
        || head_pos.1 >= ARENA_HEIGHT as i32
    {
        info!(
            "[碰撞] 撞墙！蛇头 ({}, {})，边界 {}x{}",
            head_pos.0, head_pos.1, ARENA_WIDTH, ARENA_HEIGHT
        );
        info!("[状态] Playing → GameOver");
        next_state.set(GameState::GameOver);
        return;
    }

    // 撞自己检测：蛇头与任何身体段位置重合
    for body_pos in &body_query {
        if head_pos.0 == body_pos.0 && head_pos.1 == body_pos.1 {
            info!("[碰撞] 撞到自己！蛇头 ({}, {})", head_pos.0, head_pos.1);
            info!("[状态] Playing → GameOver");
            next_state.set(GameState::GameOver);
            return;
        }
    }
}

// ==================== 食物系统 ====================

/// 系统7：吃食物检测
/// 在 snake_follow_system 之后运行
/// 检测蛇头是否与食物重合 → 吃！蛇尾增长 + 加分
pub fn eating_system(
    mut commands: Commands,
    head_query: Query<&Position, With<SnakeHead>>,
    food_query: Query<(Entity, &Position), With<Food>>,
    tail_query: Query<(Entity, &PreviousPosition), With<Tail>>,
    mut score: ResMut<Score>,
) {
    let Ok(head_pos) = head_query.single() else {
        return;
    };

    for (food_entity, food_pos) in &food_query {
        if head_pos.0 == food_pos.0 && head_pos.1 == food_pos.1 {
            // 吃掉食物
            commands.entity(food_entity).despawn();

            // 蛇尾增长：在尾部的上一步位置生成新身体段
            if let Ok((tail_entity, tail_prev)) = tail_query.single() {
                commands.spawn((
                    SnakeBody,
                    Tail, // 新段成为新蛇尾
                    Block {
                        color: Color::srgb(0.15, 0.7, 0.15),
                    },
                    Position(tail_prev.0, tail_prev.1),
                    PreviousPosition(tail_prev.0, tail_prev.1),
                    Follow(tail_entity), // 新段跟随旧蛇尾
                ));

                // 旧蛇尾不再是尾
                commands.entity(tail_entity).remove::<Tail>();
            }

            score.0 += 10;
            info!("[食物] 吃到食物，得分 +10，当前 {}", score.0);
            break;
        }
    }
}

/// 系统8：食物自动生成
/// 当场上没有食物时，在随机空位生成一个
pub fn spawn_food_system(
    mut commands: Commands,
    food_query: Query<Entity, With<Food>>,
    snake_query: Query<&Position, Or<(With<SnakeHead>, With<SnakeBody>)>>,
) {
    if !food_query.is_empty() {
        return;
    }

    // 收集被蛇占据的位置
    let occupied: Vec<(i32, i32)> = snake_query.iter().map(|p| (p.0, p.1)).collect();

    // 随机尝试空位
    let mut rng = rand::rng();
    for _ in 0..100 {
        let x: i32 = rng.random_range(0..ARENA_WIDTH as i32);
        let y: i32 = rng.random_range(0..ARENA_HEIGHT as i32);
        if !occupied.contains(&(x, y)) {
            info!("[食物] 生成食物于 ({}, {})", x, y);
            commands.spawn((
                Food,
                Block {
                    color: Color::srgb(0.9, 0.2, 0.2),
                },
                Position(x, y),
            ));
            return;
        }
    }
}

/// 系统9：分数显示更新
/// 每帧同步 Score 资源到 ScoreText 实体
pub fn update_score_display(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    if let Ok(mut text) = query.single_mut() {
        text.0 = format!("分数: {}", score.0);
    }
}
