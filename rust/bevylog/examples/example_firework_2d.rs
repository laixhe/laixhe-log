//! Bevy 0.19 入门示例：演示 bevy_firework —— CPU 粒子系统（2D 场景版）。
//!
//! bevy_firework 的粒子在 CPU 上模拟、GPU 批量合批渲染：一个发射器 = 一次 draw call，
//! 能承载上万个粒子而帧率不掉。它本质是 3D 粒子，2D 场景里直接用 XY 平面即可。
//!
//! 核心概念：
//! - ParticleSpawner：粒子发射器组件（一份配置 = 一种特效）
//!   - particle_settings：粒子外观与运动（寿命 / 大小 / 颜色渐变 / 重力 / 拖拽 / 混合）
//!   - emission_settings：如何发射（一次性 or 持续 / 发射形状 / 初速度 / 径向速度）
//!   - spawn_transform_mode：粒子出生位置取 Local（自身 Transform）或 Global
//! - FireworkGradient：颜色随粒子寿命渐变（0.0~1.0 时间轴取样）
//! - FireworkCurve：其他参数随寿命变化的曲线（如缩放）
//! - EmissionPacing：OneShot(数量) 一次性爆开 / rate(速率) 持续喷射
//! - EmissionShape：Point 点 / Circle 圆盘 / Sphere 球体 发射形状
//! - ParticleSpawnerFinished：一次性发射器播完的事件（这里用来自动清理）
//!
//! 操作：底部有持续喷发的火花喷泉；每 1.2 秒在随机位置放一次爆炸；
//! 点击鼠标左键在光标处放一个爆炸。

use avian3d::prelude::PhysicsPlugins;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::text::FontSourceTemplate;
use bevy::window::PrimaryWindow;
use bevy_firework::{
    core::{
        BlendMode, EmissionPacing, EmissionSettings, ParticleSettings, ParticleSpawner,
        ParticleSpawnerFinished, SpawnTransformMode,
    },
    curve::{FireworkCurve, FireworkGradient},
    emission_shape::EmissionShape,
    plugin::ParticleSystemPlugin,
};
use bevy_utilitarian::prelude::*;
use std::f32::consts::PI;

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            // 静默 icu_provider 缺 CJK 字典数据的警告（保留默认的 wgpu/naga 过滤）
            filter: "wgpu=error,naga=warn,icu_provider=off".to_string(),
            ..default()
        }))
        // 粒子系统插件（核心）
        .add_plugins(ParticleSystemPlugin::default())
        // bevy_firework 默认启用 physics_avian 特性，update_particles 系统
        // 强制要求 avian 的 SpatialQuery，必须注册物理插件才能提供 ColliderTrees 资源
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(ClearColor(Color::srgb(0.03, 0.03, 0.06)))
        .add_systems(Startup, setup)
        .add_systems(Update, (periodic_explosion, explosion_on_click))
        .run()
}

fn setup(mut commands: Commands) {
    // bevy_firework 的粒子注册在 3D 渲染阶段（Transparent3d），而 Camera2d 只绘制 2D 渲染阶段，
    // 粒子会完全不可见。因此改用「正交投影的 Camera3d」模拟 2D 视角（XY 平面，1 世界单位 = 1 像素）。
    commands.spawn((
        Camera3d::default(),
        Projection::Orthographic(OrthographicProjection::default_2d()),
    ));
    // 再叠一层 2D 相机：不清屏，只负责渲染提示文字（Text2d 属于 2D 渲染阶段，上面的 3D 相机不画它）。
    // order 更大 = 后渲染 = 显示在上层。
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
    ));

    // 提示文本
    commands.spawn_scene(bsn! {
        Text2d::new("持续喷发的火花 + 定时爆炸，点击左键在光标处放烟花")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(20.0),
        }
        Transform::from_xyz(0.0, -330.0, 0.0)
    });

    // ---- 持续喷射的火花喷泉（演示 continuous 发射 + 重力 + 颜色渐变）----
    // 粒子从点发射，向上喷出（带 15° 散角），再被重力拉回，Add 混合叠加发光
    commands.spawn((
        ParticleSpawner {
            particle_settings: vec![ParticleSettings {
                lifetime: RandF32 { min: 0.6, max: 1.2 },
                initial_scale: RandF32 { min: 3.0, max: 5.0 },
                scale_curve: FireworkCurve::even_samples(vec![1.0, 0.7, 0.3]),
                base_color: FireworkGradient::uneven_samples(vec![
                    (0.0, LinearRgba::new(0.6, 0.8, 1.0, 1.0)), // 亮蓝
                    (0.5, LinearRgba::new(0.3, 0.5, 1.0, 1.0)), // 蓝
                    (1.0, LinearRgba::new(0.2, 0.3, 0.8, 0.0)), // 淡出
                ]),
                blend_mode: BlendMode::Add, // 叠加模式，火花重叠处更亮
                linear_drag: 0.4,
                acceleration: Vec3::new(0.0, -250.0, 0.0), // 2D 重力（世界 y 向上，往下拉）
                fade_edge: 0.0,
                ..default()
            }],
            emission_settings: vec![EmissionSettings {
                emission_pacing: EmissionPacing::rate(300.0), // 每秒 300 个
                emission_shape: EmissionShape::Point,
                initial_velocity: RandVec3 {
                    direction: Vec3::Y,
                    magnitude: RandF32 { min: 250.0, max: 450.0 },
                    spread: 15.0 / 180.0 * PI, // 散角（弧度）
                },
                ..default()
            }],
            ..default()
        },
        Transform::from_xyz(0.0, -300.0, 0.0),
    ));
}

// ---- 一次性爆炸的通用配置：圆形爆开 + 径向速度 ----
fn spawn_explosion(commands: &mut Commands, pos: Vec3) {
    commands
        .spawn((
            ParticleSpawner {
                particle_settings: vec![ParticleSettings {
                    lifetime: RandF32 { min: 0.8, max: 1.6 },
                    initial_scale: RandF32 { min: 3.0, max: 6.0 },
                    scale_curve: FireworkCurve::even_samples(vec![1.0, 1.1, 0.6, 0.0]),
                    base_color: FireworkGradient::uneven_samples(vec![
                        (0.0, LinearRgba::new(1.0, 0.9, 0.4, 1.0)), // 亮黄
                        (0.4, LinearRgba::new(1.0, 0.5, 0.1, 1.0)), // 橙
                        (0.8, LinearRgba::new(1.0, 0.2, 0.1, 0.5)), // 红
                        (1.0, LinearRgba::new(0.6, 0.1, 0.1, 0.0)), // 淡出
                    ]),
                    blend_mode: BlendMode::Add,
                    linear_drag: 0.9, // 阻力大：爆开后很快减速
                    acceleration: Vec3::new(0.0, -200.0, 0.0),
                    fade_edge: 0.0,
                    ..default()
                }],
                emission_settings: vec![EmissionSettings {
                    emission_pacing: EmissionPacing::OneShot(50), // 一次性放出 50 个
                    // 2D 场景：法线取 Z，圆盘落在屏幕平面(XY)上，粒子才会向四面八方爆开；
                    // 若法线取 Y，圆盘落在 XZ 平面，屏幕上只会看到一条横向喷出的线。
                    emission_shape: EmissionShape::Circle {
                        normal: Vec3::Z,
                        radius: 5.0,
                    },
                    initial_velocity: RandVec3::constant(Vec3::ZERO),
                    initial_velocity_radial: RandF32 { min: 60.0, max: 180.0 }, // 径向爆开速度
                    ..default()
                }],
                spawn_transform_mode: SpawnTransformMode::Local,
                ..default()
            },
            Transform::from_translation(pos),
        ))
        // 播完自动清理自己（ParticleSpawnerFinished 事件）
        .observe(|trigger: On<ParticleSpawnerFinished>, mut commands: Commands| {
            commands.entity(trigger.event_target()).despawn();
        });
}

// 简易伪随机数（避免为教学示例引入 rand 依赖）
#[derive(Default)]
struct Rng(u32);

fn next_rand(rng: &mut Rng) -> f32 {
    rng.0 = rng.0.wrapping_mul(1664525).wrapping_add(1013904223);
    (rng.0 >> 8) as f32 / 16_777_216.0 // 0.0 ~ 1.0
}

// 每 1.2 秒在随机位置放一次爆炸
fn periodic_explosion(
    mut commands: Commands,
    time: Res<Time>,
    mut next_at: Local<f32>,
    mut rng: Local<Rng>,
) {
    if time.elapsed_secs() < *next_at {
        return;
    }
    *next_at = time.elapsed_secs() + 1.2;

    let pos = Vec3::new(
        (next_rand(&mut rng) - 0.5) * 600.0, // x: -300 ~ 300
        (next_rand(&mut rng) - 0.5) * 400.0, // y: -200 ~ 200
        0.0,
    );
    spawn_explosion(&mut commands, pos);
}

// 点击左键：在光标处放一个爆炸
fn explosion_on_click(
    mut commands: Commands,
    window: Single<&Window, With<PrimaryWindow>>,
    // 只取 3D 相机做坐标换算（粒子由它渲染），排除叠加的 2D 相机
    camera: Single<(&Camera, &GlobalTransform), With<Camera3d>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }
    let Some(cursor) = window.cursor_position() else {
        return;
    };
    let Ok(pos) = camera.0.viewport_to_world_2d(camera.1, cursor) else {
        return;
    };
    spawn_explosion(&mut commands, pos.extend(0.0));
}
