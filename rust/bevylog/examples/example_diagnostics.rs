//! Bevy 0.19 入门示例：演示诊断系统（FPS / 帧时间统计）。
//!
//! 学习重点：
//! - FrameTimeDiagnosticsPlugin 注册 FPS / 帧时间 / 帧数三个诊断指标
//! - DiagnosticsStore 资源读取诊断数据
//! - Diagnostic::smoothed 获取指数平滑后的值（比瞬时值更稳定）
//!
//! 观察：屏幕实时显示 FPS 和帧时间。

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::text::FontSourceTemplate;

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

#[derive(Component, Clone, Default)]
struct FpsText;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, update_fps_text)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn_scene(bsn! {
        FpsText
        Text2d::new("FPS: --")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(28.0),
        }
        Transform::from_xyz(0.0, 200.0, 0.0)
    });
}

fn update_fps_text(diagnostics: Res<DiagnosticsStore>, mut q: Query<&mut Text2d, With<FpsText>>) {
    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|d| d.smoothed())
        .unwrap_or(0.0);

    let frame_time = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
        .and_then(|d| d.smoothed())
        .unwrap_or(0.0);

    for mut text in &mut q {
        text.0 = format!("FPS: {fps:.1} | 帧时间: {frame_time:.2} ms");
    }
}
