//! Bevy 0.19 入门示例：演示 2D 弹跳球（sin 周期运动）。
//!
//! 学习重点：
//! - 用 sin 让物体做上下周期运动
//! - speed + phase 控制弹跳速度和相位差
//!
//! 观察：一排球以不同的速度和相位上下弹跳。

use bevy::prelude::*;

#[derive(Component)]
struct Bouncer {
    speed: f32,
    phase: f32,
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.06, 0.06, 0.08)))
        .add_systems(Startup, setup)
        .add_systems(Update, bounce)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let colors = [
        Color::srgb(0.9, 0.4, 0.4),
        Color::srgb(0.9, 0.6, 0.3),
        Color::srgb(0.9, 0.8, 0.3),
        Color::srgb(0.4, 0.8, 0.4),
        Color::srgb(0.4, 0.6, 0.9),
    ];

    for (i, color) in colors.iter().enumerate() {
        commands.spawn((
            Bouncer {
                speed: 2.0 + i as f32 * 0.5,
                phase: i as f32 * 0.8,
            },
            Mesh2d(meshes.add(Circle::new(24.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(*color))),
            Transform::from_xyz((i as f32 - 2.0) * 90.0, 0.0, 0.0),
        ));
    }
}

// 每个球按自己的速度和相位做上下弹跳
fn bounce(time: Res<Time>, mut q: Query<(&Bouncer, &mut Transform)>) {
    for (bouncer, mut tf) in &mut q {
        tf.translation.y = (time.elapsed_secs() * bouncer.speed + bouncer.phase).sin() * 150.0;
    }
}
