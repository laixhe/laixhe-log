//! Bevy 0.19 入门示例：演示相机震动（Camera Shake）。
//!
//! 学习重点：
//! - 用 Resource 保存「创伤值」trauma，表示当前震动强度
//! - 每帧按时间衰减 trauma，震动力度用 trauma² 让衰减更自然
//! - 用 rand 生成随机偏移，叠加到相机 Transform.translation
//!
//! 操作：按空格触发一次震动。

use bevy::prelude::*;
use rand::RngExt;

// 震动状态：trauma 是 0~1 的强度，base 是相机静止位置
#[derive(Resource)]
struct Shake {
    trauma: f32,
    base: Vec2,
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.08)))
        .insert_resource(Shake {
            trauma: 0.0,
            base: Vec2::ZERO,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, shake_camera)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // 放几个参照物，方便观察相机抖动
    let colors = [
        Color::srgb(0.2, 0.6, 1.0),
        Color::srgb(0.9, 0.3, 0.3),
        Color::srgb(0.3, 0.9, 0.4),
    ];
    for (i, color) in colors.into_iter().enumerate() {
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(50.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(color))),
            Transform::from_xyz((i as f32 - 1.0) * 220.0, 0.0, 0.0),
        ));
    }
}

fn shake_camera(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut shake: ResMut<Shake>,
    mut camera: Single<&mut Transform, With<Camera2d>>,
) {
    // 按空格触发震动
    if keys.just_pressed(KeyCode::Space) {
        shake.trauma = 1.0;
        info!("[相机] 触发震动!");
    }

    // 每帧衰减创伤值，衰减到 0 后停止抖动
    shake.trauma = (shake.trauma - time.delta_secs() * 1.5).max(0.0);
    if shake.trauma <= 0.0 {
        return;
    }

    // 用 trauma² 作为力度：衰减时抖动幅度下降更快，更接近真实震动
    let strength = shake.trauma * shake.trauma;
    let max_offset = 30.0;
    let mut rng = rand::rng();
    let offset = Vec2::new(
        (rng.random::<f32>() - 0.5) * 2.0 * max_offset * strength,
        (rng.random::<f32>() - 0.5) * 2.0 * max_offset * strength,
    );

    camera.translation = (shake.base + offset).extend(0.0);
}
