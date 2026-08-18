//! Bevy 0.19 入门示例：演示屏幕环绕（Wrap Around）。
//!
//! 学习重点：
//! - 物体匀速直线移动
//! - 超出屏幕边界后从另一边进入（环绕）
//!
//! 观察：几个圆点向不同方向移动，穿出屏幕后从对面重新出现。

use bevy::prelude::*;

#[derive(Component)]
struct Mover {
    velocity: Vec2,
}

// 屏幕边界（假设窗口约 800x600，2D 相机中心为原点）
const BOUNDS_X: f32 = 400.0;
const BOUNDS_Y: f32 = 300.0;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.06, 0.06, 0.08)))
        .add_systems(Startup, setup)
        .add_systems(Update, wrap)
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
        Color::srgb(0.4, 0.8, 0.4),
        Color::srgb(0.4, 0.6, 0.9),
        Color::srgb(0.9, 0.7, 0.3),
    ];

    for (i, color) in colors.iter().enumerate() {
        let angle = i as f32 * std::f32::consts::FRAC_PI_2 + 0.4;
        commands.spawn((
            Mover {
                velocity: Vec2::new(angle.cos(), angle.sin()) * 140.0,
            },
            Mesh2d(meshes.add(Circle::new(18.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(*color))),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));
    }
}

// 匀速移动 + 屏幕环绕
fn wrap(time: Res<Time>, mut q: Query<(&Mover, &mut Transform)>) {
    for (mover, mut tf) in &mut q {
        tf.translation += mover.velocity.extend(0.0) * time.delta_secs();

        if tf.translation.x > BOUNDS_X {
            tf.translation.x = -BOUNDS_X;
        }
        if tf.translation.x < -BOUNDS_X {
            tf.translation.x = BOUNDS_X;
        }
        if tf.translation.y > BOUNDS_Y {
            tf.translation.y = -BOUNDS_Y;
        }
        if tf.translation.y < -BOUNDS_Y {
            tf.translation.y = BOUNDS_Y;
        }
    }
}
