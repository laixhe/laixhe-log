//! Bevy 0.19 入门示例：avian2d —— 2D 物理引擎基础。
//!
//! 之前的手写碰撞示例（example_2d_collision）是自己实现物理，而 avian2d 是一个
//! 完整成熟的 2D 物理引擎：注册一个插件，加几个组件，碰撞、重力、反弹全自动搞定。
//!
//! 学习重点：
//! - PhysicsPlugins：一行注册整个物理引擎（固定时间步 + 求解器 + 碰撞检测 + 空间查询）
//! - Gravity 资源：全局重力，可覆盖默认值（默认 -9.81 m/s²，此处调大更直观）
//! - RigidBody：Static（静态，质量无穷大，不移动）/ Dynamic（动态，受重力/碰撞影响）
//! - Collider：物理碰撞体形状（rectangle 矩形 / circle 圆），和渲染用的网格完全独立
//! - Restitution：反弹系数（0 = 撞上就停，1 = 完全弹性反弹）
//! - Friction：摩擦系数（物体在地面上滑动时受到的减速）
//! - Forces 查询参数：apply_linear_impulse 施加瞬间冲量（直接改变速度）
//! - CollisionStart 消息：两个碰撞体开始接触时触发（需 CollisionEventsEnabled 组件）
//!
//! 操作：
//! - 点击左键：在光标处生成一个随机形状 / 反弹系数的物体
//! - 按空格：给场上所有动态物体施加一个随机方向的小冲量
//! - 按 C：清空所有动态物体（地面和墙保留）

use avian2d::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::{prelude::*, text::FontSourceTemplate};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        // 注册物理引擎（核心！一切物理功能都来自这个插件组）
        .add_plugins(PhysicsPlugins::default())
        // 覆盖默认重力。2D 场景用 Vec2（y 向下为负）。
        .insert_resource(Gravity(Vec2::new(0.0, -600.0)))
        .insert_resource(ClearColor(Color::srgb(0.06, 0.07, 0.1)))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                spawn_on_click,
                apply_impulse_on_space,
                clear_on_c,
                log_collisions,
            ),
        )
        .run()
}

// ---- 物体形状 ----
#[derive(Clone, Copy)]
enum Shape {
    Ball, // 圆
    Box,  // 方
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // 地面：Static 刚体 + 矩形碰撞体。摩擦 0.8（物体落地减速），反弹 0.4。
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(700.0, 30.0),
        Friction::new(0.8),
        Restitution::new(0.4),
        Transform::from_xyz(0.0, -260.0, 0.0),
    ));
    // 左右墙：Static 刚体 + 高反弹（球撞墙弹得更欢）
    for x in [-360.0, 360.0] {
        commands.spawn((
            RigidBody::Static,
            Collider::rectangle(30.0, 600.0),
            Restitution::new(0.7),
            Transform::from_xyz(x, 0.0, 0.0),
        ));
    }

    // 几个初始演示物体（不同形状 / 反弹系数）
    spawn_body(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec2::new(-150.0, 100.0),
        Shape::Ball,
        0.9,
        Color::srgb(0.9, 0.4, 0.3),
    );
    spawn_body(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec2::new(-40.0, 150.0),
        Shape::Box,
        0.1,
        Color::srgb(0.4, 0.7, 0.9),
    );
    spawn_body(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec2::new(80.0, 60.0),
        Shape::Ball,
        0.5,
        Color::srgb(0.5, 0.9, 0.4),
    );

    // 提示文本
    commands.spawn_scene(bsn! {
        Text2d::new("点击生成物体 | 空格施加冲量 | C 清空")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(24.0),
        }
        Transform::from_xyz(0.0, 300.0, 0.0)
    });
}

// ---- 生成一个动态物体：刚体 + 碰撞体 + 渲染网格，三样缺一不可 ----
fn spawn_body(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    pos: Vec2,
    shape: Shape,
    restitution: f32,
    color: Color,
) {
    // 根据形状决定碰撞体和渲染网格（两者尺寸保持一致）
    let (collider, mesh) = match shape {
        Shape::Ball => (Collider::circle(25.0), meshes.add(Circle::new(25.0))),
        Shape::Box => (
            Collider::rectangle(50.0, 50.0),
            meshes.add(Rectangle::new(50.0, 50.0)),
        ),
    };

    commands.spawn((
        RigidBody::Dynamic,
        collider,
        Restitution::new(restitution),
        Friction::new(0.6),
        // 开启碰撞事件，这样 log_collisions 才能收到 CollisionStart
        CollisionEventsEnabled,
        Mesh2d(mesh),
        MeshMaterial2d(materials.add(ColorMaterial::from(color))),
        Transform::from_xyz(pos.x, pos.y, 0.0),
    ));
}

// ---- 点击左键：在光标处生成随机物体 ----
fn spawn_on_click(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut rng: Local<Rng>,
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

    let shape = if next_rand(&mut rng) < 0.5 {
        Shape::Ball
    } else {
        Shape::Box
    };
    let restitution = 0.1 + next_rand(&mut rng) * 0.8;
    let color = Color::srgb(next_rand(&mut rng), next_rand(&mut rng), next_rand(&mut rng));
    spawn_body(
        &mut commands,
        &mut meshes,
        &mut materials,
        pos,
        shape,
        restitution,
        color,
    );
}

// ---- 按空格：给所有动态物体一个随机方向的小冲量 ----
fn apply_impulse_on_space(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut bodies: Query<(Forces, &RigidBody)>,
    mut rng: Local<Rng>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }
    for (mut forces, body) in &mut bodies {
        if !body.is_dynamic() {
            continue; // 静态刚体（地面/墙）不需要冲量
        }
        let impulse = Vec2::new(
            (next_rand(&mut rng) - 0.5) * 800.0,
            next_rand(&mut rng) * 600.0,
        );
        // 冲量 = 瞬间改变速度（力则是持续作用）。数值上是 Δv = 冲量 / 质量。
        forces.apply_linear_impulse(impulse);
    }
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

// 简易伪随机数（教学示例避免引入 rand 依赖）
#[derive(Default)]
struct Rng(u32);

fn next_rand(rng: &mut Rng) -> f32 {
    rng.0 = rng.0.wrapping_mul(1664525).wrapping_add(1013904223);
    (rng.0 >> 8) as f32 / 16_777_216.0 // 0.0 ~ 1.0
}
