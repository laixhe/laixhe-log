//! Bevy 0.19 入门示例：演示视差滚动（Parallax）。
//!
//! 学习重点：
//! - 多层背景以不同速度移动，产生视差（近处快、远处慢）
//! - 给每层挂一个 speed 组件，用同一系统按速度移动
//!
//! 观察：三层横条以不同速度左右摆动，速度不同产生层次感。

use bevy::prelude::*;

#[derive(Component)]
struct Parallax {
    speed: f32,
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.04, 0.05, 0.08)))
        .add_systems(Startup, setup)
        .add_systems(Update, scroll)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    let layers = [
        (0.3, Color::srgb(0.15, 0.25, 0.40)),
        (0.6, Color::srgb(0.25, 0.40, 0.55)),
        (1.0, Color::srgb(0.40, 0.60, 0.70)),
    ];

    for (i, (speed, color)) in layers.iter().enumerate() {
        commands.spawn((
            Parallax { speed: *speed },
            Sprite::from_color(*color, Vec2::new(500.0, 40.0)),
            Transform::from_xyz(0.0, 60.0 - i as f32 * 45.0, 0.0),
        ));
    }
}

// 每层按自己的速度左右摆动，产生视差
fn scroll(time: Res<Time>, mut q: Query<(&Parallax, &mut Transform)>) {
    for (parallax, mut tf) in &mut q {
        tf.translation.x = (time.elapsed_secs() * parallax.speed * 80.0).sin() * 220.0;
    }
}
