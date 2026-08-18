//! Bevy 0.19 入门示例：演示自定义 3D 网格（从顶点数据构造 Mesh）。
//! 手动构造一个三角形网格，填充顶点位置和法线，用 PBR 材质渲染。
//!
//! 学习重点：
//! - Mesh::new(拓扑类型, 用途) 创建空网格
//! - insert_attribute 填充顶点属性（位置 / 法线 / UV / 颜色等）
//! - PrimitiveTopology::TriangleList 表示每 3 个顶点构成一个三角形
//! - Mesh3d + MeshMaterial3d 挂载网格和材质
//! - 大多数内置图元（Circle / Cuboid / Sphere）内部也是这么构造网格的

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
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 3D 相机
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 方向光：照亮三角形
    commands.spawn((
        DirectionalLight {
            illuminance: 3000.0,
            ..default()
        },
        Transform::from_xyz(1.0, 2.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 1. 创建空网格：三角形列表拓扑，默认用途
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    // 2. 填充顶点位置（3 个顶点，每个是 [x, y, z]）
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [-0.5, -0.5, 0.0], // 左下
            [0.5, -0.5, 0.0],  // 右下
            [0.0, 0.5, 0.0],   // 顶部
        ],
    );

    // 3. 填充法线（3 个顶点都朝 +Z，朝向相机，这样正面能被照亮）
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 3]);

    // 4. 生成三角形：网格 + 材质 + 变换
    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.8, 0.4),
            ..default()
        })),
        Transform::default(),
    ));
}
