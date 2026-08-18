//! Bevy 0.19 入门示例：演示全屏/窗口模式切换。
//!
//! 学习重点：
//! - Window.mode 字段控制窗口模式（Windowed / Fullscreen / BorderlessFullscreen）
//! - 运行时修改 Window 组件即可切换，无需重启
//! - MonitorSelection / VideoModeSelection 指定全屏所用的显示器与分辨率
//!
//! 操作：按 F11 在全屏和窗口模式之间切换。

use bevy::prelude::*;
use bevy::window::{MonitorSelection, PrimaryWindow, VideoModeSelection, WindowMode};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, toggle_fullscreen)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    info!("[窗口] 按 F11 切换全屏/窗口");
}

fn toggle_fullscreen(
    keys: Res<ButtonInput<KeyCode>>,
    mut window: Single<&mut Window, With<PrimaryWindow>>,
) {
    if !keys.just_pressed(KeyCode::F11) {
        return;
    }

    if window.mode == WindowMode::Windowed {
        window.mode =
            WindowMode::Fullscreen(MonitorSelection::Primary, VideoModeSelection::Current);
        info!("[窗口] 切换到全屏");
    } else {
        window.mode = WindowMode::Windowed;
        info!("[窗口] 切换到窗口");
    }
}
