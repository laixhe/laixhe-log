//! Bevy 0.19 入门示例：演示缓动动画（Easing）。
//!
//! 学习重点：
//! - 缓动函数：把线性时间 t 映射到非线性进度，控制动画速度曲线
//! - 常见缓动：Linear / EaseIn / EaseOut / EaseInOut
//! - 用 (elapsed % duration) / duration 得到循环进度
//!
//! 观察：四个圆点从 -200 移动到 200，但速度曲线不同（匀速 / 加速 / 减速 / 先加速后减速）。

use bevy::prelude::*;
use bevy::text::FontSource;

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

#[derive(Component, Clone, Copy)]
enum Easing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
}

#[derive(Component)]
struct Eased {
    easing: Easing,
    duration: f32,
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.06, 0.06, 0.08)))
        .add_systems(Startup, setup)
        .add_systems(Update, animate)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);

    let easings = [
        (Easing::Linear, "Linear", Color::srgb(0.9, 0.4, 0.4)),
        (Easing::EaseIn, "EaseIn", Color::srgb(0.4, 0.7, 0.4)),
        (Easing::EaseOut, "EaseOut", Color::srgb(0.4, 0.5, 0.9)),
        (Easing::EaseInOut, "EaseInOut", Color::srgb(0.9, 0.7, 0.3)),
    ];

    for (i, (easing, label, color)) in easings.iter().enumerate() {
        let y = 90.0 - i as f32 * 60.0;
        commands.spawn((
            Eased {
                easing: *easing,
                duration: 3.0,
            },
            Mesh2d(meshes.add(Circle::new(16.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(*color))),
            Transform::from_xyz(-200.0, y, 0.0),
        ));

        commands.spawn((
            Text2d::new(*label),
            TextColor(Color::WHITE),
            TextFont {
                font: FontSource::Handle(asset_server.load(FONT_PATH)),
                font_size: FontSize::Px(18.0),
                ..default()
            },
            Transform::from_xyz(-340.0, y, 0.0),
        ));
    }
}

// 缓动函数：t 在 [0, 1]，返回映射后的进度
fn ease(t: f32, easing: Easing) -> f32 {
    match easing {
        Easing::Linear => t,
        Easing::EaseIn => t * t,
        Easing::EaseOut => 1.0 - (1.0 - t).powi(2),
        Easing::EaseInOut => {
            if t < 0.5 {
                2.0 * t * t
            } else {
                1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
            }
        }
    }
}

fn animate(time: Res<Time>, mut q: Query<(&Eased, &mut Transform)>) {
    for (eased, mut tf) in &mut q {
        let t = (time.elapsed_secs() / eased.duration) % 1.0;
        let p = ease(t, eased.easing);
        tf.translation.x = -200.0 + p * 400.0;
    }
}
