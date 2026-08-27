//! Bevy 0.19 入门示例：avian2d —— 关节（Joint）。
//!
//! 关节把两个刚体“钉”在一起，限制它们的相对运动，是模拟机械结构的基础：
//! - RevoluteJoint 铰链关节：绕指定轴旋转（2D 里就是绕屏幕平面旋转）→ 摆锤
//! - DistanceJoint  距离关节：保持两点间距离恒定（像绳子/弹簧）→ 绳链
//! - PrismaticJoint 棱柱关节：只允许沿一个轴滑动 → 滑轨
//!
//! 关节是独立的实体：在 commands.spawn(...) 里传入关节组件，指定 body1 / body2
//! 两个刚体实体，再用 with_local_anchorN 设置各自局部坐标下的锚点位置。
//!
//! 操作：观察三种关节各自的约束效果（摆锤一直在摆、绳链在荡、滑轨在来回滑），
//! 按空格再给摆锤补一个向上的冲量。

use avian2d::prelude::*;
use bevy::{prelude::*, text::FontSourceTemplate};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity(Vec2::new(0.0, -600.0)))
        .insert_resource(ClearColor(Color::srgb(0.06, 0.07, 0.1)))
        // 每 1.2 秒把滑轨方块的速度反向，让它来回滑动
        .insert_resource(SlideTimer(Timer::from_seconds(1.2, TimerMode::Repeating)))
        .insert_resource(SlideDirection(1))
        .add_systems(Startup, setup)
        .add_systems(Update, (kick_pendulum, flip_slider_direction))
        .run()
}

// 标记摆锤的杆，方便空格时对它施加冲量
#[derive(Component)]
struct PendulumRod;

// 标记滑轨方块，方便定时反向它的速度
#[derive(Component)]
struct SliderBox;

// 滑轨方块速度反向计时器
#[derive(Resource)]
struct SlideTimer(Timer);

// 滑轨方块当前运动方向（1 = 右，-1 = 左）。
// 不能直接对当前速度取反：撞上限位后速度已被物理引擎归零，-0 还是 0。
#[derive(Resource, Default)]
struct SlideDirection(i32);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // 顶部横梁：所有关节都挂在这条静态刚体上
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(900.0, 20.0),
        Transform::from_xyz(0.0, 280.0, 0.0),
    ));

    // ---- 1. 摆锤：RevoluteJoint（铰链）----
    // 静态锚点 + 动态杆，铰链钉在杆的顶端。杆初始倾斜 60°，重力会让它立刻摆动起来。
    let pivot = Vec2::new(0.0, 220.0);
    let angle = 60f32.to_radians();
    // 杆中心 = 铰链点 - 旋转后的「杆顶端到中心的向量」，保证旋转后顶端仍对准铰链点
    let rod_center = pivot - Vec2::from_angle(angle).rotate(Vec2::new(0.0, 75.0));
    let anchor = commands
        .spawn((
            RigidBody::Static,
            Transform::from_xyz(pivot.x, pivot.y, 0.0),
        ))
        .id();
    let rod = commands
        .spawn((
            RigidBody::Dynamic,
            Collider::rectangle(12.0, 150.0),
            Restitution::new(0.2),
            SleepingDisabled, // 防止摆到速度最低点（端点）时被睡眠系统冻住
            Mesh2d(meshes.add(Rectangle::new(12.0, 150.0))),
            MeshMaterial2d(materials.add(Color::srgb(0.9, 0.6, 0.2))),
            Transform::from_xyz(rod_center.x, rod_center.y, 0.0)
                .with_rotation(Quat::from_rotation_z(angle)),
            PendulumRod,
        ))
        .id();
    commands.spawn(
        RevoluteJoint::new(anchor, rod)
            .with_local_anchor1(Vec2::ZERO)          // 锚点：锚实体原点
            .with_local_anchor2(Vec2::new(0.0, 75.0)), // 杆顶端（杆高 150，局部坐标 +75）
    );

    // ---- 2. 绳链：DistanceJoint（距离约束）----
    // 一串圆球从上往下，每相邻两个用距离约束连接，像一条绳子。
    let chain_anchor = commands
        .spawn((
            RigidBody::Static,
            Transform::from_xyz(-260.0, 220.0, 0.0),
        ))
        .id();
    let spacing = 45.0;
    let mut prev = chain_anchor;
    for i in 0..6 {
        let mut bundle = (
            RigidBody::Dynamic,
            Collider::circle(15.0),
            Restitution::new(0.1),
            SleepingDisabled, // 防止链节在摆动最低点瞬间被睡眠系统冻结
            Mesh2d(meshes.add(Circle::new(15.0))),
            MeshMaterial2d(materials.add(Color::srgb(0.3, 0.8, 0.9))),
            Transform::from_xyz(-260.0, 220.0 - spacing * (i + 1) as f32, 0.0),
        );
        // 最上面一个链节给一个横向初速度，让整条绳链荡起来
        if i == 0 {
            bundle.6.translation.x += 60.0;
        }
        let mut builder = commands.spawn(bundle);
        if i == 0 {
            builder.insert(LinearVelocity(Vec2::new(220.0, 0.0)));
        }
        let ball = builder.id();
        // 与上一个链节用距离约束连接；limits 上下限留一点余量 = 略松弛的绳子
        commands.spawn((
            DistanceJoint::new(prev, ball).with_limits(spacing * 0.85, spacing * 1.15),
            JointCollisionDisabled, // 相邻链节之间不产生碰撞（否则会互相推开）
        ));
        prev = ball;
    }

    // ---- 3. 滑轨：PrismaticJoint（棱柱关节）----
    // 方块只能沿 X 轴滑动，不能上下移动、不能旋转，滑动范围 ±160。
    let rail_anchor = commands
        .spawn((
            RigidBody::Static,
            Transform::from_xyz(260.0, 100.0, 0.0),
        ))
        .id();
    let slider = commands
        .spawn((
            RigidBody::Dynamic,
            Collider::rectangle(40.0, 40.0),
            Restitution::new(0.5),
            SleepingDisabled, // 撞上滑轨限位后速度会被归零，禁睡才能被定时反向继续滑动
            Mesh2d(meshes.add(Rectangle::new(40.0, 40.0))),
            MeshMaterial2d(materials.add(Color::srgb(0.6, 0.9, 0.4))),
            Transform::from_xyz(260.0, 100.0, 0.0),
            LinearVelocity(Vec2::new(150.0, 0.0)), // 初始向右滑
            SliderBox,
        ))
        .id();
    commands.spawn(
        PrismaticJoint::new(rail_anchor, slider)
            .with_slider_axis(Vec2::X)
            .with_local_anchor1(Vec2::ZERO)
            .with_local_anchor2(Vec2::ZERO)
            .with_limits(-160.0, 160.0), // 滑动范围限制
    );

    // 提示文本
    commands.spawn_scene(bsn! {
        Text2d::new("左：摆锤(铰链)  中：绳链(距离)  右：滑轨(棱柱) | 空格给摆锤一个冲量")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(22.0),
        }
        Transform::from_xyz(0.0, 300.0, 0.0)
    });
}

// ---- 按空格：给摆锤的杆一个向上的冲量，让它摆动起来 ----
fn kick_pendulum(keyboard: Res<ButtonInput<KeyCode>>, mut rods: Query<Forces, With<PendulumRod>>) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }
    for mut forces in &mut rods {
        forces.apply_linear_impulse(Vec2::new(0.0, 300.0));
    }
}

// ---- 定时反向滑轨方块的速度：让它沿导轨来回滑动 ----
fn flip_slider_direction(
    time: Res<Time>,
    mut timer: ResMut<SlideTimer>,
    mut direction: ResMut<SlideDirection>,
    mut sliders: Query<&mut LinearVelocity, With<SliderBox>>,
) {
    timer.0.tick(time.delta());
    if !timer.0.just_finished() {
        return;
    }
    // 翻转方向并写入绝对目标速度（不能用 -当前速度，见 SlideDirection 注释）
    direction.0 = -direction.0;
    let target = direction.0 as f32 * 150.0;
    for mut velocity in &mut sliders {
        velocity.0.x = target;
    }
}
