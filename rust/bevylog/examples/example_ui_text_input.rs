//! Bevy 0.19 入门示例：演示文本输入（EditableText 可编辑文本组件）。
//! 一个可编辑文本框，点击聚焦后输入文字，按回车提交并清空。
//!
//! 学习重点：
//! - EditableText：Bevy 0.19 新增的可编辑文本组件（文本框 widget）
//! - 配合 Node / TextFont / TextColor / TextCursorStyle 使用
//! - InputFocus 资源：记录当前聚焦的实体（点击文本框可聚焦）
//! - 读取用 input.value()，清空用 input.clear()

use bevy::input_focus::InputFocus;
use bevy::prelude::*;
use bevy::text::{EditableText, FontSource, FontSourceTemplate, TextCursorStyle};

// 中文字体路径：bsn! 用 FontSourceTemplate 自动加载；普通 spawn 用 FontSource::Handle + asset_server.load
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, on_submit)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // 根 UI 节点：全屏居中布局，把输入框放到屏幕中央
    commands
        .spawn(Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            // 可编辑文本框：带边框的输入框
            parent.spawn((
                Node {
                    width: px(300),
                    height: px(50),
                    border: UiRect::all(px(2)),
                    padding: UiRect::all(px(8)),
                    ..default()
                },
                BorderColor::from(Color::WHITE),
                BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                EditableText::default(),
                TextFont {
                    font: FontSource::Handle(asset_server.load(FONT_PATH)),
                    font_size: FontSize::Px(24.0),
                    ..default()
                },
                TextColor(Color::WHITE),
                TextCursorStyle::default(),
            ));
        });

    // 提示文本（世界坐标，显示在输入框上方）
    commands.spawn_scene(bsn! {
        Text2d::new("点击下方输入框输入文字，按回车提交")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(24.0),
        }
        Transform::from_xyz(0.0, 120.0, 0.0)
    });
}

// 按回车提交：读取并清空当前聚焦的输入框内容
fn on_submit(
    input_focus: Res<InputFocus>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut inputs: Query<&mut EditableText>,
) {
    // let 链（edition 2024）：if 条件里用 && 连接 let 模式匹配
    if keyboard.just_pressed(KeyCode::Enter)
        && let Some(entity) = input_focus.get()
        && let Ok(mut input) = inputs.get_mut(entity)
    {
        info!("[文本输入] 提交：{}", input.value());
        input.clear();
    }
}
