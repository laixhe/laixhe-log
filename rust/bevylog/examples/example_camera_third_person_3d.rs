//! Bevy 0.19 入门示例：演示 3D 第三人称跟随相机（Third-Person Camera）。
//!
//! 学习重点：
//! - 角色用 yaw 记录朝向，WASD 相对朝向移动，Q/E 转身
//! - 相机根据角色朝向，始终保持在角色「后方 + 上方」
//! - look_at 让相机始终看向角色
//! - move_player 与 follow_camera 用 .chain() 保证先后顺序
//!
//! 操作：WASD 移动角色，Q/E 转身，相机自动跟随。

use bevy::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Resource)]
struct PlayerYaw(f32);

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.1, 0.12, 0.18)))
        .insert_resource(PlayerYaw(0.0))
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, follow_camera).chain())
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 相机初始位置（follow_camera 系统每帧会覆盖）
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 6.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 方向光
    commands.spawn((
        DirectionalLight {
            illuminance: 8000.0,
            ..default()
        },
        Transform::from_xyz(3.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 地面
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(20.0)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.3, 0.35),
            perceptual_roughness: 0.9,
            ..default()
        })),
    ));

    // 玩家：一个亮色立方体
    commands.spawn((
        Player,
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.8, 0.5),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    // 散布参照物
    for i in -2..=2 {
        for j in -2..=2 {
            if i == 0 && j == 0 {
                continue;
            }
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgb(0.4, 0.5, 0.7),
                    ..default()
                })),
                Transform::from_xyz(i as f32 * 4.0, 0.5, j as f32 * 4.0),
            ));
        }
    }
}

fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut yaw: ResMut<PlayerYaw>,
    mut player: Single<&mut Transform, With<Player>>,
) {
    let dt = time.delta_secs();

    // 由 yaw 推导 forward / right
    let forward = Vec3::new(yaw.0.sin(), 0.0, -yaw.0.cos());
    let right = Vec3::new(yaw.0.cos(), 0.0, yaw.0.sin());

    let mut dir = Vec3::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        dir += forward;
    }
    if keys.pressed(KeyCode::KeyS) {
        dir -= forward;
    }
    if keys.pressed(KeyCode::KeyD) {
        dir += right;
    }
    if keys.pressed(KeyCode::KeyA) {
        dir -= right;
    }
    if dir != Vec3::ZERO {
        player.translation += dir.normalize() * 5.0 * dt;
    }

    // Q / E 转身
    if keys.pressed(KeyCode::KeyQ) {
        yaw.0 += 2.0 * dt;
    }
    if keys.pressed(KeyCode::KeyE) {
        yaw.0 -= 2.0 * dt;
    }

    // 角色面向移动方向
    player.rotation = Quat::from_rotation_y(yaw.0);
}

fn follow_camera(
    player: Single<&Transform, With<Player>>,
    yaw: Res<PlayerYaw>,
    mut camera: Single<&mut Transform, With<Camera3d>>,
) {
    let forward = Vec3::new(yaw.0.sin(), 0.0, -yaw.0.cos());
    let target = player.translation;

    // 相机在角色后方（-forward）上方
    let distance = 10.0;
    let height = 5.0;
    camera.translation = target - forward * distance + Vec3::Y * height;
    camera.look_at(target, Vec3::Y);
}
