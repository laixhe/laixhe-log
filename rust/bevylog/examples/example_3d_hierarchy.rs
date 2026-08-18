//! Bevy 0.19 入门示例：演示 3D 父子层级（太阳-行星-卫星）。
//!
//! 学习重点：
//! - with_children 嵌套建立父子关系（ChildOf / Children）
//! - 子实体的 Transform 是「相对父实体」的局部坐标
//! - 父实体旋转时，子实体会随之绕父实体公转
//! - 用 Rotator 组件分别驱动太阳自转、行星自转，卫星跟着行星转
//!
//! 观察：太阳自转 → 行星绕太阳公转 → 卫星绕行星公转。

use bevy::prelude::*;

#[derive(Component)]
struct Rotator {
    speed: f32,
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.03, 0.03, 0.06)))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate)
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
        Transform::from_xyz(0.0, 4.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 方向光
    commands.spawn((
        DirectionalLight {
            illuminance: 6000.0,
            ..default()
        },
        Transform::from_xyz(2.0, 6.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 太阳（父实体，自转）
    commands
        .spawn((
            Rotator { speed: 0.6 },
            Mesh3d(meshes.add(Sphere::new(0.9))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 0.8, 0.2),
                emissive: LinearRgba::rgb(1.0, 0.6, 0.1),
                ..default()
            })),
            Transform::default(),
        ))
        .with_children(|sun| {
            // 行星（太阳的子实体，随太阳公转，同时自转）
            sun.spawn((
                Rotator { speed: 1.2 },
                Mesh3d(meshes.add(Sphere::new(0.35))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgb(0.2, 0.5, 0.9),
                    ..default()
                })),
                Transform::from_xyz(3.0, 0.0, 0.0),
            ))
            .with_children(|planet| {
                // 卫星（行星的子实体，随行星公转）
                planet.spawn((
                    Mesh3d(meshes.add(Sphere::new(0.15))),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: Color::srgb(0.85, 0.85, 0.85),
                        ..default()
                    })),
                    Transform::from_xyz(0.7, 0.0, 0.0),
                ));
            });
        });
}

// 驱动带 Rotator 的实体绕自身 Y 轴旋转
fn rotate(time: Res<Time>, mut q: Query<(&mut Transform, &Rotator)>) {
    for (mut tf, rotator) in &mut q {
        tf.rotate_y(rotator.speed * time.delta_secs());
    }
}
