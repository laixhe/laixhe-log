//! Bevy 0.19 入门示例：演示自定义 2D 网格（从顶点数据构造 Mesh2d）。
//!
//! 学习重点：
//! - Mesh::new(PrimitiveTopology::TriangleList, ...) 创建空网格
//! - insert_attribute 填充顶点位置（2D 顶点 z = 0）
//! - Mesh2d + MeshMaterial2d 挂载网格和颜色材质
//!
//! 观察：左侧是自定义三角形，右侧是用两个三角形拼成的四边形。

use bevy::asset::RenderAssetUsages;
use bevy::mesh::PrimitiveTopology;
use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.08)))
        .add_systems(Startup, setup)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // 自定义三角形（3 个顶点，z 均为 0）
    let mut triangle = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    triangle.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [-60.0, -40.0, 0.0], // 左下
            [60.0, -40.0, 0.0],  // 右下
            [0.0, 60.0, 0.0],    // 顶部
        ],
    );

    commands.spawn((
        Mesh2d(meshes.add(triangle)),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.9, 0.4, 0.4)))),
        Transform::from_xyz(-120.0, 0.0, 0.0),
    ));

    // 自定义四边形：两个三角形（6 个顶点，不共享顶点）
    let mut quad = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    quad.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            // 第一个三角形（右下）
            [-50.0, -50.0, 0.0],
            [50.0, -50.0, 0.0],
            [50.0, 50.0, 0.0],
            // 第二个三角形（左上）
            [-50.0, -50.0, 0.0],
            [50.0, 50.0, 0.0],
            [-50.0, 50.0, 0.0],
        ],
    );

    commands.spawn((
        Mesh2d(meshes.add(quad)),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.4, 0.6, 0.9)))),
        Transform::from_xyz(120.0, 0.0, 0.0),
    ));
}
