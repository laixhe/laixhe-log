//! Bevy 0.19 入门示例：演示 bevy_firework —— CPU 粒子系统（3D 场景版）。
//!
//! 与 2D 版的区别：
//! - particle_settings 里 `pbr: true`：粒子走 PBR 渲染，受场景光照影响（需配光源）
//! - 发射形状用 Sphere（球形爆开），径向速度在三维空间发散
//! - 相机开启 Hdr + Bloom，Add 叠加的粒子会带发光光晕
//!
//! 操作：每 1.5 秒在随机位置放一次球形爆炸；按空格键随机放一个；
//! 点击鼠标左键在地面（y=0 平面）上放一个。

use avian3d::prelude::PhysicsPlugins;
use bevy::{
    camera::Hdr,
    log::LogPlugin,
    post_process::bloom::Bloom,
    prelude::*,
    window::PrimaryWindow,
};
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

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            // 静默 icu_provider 缺 CJK 字典数据的警告（保留默认的 wgpu/naga 过滤）
            filter: "wgpu=error,naga=warn,icu_provider=off".to_string(),
            ..default()
        }))
        .add_plugins(ParticleSystemPlugin::default())
        // bevy_firework 默认启用 physics_avian 特性，update_particles 系统
        // 强制要求 avian 的 SpatialQuery，必须注册物理插件才能提供 ColliderTrees 资源
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(ClearColor(Color::srgb(0.02, 0.02, 0.04)))
        .add_systems(Startup, setup)
        .add_systems(Update, (periodic_explosion, explode_on_space, explosion_on_click))
        .run()
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 相机：Hdr + Bloom 让叠加发光的粒子有光晕
    commands.spawn((
        Camera3d::default(),
        Hdr,
        Transform::from_xyz(10.0, 6.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
        Bloom::default(),
    ));

    // 光源：pbr 粒子需要光照才可见（暖色点光源，高亮度）
    commands.spawn((
        PointLight {
            intensity: 2_000_000.0,
            color: Color::srgb(1.0, 0.9, 0.8),
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 12.0, 0.0),
    ));

    // 地面参考平面
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.1, 0.1, 0.12))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // 提示文字：用 UI Text 节点（屏幕空间），由 3D 相机直接渲染（bevy 的 UI 阶段同时注册在
    // Core2d 和 Core3d 渲染图里），不需要额外的 2D 相机，也就没有双相机合成的问题。
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(20.0),
            top: Val::Px(20.0),
            ..default()
        },
        Text::new("每 1.5 秒随机爆炸，空格随机放一个，点击地面放一个"),
        TextColor(Color::WHITE),
        TextFont {
            font: FontSource::Handle(asset_server.load(FONT_PATH)),
            font_size: FontSize::Px(20.0),
            ..default()
        },
    ));
}

// ---- 一次性球形爆炸的通用配置：Sphere 发射形状 + 径向速度 + PBR ----
fn spawn_explosion(commands: &mut Commands, pos: Vec3) {
    commands
        .spawn((
            ParticleSpawner {
                particle_settings: vec![ParticleSettings {
                    lifetime: RandF32 { min: 1.0, max: 2.0 },
                    initial_scale: RandF32 { min: 0.08, max: 0.2 },
                    scale_curve: FireworkCurve::even_samples(vec![1.0, 1.2, 0.5, 0.0]),
                    base_color: FireworkGradient::uneven_samples(vec![
                        (0.0, LinearRgba::new(1.0, 0.8, 0.3, 1.0)), // 亮橙黄
                        (0.4, LinearRgba::new(1.0, 0.4, 0.1, 1.0)), // 橙
                        (0.8, LinearRgba::new(0.8, 0.2, 0.1, 0.5)), // 红
                        (1.0, LinearRgba::new(0.4, 0.1, 0.1, 0.0)), // 淡出
                    ]),
                    blend_mode: BlendMode::Add,
                    linear_drag: 0.6,
                    acceleration: Vec3::new(0.0, -2.0, 0.0), // 3D 重力
                    pbr: true,                              // 走 PBR 渲染，受光照影响
                    fade_edge: 0.0,
                    ..default()
                }],
                emission_settings: vec![EmissionSettings {
                    emission_pacing: EmissionPacing::OneShot(80),
                    emission_shape: EmissionShape::Sphere(0.3), // 球面出生点
                    initial_velocity: RandVec3::constant(Vec3::ZERO),
                    initial_velocity_radial: RandF32 { min: 3.0, max: 8.0 }, // 三维径向爆发
                    ..default()
                }],
                spawn_transform_mode: SpawnTransformMode::Local,
                ..default()
            },
            Transform::from_translation(pos),
        ))
        // 播完自动清理自己
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

// 每 1.5 秒在随机位置放一次爆炸
fn periodic_explosion(
    mut commands: Commands,
    time: Res<Time>,
    mut next_at: Local<f32>,
    mut rng: Local<Rng>,
) {
    if time.elapsed_secs() < *next_at {
        return;
    }
    *next_at = time.elapsed_secs() + 1.5;

    let pos = Vec3::new(
        (next_rand(&mut rng) - 0.5) * 10.0, // x: -5 ~ 5
        (next_rand(&mut rng) - 0.5) * 4.0 + 3.0, // y: 1 ~ 5
        (next_rand(&mut rng) - 0.5) * 10.0, // z: -5 ~ 5
    );
    spawn_explosion(&mut commands, pos);
}

// 按空格：在随机位置放一个爆炸
fn explode_on_space(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut rng: Local<Rng>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }
    let pos = Vec3::new(
        (next_rand(&mut rng) - 0.5) * 10.0,
        (next_rand(&mut rng) - 0.5) * 4.0 + 3.0,
        (next_rand(&mut rng) - 0.5) * 10.0,
    );
    spawn_explosion(&mut commands, pos);
}

// 点击左键：从相机射出穿过光标的射线，与地面平面（y=0）求交，在交点放一个爆炸
fn explosion_on_click(
    mut commands: Commands,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }
    let Some(cursor) = window.cursor_position() else {
        return;
    };
    let Ok(ray) = camera.0.viewport_to_world(camera.1, cursor) else {
        return;
    };
    let Some(distance) = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y)) else {
        return;
    };
    spawn_explosion(&mut commands, ray.get_point(distance));
}
