//! Bevy 0.19 入门示例：演示 3D 第一人称相机（First-Person Camera）。
//!
//! 学习重点：
//! - 用 yaw（水平角）+ pitch（俯仰角）描述朝向，存进 Resource
//! - 鼠标移动控制视角：AccumulatedMouseMotion 读每帧鼠标位移
//! - 由 yaw 推导 forward / right 向量，实现相对朝向的 WASD 移动
//! - Quat::from_euler 把欧拉角转成四元数，赋给相机 Transform.rotation
//!
//! 操作：移动鼠标看四周，WASD 移动。

use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;

#[derive(Resource)]
struct Fps {
    yaw: f32,   // 水平角（绕 Y 轴）
    pitch: f32, // 俯仰角
    position: Vec3,
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.1, 0.12, 0.18)))
        .insert_resource(Fps {
            yaw: 0.0,
            pitch: 0.0,
            position: Vec3::new(0.0, 1.7, 8.0),
        })
        .add_systems(Startup, setup)
        .add_systems(Update, fps_camera)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 相机初始位置（fps_camera 系统每帧会覆盖）
    commands.spawn((Camera3d::default(), Transform::from_xyz(0.0, 1.7, 8.0)));

    // 方向光
    commands.spawn((
        DirectionalLight {
            illuminance: 8000.0,
            ..default()
        },
        Transform::from_xyz(3.0, 8.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
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

    // 散布一些立方体作为移动参照物
    for i in -2..=2 {
        for j in -2..=2 {
            if i == 0 && j == 0 {
                continue;
            }
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.0, 2.0, 1.0))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgb(0.4, 0.6, 0.9),
                    ..default()
                })),
                Transform::from_xyz(i as f32 * 4.0, 1.0, j as f32 * 4.0),
            ));
        }
    }
}

fn fps_camera(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<AccumulatedMouseMotion>,
    time: Res<Time>,
    mut fps: ResMut<Fps>,
    mut camera: Single<&mut Transform, With<Camera3d>>,
) {
    let dt = time.delta_secs();

    // 鼠标控制视角（delta 是每帧像素位移，sens 控制灵敏度）
    let sens = 0.002;
    fps.yaw -= mouse.delta.x * sens;
    fps.pitch -= mouse.delta.y * sens;
    fps.pitch = fps.pitch.clamp(-1.4, 1.4);

    // 由 yaw 推导水平面内的 forward / right 向量
    let forward = Vec3::new(fps.yaw.sin(), 0.0, -fps.yaw.cos());
    let right = Vec3::new(fps.yaw.cos(), 0.0, fps.yaw.sin());

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
        fps.position += dir.normalize() * 6.0 * dt;
    }

    // 应用旋转与位置（YXZ 顺序：先 yaw 再 pitch）
    camera.rotation = Quat::from_euler(EulerRot::YXZ, fps.yaw, fps.pitch, 0.0);
    camera.translation = fps.position;
}
