//! Bevy 0.19 入门示例：演示 2D 缩放脉冲（Transform.scale 动画）。
//!
//! 学习重点：
//! - Transform.scale：缩放物体
//! - 用 sin 让缩放比例周期性变化（脉冲效果）
//! - phase 让多个物体的脉冲错开
//!
//! 观察：三个圆以不同相位做缩放脉冲动画。

use bevy::prelude::*;

#[derive(Component)]
struct Pulse {
    base: f32,
    phase: f32,
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.06, 0.06, 0.08)))
        .add_systems(Startup, setup)
        .add_systems(Update, pulse)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    for i in 0..3 {
        commands.spawn((
            Pulse {
                base: 1.0,
                phase: i as f32 * 1.1,
            },
            Mesh2d(meshes.add(Circle::new(40.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(
                0.4 + i as f32 * 0.2,
                0.6,
                0.8 - i as f32 * 0.2,
            )))),
            Transform::from_xyz((i as f32 - 1.0) * 150.0, 0.0, 0.0),
        ));
    }
}

// 让每个圆按自己的相位做缩放脉冲
fn pulse(time: Res<Time>, mut q: Query<(&Pulse, &mut Transform)>) {
    for (pulse, mut tf) in &mut q {
        let s = pulse.base * (1.0 + 0.3 * (time.elapsed_secs() * 3.0 + pulse.phase).sin());
        tf.scale = Vec3::splat(s);
    }
}
