//! Bevy 0.19 入门示例：演示 UI 渐变背景（Gradient）。
//!
//! 学习重点：
//! - `BackgroundGradient`：替代纯色背景，用渐变填充 Node
//! - `LinearGradient`：线性渐变（角度 + 颜色停靠点）
//! - `RadialGradient`：径向渐变（中心 + 形状 + 颜色停靠点）
//! - `ConicGradient`：锥形渐变（角度颜色停靠点）
//! - `ColorStop` / `AngularColorStop`：定义渐变的颜色停靠点
//!
//! 观察：三个卡片分别显示线性 / 径向 / 锥形渐变效果。

use bevy::prelude::*;
use bevy::text::FontSource;
use std::f32::consts::TAU;

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
            let font = |size: f32| TextFont {
                font: FontSource::Handle(asset_server.load(FONT_PATH)),
                font_size: FontSize::Px(size),
                ..default()
            };

            parent.spawn((
                Text::new("UI 渐变背景"),
                TextColor(Color::WHITE),
                font(24.0),
            ));

            // 三个渐变卡片横向排列
            parent
                .spawn(Node {
                    column_gap: px(20),
                    ..default()
                })
                .with_children(|row| {
                    // ---- 线性渐变：从左到右 红 -> 蓝 ----
                    row.spawn((
                        Node {
                            width: px(160),
                            height: px(160),
                            border_radius: BorderRadius::all(px(12)),
                            ..default()
                        },
                        BackgroundGradient::from(LinearGradient::to_right(vec![
                            ColorStop::percent(Color::srgb(0.90, 0.20, 0.20), 0.0),
                            ColorStop::percent(Color::srgb(0.20, 0.30, 0.90), 100.0),
                        ])),
                    ));

                    // ---- 径向渐变：中心白 -> 边缘紫 ----
                    row.spawn((
                        Node {
                            width: px(160),
                            height: px(160),
                            border_radius: BorderRadius::all(px(12)),
                            ..default()
                        },
                        BackgroundGradient::from(RadialGradient::new(
                            UiPosition::CENTER,
                            RadialGradientShape::ClosestSide,
                            vec![
                                ColorStop::percent(Color::WHITE, 0.0),
                                ColorStop::percent(Color::srgb(0.60, 0.20, 0.80), 100.0),
                            ],
                        )),
                    ));

                    // ---- 锥形渐变：环绕一圈 红 -> 绿 -> 蓝 ----
                    row.spawn((
                        Node {
                            width: px(160),
                            height: px(160),
                            border_radius: BorderRadius::all(px(12)),
                            ..default()
                        },
                        BackgroundGradient::from(ConicGradient::new(
                            UiPosition::CENTER,
                            vec![
                                AngularColorStop::new(Color::srgb(0.90, 0.20, 0.20), 0.0),
                                AngularColorStop::new(Color::srgb(0.20, 0.80, 0.30), TAU / 3.0),
                                AngularColorStop::new(
                                    Color::srgb(0.20, 0.30, 0.90),
                                    2.0 * TAU / 3.0,
                                ),
                                AngularColorStop::new(Color::srgb(0.90, 0.20, 0.20), TAU),
                            ],
                        )),
                    ));
                });
        });
}
