//! Bevy 0.19 入门示例：演示 3D 轨道相机（Orbit Camera）。
//!
//! 学习重点：
//! - 用球坐标（yaw 水平角、pitch 俯仰角、distance 距离）描述相机绕目标的位置
//! - 球坐标转笛卡尔坐标，再用 look_at 让相机始终看向目标
//! - 键盘旋转、滚轮调整距离，形成「绕物体观察」效果
//!
//! 操作：方向键旋转视角，滚轮拉近拉远。

use bevy::input::mouse::AccumulatedMouseScroll;
use bevy::prelude::*;

#[derive(Resource)]
struct Orbit {
    yaw: f32,      // 水平旋转角（绕 Y 轴）
    pitch: f32,    // 俯仰角
    distance: f32, // 相机到目标的距离
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.1, 0.12, 0.18)))
        .insert_resource(Orbit {
            yaw: 0.0,
            pitch: 0.4,
            distance: 8.0,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, orbit_camera)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 相机初始位置（orbit 系统每帧都会重新计算并覆盖）
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 3.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 方向光
    commands.spawn((
        DirectionalLight {
            illuminance: 6000.0,
            ..default()
        },
        Transform::from_xyz(3.0, 8.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 地面
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(8.0)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.3, 0.35),
            perceptual_roughness: 0.9,
            ..default()
        })),
    ));

    // 目标物体：原点立方体（相机始终围绕它旋转）
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.2, 1.2, 1.2))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.3, 0.2),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.6, 0.0),
    ));
}

fn orbit_camera(
    keys: Res<ButtonInput<KeyCode>>,
    scroll: Res<AccumulatedMouseScroll>,
    time: Res<Time>,
    mut orbit: ResMut<Orbit>,
    mut camera: Single<&mut Transform, With<Camera3d>>,
) {
    let dt = time.delta_secs();

    // 方向键旋转视角
    if keys.pressed(KeyCode::ArrowLeft) {
        orbit.yaw -= 1.5 * dt;
    }
    if keys.pressed(KeyCode::ArrowRight) {
        orbit.yaw += 1.5 * dt;
    }
    if keys.pressed(KeyCode::ArrowUp) {
        orbit.pitch += 1.0 * dt;
    }
    if keys.pressed(KeyCode::ArrowDown) {
        orbit.pitch -= 1.0 * dt;
    }
    // 限制俯仰角，避免翻转到头顶/脚下
    orbit.pitch = orbit.pitch.clamp(-1.4, 1.4);

    // 滚轮调整距离
    orbit.distance = (orbit.distance * (1.0 - scroll.delta.y * 0.1)).clamp(2.0, 20.0);

    // 球坐标 -> 笛卡尔坐标
    let x = orbit.distance * orbit.pitch.cos() * orbit.yaw.sin();
    let y = orbit.distance * orbit.pitch.sin();
    let z = orbit.distance * orbit.pitch.cos() * orbit.yaw.cos();

    camera.translation = Vec3::new(x, y, z);
    camera.look_at(Vec3::ZERO, Vec3::Y);
}
