//! Bevy 0.19 组合学习示例：2D 综合小游戏「星空收集」。
//! Menu → Playing → GameOver 三态小游戏：移动玩家收集旋转的星星，躲避落下的陨石。
//!
//! 学习重点（组合了 2D 阶段的多个核心概念）：
//! - 状态机：GameState（Menu / Playing / GameOver）+ OnEnter / OnExit / in_state 整页切换
//! - 相机平滑跟随：lerp + 帧率无关 factor，相机追向玩家
//! - 视差滚动：多层背景以不同速度随相机移动（factor 越小越远越慢）
//! - 图集动画：TextureAtlasLayout::from_grid + Sprite.texture_atlas.index 帧切换（星星旋转）
//! - 定时生成：Timer::Repeating 定时生成星星 / 陨石，超出范围 despawn
//! - 粒子系统：玩家拖尾 + 收集爆炸（速度 + 生命周期 + 淡出缩放）
//! - 圆形碰撞：距离平方检测收集星星 / 被陨石击中
//! - 音频：AudioPlayer + PlaybackSettings::DESPAWN 播放一次性收集音效
//! - UI：屏幕左上角得分显示，resource_changed 节流更新
//!
//! 操作：WASD / 方向键移动 | 收集星星 +1 分 | 被陨石击中游戏结束 | ESC 返回菜单。

use bevy::prelude::*;

mod pages;
use pages::router::GameState;

// === 主函数：应用初始化 ===
fn main() -> AppExit {
    App::new()
        // 配置窗口标题
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "星空收集 Star Collector".to_string(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.04, 0.05, 0.09)))
        // 页面状态机（默认进入 Menu）
        .init_state::<GameState>()
        // 全局资源：得分 + 两个定时器（星星 / 陨石生成）
        .insert_resource(pages::game::Score(0))
        .insert_resource(pages::game::StarSpawnTimer(Timer::from_seconds(
            0.8,
            TimerMode::Repeating,
        )))
        .insert_resource(pages::game::MeteorSpawnTimer(Timer::from_seconds(
            2.0,
            TimerMode::Repeating,
        )))
        // 启动时执行一次：生成 2D 相机
        .add_systems(Startup, setup_camera)
        // === Menu 页 ===
        .add_systems(
            OnEnter(GameState::Menu),
            (pages::menu::setup_menu, log_state_enter),
        )
        .add_systems(OnExit(GameState::Menu), pages::menu::cleanup_menu)
        .add_systems(
            Update,
            (
                pages::menu::handle_menu_buttons,
                pages::menu::button_hover_effects,
            )
                .run_if(in_state(GameState::Menu)),
        )
        // === Playing 页 ===
        .add_systems(
            OnEnter(GameState::Playing),
            (pages::game::setup_game, log_state_enter),
        )
        .add_systems(OnExit(GameState::Playing), pages::game::cleanup_game)
        // 游戏主循环：按依赖顺序 .chain() 执行
        .add_systems(
            Update,
            (
                pages::game::player_movement_system,
                pages::game::player_trail_system,
                pages::game::update_particles,
                pages::game::star_spawn_system,
                pages::game::star_animate_system,
                pages::game::meteor_spawn_system,
                pages::game::meteor_move_system,
                pages::game::collect_stars_system,
                pages::game::meteor_hit_system,
                pages::game::camera_follow_system,
                pages::game::parallax_system,
                pages::game::esc_to_menu,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        )
        // 得分变化时更新 UI（独立系统，与主循环并行）
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
                pages::gameover::handle_buttons.run_if(in_state(GameState::GameOver)),
                pages::gameover::button_hover_effects.run_if(in_state(GameState::GameOver)),
            ),
        )
        .run()
}

// === 相机 ===
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

// === 状态切换日志 ===
fn log_state_enter(state: Res<State<GameState>>) {
    info!("[状态] 进入 {:?}", state.get());
}
