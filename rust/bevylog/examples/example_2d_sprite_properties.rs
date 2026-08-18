//! Bevy 0.19 入门示例：演示 Sprite 组件的高级属性。
//!
//! 同一个图片，通过调整 Sprite 字段展示不同渲染效果：
//! - flip_x / flip_y：水平 / 垂直翻转
//! - color：颜色着色（tint）
//! - custom_size：自定义渲染尺寸（覆盖原图尺寸）
//! - Anchor：锚点组件（Bevy 0.19 是独立组件，不是 Sprite 字段）
//!
//! 学习重点：
//! - Sprite 组件的常用字段
//! - Anchor(pub Vec2) 是独立组件，控制「以 Transform 为中心」的偏移
//! - Sprite::from_color 生成纯色方块（无需图片）

use bevy::{prelude::*, sprite::Anchor, text::FontSourceTemplate};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let image = asset_server.load("images/bevy_bird_dark.png");

    // 第一行：用同一张鸟图展示翻转 / 着色 / 缩放
    spawn_label(&mut commands, "原始", -320.0, 140.0);
    commands.spawn((
        Sprite::from_image(image.clone()),
        Transform::from_xyz(-320.0, 60.0, 0.0),
    ));

    spawn_label(&mut commands, "flip_x", -160.0, 140.0);
    commands.spawn((
        Sprite {
            image: image.clone(),
            flip_x: true,
            ..default()
        },
        Transform::from_xyz(-160.0, 60.0, 0.0),
    ));

    spawn_label(&mut commands, "flip_y", 0.0, 140.0);
    commands.spawn((
        Sprite {
            image: image.clone(),
            flip_y: true,
            ..default()
        },
        Transform::from_xyz(0.0, 60.0, 0.0),
    ));

    spawn_label(&mut commands, "color 着色", 160.0, 140.0);
    commands.spawn((
        Sprite {
            image: image.clone(),
            color: Color::srgb(1.0, 0.4, 0.4),
            ..default()
        },
        Transform::from_xyz(160.0, 60.0, 0.0),
    ));

    spawn_label(&mut commands, "custom_size 放大", 320.0, 140.0);
    commands.spawn((
        Sprite {
            image: image.clone(),
            custom_size: Some(Vec2::new(160.0, 160.0)),
            ..default()
        },
        Transform::from_xyz(320.0, 60.0, 0.0),
    ));

    // 第二行：用纯色方块展示 Anchor 锚点的作用。
    // 三个方块 Transform 都在同一 x 位置，但锚点不同，导致方块偏移方向不同。
    spawn_label(&mut commands, "Anchor::BOTTOM_LEFT", -200.0, -80.0);
    commands.spawn((
        Sprite::from_color(Color::srgb(0.9, 0.7, 0.2), Vec2::new(60.0, 60.0)),
        Anchor::BOTTOM_LEFT,
        Transform::from_xyz(-200.0, -160.0, 0.0),
    ));

    spawn_label(&mut commands, "Anchor::CENTER", 0.0, -80.0);
    commands.spawn((
        Sprite::from_color(Color::srgb(0.3, 0.8, 0.4), Vec2::new(60.0, 60.0)),
        Anchor::CENTER,
        Transform::from_xyz(0.0, -160.0, 0.0),
    ));

    spawn_label(&mut commands, "Anchor::TOP_RIGHT", 200.0, -80.0);
    commands.spawn((
        Sprite::from_color(Color::srgb(0.3, 0.5, 0.9), Vec2::new(60.0, 60.0)),
        Anchor::TOP_RIGHT,
        Transform::from_xyz(200.0, -160.0, 0.0),
    ));

    commands.spawn_scene(bsn! {
        Text2d::new("Sprite 高级属性")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(30.0),
        }
        Transform::from_xyz(0.0, 220.0, 0.0)
    });
}

// 生成一个居中的文字标签
fn spawn_label(commands: &mut Commands, text: &'static str, x: f32, y: f32) {
    commands.spawn_scene(bsn! {
        Text2d::new(text)
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(16.0),
        }
        Transform::from_xyz(x, y, 0.0)
    });
}
