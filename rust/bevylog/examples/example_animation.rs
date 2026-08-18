//! Bevy 0.19 入门示例：演示动画系统（基于时间的 Transform 动画）。
//! 中心太阳脉冲缩放，3 个方形行星围绕太阳轨道运动并自转。
//!
//! 学习重点：
//! - Time::elapsed_secs() 获取应用运行总时间（秒），驱动基于时间的动画
//! - Transform 的三个字段都能做动画：translation（位置）、rotation（旋转）、scale（缩放）
//! - sin / cos 实现周期性运动（轨道运动 + 脉冲缩放）
//! - 组件存储动画参数（速度、半径等），系统读取参数计算每帧变换
//! - Quat::from_rotation_z 实现 2D 旋转（绕 Z 轴）
//! - Single 查询获取单个实体（太阳），Query 遍历多个实体（行星）

use bevy::{prelude::*, text::FontSourceTemplate};

// 中文字体路径
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        // 背景色设为黑色，让行星和太阳更醒目
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        // 两个动画系统：orbit_planets 更新行星位置/旋转，pulse_sun 更新太阳缩放。
        // 它们访问不同实体，无冲突，不需要 .chain() 排序。
        .add_systems(Update, (orbit_planets, pulse_sun))
        .run()
}

// 太阳标记组件：用于 Single 查询找到太阳实体。
// 没有字段，仅用来给实体打「这是太阳」的标签。
#[derive(Component)]
struct Sun;

// 行星组件：存储轨道动画参数。
// 演示 Bevy 惯用做法——把动画参数挂在组件上，而不是在系统里硬编码。
#[derive(Component)]
struct Planet {
    // 公转速度（弧度/秒）：每秒绕太阳转多少弧度
    orbit_speed: f32,
    // 轨道半径（像素）：离太阳中心的距离
    orbit_radius: f32,
    // 自转速度（弧度/秒）：每秒自身转多少弧度
    spin_speed: f32,
    // 初始角度（弧度）：决定行星的起始位置，让多个行星分散在不同方位
    initial_angle: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 生成 2D 相机（必须有一个 Camera2d 才能看到画面）
    commands.spawn(Camera2d);

    // 中心：太阳（黄色圆形），带 Sun 标记组件供 pulse_sun 系统查询
    commands.spawn((
        // Circle::new(40.0) 的 40.0 是半径（像素）
        Mesh2d(meshes.add(Circle::new(40.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(1.0, 0.8, 0.2)))),
        Transform::default(),
        Sun,
    ));

    // 3 个行星：不同颜色、轨道半径、公转/自转速度、初始角度
    // 数组元素：(轨道半径, 公转速度, 自转速度, 初始角度, 颜色)
    // 显式标注 f32 类型：浮点字面量默认是 f64，而 Planet 字段是 f32
    let planets: [(f32, f32, f32, f32, Color); 3] = [
        (100.0, 1.0, 3.0, 0.0, Color::srgb(0.2, 0.6, 1.0)), // 蓝色：近、快公转、快自转
        (180.0, 0.7, 2.0, 2.0, Color::srgb(0.9, 0.3, 0.3)), // 红色：中、中速
        (260.0, 0.5, 1.5, 4.0, Color::srgb(0.3, 0.9, 0.4)), // 绿色：远、慢速
    ];

    for (radius, orbit_speed, spin_speed, initial_angle, color) in planets {
        // 计算初始位置，避免第一帧行星闪在原点（太阳中心）再跳到轨道上
        let x = initial_angle.cos() * radius;
        let y = initial_angle.sin() * radius;
        commands.spawn((
            // Rectangle::new(30.0, 30.0) 的参数是完整宽高（内部存半宽高），30×30 像素方形
            Mesh2d(meshes.add(Rectangle::new(30.0, 30.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(color))),
            Transform::from_xyz(x, y, 0.0),
            Planet {
                orbit_speed,
                orbit_radius: radius,
                spin_speed,
                initial_angle,
            },
        ));
    }

    // 底部提示文本（spawn_scene + bsn! 宏声明式构建实体）
    commands.spawn_scene(bsn! {
        Text2d::new("动画系统：太阳脉冲 + 行星轨道运动 + 自转")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(30.0),
        }
        Transform::from_xyz(0.0, -300.0, 0.0)
    });
}

// 行星轨道运动系统：每帧更新行星的 translation（位置）和 rotation（旋转）。
fn orbit_planets(
    // 时间资源：elapsed_secs() 返回应用启动以来的总时间（秒），是驱动动画的常用时间源
    time: Res<Time>,
    // 查询所有行星：同时取 &Planet（只读动画参数）和 &mut Transform（可写变换）
    mut planets: Query<(&Planet, &mut Transform)>,
) {
    // 用运行总时间驱动动画：无论帧率高低，相同时间下动画进度一致（帧率无关）
    let t = time.elapsed_secs();
    for (planet, mut transform) in &mut planets {
        // 计算轨道角度：初始角度 + 时间 × 公转速度
        let angle = planet.initial_angle + t * planet.orbit_speed;
        // 用 cos/sin 计算圆形轨道上的位置（极坐标 → 直角坐标）：
        // cos(angle) = x 方向分量，sin(angle) = y 方向分量，乘以半径得到实际坐标
        transform.translation.x = angle.cos() * planet.orbit_radius;
        transform.translation.y = angle.sin() * planet.orbit_radius;
        // 自转：绕 Z 轴旋转。2D 中只有 Z 轴旋转有意义（XY 平面内的旋转）。
        // Quat::from_rotation_z 接收弧度，返回四元数（Quat）赋给 Transform::rotation
        transform.rotation = Quat::from_rotation_z(t * planet.spin_speed);
    }
}

// 太阳脉冲系统：让太阳周期性缩放（scale 动画）。
fn pulse_sun(
    time: Res<Time>,
    // Single 查询：期望恰好一个带 Sun 组件的实体，取出其 Transform 做缩放
    mut sun: Single<&mut Transform, With<Sun>>,
) {
    let t = time.elapsed_secs();
    // sin 实现周期性缩放：基础大小 1.0，振幅 0.1（±10%），频率 2 弧度/秒
    // sin 返回值范围 [-1, 1]，乘以 0.1 后范围 [-0.1, 0.1]，加 1.0 后范围 [0.9, 1.1]
    let scale = 1.0 + (t * 2.0).sin() * 0.1;
    // Vec3::splat 让 x/y/z 三个分量相同（等比缩放），避免变形
    sun.scale = Vec3::splat(scale);
}
