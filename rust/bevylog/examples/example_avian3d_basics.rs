//! Bevy 0.19 入门示例：avian3d —— 3D 物理引擎基础。
//!
//! 与 2D 版（example_avian2d_basics）同一套 API，只是向量从 Vec2 变成 Vec3、
//! 碰撞体变成 3D 形状（sphere 球 / cuboid 方块 / cylinder 圆柱）。
//!
//! 学习重点：
//! - PhysicsPlugins + Gravity：注册物理引擎、设置全局重力
//! - RigidBody::Static / Dynamic：静态（地面）与动态（可碰撞、受重力）
//! - Collider::sphere / cuboid / cylinder：3D 碰撞体
//! - Restitution / Friction：反弹与摩擦
//! - LinearVelocity 组件：直接给刚体一个初速度（发射物体）
//! - Forces 查询参数：apply_linear_impulse 施加瞬间冲量
//! - CollisionStart 消息：碰撞事件（需 CollisionEventsEnabled）
//!
//! 操作：
//! - 按空格：从相机位置向前发射一个高反弹的球
//! - 点击左键：在鼠标射线与地面的交点上生成一个方块
//! - 按 C：清空所有动态物体（地面保留）

use avian3d::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::{prelude::*, text::FontSource};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        // 注册 3D 物理引擎
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(ClearColor(Color::srgb(0.04, 0.05, 0.08)))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                launch_ball_on_space,
                spawn_cube_on_click,
                clear_on_c,
                log_collisions,
            ),
        )
        .run()
}

// ---- 3D 物体形状 ----
#[derive(Clone, Copy)]
enum BodyKind {
    Cube,
    Ball,
    Cylinder,
}

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

    // 地面：Static 刚体 + 方块碰撞体（宽 20，高 1，顶面在 y=0）
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(20.0, 1.0, 20.0),
        Friction::new(0.8),
        Restitution::new(0.3),
        Transform::from_xyz(0.0, -0.5, 0.0),
    ));
    // 地面的渲染网格（与碰撞体无关，纯外观）
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.15, 0.16, 0.2))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // 初始演示物体：不同形状、不同反弹系数，从空中落下堆在一起
    spawn_demo_body(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(-1.0, 1.0, -1.0),
        BodyKind::Cube,
        0.2,
        Color::srgb(0.9, 0.4, 0.3),
    );
    spawn_demo_body(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(0.5, 1.5, 0.5),
        BodyKind::Ball,
        0.8,
        Color::srgb(0.3, 0.7, 0.9),
    );
    spawn_demo_body(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(1.2, 1.0, -0.5),
        BodyKind::Cylinder,
        0.4,
        Color::srgb(0.5, 0.9, 0.4),
    );

    // 提示文字：UI 节点（屏幕空间），由 3D 相机直接渲染
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(20.0),
            top: Val::Px(20.0),
            ..default()
        },
        Text::new("空格发射球 | 点击生成方块 | C 清空 | 看控制台碰撞日志"),
        TextColor(Color::WHITE),
        TextFont {
            font: FontSource::Handle(asset_server.load(FONT_PATH)),
            font_size: FontSize::Px(20.0),
            ..default()
        },
    ));
}

// ---- 生成一个动态刚体：碰撞体 + 渲染网格 ----
fn spawn_demo_body(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    pos: Vec3,
    kind: BodyKind,
    restitution: f32,
    color: Color,
) {
    let (collider, mesh) = match kind {
        BodyKind::Cube => (
            Collider::cuboid(0.8, 0.8, 0.8),
            meshes.add(Cuboid::new(0.8, 0.8, 0.8)),
        ),
        BodyKind::Ball => (Collider::sphere(0.5), meshes.add(Sphere::new(0.5))),
        BodyKind::Cylinder => (
            Collider::cylinder(0.4, 1.0),
            meshes.add(Cylinder::new(0.4, 1.0)),
        ),
    };

    commands.spawn((
        RigidBody::Dynamic,
        collider,
        Restitution::new(restitution),
        Friction::new(0.5),
        CollisionEventsEnabled,
        Mesh3d(mesh),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: color,
            perceptual_roughness: 0.4,
            ..default()
        })),
        Transform::from_translation(pos),
    ));
}

// ---- 按空格：从相机位置向前发射一个球 ----
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
    // 相机朝向 = 发射方向
    let forward = camera.1.forward();
    commands.spawn((
        RigidBody::Dynamic,
        Collider::sphere(0.3),
        Restitution::new(0.8),
        Friction::new(0.3),
        CollisionEventsEnabled,
        Mesh3d(meshes.add(Sphere::new(0.3))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.4, 0.3),
            perceptual_roughness: 0.3,
            ..default()
        })),
        // 出生在相机前方 2 个单位处，并给一个沿相机方向的初速度
        Transform::from_translation(camera.1.translation() + forward * 2.0),
        LinearVelocity(forward * 20.0),
    ));
}

// ---- 点击左键：鼠标射线与地面(y=0)求交，在交点生成一个方块 ----
fn spawn_cube_on_click(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
    // 射线与地面平面求交（地面在 y=0）
    let Some(distance) = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y)) else {
        return;
    };
    let pos = ray.get_point(distance) + Vec3::Y * 0.4; // 抬高一点，别插进地面
    spawn_demo_body(
        &mut commands,
        &mut meshes,
        &mut materials,
        pos,
        BodyKind::Cube,
        0.3,
        Color::srgb(0.9, 0.8, 0.2),
    );
}

// ---- 按 C：清空所有动态物体 ----
fn clear_on_c(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    bodies: Query<(Entity, &RigidBody)>,
) {
    if !keyboard.just_pressed(KeyCode::KeyC) {
        return;
    }
    for (entity, body) in &bodies {
        if body.is_dynamic() {
            commands.entity(entity).despawn();
        }
    }
}

// ---- 碰撞事件：两个碰撞体开始接触时打印日志 ----
fn log_collisions(mut reader: MessageReader<CollisionStart>) {
    for event in reader.read() {
        info!(
            "碰撞开始：实体 {:?} 与 {:?}",
            event.collider1, event.collider2
        );
    }
}
