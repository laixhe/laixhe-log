//! Bevy 0.19 入门示例：演示粒子尾迹（跟随鼠标的粒子）。
//!
//! 学习重点：
//! - 每帧在鼠标位置生成若干粒子，形成尾迹
//! - 粒子带速度 + 生命周期，向外漂移、缩小、淡出，最后销毁
//! - Sprite::from_color + Color::set_alpha 实现淡出（Sprite 默认支持透明度混合）
//! - viewport_to_world_2d 把鼠标屏幕坐标转成世界坐标
//!
//! 观察：移动鼠标，身后留下逐渐消散的蓝色尾迹。

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::RngExt;

#[derive(Component)]
struct Particle {
    velocity: Vec2,
    lifetime: f32,
    max_lifetime: f32,
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .add_systems(Update, (spawn_particles, update_particles))
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

// 每帧在鼠标位置生成粒子
fn spawn_particles(
    mut commands: Commands,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
) {
    let (camera, cam_tf) = *camera;
    let Some(screen) = window.cursor_position() else {
        return;
    };
    let Ok(world) = camera.viewport_to_world_2d(cam_tf, screen) else {
        return;
    };

    let mut rng = rand::rng();
    for _ in 0..4 {
        let velocity = Vec2::new(rng.random_range(-80.0..80.0), rng.random_range(-80.0..80.0));
        commands.spawn((
            Particle {
                velocity,
                lifetime: 1.0,
                max_lifetime: 1.0,
            },
            Sprite::from_color(Color::srgb(0.3, 0.8, 1.0), Vec2::splat(12.0)),
            Transform::from_xyz(world.x, world.y, 0.0),
        ));
    }
}

// 更新粒子：漂移 + 缩小 + 淡出 + 销毁
fn update_particles(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut Particle, &mut Transform, &mut Sprite)>,
) {
    for (entity, mut particle, mut tf, mut sprite) in &mut q {
        particle.lifetime -= time.delta_secs();
        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }

        // 向外漂移
        tf.translation += particle.velocity.extend(0.0) * time.delta_secs();

        // 随剩余生命比例缩小 + 淡出
        let t = particle.lifetime / particle.max_lifetime;
        tf.scale = Vec3::splat(t);
        sprite.color.set_alpha(t);
    }
}
