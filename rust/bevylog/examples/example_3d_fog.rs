//! Bevy 0.19 入门示例：演示 3D 雾效（DistanceFog）。
//!
//! 学习重点：
//! - DistanceFog 组件挂在相机上启用距离雾
//! - FogFalloff::Linear { start, end } 控制雾的起始/结束距离
//! - 物体越远越被雾色覆盖，产生大气透视效果
//!
//! 观察：一排柱子向远处延伸，逐渐隐没在雾中。

use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        // 背景色与雾色一致，让远处自然融入
        .insert_resource(ClearColor(Color::srgb(0.7, 0.75, 0.8)))
        .add_systems(Startup, setup)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 相机 + 距离雾
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        DistanceFog {
            color: Color::srgb(0.7, 0.75, 0.8),
            falloff: FogFalloff::Linear {
                start: 8.0,
                end: 40.0,
            },
            ..default()
        },
    ));

    // 方向光
    commands.spawn((
        DirectionalLight {
            illuminance: 6000.0,
            ..default()
        },
        Transform::from_xyz(3.0, 8.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 地面（向远处延伸）
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(40.0)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.35, 0.4, 0.45),
            perceptual_roughness: 0.9,
            ..default()
        })),
    ));

    // 一排柱子，向远处（-z）延伸，展示雾的渐隐
    for i in 0..12 {
        let z = -i as f32 * 3.0;
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 3.0, 1.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.5, 0.2, 0.2),
                perceptual_roughness: 0.6,
                ..default()
            })),
            Transform::from_xyz(0.0, 1.5, z),
        ));
    }
}
