//! Bevy 0.19 入门示例：演示 UI 阴影（BoxShadow）与描边（Outline）。
//!
//! 学习重点：
//! - BoxShadow::new：给 Node 添加单个阴影（颜色 + 偏移 + 扩散 + 模糊）
//! - BoxShadow(vec![ShadowStyle, ...])：叠加多层阴影（如发光效果）
//! - Outline：在节点外描边（不占布局空间）
//!
//! 观察：三张卡片分别显示普通阴影、多层发光、描边效果。

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
            row_gap: px(24),
            ..default()
        })
        .with_children(|parent| {
            // 标题
            parent.spawn((
                Text::new("UI 阴影 / 描边"),
                TextColor(Color::WHITE),
                TextFont {
                    font: FontSource::Handle(asset_server.load(FONT_PATH)),
                    font_size: FontSize::Px(22.0),
                    ..default()
                },
            ));

            // 三张卡片横向排列
            parent
                .spawn(Node {
                    column_gap: px(40),
                    ..default()
                })
                .with_children(|row| {
                    // 普通阴影：右下偏移 + 模糊
                    row.spawn((
                        Node {
                            width: px(140),
                            height: px(140),
                            border_radius: BorderRadius::all(px(12)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.25, 0.40, 0.85)),
                        BoxShadow::new(
                            Color::srgba(0.0, 0.0, 0.0, 0.6),
                            px(8.0),  // x 偏移
                            px(8.0),  // y 偏移
                            px(0.0),  // 扩散半径
                            px(16.0), // 模糊半径
                        ),
                    ));

                    // 多层阴影：粉色 + 蓝色发光
                    row.spawn((
                        Node {
                            width: px(140),
                            height: px(140),
                            border_radius: BorderRadius::all(px(12)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.85, 0.40, 0.60)),
                        BoxShadow(vec![
                            ShadowStyle {
                                color: Color::srgba(0.9, 0.3, 0.5, 0.5),
                                x_offset: px(0.0),
                                y_offset: px(0.0),
                                spread_radius: px(6.0),
                                blur_radius: px(20.0),
                            },
                            ShadowStyle {
                                color: Color::srgba(0.3, 0.5, 0.9, 0.5),
                                x_offset: px(0.0),
                                y_offset: px(0.0),
                                spread_radius: px(0.0),
                                blur_radius: px(40.0),
                            },
                        ]),
                    ));

                    // 描边：外发光边框，不占布局空间
                    row.spawn((
                        Node {
                            width: px(140),
                            height: px(140),
                            border_radius: BorderRadius::all(px(12)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.20, 0.70, 0.50)),
                        Outline::new(px(4.0), px(6.0), Color::srgb(0.9, 0.9, 0.3)),
                    ));
                });
        });
}
