//! Bevy 0.19 入门示例：演示窗口事件（WindowEvent）。
//!
//! 窗口的各种状态变化（缩放、焦点、移动鼠标、请求关闭等）会以 WindowEvent 消息发送，
//! 系统用 MessageReader<WindowEvent> 读取并响应。
//!
//! 学习重点：
//! - WindowEvent 是 Message，用 MessageReader 读取
//! - WindowResized / WindowFocused / WindowCloseRequested 等变体
//! - CursorMoved 携带鼠标位置（事件量很大，注意节流）

use bevy::window::WindowEvent;
use bevy::{prelude::*, text::FontSourceTemplate};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        .add_systems(Update, read_window_events)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn_scene(bsn! {
        Text2d::new("窗口事件：缩放窗口 / 切换焦点 / 移动鼠标（观察日志）")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(24.0),
        }
        Transform::from_xyz(0.0, -260.0, 0.0)
    });
}

// 读取窗口事件并打印（WindowResized / WindowFocused / WindowCloseRequested 触发频率低）
fn read_window_events(mut events: MessageReader<WindowEvent>) {
    for event in events.read() {
        match event {
            WindowEvent::WindowResized(e) => {
                info!("[窗口] 尺寸变化：{} x {}", e.width, e.height);
            }
            WindowEvent::WindowFocused(e) => {
                info!("[窗口] 焦点变化：{}", e.focused);
            }
            WindowEvent::WindowCloseRequested(_) => {
                info!("[窗口] 收到关闭窗口请求");
            }
            // CursorMoved 每移动一下鼠标就发一条，事件量很大，这里不打印以免刷屏
            _ => {}
        }
    }
}
