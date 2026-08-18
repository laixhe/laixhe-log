//! Bevy 0.19 入门示例：演示 UI 变换（UiTransform）。
//!
//! 学习重点：
//! - `UiTransform`：UI 节点的相对 2D 变换（translation / scale / rotation）
//! - 与 `Node` 的布局定位不同，`UiTransform` 在布局之后叠加位移/缩放/旋转
//! - 配合 `Time` 让 UI 元素旋转、脉动、上下浮动
//!
//! 观察：三个卡片分别做旋转、缩放脉动、上下浮动。

use bevy::math::Rot2;
use bevy::prelude::*;
use bevy::text::FontSource;

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// 旋转标记：speed 控制旋转角速度（弧度/秒）
#[derive(Component)]
struct Spin {
    speed: f32,
}

// 脉动标记：base 为基础缩放值
#[derive(Component)]
struct Pulse {
    base: f32,
}

// 浮动标记
#[derive(Component)]
struct Bob;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, animate)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands
        .spawn(Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: px(24),
            ..default()
        })
        .with_children(|parent| {
            let font = |size: f32| TextFont {
                font: FontSource::Handle(asset_server.load(FONT_PATH)),
                font_size: FontSize::Px(size),
                ..default()
            };

            parent.spawn((Text::new("UI 变换"), TextColor(Color::WHITE), font(24.0)));

            // 三个卡片横向排列
            parent
                .spawn(Node {
                    column_gap: px(40),
                    ..default()
                })
                .with_children(|row| {
                    // 旋转卡片
                    row.spawn((
                        Spin { speed: 1.5 },
                        UiTransform::from_rotation(Rot2::radians(0.0)),
                        Node {
                            width: px(120),
                            height: px(120),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border_radius: BorderRadius::all(px(12)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.30, 0.50, 0.90)),
                    ))
                    .with_children(|card| {
                        card.spawn((Text::new("旋转"), TextColor(Color::WHITE), font(18.0)));
                    });

                    // 缩放脉动卡片
                    row.spawn((
                        Pulse { base: 1.0 },
                        UiTransform::from_scale(Vec2::splat(1.0)),
                        Node {
                            width: px(120),
                            height: px(120),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border_radius: BorderRadius::all(px(12)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.85, 0.40, 0.60)),
                    ))
                    .with_children(|card| {
                        card.spawn((Text::new("脉动"), TextColor(Color::WHITE), font(18.0)));
                    });

                    // 上下浮动卡片
                    row.spawn((
                        Bob,
                        UiTransform::IDENTITY,
                        Node {
                            width: px(120),
                            height: px(120),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border_radius: BorderRadius::all(px(12)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.20, 0.70, 0.50)),
                    ))
                    .with_children(|card| {
                        card.spawn((Text::new("浮动"), TextColor(Color::WHITE), font(18.0)));
                    });
                });
        });
}

// 每帧更新 UiTransform，实现旋转 / 脉动 / 浮动动画
fn animate(
    time: Res<Time>,
    mut q_spin: Query<(&mut UiTransform, &Spin)>,
    mut q_pulse: Query<(&mut UiTransform, &Pulse)>,
    mut q_bob: Query<&mut UiTransform, With<Bob>>,
) {
    let t = time.elapsed_secs();

    for (mut tf, spin) in &mut q_spin {
        tf.rotation = Rot2::radians(t * spin.speed);
    }

    for (mut tf, pulse) in &mut q_pulse {
        let s = pulse.base * (1.0 + 0.15 * (t * 3.0).sin());
        tf.scale = Vec2::splat(s);
    }

    for mut tf in &mut q_bob {
        tf.translation = Val2::px(0.0, (t * 4.0).sin() * 20.0);
    }
}
