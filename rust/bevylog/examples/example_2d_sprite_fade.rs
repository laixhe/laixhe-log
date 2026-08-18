//! Bevy 0.19 入门示例：演示精灵淡入淡出（透明度动画）。
//!
//! 学习重点：
//! - Sprite::from_color：创建纯色精灵
//! - Color::set_alpha：修改颜色的透明度
//! - 用 Time::elapsed_secs + sin 让透明度周期性变化
//!
//! 观察：多个纯色方块以不同的相位做淡入淡出动画。

use bevy::prelude::*;

#[derive(Component)]
struct Fading {
    // 相位偏移，让多个精灵的淡入淡出错开
    phase: f32,
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.06, 0.06, 0.08)))
        .add_systems(Startup, setup)
        .add_systems(Update, fade)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    let colors = [
        Color::srgb(0.9, 0.4, 0.4),
        Color::srgb(0.4, 0.7, 0.4),
        Color::srgb(0.4, 0.5, 0.9),
    ];

    for (i, color) in colors.iter().enumerate() {
        commands.spawn((
            Fading {
                phase: i as f32 * 1.2,
            },
            Sprite::from_color(*color, Vec2::splat(120.0)),
            Transform::from_xyz(i as f32 * 180.0 - 180.0, 0.0, 0.0),
        ));
    }
}

// 让每个精灵的透明度随 sin 周期变化（相位错开）
fn fade(time: Res<Time>, mut q: Query<(&Fading, &mut Sprite)>) {
    for (fading, mut sprite) in &mut q {
        let alpha = ((time.elapsed_secs() * 2.0 + fading.phase).sin() * 0.5 + 0.5).clamp(0.0, 1.0);
        sprite.color.set_alpha(alpha);
    }
}
