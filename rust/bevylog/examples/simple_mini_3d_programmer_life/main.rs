//! 程序员求职生存模拟
//!
//! 3D 45° 俯视低模养成游戏：扮演计算机专业学生，从大三暑期找实习开始，
//! 投简历、笔试、面试、通勤、实习转正、秋招、入职，一路走到职场日常。
//!
//! 技术栈：Bevy 0.19 + Rust edition 2024，参考 examples 目录用例。
//! 本示例全部用「命令式」API 构建场景与 UI（commands.spawn + with_children），
//! 未使用 Bevy 0.19 的声明式场景宏 bsn!，方便新手按普通 Rust 代码阅读。

mod game;
mod gameover;
mod menu;
mod router;

use bevy::prelude::*;
use bevy::window::{MonitorSelection, WindowPosition};

use router::GameState;

fn main() -> AppExit {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "程序员求职生存模拟".to_string(),
                        resolution: (1280, 720).into(),
                        resizable: true,
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    // 用绝对路径，保证无论从哪个目录启动都能找到资源
                    file_path: concat!(env!("CARGO_MANIFEST_DIR"), "/assets").to_string(),
                    ..default()
                }),
        )
        // 纸张底色
        .insert_resource(ClearColor(Color::srgb(0.93, 0.88, 0.76)))
        // 页面状态机
        .init_state::<GameState>()
        .add_plugins(game::GamePlugin)
        // 常驻 2D 相机：渲染所有页面的 UI
        .add_systems(Startup, setup_camera)
        // === Menu 页 ===
        .add_systems(OnEnter(GameState::Menu), menu::setup_menu)
        .add_systems(OnExit(GameState::Menu), menu::cleanup_menu)
        .add_systems(
            Update,
            menu::handle_menu_buttons.run_if(in_state(GameState::Menu)),
        )
        // === GameOver 页 ===
        .add_systems(OnEnter(GameState::GameOver), gameover::setup_game_over)
        .add_systems(OnExit(GameState::GameOver), gameover::cleanup_game_over)
        .add_systems(
            Update,
            gameover::handle_buttons.run_if(in_state(GameState::GameOver)),
        )
        .run()
}

fn setup_camera(mut commands: Commands) {
    // 常驻 2D 相机渲染 UI：order 高于 3D 相机，且不清理颜色（保留 3D 画面）
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
    ));
}
