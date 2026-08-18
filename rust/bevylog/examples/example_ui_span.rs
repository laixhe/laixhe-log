//! Bevy 0.19 入门示例：演示富文本（TextSpan，多段不同样式）。
//!
//! 学习重点：
//! - TextSpan：Text 的子节点，按顺序追加文本到父 Text
//! - 每个 TextSpan 拥有独立的 TextColor / TextFont
//! - 用 TextSpan 在一段文本里混合不同颜色、字号
//!
//! 观察：一行文字里「红色」「大号」「蓝色」各用不同样式拼接。

use bevy::prelude::*;
use bevy::text::{FontSource, TextSpan};

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
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            // 父 Text 提供基础文本，子 TextSpan 依次追加
            parent
                .spawn((
                    Text::new("富文本："),
                    TextColor(Color::WHITE),
                    TextFont {
                        font: font.clone(),
                        font_size: FontSize::Px(24.0),
                        ..default()
                    },
                ))
                .with_children(|text| {
                    // 红色 span
                    text.spawn((
                        TextSpan::new("红色"),
                        TextColor(Color::srgb(0.9, 0.2, 0.2)),
                        TextFont {
                            font: font.clone(),
                            font_size: FontSize::Px(24.0),
                            ..default()
                        },
                    ));

                    // 大号 span
                    text.spawn((
                        TextSpan::new(" + 大号"),
                        TextColor(Color::srgb(0.9, 0.7, 0.3)),
                        TextFont {
                            font: font.clone(),
                            font_size: FontSize::Px(36.0),
                            ..default()
                        },
                    ));

                    // 蓝色 span
                    text.spawn((
                        TextSpan::new(" + 蓝色"),
                        TextColor(Color::srgb(0.3, 0.5, 0.9)),
                        TextFont {
                            font: font.clone(),
                            font_size: FontSize::Px(24.0),
                            ..default()
                        },
                    ));
                });
        });
}
