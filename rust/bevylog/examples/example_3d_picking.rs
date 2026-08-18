//! Bevy 0.19 入门示例：演示 3D 拾取（MeshPicking + Click 事件）。
//!
//! 学习重点：
//! - MeshPickingPlugin：mesh 射线拾取后端（2D / 3D 通用）
//! - On<Pointer<Click>>：用 observer 监听实体的点击事件
//! - Pointer 事件携带被点击实体，改 StandardMaterial 实现高亮
//!
//! 操作：点击立方体或球体，物体变黄色，终端打印被点击实体。

use bevy::picking::prelude::*;
use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MeshPickingPlugin)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
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
        Transform::from_xyz(0.0, 3.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 方向光
    commands.spawn((
        DirectionalLight {
            illuminance: 6000.0,
            ..default()
        },
        Transform::from_xyz(2.0, 5.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 可拾取的立方体
    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(1.2, 1.2, 1.2))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.9, 0.4, 0.4),
                ..default()
            })),
            Transform::from_xyz(-1.6, 0.6, 0.0),
        ))
        .observe(on_click);

    // 可拾取的球体
    commands
        .spawn((
            Mesh3d(meshes.add(Sphere::new(0.7))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.4, 0.5, 0.9),
                ..default()
            })),
            Transform::from_xyz(1.6, 0.7, 0.0),
        ))
        .observe(on_click);
}

// 点击事件：把被点击物体的材质改成黄色高亮
fn on_click(
    event: On<Pointer<Click>>,
    q_material: Query<&MeshMaterial3d<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let entity = event.event().entity;
    info!("[3D拾取] 点击了实体 {:?}", entity);

    if let Ok(mat) = q_material.get(entity) {
        if let Some(mut material) = materials.get_mut(&mat.0) {
            material.base_color = Color::srgb(0.9, 0.8, 0.3);
        }
    }
}
