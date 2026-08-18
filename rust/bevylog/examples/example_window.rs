//! Bevy 0.19 入门示例：演示窗口配置（WindowPlugin / Window）。
//! 设置窗口标题、分辨率、位置、是否可缩放等，并监听窗口尺寸变化。
//!
//! 学习重点：
//! - WindowPlugin 是 DefaultPlugins 的一部分，用 .set(WindowPlugin { ... }) 覆盖默认配置
//! - primary_window 是主窗口配置；设为 None 则不创建窗口
//! - Window 结构体字段：title（标题）、resolution（分辨率）、resizable（是否可缩放）、position（位置）
//! - 这些只是启动时的初始配置；运行中修改要用 Query<&mut Window>

use bevy::window::PrimaryWindow;
use bevy::{prelude::*, text::FontSourceTemplate};

// 中文字体路径（bsn! 里用 FontSourceTemplate 自动加载）
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        // 覆盖默认窗口配置：标题、分辨率、居中、可缩放
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy 窗口配置示例".to_string(),
                resolution: (800, 600).into(),
                resizable: true,
                position: WindowPosition::Centered(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        // 监听窗口尺寸变化
        .add_systems(Update, on_window_resized)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // 提示文本
    commands.spawn_scene(bsn! {
        Text2d::new("拖拽窗口边缘调整大小，观察终端日志")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(28.0),
        }
        Transform::from_xyz(0.0, -200.0, 0.0)
    });
}

// 监听窗口尺寸变化：用 Changed<Window> 只在变化时打印，避免每帧刷屏
fn on_window_resized(window: Single<&Window, (With<PrimaryWindow>, Changed<Window>)>) {
    info!("[窗口] 尺寸变化为 {} x {}", window.width(), window.height());
}
