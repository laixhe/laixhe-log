//! Bevy 0.19 入门示例：演示文本边界（TextBounds）与自动换行。
//!
//! 学习重点：
//! - TextBounds::new_horizontal：限定文本最大宽度，超出自动换行
//! - 配合 TextLayout + Justify 控制多行文本对齐
//!
//! 观察：同一段长文本分别用左对齐 / 居中 / 右对齐，在限定宽度内自动换行。

use bevy::prelude::*;
use bevy::text::{FontSource, Justify, TextBounds, TextLayout};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let font = FontSource::Handle(asset_server.load(FONT_PATH));

    commands
        .spawn(Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: px(24),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new("UI 文本边界与换行"),
                TextColor(Color::WHITE),
                TextFont {
                    font: font.clone(),
                    font_size: FontSize::Px(22.0),
                    ..default()
                },
            ));

            let long_text = "这是一段很长的文字，用来演示限定宽度后自动换行的效果。";

            // 左对齐 + 限定宽度换行
            parent.spawn((
                Text::new(long_text),
                TextColor(Color::srgb(0.9, 0.6, 0.4)),
                TextFont {
                    font: font.clone(),
                    font_size: FontSize::Px(18.0),
                    ..default()
                },
                TextBounds::new_horizontal(220.0),
                TextLayout::justify(Justify::Left),
            ));

            // 居中 + 限定宽度换行
            parent.spawn((
                Text::new(long_text),
                TextColor(Color::srgb(0.5, 0.8, 0.5)),
                TextFont {
                    font: font.clone(),
                    font_size: FontSize::Px(18.0),
                    ..default()
                },
                TextBounds::new_horizontal(220.0),
                TextLayout::justify(Justify::Center),
            ));

            // 右对齐 + 限定宽度换行
            parent.spawn((
                Text::new(long_text),
                TextColor(Color::srgb(0.5, 0.6, 0.9)),
                TextFont {
                    font: font.clone(),
                    font_size: FontSize::Px(18.0),
                    ..default()
                },
                TextBounds::new_horizontal(220.0),
                TextLayout::justify(Justify::Right),
            ));
        });
}
