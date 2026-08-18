//! Bevy 0.19 入门示例：3D 基础场景。
//! 相机 + 灯光 + 三种基本几何体（立方体/球体/平面）+ 旋转动画。
//!
//! 学习重点：
//! - Camera3d + looking_at 设置相机位置和朝向
//! - DirectionalLight 方向光（模拟太阳的平行光）
//! - Mesh3d + MeshMaterial3d + StandardMaterial 渲染 3D 网格
//! - 基本图元：Cuboid（立方体）/ Sphere（球体）/ Plane3d（平面）
//! - Transform.rotate_y 绕 Y 轴旋转动画

use bevy::prelude::*;

// 标记组件：标识「会旋转」的物体
#[derive(Component)]
struct Rotating;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.1, 0.12, 0.18)))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_objects)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 3D 相机：从 (0, 3, 8) 看向原点，Y 轴朝上
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 3.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 方向光：平行光（如太阳），illuminance 单位 lux（勒克斯）
    commands.spawn((
        DirectionalLight {
            illuminance: 8000.0,
            ..default()
        },
        Transform::from_xyz(3.0, 8.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 地面平面：half_size = 6 表示半宽半高，实际 12x12
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(6.0)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.3, 0.35),
            perceptual_roughness: 0.9,
            ..default()
        })),
    ));

    // 红色立方体（会旋转）
    commands.spawn((
        Rotating,
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.2),
            ..default()
        })),
        Transform::from_xyz(-2.0, 0.5, 0.0),
    ));

    // 蓝色球体（会旋转，带一点金属感）
    commands.spawn((
        Rotating,
        Mesh3d(meshes.add(Sphere::new(0.6))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.4, 0.9),
            metallic: 0.3,
            perceptual_roughness: 0.4,
            ..default()
        })),
        Transform::from_xyz(2.0, 0.6, 0.0),
    ));
}

// 让带 Rotating 标记的物体绕 Y 轴旋转
fn rotate_objects(time: Res<Time>, mut query: Query<&mut Transform, With<Rotating>>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs());
    }
}
