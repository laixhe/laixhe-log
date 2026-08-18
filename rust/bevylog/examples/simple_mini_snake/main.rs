//! Bevy 0.19 入门示例：贪吃蛇（页面状态机 + 数据/渲染分离的小游戏）。
//! Loading → Menu → Playing → GameOver 四态切换，网格坐标驱动、Block → Sprite 自动渲染。
//!
//! 学习重点：
//! - 页面状态机：GameState（Loading / Menu / Playing / GameOver）驱动整页切换，
//!   用 OnEnter / OnExit 管理每页的生成与清理，in_state 条件控制每页的 Update 系统
//! - 数据与渲染分离：逻辑层只存 Block / Position 等数据，block_render_system 自动生成/更新 Sprite
//! - 蛇身跟随：Follow(Entity) 链 + PreviousPosition 快照，每 tick 逐段跟随前一段的轨迹
//! - MoveTimer / MoveTick：用 Timer 把「每帧」降频为「每 tick」，移动管线只在 tick 到达时运行
//! - .chain() 移动管线：快照 → 移动 → 跟随 → 碰撞 → 吃食物 → 生成食物 → 同步位置 → 渲染

use bevy::prelude::*;

mod pages;
use pages::router::GameState;

// === 主函数：应用初始化 ===
fn main() -> AppExit {
    App::new()
        // 配置窗口标题
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake Game".to_string(),
                ..default()
            }),
            ..default()
        }))
        // 初始化页面状态机（默认进入 Loading）
        .init_state::<GameState>()
        // 注册全局资源：
        // - MoveTick：每帧由 tick_move_timer 更新，供移动管线 run_if 判断「本帧是否该移动」
        // - Score：当前得分
        // - MoveTimer：重复定时器，每 0.15 秒触发一次 tick（控制蛇的移动速度）
        .init_resource::<pages::game::MoveTick>()
        .insert_resource(pages::game::Score(0))
        .insert_resource(pages::game::MoveTimer(Timer::from_seconds(
            0.15,
            TimerMode::Repeating,
        )))
        // 启动时执行一次：生成相机
        .add_systems(Startup, setup_camera)
        // === Loading 页 ===
        .add_systems(
            OnEnter(GameState::Loading),
            (pages::loading::setup_loading, log_state_enter),
        )
        .add_systems(OnExit(GameState::Loading), pages::loading::cleanup_loading)
        .add_systems(
            Update,
            pages::loading::countdown.run_if(in_state(GameState::Loading)),
        )
        // === Menu 页 ===
        .add_systems(
            OnEnter(GameState::Menu),
            (pages::menu::setup_menu, log_state_enter),
        )
        .add_systems(OnExit(GameState::Menu), pages::menu::cleanup_menu)
        .add_systems(
            Update,
            (
                pages::menu::handle_menu_buttons.run_if(in_state(GameState::Menu)),
                pages::menu::button_hover_effects.run_if(in_state(GameState::Menu)),
            ),
        )
        // === Playing 页 ===
        .add_systems(
            OnEnter(GameState::Playing),
            (pages::game::setup_game, log_state_enter),
        )
        .add_systems(OnExit(GameState::Playing), pages::game::cleanup_game)
        // 每帧：读取键盘更新玩家方向
        .add_systems(
            Update,
            pages::game::input_direction_system.run_if(in_state(GameState::Playing)),
        )
        // 每帧：推进 MoveTimer 定时器（更新 MoveTick 标志）
        .add_systems(
            Update,
            pages::game::tick_move_timer.run_if(in_state(GameState::Playing)),
        )
        // 每个 tick：移动管线（按顺序 .chain() 执行）。
        // should_move 读取 MoveTick，只在定时器 tick 到达的那一帧才运行整条管线；
        // 其余帧跳过，从而实现「每 0.15 秒移动一格」的节拍感。
        .add_systems(
            Update,
            (
                pages::game::snapshot_positions,
                pages::game::move_system,
                pages::game::snake_follow_system,
                pages::game::collision_system,
                pages::game::eating_system,
                pages::game::spawn_food_system,
                pages::game::sync_positions,
                pages::game::block_render_system,
            )
                .chain()
                .run_if(pages::game::should_move)
                .run_if(in_state(GameState::Playing)),
        )
        // 分数变化时更新分数 UI：resource_changed 避免每帧重写文本触发布局重排
        .add_systems(
            Update,
            pages::game::update_score_display
                .run_if(in_state(GameState::Playing))
                .run_if(resource_changed::<pages::game::Score>),
        )
        // === GameOver 页 ===
        .add_systems(
            OnEnter(GameState::GameOver),
            (pages::gameover::setup_game_over, log_state_enter),
        )
        .add_systems(
            OnExit(GameState::GameOver),
            pages::gameover::cleanup_game_over,
        )
        .add_systems(
            Update,
            (
                pages::gameover::handle_gameover_buttons.run_if(in_state(GameState::GameOver)),
                pages::gameover::button_hover_effects.run_if(in_state(GameState::GameOver)),
            ),
        )
        .run()
}

// === 相机 ===
fn setup_camera(mut commands: Commands) {
    info!("[初始化] 创建 2D 相机");
    commands.spawn(Camera2d);
}

// === 状态切换日志 ===
// 进入任意状态时打印日志，便于排查运行时流程
fn log_state_enter(state: Res<State<GameState>>) {
    info!("[状态] 进入 {:?}", state.get());
}
