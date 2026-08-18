//! Bevy 0.19 入门示例：演示 2D 图元集合（Mesh2d + 各种形状）。
//!
//! 学习重点：
//! - Circle / Rectangle / RegularPolygon / Capsule2d / Ellipse / Annulus / Triangle2d
//! - 用 Mesh2d + MeshMaterial2d 渲染各种 2D 形状
//! - Text2d 在 2D 世界空间显示文字标签
//!
//! 观察：屏幕上整齐排列多种 2D 图元，每种带文字标签。

use bevy::prelude::*;
use bevy::text::FontSource;

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.06, 0.06, 0.08)))
        .add_systems(Startup, setup)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);

    let font = FontSource::Handle(asset_server.load(FONT_PATH));

    // 各种 2D 图元（名字 + 网格句柄）
    let shapes: Vec<(&str, Handle<Mesh>)> = vec![
        ("圆", meshes.add(Circle::new(45.0))),
        ("矩形", meshes.add(Rectangle::new(110.0, 70.0))),
        ("五边形", meshes.add(RegularPolygon::new(50.0, 5))),
        ("胶囊", meshes.add(Capsule2d::new(18.0, 50.0))),
        ("椭圆", meshes.add(Ellipse::new(55.0, 32.0))),
        ("环形", meshes.add(Annulus::new(28.0, 48.0))),
        (
            "三角形",
            meshes.add(Triangle2d::new(
                Vec2::new(-45.0, -30.0),
                Vec2::new(45.0, -30.0),
                Vec2::new(0.0, 40.0),
            )),
        ),
    ];

    let colors = [
        Color::srgb(0.9, 0.4, 0.4),
        Color::srgb(0.4, 0.7, 0.4),
        Color::srgb(0.4, 0.5, 0.9),
        Color::srgb(0.9, 0.7, 0.3),
        Color::srgb(0.7, 0.4, 0.9),
        Color::srgb(0.4, 0.9, 0.7),
        Color::srgb(0.9, 0.5, 0.6),
    ];

    for (i, (name, mesh)) in shapes.iter().enumerate() {
        // 4 列网格布局
        let x = (i as f32 % 4.0) * 170.0 - 255.0;
        let y = -(i as f32 / 4.0) * 170.0 + 85.0;

        commands.spawn((
            Mesh2d(mesh.clone()),
            MeshMaterial2d(materials.add(ColorMaterial::from(colors[i]))),
            Transform::from_xyz(x, y, 0.0),
        ));

        commands.spawn((
            Text2d::new(*name),
            TextColor(Color::WHITE),
            TextFont {
                font: font.clone(),
                font_size: FontSize::Px(18.0),
                ..default()
            },
            Transform::from_xyz(x, y - 60.0, 0.0),
        ));
    }
}
