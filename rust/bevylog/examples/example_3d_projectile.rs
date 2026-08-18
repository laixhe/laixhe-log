//! Bevy 0.19 入门示例：演示 3D 抛体运动（Projectile）。
//!
//! 学习重点：
//! - 初始速度向量决定抛射方向：水平分量 + 垂直分量
//! - 重力只影响 y 分量，水平方向匀速 → 形成抛物线轨迹
//! - 按空格用 commands.spawn 动态生成抛体
//! - 地面碰撞：反弹并乘恢复系数，能量逐渐耗尽后停下
//!
//! 操作：按空格发射一个球，观察抛物线轨迹与落地反弹。

use bevy::prelude::*;

// 重力加速度（世界单位/秒²，向下）
const GRAVITY: f32 = -9.8;
// 地面高度
const GROUND_Y: f32 = 0.0;

#[derive(Component)]
struct Projectile {
    radius: f32,
}

#[derive(Component)]
struct Velocity(Vec3);

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        .add_systems(Update, (launch, step, bounce).chain())
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
        Transform::from_xyz(0.0, 4.0, 12.0).looking_at(Vec3::new(0.0, 2.0, 0.0), Vec3::Y),
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
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.0)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.3, 0.35),
            perceptual_roughness: 0.9,
            ..default()
        })),
    ));

    // 发射台标记（一个小立方体）
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.6, 0.2),
            ..default()
        })),
        Transform::from_xyz(-5.0, 0.5, 0.0),
    ));
}

// 按空格发射一个抛体
fn launch(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        commands.spawn((
            Projectile { radius: 0.4 },
            // 初速度：向右上方（水平 +x，垂直 +y）
            Velocity(Vec3::new(9.0, 14.0, 0.0)),
            Mesh3d(meshes.add(Sphere::new(0.4))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.9, 0.4, 0.4),
                perceptual_roughness: 0.4,
                ..default()
            })),
            Transform::from_xyz(-5.0, 0.4, 0.0),
        ));
        info!("[抛体] 发射!");
    }
}

// 物理步进：重力 + 速度积分
fn step(time: Res<Time>, mut q: Query<(&mut Velocity, &mut Transform), With<Projectile>>) {
    let dt = time.delta_secs();
    for (mut velocity, mut tf) in &mut q {
        velocity.0.y += GRAVITY * dt;
        tf.translation += velocity.0 * dt;
    }
}

// 地面碰撞：落地反弹
fn bounce(mut q: Query<(&Projectile, &mut Transform, &mut Velocity)>) {
    for (projectile, mut tf, mut velocity) in &mut q {
        if tf.translation.y - projectile.radius < GROUND_Y {
            tf.translation.y = GROUND_Y + projectile.radius;
            if velocity.0.y < 0.0 {
                let bounced = -velocity.0.y * 0.5;
                velocity.0.y = if bounced < 0.5 { 0.0 } else { bounced };
            }
        }
    }
}
