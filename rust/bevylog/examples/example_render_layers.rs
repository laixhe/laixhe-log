//! Bevy 0.19 入门示例：演示渲染层（RenderLayers）。
//!
//! RenderLayers 让实体归属到某个渲染层，相机也归属到某个层，
//! 只有「相机层 ∩ 实体层」非空时，实体才会被该相机渲染。
//! 常用于：小地图、UI 覆盖层、分屏渲染、让某相机忽略特定对象等。
//!
//! 学习重点：
//! - RenderLayers::layer(n)：归属到第 n 层
//! - 相机也挂 RenderLayers，决定它渲染哪些层
//! - 多相机叠加：order + ClearColorConfig::None 不清屏

use bevy::camera::visibility::RenderLayers;
use bevy::{prelude::*, text::FontSourceTemplate};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 相机 A：默认渲染 layer 0
    commands.spawn(Camera2d);

    // 相机 B：只渲染 layer 1，叠加在 A 之上（clear_color = None 不清屏）
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        RenderLayers::layer(1),
    ));

    // 红色圆：默认 layer 0，只有相机 A 可见
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(60.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.8, 0.2, 0.2)))),
        Transform::from_xyz(-150.0, 0.0, 0.0),
    ));

    // 绿色圆：layer 1，只有相机 B 可见
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(60.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.2, 0.8, 0.4)))),
        Transform::from_xyz(150.0, 0.0, 0.0),
        RenderLayers::layer(1),
    ));

    commands.spawn_scene(bsn! {
        Text2d::new("RenderLayers：红色=layer0（相机A），绿色=layer1（相机B）")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(24.0),
        }
        Transform::from_xyz(0.0, -260.0, 0.0)
    });
}
