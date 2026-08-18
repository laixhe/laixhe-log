//! Bevy 0.19 入门示例：演示 UI 溢出裁剪（Overflow）。
//!
//! 学习重点：
//! - Overflow::visible：溢出内容可见（默认）
//! - Overflow::clip：裁剪溢出内容（不改变布局）
//! - Overflow::hidden：裁剪溢出内容（影响布局）
//!
//! 观察：三个固定尺寸的容器里各放一个更大的方块，分别显示溢出、裁剪、隐藏效果。

use bevy::prelude::*;

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

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
            parent.spawn((
                Text::new("UI 溢出裁剪"),
                TextColor(Color::WHITE),
                TextFont {
                    font: FontSource::Handle(asset_server.load(FONT_PATH)),
                    font_size: FontSize::Px(22.0),
                    ..default()
                },
            ));

            // 三个容器并排，分别用 visible / clip / hidden
            parent
                .spawn(Node {
                    column_gap: px(24),
                    ..default()
                })
                .with_children(|row| {
                    for (label, overflow) in [
                        ("visible", Overflow::visible()),
                        ("clip", Overflow::clip()),
                        ("hidden", Overflow::hidden()),
                    ] {
                        row.spawn((
                            Node {
                                width: px(120),
                                height: px(120),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                overflow,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.12, 0.12, 0.15)),
                        ))
                        .with_children(|container| {
                            // 超出容器的大方块
                            container.spawn((
                                Node {
                                    width: px(180),
                                    height: px(180),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.3, 0.6, 0.9)),
                            ));

                            container.spawn((
                                Text::new(label),
                                TextColor(Color::WHITE),
                                TextFont {
                                    font: FontSource::Handle(asset_server.load(FONT_PATH)),
                                    font_size: FontSize::Px(18.0),
                                    ..default()
                                },
                            ));
                        });
                    }
                });
        });
}
