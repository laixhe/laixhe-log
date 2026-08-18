//! Bevy 0.19 入门示例：演示 3D 程序化网格（带索引的立方体）。
//!
//! 与 example_3d_custom_mesh 的单个三角形不同，本示例构造一个完整立方体：
//! 8 个顶点 + 36 个索引（12 个三角形），并让它在 3D 空间中旋转。
//!
//! 学习重点：
//! - Mesh::new 创建空网格，insert_attribute 填顶点属性（位置 / 法线）
//! - insert_indices(Indices::U16(...)) 用索引复用顶点（8 个顶点即可表达 6 个面）
//! - Mesh3d + MeshMaterial3d 挂载，StandardMaterial 双面渲染
//! - 索引网格 vs 无索引网格：顶点少、省内存，是 3D 模型的常规做法

use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.08)))
        .add_systems(Startup, setup)
        .add_systems(Update, spin)
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
        Transform::from_xyz(2.0, 1.5, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 方向光
    commands.spawn((
        DirectionalLight {
            illuminance: 4000.0,
            ..default()
        },
        Transform::from_xyz(2.0, 3.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 1. 立方体 8 个顶点（边长 1，中心在原点）
    let positions: Vec<[f32; 3]> = vec![
        [-0.5, -0.5, -0.5], // 0
        [0.5, -0.5, -0.5],  // 1
        [0.5, 0.5, -0.5],   // 2
        [-0.5, 0.5, -0.5],  // 3
        [-0.5, -0.5, 0.5],  // 4
        [0.5, -0.5, 0.5],   // 5
        [0.5, 0.5, 0.5],    // 6
        [-0.5, 0.5, 0.5],   // 7
    ];

    // 2. 法线：从中心指向每个顶点（简化处理，渲染为平滑着色）
    let normals: Vec<[f32; 3]> = positions
        .iter()
        .map(|p| {
            let len = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
            [p[0] / len, p[1] / len, p[2] / len]
        })
        .collect();

    // 3. 索引：12 个三角形（6 个面），复用 8 个顶点
    let indices = vec![
        4, 5, 6, 4, 6, 7, // 前面 (z=0.5)
        1, 0, 3, 1, 3, 2, // 后面 (z=-0.5)
        0, 4, 7, 0, 7, 3, // 左面 (x=-0.5)
        5, 1, 2, 5, 2, 6, // 右面 (x=0.5)
        7, 6, 2, 7, 2, 3, // 顶面 (y=0.5)
        0, 1, 5, 0, 5, 4, // 底面 (y=-0.5)
    ];

    // 4. 组装网格
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_indices(Indices::U16(indices));

    // 5. 生成实体
    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.7, 0.4),
            double_sided: true,
            ..default()
        })),
        Transform::default(),
    ));
}

// 让立方体绕 Y 轴和 X 轴缓慢旋转，展示 3D 形态
fn spin(time: Res<Time>, mut query: Single<&mut Transform, With<Mesh3d>>) {
    query.rotation = Quat::from_euler(
        EulerRot::XYZ,
        time.elapsed_secs() * 0.6,
        time.elapsed_secs() * 0.9,
        0.0,
    );
}
