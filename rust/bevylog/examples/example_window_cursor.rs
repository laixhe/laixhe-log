//! Bevy 0.19 入门示例：演示光标捕获与可见性。
//! 通过 `CursorOptions` 组件控制鼠标光标的「是否可见」和「捕获模式」。
//!
//! 学习重点：
//! - Bevy 0.19 把光标配置从 `Window` 拆成了独立组件 `CursorOptions`（`Window` 用 `#[require(CursorOptions)]` 自动挂载）
//! - `CursorOptions.visible`：控制光标是否显示（隐藏后移动鼠标仍能产生位移）
//! - `CursorOptions.grab_mode`：光标捕获模式
//!   - `CursorGrabMode::None`：自由进出窗口（默认）
//!   - `CursorGrabMode::Confined`：光标被限制在窗口内
//!   - `CursorGrabMode::Locked`：光标锁定（FPS 视角常用，光标不可见且相对位移可无限累加）
//! - 用 `Single<&mut CursorOptions, With<PrimaryWindow>>` 定位主窗口的光标配置
//!
//! 操作方式：
//! - V：切换光标可见 / 隐藏
//! - G：循环切换捕获模式（无 → 限制 → 锁定 → 无）

use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};
use bevy::{prelude::*, text::FontSourceTemplate};

// 中文字体路径
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        // .chain() 保证顺序：先处理输入，再更新文本
        .add_systems(Update, (handle_input, update_text).chain())
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn_scene(bsn! {
        Text2d::new("")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(22.0),
        }
        Transform::from_xyz(0.0, -200.0, 0.0)
    });
}

// 处理输入：V 切换可见性，G 循环捕获模式。
fn handle_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut cursor: Single<&mut CursorOptions, With<PrimaryWindow>>,
) {
    if keys.just_pressed(KeyCode::KeyV) {
        cursor.visible = !cursor.visible;
        info!("[光标] 可见性 = {}", cursor.visible);
    }

    if keys.just_pressed(KeyCode::KeyG) {
        cursor.grab_mode = match cursor.grab_mode {
            CursorGrabMode::None => CursorGrabMode::Confined,
            CursorGrabMode::Confined => CursorGrabMode::Locked,
            CursorGrabMode::Locked => CursorGrabMode::None,
        };
        info!("[光标] 捕获模式 = {}", grab_mode_label(cursor.grab_mode));
    }
}

// 把捕获模式转成可读文本（CursorGrabMode 是枚举，直接转字符串更友好）
fn grab_mode_label(mode: CursorGrabMode) -> &'static str {
    match mode {
        CursorGrabMode::None => "无（自由）",
        CursorGrabMode::Confined => "限制在窗口内",
        CursorGrabMode::Locked => "锁定",
    }
}

// 更新提示文本：显示当前光标可见性和捕获模式。
fn update_text(
    cursor: Single<&CursorOptions, With<PrimaryWindow>>,
    mut text: Single<&mut Text2d>,
    mut last: Local<String>,
) {
    let new_text = format!(
        "V：切换可见性  |  G：切换捕获模式  |  可见：{}  |  模式：{}",
        cursor.visible,
        grab_mode_label(cursor.grab_mode)
    );

    // 节流：内容变化才更新文本，避免 CJK 重排刷屏
    if *last != new_text {
        *last = new_text.clone();
        text.0 = new_text;
    }
}
