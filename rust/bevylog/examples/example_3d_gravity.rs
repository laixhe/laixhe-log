//! Bevy 0.19 入门示例：演示 3D 重力模拟（下落 + 地面碰撞反弹）。
//!
//! 学习重点：
//! - Velocity(Vec3) 速度组件，存储运动状态
//! - 重力加速度：每帧 velocity.y += GRAVITY * dt，速度积分得到位移
//! - 地面碰撞：球心高度 < 半径时反弹，反转 y 速度并乘恢复系数 restitution
//! - restitution 恢复系数：0 不反弹，1 完全弹性，0~1 之间损失能量
//! - .chain() 保证「物理步进 → 碰撞响应」的执行顺序
//!
//! 观察：四个球从不同高度落下，恢复系数不同，反弹次数也不同。

use bevy::prelude::*;

// 重力加速度（世界单位/秒²，负号表示向下）
const GRAVITY: f32 = -9.8;
// 地面高度（Plane3d 位于 y=0）
const GROUND_Y: f32 = 0.0;

#[derive(Component)]
struct Ball {
    radius: f32,
    restitution: f32, // 恢复系数：0~1
}

#[derive(Component)]
struct Velocity(Vec3);

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        .add_systems(Update, (step_physics, bounce_floor).chain())
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
        Transform::from_xyz(0.0, 3.0, 10.0).looking_at(Vec3::new(0.0, 3.0, 0.0), Vec3::Y),
    ));

    // 方向光
    commands.spawn((
        DirectionalLight {
            illuminance: 6000.0,
            ..default()
        },
        Transform::from_xyz(2.0, 6.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
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

    // 四个球：不同高度和恢复系数 (x, 初始高度, 恢复系数, 颜色)
    let balls = [
        (-3.0, 6.0, 0.5, Color::srgb(0.9, 0.3, 0.3)),
        (-1.0, 8.0, 0.7, Color::srgb(0.3, 0.7, 0.3)),
        (1.0, 10.0, 0.9, Color::srgb(0.3, 0.5, 0.9)),
        (3.0, 12.0, 1.0, Color::srgb(0.9, 0.7, 0.2)),
    ];
    for (x, height, restitution, color) in balls {
        commands.spawn((
            Ball {
                radius: 0.5,
                restitution,
            },
            Velocity(Vec3::ZERO),
            Mesh3d(meshes.add(Sphere::new(0.5))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: color,
                perceptual_roughness: 0.4,
                ..default()
            })),
            Transform::from_xyz(x, height, 0.0),
        ));
    }
}

// 物理步进：重力加速度 + 速度积分更新位置
fn step_physics(time: Res<Time>, mut q: Query<(&mut Velocity, &mut Transform), With<Ball>>) {
    let dt = time.delta_secs();
    for (mut velocity, mut tf) in &mut q {
        velocity.0.y += GRAVITY * dt;
        tf.translation += velocity.0 * dt;
    }
}

// 地面碰撞：球落到地面时反弹
fn bounce_floor(mut q: Query<(&Ball, &mut Transform, &mut Velocity)>) {
    for (ball, mut tf, mut velocity) in &mut q {
        if tf.translation.y - ball.radius < GROUND_Y {
            // 把球推回地面之上，避免穿地
            tf.translation.y = GROUND_Y + ball.radius;

            // 反弹：反转 y 速度并乘恢复系数（能量损失）
            if velocity.0.y < 0.0 {
                let bounced = -velocity.0.y * ball.restitution;
                // 速度太小时直接停下，避免无限微弹
                velocity.0.y = if bounced < 0.5 { 0.0 } else { bounced };
            }
        }
    }
}
