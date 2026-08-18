//! Bevy 0.19 入门示例：演示 2D 相机旋转（Camera Rotation）。
//!
//! 学习重点：
//! - 相机也是实体，旋转它的 Transform 会让整个画面绕屏幕中心旋转
//! - 用 rotate_z 绕 Z 轴旋转（2D 平面的旋转轴）
//! - 对比「旋转物体」和「旋转相机」：旋转相机会让整个场景一起转
//!
//! 操作：Q / E 逆时针 / 顺时针旋转相机，R 复位。

use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.08)))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_camera)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // 网格参照点，旋转时能清楚看到整体转动
    for i in -3..=3 {
        for j in -3..=3 {
            commands.spawn((
                Mesh2d(meshes.add(Circle::new(8.0))),
                MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.3, 0.35, 0.45)))),
                Transform::from_xyz(i as f32 * 120.0, j as f32 * 120.0, 0.0),
            ));
        }
    }

    // 中心一个带方向的长条矩形，便于判断旋转方向
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(220.0, 40.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.9, 0.5, 0.2)))),
        Transform::default(),
    ));
}

fn rotate_camera(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut camera: Single<&mut Transform, With<Camera2d>>,
) {
    let speed = 1.5 * time.delta_secs();

    if keys.pressed(KeyCode::KeyQ) {
        camera.rotate_z(speed);
    }
    if keys.pressed(KeyCode::KeyE) {
        camera.rotate_z(-speed);
    }
    if keys.just_pressed(KeyCode::KeyR) {
        camera.rotation = Quat::IDENTITY;
        info!("[相机] 复位旋转");
    }
}
