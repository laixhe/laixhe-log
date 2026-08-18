//! Bevy 0.19 入门示例：演示交互状态（Interaction 三态）。
//!
//! 学习重点：
//! - Interaction::{Pressed, Hovered, None}：按钮的按下 / 悬停 / 无 三种状态
//! - Changed<Interaction>：只在状态切换时触发，用于检测状态转换
//! - 配合 UiTransform 实现按下缩放、悬停高亮
//!
//! 观察：把鼠标移到按钮上、按下、移开，按钮颜色和缩放随之变化，终端打印状态。

use bevy::prelude::*;
use bevy::text::FontSource;

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_interaction)
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
            row_gap: px(16),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new("悬停 / 按下 / 移开试试"),
                TextColor(Color::WHITE),
                TextFont {
                    font: FontSource::Handle(asset_server.load(FONT_PATH)),
                    font_size: FontSize::Px(20.0),
                    ..default()
                },
            ));

            // 按钮：Button 组件自动附带 Interaction 状态
            parent
                .spawn((
                    Button,
                    Node {
                        width: px(200),
                        height: px(90),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        border_radius: BorderRadius::all(px(10)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.25)),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new("交互按钮"),
                        TextColor(Color::WHITE),
                        TextFont {
                            font: FontSource::Handle(asset_server.load(FONT_PATH)),
                            font_size: FontSize::Px(22.0),
                            ..default()
                        },
                    ));
                });
        });
}

// 只在状态切换时触发，更新颜色 + 缩放 + 打印日志
fn handle_interaction(
    mut q: Query<
        (&Interaction, &mut BackgroundColor, &mut UiTransform),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut bg, mut tf) in &mut q {
        match interaction {
            Interaction::Pressed => {
                *bg = BackgroundColor(Color::srgb(0.9, 0.3, 0.3));
                tf.scale = Vec2::splat(0.9);
                info!("[UI交互] Pressed 按下");
            }
            Interaction::Hovered => {
                *bg = BackgroundColor(Color::srgb(0.3, 0.6, 0.9));
                tf.scale = Vec2::splat(1.05);
                info!("[UI交互] Hovered 悬停");
            }
            Interaction::None => {
                *bg = BackgroundColor(Color::srgb(0.2, 0.2, 0.25));
                tf.scale = Vec2::splat(1.0);
                info!("[UI交互] None 无");
            }
        }
    }
}
