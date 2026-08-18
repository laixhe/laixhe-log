//! Bevy 0.19 组合学习示例：3D 综合小游戏「3D 收集金币」。
//! Menu → Playing → GameOver 三态小游戏：第三人称控制角色跳跃移动，收集场地金币，
//! 在倒计时结束前集满 10 枚获胜，超时则失败。
//!
//! 学习重点（组合了 3D 阶段的多个核心概念）：
//! - 3D 场景：Camera3d + DirectionalLight 方向光 + Plane3d 地面 + PBR StandardMaterial
//! - PBR 材质：metallic=1 / low roughness 的金属金币，产生镜面高光
//! - 第三人称相机：yaw 推导 forward/right，相机始终在角色「后方 + 上方」并 look_at
//! - 重力 + 跳跃：Velocity(Vec3) 速度积分，重力加速度作用于 y，地面 clamp + 起跳
//! - 3D 碰撞：球心距离检测收集金币
//! - 动画：金币绕 y 轴旋转（Transform::rotate_y）
//! - 随机生成：rand 随机地面坐标生成金币
//! - UI 叠加：屏幕空间 Node 显示得分 + 倒计时
//! - 状态机 + 音频：页面切换 + AudioPlayer 收集音效
//!
//! 操作：WASD 移动 | 空格跳跃 | Q/E 转身 | 集满 10 枚金币获胜 | ESC 返回菜单。

use bevy::prelude::*;

mod pages;
use pages::router::GameState;

// === 主函数：应用初始化 ===
fn main() -> AppExit {
    App::new()
        // 配置窗口标题
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "3D 收集金币 Coin Collector".to_string(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.1, 0.12, 0.18)))
        // 页面状态机（默认进入 Menu）
        .init_state::<GameState>()
        // 全局资源：得分 / 角色朝向 / 胜负标记 / 倒计时
        .insert_resource(pages::game::Score(0))
        .insert_resource(pages::game::PlayerYaw(0.0))
        .init_resource::<pages::game::WinFlag>()
        .insert_resource(pages::game::GameTimer(Timer::from_seconds(
            pages::game::GAME_TIME,
            TimerMode::Once,
        )))
        // 启动时执行一次：生成常驻 2D 相机，负责渲染所有页面的 UI（菜单/游戏 HUD/结算页）。
        // Playing 页的 3D 相机在 setup_game 中单独创建（带 GameRoot 标记，退出时一并清理）。
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
                pages::game::move_player,
                pages::game::apply_physics,
                pages::game::rotate_coins,
                pages::game::collect_coins,
                pages::game::game_timer_system,
                pages::game::follow_camera,
                pages::game::esc_to_menu,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        )
        // HUD 更新（独立系统，与主循环并行）
        .add_systems(
            Update,
            pages::game::update_score_display
                .run_if(in_state(GameState::Playing))
                .run_if(resource_changed::<pages::game::Score>),
        )
        .add_systems(
            Update,
            pages::game::update_timer_display.run_if(in_state(GameState::Playing)),
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
// 常驻 2D 相机：渲染 UI（Node/Text）。不挂页面标记，因此不会被任何 cleanup 清理，
// 三个页面（Menu / Playing HUD / GameOver）都能用它显示界面。
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

// === 状态切换日志 ===
fn log_state_enter(state: Res<State<GameState>>) {
    info!("[状态] 进入 {:?}", state.get());
}
