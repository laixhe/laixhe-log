//! Bevy 0.19 入门示例：演示文本样式（阴影 / 背景色 / 对齐 / 行高 / 字间距）。
//!
//! 学习重点：
//! - TextShadow：文字阴影（偏移 + 颜色）
//! - TextBackgroundColor：文字背景色
//! - TextLayout + Justify：多行文本对齐（居中 / 右对齐等）
//! - LineHeight：行高；LetterSpacing：字间距
//!
//! 观察：几段文本分别展示不同的排版样式。

use bevy::prelude::*;
use bevy::text::{FontSource, Justify, LetterSpacing, LineHeight, TextLayout};

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
            row_gap: px(20),
            ..default()
        })
        .with_children(|parent| {
            // 文字阴影
            parent.spawn((
                Text::new("带阴影的文字"),
                TextColor(Color::srgb(1.0, 0.9, 0.5)),
                TextFont {
                    font: font.clone(),
                    font_size: FontSize::Px(28.0),
                    ..default()
                },
                TextShadow {
                    offset: Vec2::new(3.0, 3.0),
                    color: Color::srgba(0.9, 0.2, 0.2, 0.8),
                },
            ));

            // 文字背景色
            parent.spawn((
                Text::new("带背景色的文字"),
                TextColor(Color::WHITE),
                TextFont {
                    font: font.clone(),
                    font_size: FontSize::Px(24.0),
                    ..default()
                },
                TextBackgroundColor(Color::srgb(0.25, 0.4, 0.85)),
            ));

            // 多行文本居中
            parent.spawn((
                Text::new("第一行\n第二行更长\n第三行"),
                TextColor(Color::WHITE),
                TextFont {
                    font: font.clone(),
                    font_size: FontSize::Px(20.0),
                    ..default()
                },
                TextLayout::justify(Justify::Center),
            ));

            // 行高 + 字间距
            parent.spawn((
                Text::new("大行高 + 宽字间距"),
                TextColor(Color::srgb(0.6, 0.9, 0.7)),
                TextFont {
                    font: font.clone(),
                    font_size: FontSize::Px(22.0),
                    ..default()
                },
                LineHeight::Px(40.0),
                LetterSpacing::Px(6.0),
            ));
        });
}
