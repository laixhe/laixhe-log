//! Bevy 0.19 入门示例：演示 UI 图片节点（ImageNode）。
//!
//! 学习重点：
//! - ImageNode：在 UI 中显示图片（require Node）
//! - NodeImageMode::Stretch / Tiled：拉伸填充 / 平铺填充
//! - flip_x / flip_y：翻转图片
//! - color：图片着色（tint，与原图颜色相乘）
//! - ImageNode::solid_color：纯色填充（调试 / 占位用）
//!
//! 观察：同一张图片分别以拉伸、平铺、翻转、着色、纯色几种方式展示。

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

    let texture = asset_server.load("images/bevy_logo.png");

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
            // 标题
            parent.spawn((
                Text::new("UI 图片节点（ImageNode）"),
                TextColor(Color::WHITE),
                TextFont {
                    font: FontSource::Handle(asset_server.load(FONT_PATH)),
                    font_size: FontSize::Px(22.0),
                    ..default()
                },
            ));

            // 第一行：拉伸 / 平铺 / 翻转
            parent
                .spawn(Node {
                    column_gap: px(16),
                    ..default()
                })
                .with_children(|row| {
                    // 拉伸：忽略原图比例，铺满节点
                    row.spawn((
                        ImageNode {
                            image: texture.clone(),
                            image_mode: NodeImageMode::Stretch,
                            ..default()
                        },
                        Node {
                            width: px(120),
                            height: px(120),
                            ..default()
                        },
                    ));

                    // 平铺：重复填充
                    row.spawn((
                        ImageNode {
                            image: texture.clone(),
                            image_mode: NodeImageMode::Tiled {
                                tile_x: true,
                                tile_y: true,
                                stretch_value: 1.0,
                            },
                            ..default()
                        },
                        Node {
                            width: px(120),
                            height: px(120),
                            ..default()
                        },
                    ));

                    // 水平翻转 + 拉伸
                    row.spawn((
                        ImageNode {
                            image: texture.clone(),
                            image_mode: NodeImageMode::Stretch,
                            flip_x: true,
                            ..default()
                        },
                        Node {
                            width: px(120),
                            height: px(120),
                            ..default()
                        },
                    ));
                });

            // 第二行：着色 / 纯色 / 原始
            parent
                .spawn(Node {
                    column_gap: px(16),
                    ..default()
                })
                .with_children(|row| {
                    // 着色（tint）：绿色滤镜
                    row.spawn((
                        ImageNode {
                            image: texture.clone(),
                            image_mode: NodeImageMode::Stretch,
                            color: Color::srgb(0.3, 0.9, 0.5),
                            ..default()
                        },
                        Node {
                            width: px(120),
                            height: px(120),
                            ..default()
                        },
                    ));

                    // 纯色填充：不需要图片资产
                    row.spawn((
                        ImageNode::solid_color(Color::srgb(0.3, 0.5, 0.9)),
                        Node {
                            width: px(120),
                            height: px(120),
                            ..default()
                        },
                    ));

                    // 原始图片：Auto 按原图尺寸显示
                    row.spawn((
                        ImageNode::new(texture.clone()),
                        Node {
                            width: px(120),
                            height: px(120),
                            ..default()
                        },
                    ));
                });
        });
}
