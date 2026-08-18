//! Bevy 0.19 入门示例：演示状态系统（States / OnEnter / OnExit / in_state / set_state）。
//! 三个状态：菜单 → 游戏中 → 暂停，按空格 / P / ESC 切换。
//!
//! 学习重点：
//! - States trait 定义应用状态（用 enum + #[derive(States)]）
//! - OnEnter / OnExit 调度：进入 / 离开状态时执行一次（类似 Startup，但每次切换都触发）
//! - in_state 运行条件：控制 Update 系统只在特定状态下运行
//! - commands.set_state 切换状态（等价于设置 NextState 资源，帧结束时由 Bevy 应用）
//! - 状态切换的完整流程：set_state → 帧结束应用 → OnExit(旧) → OnEnter(新) → 下一帧 Update

use bevy::{
    prelude::*,
    text::{FontSource, FontSourceTemplate},
};

// 中文字体路径
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// 应用状态：用 enum 定义所有可能的状态。
// #[derive(States)] 让它成为 Bevy 状态类型（自动实现 FreelyMutableState + FromWorld）。
// #[default] 指定初始状态为 Menu（应用启动时自动进入菜单）。
// States 要求实现 Clone + PartialEq + Eq + Hash + Debug（用于状态比较和调度）。
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum AppState {
    #[default]
    Menu, // 菜单：按空格开始游戏
    Playing, // 游戏中：方向键 / WASD 移动玩家，P 或 ESC 暂停
    Paused,  // 暂停：P 继续，ESC 回菜单
}

// 标记组件：用于标识不同状态的 UI / 玩家实体，方便 OnExit 时销毁
// bsn! 宏要求组件实现 Clone + Default（宏内部用模板反射构造实体）
#[derive(Component, Clone, Default)]
struct MenuUI;

#[derive(Component, Clone, Default)]
struct Player;

#[derive(Component, Clone, Default)]
struct PausedUI;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK))
        // 初始化状态：init_state 注册 AppState 并设置为 #[default]（即 Menu）。
        // 这会自动注册 OnEnter / OnExit 调度集和状态转换系统。
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        // OnEnter 调度：进入状态时执行一次（类似 Startup，但每次进入该状态都触发）。
        // OnExit 调度：离开状态时执行一次。两者配合管理实体的生成 / 销毁。
        .add_systems(OnEnter(AppState::Menu), setup_menu)
        .add_systems(OnExit(AppState::Menu), teardown_menu)
        .add_systems(OnEnter(AppState::Playing), setup_playing)
        .add_systems(OnExit(AppState::Playing), teardown_playing)
        .add_systems(OnEnter(AppState::Paused), setup_paused)
        .add_systems(OnExit(AppState::Paused), teardown_paused)
        // Update 调度：用 in_state 条件控制只在特定状态下运行系统。
        // run_if(in_state(...)) 是运行条件：状态不匹配时系统直接跳过（不执行）。
        .add_systems(
            Update,
            (
                menu_input.run_if(in_state(AppState::Menu)),
                move_player.run_if(in_state(AppState::Playing)),
                playing_input.run_if(in_state(AppState::Playing)),
                paused_input.run_if(in_state(AppState::Paused)),
            ),
        )
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 生成 2D 相机（只在启动时生成一次，不随状态变化）
    commands.spawn(Camera2d);
    // 玩家实体只在启动时生成一次，跨状态持久存在。
    // 进入 / 离开 Playing 通过切换 Visibility 隐藏 / 显示，而不是销毁重建，
    // 这样「暂停 → 继续」时玩家位置不会被重置（Visibility 方案相较 despawn 方案的关键优势）。
    // 注意：这里用 commands.spawn 元组（而非 bsn!）构建文本玩家，因为要设置初始 Visibility::Hidden；
    // 文本字体用 FontSource::Handle + asset_server.load 手动加载（bsn! 里的 FontSourceTemplate 只是模板语法）。
    commands.spawn((
        Player,
        Text2d::new("Movement"),
        TextColor(Color::WHITE),
        TextFont {
            font: FontSource::Handle(asset_server.load(FONT_PATH)),
            font_size: FontSize::Px(30.0),
            ..default()
        },
        Transform::default(),
        // 初始隐藏：只有进入 Playing 状态才显示
        Visibility::Hidden,
    ));
}

// === 菜单状态 ===

fn setup_menu(mut commands: Commands) {
    // 进入菜单：生成菜单提示文本，带 MenuUI 标记方便离开时销毁
    commands.spawn_scene(bsn! {
        MenuUI
        Text2d::new("按 空格 开始游戏")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(30.0),
        }
        Transform::default()
    });
    info!("[状态] 进入菜单");
}

fn teardown_menu(mut commands: Commands, query: Query<Entity, With<MenuUI>>) {
    // 离开菜单：销毁所有带 MenuUI 标记的实体
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn menu_input(keyboard: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    if keyboard.just_pressed(KeyCode::Space) {
        // set_state 通过 Commands 设置下一个状态。
        // 状态转换不是立即生效的：当前帧结束后由 Bevy 内部系统应用，
        // 然后依次触发 OnExit(Menu) → OnEnter(Playing)，下一帧 Update 才在 Playing 状态下运行。
        // 这等价于 ResMut<NextState<AppState>> 的 set 方法，只是更简洁。
        commands.set_state(AppState::Playing);
        info!("[状态] 菜单 → 游戏中");
    }
}

// === 游戏中状态 ===

fn setup_playing(
    // Single 查询：期望恰好一个 Player 实体（已在 Startup 生成），进入游戏时把它显示出来
    player: Single<&mut Visibility, With<Player>>,
) {
    // into_inner() 取出 Single 内部的数据（这里是 Mut<Visibility>，即可变引用包装），
    // 再用 * 解引用整体赋值，等价于把玩家可见性设为 Visible。
    let mut visibility = player.into_inner();
    *visibility = Visibility::Visible;
    info!("[状态] 进入游戏");
}

fn teardown_playing(
    // Single 查询：期望恰好一个 Player 实体（已在 Startup 生成），离开游戏时把它隐藏
    player: Single<&mut Visibility, With<Player>>,
) {
    // 离开游戏：隐藏玩家实体（不销毁）。
    // 注意：Playing → Paused 也会触发 OnExit(Playing)，所以暂停后玩家被隐藏；
    // 继续（Paused → Playing）时玩家重新显示，位置保持不变——因为实体从未被销毁。
    let mut visibility = player.into_inner();
    *visibility = Visibility::Hidden;
}

fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    // Single 查询：期望恰好一个带 Player 组件的实体
    mut player: Single<&mut Transform, With<Player>>,
) {
    // 方向累加：每个方向同时支持方向键和 WASD
    let mut direction = Vec2::ZERO;
    if keyboard.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        direction.x -= 1.0;
    }
    if keyboard.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        direction.x += 1.0;
    }
    if keyboard.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
        direction.y += 1.0;
    }
    if keyboard.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
        direction.y -= 1.0;
    }
    // 帧率无关移动：只在有方向输入时移动（避免零向量 normalize 得 NaN）
    if direction != Vec2::ZERO {
        let speed = 300.0;
        let delta = direction.normalize() * speed * time.delta_secs();
        player.translation += delta.extend(0.0);
    }
}

fn playing_input(keyboard: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    // P 或 ESC 暂停游戏
    if keyboard.just_pressed(KeyCode::KeyP) || keyboard.just_pressed(KeyCode::Escape) {
        commands.set_state(AppState::Paused);
        info!("[状态] 游戏中 → 暂停");
    }
}

// === 暂停状态 ===

fn setup_paused(mut commands: Commands) {
    // 进入暂停：生成暂停提示文本，带 PausedUI 标记
    commands.spawn_scene(bsn! {
        PausedUI
        Text2d::new("暂停 | P 继续 | ESC 回菜单")
        TextColor(Color::srgb(1.0, 0.8, 0.2))
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(30.0),
        }
        Transform::from_xyz(0.0, 100.0, 0.0)
    });
    info!("[状态] 进入暂停");
}

fn teardown_paused(mut commands: Commands, query: Query<Entity, With<PausedUI>>) {
    // 离开暂停：销毁暂停提示文本
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn paused_input(keyboard: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    // P 继续游戏（回到 Playing）
    if keyboard.just_pressed(KeyCode::KeyP) {
        commands.set_state(AppState::Playing);
        info!("[状态] 暂停 → 游戏中");
    }
    // ESC 回到菜单
    if keyboard.just_pressed(KeyCode::Escape) {
        commands.set_state(AppState::Menu);
        info!("[状态] 暂停 → 菜单");
    }
}
