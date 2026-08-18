//! Bevy 0.19 入门示例：演示 3D 球体弹性碰撞（封闭盒内）。
//!
//! 学习重点：
//! - Velocity(Vec3) 速度组件 + Ball 半径组件
//! - 边界碰撞：球碰到盒子的六个面反弹（反转对应轴速度）
//! - 球间碰撞：三维距离 < 半径之和 → 碰撞，沿法线分离 + 交换法线速度
//! - iter_combinations_mut 遍历实体对，处理两两交互
//! - Gizmos 绘制线框盒子，可视化碰撞边界
//! - .chain() 保证「移动 → 边界碰撞 → 球间碰撞」的执行顺序
//!
//! 观察：多个球在透明线框盒内运动，碰到边界或彼此会弹性反弹。

use bevy::prelude::*;

// 盒子半尺寸（长宽高的一半）
const BOX_HALF: Vec3 = Vec3::new(5.0, 5.0, 5.0);

#[derive(Component)]
struct Ball {
    radius: f32,
}

#[derive(Component)]
struct Velocity(Vec3);

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (move_balls, wall_collisions, ball_collisions).chain(),
        )
        .add_systems(Update, draw_box)
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
        Transform::from_xyz(0.0, 6.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 方向光
    commands.spawn((
        DirectionalLight {
            illuminance: 6000.0,
            ..default()
        },
        Transform::from_xyz(3.0, 8.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 6 个球：不同位置、速度、颜色
    let balls = [
        (
            Vec3::new(-2.0, -2.0, -2.0),
            Vec3::new(3.0, 2.0, 1.0),
            Color::srgb(0.9, 0.3, 0.3),
        ),
        (
            Vec3::new(2.0, -2.0, 2.0),
            Vec3::new(-3.0, 2.5, -1.0),
            Color::srgb(0.3, 0.7, 0.3),
        ),
        (
            Vec3::new(-2.0, 2.0, 2.0),
            Vec3::new(2.0, -3.0, 2.0),
            Color::srgb(0.3, 0.5, 0.9),
        ),
        (
            Vec3::new(2.0, 2.0, -2.0),
            Vec3::new(-2.0, -2.5, -2.0),
            Color::srgb(0.9, 0.7, 0.2),
        ),
        (
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(3.5, 1.5, -2.5),
            Color::srgb(0.7, 0.4, 0.9),
        ),
        (
            Vec3::new(3.0, 1.0, 1.0),
            Vec3::new(-3.0, -1.0, 3.0),
            Color::srgb(0.4, 0.9, 0.7),
        ),
    ];
    for (position, velocity, color) in balls {
        commands.spawn((
            Ball { radius: 0.5 },
            Velocity(velocity),
            Mesh3d(meshes.add(Sphere::new(0.5))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: color,
                perceptual_roughness: 0.4,
                ..default()
            })),
            Transform::from_translation(position),
        ));
    }
}

// 移动：速度积分更新位置
fn move_balls(time: Res<Time>, mut q: Query<(&Velocity, &mut Transform), With<Ball>>) {
    let dt = time.delta_secs();
    for (velocity, mut tf) in &mut q {
        tf.translation += velocity.0 * dt;
    }
}

// 边界碰撞：碰到盒子六个面反弹
fn wall_collisions(mut q: Query<(&Ball, &mut Transform, &mut Velocity)>) {
    for (ball, mut tf, mut velocity) in &mut q {
        let r = ball.radius;
        let h = BOX_HALF;

        if tf.translation.x + r > h.x {
            tf.translation.x = h.x - r;
            velocity.0.x = -velocity.0.x.abs();
        } else if tf.translation.x - r < -h.x {
            tf.translation.x = -h.x + r;
            velocity.0.x = velocity.0.x.abs();
        }

        if tf.translation.y + r > h.y {
            tf.translation.y = h.y - r;
            velocity.0.y = -velocity.0.y.abs();
        } else if tf.translation.y - r < -h.y {
            tf.translation.y = -h.y + r;
            velocity.0.y = velocity.0.y.abs();
        }

        if tf.translation.z + r > h.z {
            tf.translation.z = h.z - r;
            velocity.0.z = -velocity.0.z.abs();
        } else if tf.translation.z - r < -h.z {
            tf.translation.z = -h.z + r;
            velocity.0.z = velocity.0.z.abs();
        }
    }
}

// 球间碰撞：三维球体弹性碰撞
fn ball_collisions(mut balls: Query<(&Ball, &mut Transform, &mut Velocity)>) {
    let mut combinations = balls.iter_combinations_mut();
    while let Some(
        [
            (ball_a, mut transform_a, mut vel_a),
            (ball_b, mut transform_b, mut vel_b),
        ],
    ) = combinations.fetch_next()
    {
        // 球心向量（A - B）
        let delta = transform_a.translation - transform_b.translation;
        let distance_sq = delta.length_squared();
        let min_distance = ball_a.radius + ball_b.radius;

        if distance_sq < min_distance * min_distance && distance_sq > 0.0 {
            let distance = distance_sq.sqrt();
            // 碰撞法线：从 B 指向 A 的单位向量
            let normal = delta / distance;

            // 1. 分离重叠，消除穿透
            let overlap = min_distance - distance;
            let separation = normal * (overlap * 0.5);
            transform_a.translation += separation;
            transform_b.translation -= separation;

            // 2. 交换法线方向速度分量（等质量弹性碰撞）
            let rel_velocity = vel_a.0 - vel_b.0;
            let vel_along_normal = rel_velocity.dot(normal);
            if vel_along_normal < 0.0 {
                let impulse = normal * vel_along_normal;
                vel_a.0 -= impulse;
                vel_b.0 += impulse;
            }
        }
    }
}

// 用 Gizmos 绘制线框盒子
fn draw_box(mut gizmos: Gizmos) {
    let h = BOX_HALF;
    let corners = [
        Vec3::new(-h.x, -h.y, -h.z),
        Vec3::new(h.x, -h.y, -h.z),
        Vec3::new(h.x, h.y, -h.z),
        Vec3::new(-h.x, h.y, -h.z),
        Vec3::new(-h.x, -h.y, h.z),
        Vec3::new(h.x, -h.y, h.z),
        Vec3::new(h.x, h.y, h.z),
        Vec3::new(-h.x, h.y, h.z),
    ];
    // 12 条边：后面 4 + 前面 4 + 竖直连接 4
    let edges = [
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 0),
        (4, 5),
        (5, 6),
        (6, 7),
        (7, 4),
        (0, 4),
        (1, 5),
        (2, 6),
        (3, 7),
    ];
    for (a, b) in edges {
        gizmos.line(corners[a], corners[b], Color::srgb(0.5, 0.6, 0.7));
    }
}
