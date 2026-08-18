//! Bevy 0.19 入门示例：演示 2D 世界空间文本（Text2d）。
//!
//! 学习重点：
//! - Text2d：在 2D 世界空间渲染文本（区别于 UI 的 Text）
//! - TextColor / TextFont 控制颜色与字体
//! - Text2dShadow 给 2D 文本加阴影
//! - Transform 控制文本位置与旋转
//!
//! 观察：屏幕上显示多段 2D 文本，含阴影、旋转、不同字号。

use bevy::prelude::*;
use bevy::sprite::Text2dShadow;
use bevy::text::FontSource;

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.08)))
        .add_systems(Startup, setup)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let font = FontSource::Handle(asset_server.load(FONT_PATH));

    // 大号标题
    commands.spawn((
        Text2d::new("2D 世界空间文本"),
        TextColor(Color::srgb(1.0, 0.9, 0.5)),
        TextFont {
            font: font.clone(),
            font_size: FontSize::Px(40.0),
            ..default()
        },
        Transform::from_xyz(0.0, 140.0, 0.0),
    ));

    // 带阴影的文本
    commands.spawn((
        Text2d::new("带阴影的文字"),
        TextColor(Color::srgb(0.9, 0.7, 0.3)),
        TextFont {
            font: font.clone(),
            font_size: FontSize::Px(32.0),
            ..default()
        },
        Text2dShadow {
            offset: Vec2::new(4.0, -4.0),
            color: Color::srgba(0.9, 0.2, 0.2, 0.8),
        },
        Transform::from_xyz(0.0, 60.0, 0.0),
    ));

    // 旋转的文本
    commands.spawn((
        Text2d::new("旋转的文本"),
        TextColor(Color::srgb(0.5, 0.8, 0.5)),
        TextFont {
            font: font.clone(),
            font_size: FontSize::Px(28.0),
            ..default()
        },
        Transform::from_xyz(-120.0, -60.0, 0.0).with_rotation(Quat::from_rotation_z(-0.4)),
    ));

    // 小号多色文本
    commands.spawn((
        Text2d::new("小号文字"),
        TextColor(Color::srgb(0.5, 0.6, 0.9)),
        TextFont {
            font: font.clone(),
            font_size: FontSize::Px(20.0),
            ..default()
        },
        Transform::from_xyz(130.0, -60.0, 0.0),
    ));
}
