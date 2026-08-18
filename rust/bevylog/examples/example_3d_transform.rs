//! Bevy 0.19 入门示例：演示 3D 变换（平移 / 旋转 / 缩放）。
//! 一个 3D 立方体，键盘控制平移、旋转和缩放。
//!
//! 学习重点：
//! - Transform 的三部分：translation（位置）、rotation（旋转，Quat）、scale（缩放）
//! - 旋转用四元数 Quat 表示；rotate_local_x/y/z 绕局部轴旋转
//! - 平移直接改 translation；缩放用 Vec3::splat(倍数) 等比缩放
//! - 四元数内部用 Quat，避免万向锁，是 3D 旋转的标准表示

use bevy::prelude::*;

// 立方体标记
#[derive(Component)]
struct Controlled;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.08)))
        .add_systems(Startup, setup)
        .add_systems(Update, control_transform)
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
        Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 方向光
    commands.spawn((
        DirectionalLight {
            illuminance: 3000.0,
            ..default()
        },
        Transform::from_xyz(1.0, 2.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 立方体（非对称颜色便于观察旋转方向）
    commands.spawn((
        Controlled,
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.5, 0.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.8, 0.4),
            ..default()
        })),
        Transform::default(),
    ));
}

// 键盘控制：WASD/方向键平移，Q/E/R/F 旋转，+/- 缩放
fn control_transform(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut cube: Single<&mut Transform, With<Controlled>>,
) {
    let dt = time.delta_secs();

    // 平移：WASD / 方向键（在 XY 平面移动）
    let mut move_dir = Vec3::ZERO;
    if keyboard.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        move_dir.x -= 1.0;
    }
    if keyboard.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        move_dir.x += 1.0;
    }
    if keyboard.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
        move_dir.y += 1.0;
    }
    if keyboard.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
        move_dir.y -= 1.0;
    }
    if move_dir != Vec3::ZERO {
        cube.translation += move_dir.normalize() * 3.0 * dt;
    }

    // 旋转：绕局部轴旋转（角度单位弧度）
    if keyboard.pressed(KeyCode::KeyQ) {
        cube.rotate_local_z(2.0 * dt);
    }
    if keyboard.pressed(KeyCode::KeyE) {
        cube.rotate_local_z(-2.0 * dt);
    }
    if keyboard.pressed(KeyCode::KeyR) {
        cube.rotate_local_y(2.0 * dt);
    }
    if keyboard.pressed(KeyCode::KeyF) {
        cube.rotate_local_y(-2.0 * dt);
    }

    // 缩放：+/- 等比缩放（0.5 ~ 3.0 之间）
    if keyboard.just_pressed(KeyCode::Equal) || keyboard.just_pressed(KeyCode::NumpadAdd) {
        cube.scale = (cube.scale * 1.2).clamp(Vec3::splat(0.5), Vec3::splat(3.0));
    }
    if keyboard.just_pressed(KeyCode::Minus) || keyboard.just_pressed(KeyCode::NumpadSubtract) {
        cube.scale = (cube.scale * 0.8).clamp(Vec3::splat(0.5), Vec3::splat(3.0));
    }
}
