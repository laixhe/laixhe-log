//! Bevy 0.19 入门示例：avian3d —— 射线检测（SpatialQuery）与传感器（Sensor）。
//!
//! 除了刚体和碰撞，物理引擎还提供两种"探测"能力：
//! - 射线检测：从任意位置朝任意方向发射一条射线，返回命中的第一个碰撞体。
//!   这里用"鼠标点击 → 相机穿过光标的射线 → 命中点生成小球"来演示。
//! - 传感器：带 Sensor 组件的碰撞体不产生物理阻挡，但会触发碰撞事件。
//!   这里在场景中央放一个红色区域，小球穿过时触发事件被销毁（观察者实现）。
//!
//! 学习重点：
//! - SpatialQuery 系统参数：cast_ray 返回最近命中（RayHitData：entity / distance / normal）
//! - SpatialQueryFilter：射线过滤（这里用默认，即命中所有碰撞体）
//! - Sensor 组件：触发器，只检测不阻挡
//! - CollisionEventsEnabled + 观察者 On<CollisionStart>：在传感器上监听进入事件
//! - Timer 资源：周期性从空中掉球
//!
//! 操作：
//! - 点击左键：射线检测，在鼠标指到的位置生成一个小球
//! - 按空格：从相机发射一个高速球
//! - 观察：掉落的球和发射的球穿过红色传感器区域时会被销毁（触发事件）

use avian3d::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::{prelude::*, text::FontSource};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(ClearColor(Color::srgb(0.04, 0.05, 0.08)))
        // 每 0.8 秒从空中掉一个小球
        .insert_resource(SpawnTimer(Timer::from_seconds(0.8, TimerMode::Repeating)))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                spawn_ball_on_click,
                launch_ball_on_space,
                drop_ball_periodically,
                draw_sensor_bounds,
            ),
        )
        .run()
}

// 掉球计时器
#[derive(Resource)]
struct SpawnTimer(Timer);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 相机 + 光照
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 6.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 8000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 10.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 地面
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(20.0, 1.0, 20.0),
        Friction::new(0.8),
        Restitution::new(0.3),
        Transform::from_xyz(0.0, -0.5, 0.0),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.15, 0.16, 0.2))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // 传感器区域：红色半透明方块，位于地面上方。
    // 只检测碰撞、不产生阻挡；小球穿过时触发观察者被销毁。
    commands
        .spawn((
            RigidBody::Static,
            Collider::cuboid(4.0, 1.5, 4.0),
            Sensor,
            // 关闭与地面的碰撞事件噪音：传感器只关心"物体"进入。
            // （这里地面在传感器下方不重叠，无需特殊处理，保持默认即可）
            CollisionEventsEnabled,
            Mesh3d(meshes.add(Cuboid::new(4.0, 1.5, 4.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgba(1.0, 0.3, 0.3, 0.35),
                alpha_mode: AlphaMode::Blend,
                unlit: true, // 半透明材质不受光照，颜色更纯粹
                ..default()
            })),
            Transform::from_xyz(0.0, 3.0, 0.0),
        ))
        .observe(on_ball_entered_sensor);

    // 提示文字（UI 节点，由 3D 相机直接渲染）
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(20.0),
            top: Val::Px(20.0),
            ..default()
        },
        Text::new("点击：射线检测生成球 | 空格：发射球 | 红色区域是传感器：球穿过会被销毁"),
        TextColor(Color::WHITE),
        TextFont {
            font: FontSource::Handle(asset_server.load(FONT_PATH)),
            font_size: FontSize::Px(20.0),
            ..default()
        },
    ));
}

// ---- 传感器观察者：有物体进入（CollisionStart）就销毁它 ----
fn on_ball_entered_sensor(trigger: On<CollisionStart>, mut commands: Commands) {
    // 观察者挂在传感器上，所以事件里 collider1 是传感器自身、collider2 是进入的物体
    info!("物体 {:?} 进入传感器，销毁", trigger.collider2);
    commands.entity(trigger.collider2).despawn();
}

// ---- 点击左键：SpatialQuery 射线检测，在命中点生成小球 ----
fn spawn_ball_on_click(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mouse: Res<ButtonInput<MouseButton>>,
    spatial_query: SpatialQuery,
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

    // 从相机位置沿光标方向发射射线，找最近命中的碰撞体（地面、球等）
    let filter = SpatialQueryFilter::default();
    if let Some(hit) = spatial_query.cast_ray(ray.origin, ray.direction, 100.0, true, &filter) {
        // 命中点 = 起点 + 方向 × 距离；再沿法线抬高一点避免陷入碰撞体
        let hit_point = ray.origin + *ray.direction * hit.distance;
        commands.spawn((
            RigidBody::Dynamic,
            Collider::sphere(0.3),
            Restitution::new(0.6),
            Friction::new(0.4),
            Mesh3d(meshes.add(Sphere::new(0.3))),
            MeshMaterial3d(materials.add(Color::srgb(0.9, 0.8, 0.2))),
            Transform::from_translation(hit_point + hit.normal * 0.4),
        ));
    }
}

// ---- 按空格：从相机位置向前发射一个高速球 ----
fn launch_ball_on_space(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }
    let forward = camera.1.forward();
    commands.spawn((
        RigidBody::Dynamic,
        Collider::sphere(0.3),
        Restitution::new(0.7),
        Friction::new(0.3),
        Mesh3d(meshes.add(Sphere::new(0.3))),
        MeshMaterial3d(materials.add(Color::srgb(0.9, 0.4, 0.3))),
        Transform::from_translation(camera.1.translation() + forward * 2.0),
        LinearVelocity(forward * 25.0),
    ));
}

// ---- 定时从空中掉球 ----
fn drop_ball_periodically(
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rng: Local<Rng>,
) {
    timer.0.tick(time.delta());
    if !timer.0.just_finished() {
        return;
    }
    let x = (next_rand(&mut rng) - 0.5) * 8.0;
    let z = (next_rand(&mut rng) - 0.5) * 8.0;
    commands.spawn((
        RigidBody::Dynamic,
        Collider::sphere(0.4),
        Restitution::new(0.3),
        Friction::new(0.5),
        Mesh3d(meshes.add(Sphere::new(0.4))),
        MeshMaterial3d(materials.add(Color::srgb(0.4, 0.7, 0.9))),
        Transform::from_xyz(x, 8.0, z),
    ));
}

// ---- 用 Gizmos 画出传感器区域线框（辅助观察）----
fn draw_sensor_bounds(mut gizmos: Gizmos) {
    gizmos.cube(
        Transform::from_xyz(0.0, 3.0, 0.0),
        Color::srgba(1.0, 0.3, 0.3, 0.9),
    );
}

// 简易伪随机数（教学示例避免引入 rand 依赖）
#[derive(Default)]
struct Rng(u32);

fn next_rand(rng: &mut Rng) -> f32 {
    rng.0 = rng.0.wrapping_mul(1664525).wrapping_add(1013904223);
    (rng.0 >> 8) as f32 / 16_777_216.0 // 0.0 ~ 1.0
}
