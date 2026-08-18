//! Bevy 0.19 入门示例：演示 UI 缩放（UiScale）。
//!
//! 学习重点：
//! - UiScale：全局 UI 缩放资源，影响所有 Val::Px 固定值的尺寸
//! - 通过 ResMut<UiScale> 在运行时动态缩放整个 UI
//!
//! 操作：按 + / - 键放大 / 缩小 UI（范围 0.5 ~ 2.0）。

use bevy::prelude::*;
use bevy::text::FontSource;

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, adjust_scale)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // 这些固定像素尺寸的 UI 会随 UiScale 整体缩放
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
                Text::new("按 + / - 缩放 UI"),
                TextColor(Color::WHITE),
                TextFont {
                    font: FontSource::Handle(asset_server.load(FONT_PATH)),
                    font_size: FontSize::Px(22.0),
                    ..default()
                },
            ));

            // 固定 160x160 的方块，缩放时尺寸随之变化
            parent.spawn((
                Node {
                    width: px(160),
                    height: px(160),
                    border_radius: BorderRadius::all(px(12)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.3, 0.6, 0.9)),
            ));
        });
}

fn adjust_scale(keys: Res<ButtonInput<KeyCode>>, mut ui_scale: ResMut<UiScale>) {
    if keys.just_pressed(KeyCode::Equal) {
        ui_scale.0 += 0.1;
    }
    if keys.just_pressed(KeyCode::Minus) {
        ui_scale.0 -= 0.1;
    }
    ui_scale.0 = ui_scale.0.clamp(0.5, 2.0);

    if keys.just_pressed(KeyCode::Equal) || keys.just_pressed(KeyCode::Minus) {
        info!("[UI缩放] UiScale = {:.1}", ui_scale.0);
    }
}
