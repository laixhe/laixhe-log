//! Bevy 0.19 入门示例：演示 2D 重力模拟与落地反弹。
//!
//! 学习重点：
//! - 用速度向量 + 重力加速度做简单物理模拟
//! - 速度积分更新位置（x = x + v * dt）
//! - 落地时反转速度并乘以阻尼，模拟能量损失
//!
//! 观察：三个球下落并在地面反复弹跳，弹跳高度逐渐降低。

use bevy::prelude::*;

#[derive(Component)]
struct Ball {
    velocity: Vec2,
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.06, 0.06, 0.08)))
        .add_systems(Startup, setup)
        .add_systems(Update, gravity)
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
            Ball {
                velocity: Vec2::new((i as f32 - 1.0) * 60.0, 0.0),
            },
            Mesh2d(meshes.add(Circle::new(28.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(
                0.3 + i as f32 * 0.25,
                0.5,
                0.8 - i as f32 * 0.15,
            )))),
            Transform::from_xyz((i as f32 - 1.0) * 150.0, 220.0, 0.0),
        ));
    }
}

// 重力 + 落地反弹
fn gravity(time: Res<Time>, mut q: Query<(&mut Transform, &mut Ball)>) {
    for (mut tf, mut ball) in &mut q {
        // 重力加速度向下
        ball.velocity.y -= 600.0 * time.delta_secs();

        // 速度积分更新位置
        tf.translation.x += ball.velocity.x * time.delta_secs();
        tf.translation.y += ball.velocity.y * time.delta_secs();

        // 地面反弹（y = -200 为地面）
        if tf.translation.y < -200.0 {
            tf.translation.y = -200.0;
            ball.velocity.y = -ball.velocity.y * 0.8; // 阻尼，损失 20% 能量
        }
    }
}
