//! Bevy 0.19 入门示例：演示鼠标光标（CursorIcon 组件）。
//!
//! 通过把 `CursorIcon` 组件挂到窗口实体上，动态改变鼠标指针样式。
//!
//! 学习重点：
//! - CursorIcon 是挂在「窗口实体」上的组件（不是全局资源）
//! - SystemCursorIcon 提供各种系统内置光标（Pointer / Crosshair / Text / Grab 等）
//! - 用 PrimaryWindow 拿到主窗口实体，按数字键切换光标
//!
//! 操作：按数字键 1~6 切换光标样式。

use bevy::prelude::*;
use bevy::window::{CursorIcon, PrimaryWindow, SystemCursorIcon};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, change_cursor)
        .run()
}

// 初始光标：默认箭头
fn setup(mut commands: Commands, window: Single<Entity, With<PrimaryWindow>>) {
    commands
        .entity(*window)
        .insert(CursorIcon::System(SystemCursorIcon::Default));
}

fn change_cursor(
    keys: Res<ButtonInput<KeyCode>>,
    window: Single<Entity, With<PrimaryWindow>>,
    mut commands: Commands,
) {
    // 按数字键 1~6 切换不同光标
    let icon = if keys.just_pressed(KeyCode::Digit1) {
        Some((SystemCursorIcon::Pointer, "Pointer 手型"))
    } else if keys.just_pressed(KeyCode::Digit2) {
        Some((SystemCursorIcon::Crosshair, "Crosshair 十字"))
    } else if keys.just_pressed(KeyCode::Digit3) {
        Some((SystemCursorIcon::Text, "Text 文本 I 型"))
    } else if keys.just_pressed(KeyCode::Digit4) {
        Some((SystemCursorIcon::Grab, "Grab 抓取"))
    } else if keys.just_pressed(KeyCode::Digit5) {
        Some((SystemCursorIcon::Grabbing, "Grabbing 抓取中"))
    } else if keys.just_pressed(KeyCode::Digit6) {
        Some((SystemCursorIcon::NotAllowed, "NotAllowed 禁止"))
    } else {
        None
    };

    if let Some((icon, name)) = icon {
        commands.entity(*window).insert(CursorIcon::System(icon));
        info!("[光标] 切换到 {name}");
    }
}
