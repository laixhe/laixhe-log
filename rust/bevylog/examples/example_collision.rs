//! Bevy 0.19 入门示例：演示物理碰撞（手动实现的圆形碰撞检测与响应）。
//! 多个球在屏幕内自由移动，碰到边界反弹，球与球之间也会弹性碰撞。
//!
//! 学习重点：
//! - 速度作为组件存储：Velocity(Vec2) 让每个球有独立运动方向
//! - 边界碰撞：检查位置是否超出屏幕范围，反弹时反转对应轴的速度
//! - 圆形碰撞检测：两圆心距离 < 半径之和 → 发生碰撞
//! - 性能优化：用 distance_squared 避免开方运算（比较平方等价于比较距离）
//! - 碰撞响应：沿碰撞法线分离重叠 + 交换法线方向速度（等质量弹性碰撞）
//! - query.iter_combinations_mut() 遍历实体对，处理两两交互
//! - .chain() 排序多个系统（保证移动 → 边界碰撞 → 球间碰撞的执行顺序）

use bevy::{prelude::*, text::FontSourceTemplate};
use bevy::window::PrimaryWindow;

// 中文字体路径
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        // 背景色设为黑色，让彩色球更醒目
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        // 三个系统按顺序执行：先移动，再处理边界碰撞，最后处理球间碰撞。
        // 用 .chain() 保证顺序：否则系统可能以任意顺序运行，导致穿墙或穿透。
        .add_systems(
            Update,
            (move_balls, handle_wall_collisions, handle_ball_collisions).chain(),
        )
        .run()
}

// 球组件：存储半径，用于碰撞检测和渲染。
#[derive(Component)]
struct Ball {
    radius: f32,
}

// 速度组件：每帧移动的方向和速度（像素/秒）。
// 用元组结构体包装 Vec2，让速度成为独立的组件，可以挂载到任何实体上。
#[derive(Component)]
struct Velocity(Vec2);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 生成 2D 相机（必须有一个 Camera2d 才能看到画面）
    commands.spawn(Camera2d);

    // 4 个球：不同位置、速度、颜色。
    // 数组元素：(初始位置, 初始速度, 半径, 颜色)
    // 左上↗ 和 右上↖ 会向中间靠拢 → 上方碰撞；左下↘ 和 右下↙ → 下方碰撞
    let balls: [(Vec2, Vec2, f32, Color); 4] = [
        (Vec2::new(-250.0, 150.0), Vec2::new(220.0, 100.0), 40.0, Color::srgb(0.2, 0.6, 1.0)), // 蓝
        (Vec2::new(250.0, 150.0), Vec2::new(-180.0, 160.0), 40.0, Color::srgb(0.9, 0.3, 0.3)), // 红
        (Vec2::new(-250.0, -150.0), Vec2::new(200.0, -140.0), 40.0, Color::srgb(0.3, 0.9, 0.4)), // 绿
        (Vec2::new(250.0, -150.0), Vec2::new(-210.0, -120.0), 40.0, Color::srgb(0.9, 0.8, 0.2)), // 黄
    ];

    for (position, velocity, radius, color) in balls {
        commands.spawn((
            // Circle::new(radius) 的参数是半径（像素）
            Mesh2d(meshes.add(Circle::new(radius))),
            MeshMaterial2d(materials.add(ColorMaterial::from(color))),
            Transform::from_xyz(position.x, position.y, 0.0),
            Ball { radius },
            Velocity(velocity),
        ));
    }

    // 底部提示文本（spawn_scene + bsn! 宏声明式构建实体）
    commands.spawn_scene(bsn! {
        Text2d::new("物理碰撞：球碰边界反弹 + 球间弹性碰撞")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(30.0),
        }
        Transform::from_xyz(0.0, -300.0, 0.0)
    });
}

// 移动系统：根据速度更新球的位置。
fn move_balls(
    time: Res<Time>,
    mut balls: Query<(&Velocity, &mut Transform), With<Ball>>,
) {
    let dt = time.delta_secs();
    for (velocity, mut transform) in &mut balls {
        // 位移 = 速度 × 时间（帧率无关移动）。
        // velocity.0 是 Vec2（2D），extend(0.0) 转成 Vec3 匹配 Transform::translation。
        transform.translation += velocity.0.extend(0.0) * dt;
    }
}

// 边界碰撞系统：球碰到屏幕边缘时反弹。
fn handle_wall_collisions(
    // 获取主窗口尺寸：With<PrimaryWindow> 过滤出主窗口
    window: Single<&Window, With<PrimaryWindow>>,
    mut balls: Query<(&Ball, &mut Transform, &mut Velocity)>,
) {
    // Camera2d 原点在屏幕中心，所以世界坐标边界是 ±宽/2 和 ±高/2。
    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;

    for (ball, mut transform, mut velocity) in &mut balls {
        let r = ball.radius;
        // X 轴边界：球右边缘超过屏幕右边界 → 反弹
        if transform.translation.x + r > half_width {
            // 把球推回边界内（防止卡在墙里）
            transform.translation.x = half_width - r;
            // 反转 X 方向速度，并用 abs() 确保方向朝内（向左为负）
            velocity.0.x = -velocity.0.x.abs();
        } else if transform.translation.x - r < -half_width {
            transform.translation.x = -half_width + r;
            velocity.0.x = velocity.0.x.abs(); // 确保方向朝内（向右为正）
        }
        // Y 轴边界：同理
        if transform.translation.y + r > half_height {
            transform.translation.y = half_height - r;
            velocity.0.y = -velocity.0.y.abs(); // 朝下为负
        } else if transform.translation.y - r < -half_height {
            transform.translation.y = -half_height + r;
            velocity.0.y = velocity.0.y.abs(); // 朝上为正
        }
    }
}

// 球间碰撞系统：检测并响应球与球的碰撞。
fn handle_ball_collisions(
    mut balls: Query<(&Ball, &mut Transform, &mut Velocity)>,
) {
    // iter_combinations_mut 遍历所有不重复的实体对（i < j），
    // 每次返回两个可变引用，用于处理两两交互（如碰撞检测）。
    // 相比双重 for 循环，它保证不会重复处理 (a,b) 和 (b,a)。
    let mut combinations = balls.iter_combinations_mut();
    while let Some([(ball_a, mut transform_a, mut vel_a), (ball_b, mut transform_b, mut vel_b)]) =
        combinations.fetch_next()
    {
        // 球心之间的向量（A - B），truncate() 把 Vec3 转成 Vec2（去掉 z 分量）
        let delta = transform_a.translation - transform_b.translation;
        let distance_sq = delta.truncate().length_squared();
        let min_distance = ball_a.radius + ball_b.radius;

        // 碰撞检测：两圆心距离 < 半径之和 → 发生碰撞。
        // 用 distance_squared 避免开方运算（性能优化）：
        // 比较 a² < b² 等价于比较 a < b（当 a, b 都为正数时）。
        // distance_sq > 0.0 防止两球完全重合时除以零（无法计算法线方向）。
        if distance_sq < min_distance * min_distance && distance_sq > 0.0 {
            let distance = distance_sq.sqrt();
            // 碰撞法线：从 B 指向 A 的单位向量（向量 / 长度 = 单位向量）
            let normal = delta.truncate() / distance;

            // 1. 分离重叠：把两球沿法线方向各推开一半，消除穿透。
            //    不分离的话球会粘在一起反复触发碰撞。
            let overlap = min_distance - distance;
            let separation = normal * (overlap * 0.5);
            transform_a.translation += separation.extend(0.0);
            transform_b.translation -= separation.extend(0.0);

            // 2. 交换法线方向的速度分量（等质量弹性碰撞）。
            //    相对速度 = A 的速度 - B 的速度
            let rel_velocity = vel_a.0 - vel_b.0;
            // 相对速度在法线上的投影（标量，有正负）。
            let vel_along_normal = rel_velocity.dot(normal);

            // 只在两球正在靠近时才交换速度（避免重复处理已分离的球）。
            // vel_along_normal < 0 表示 A 正朝 B 移动
            // （法线从 B 指向 A，A 朝 B 移动时投影为负）。
            if vel_along_normal < 0.0 {
                // 等质量弹性碰撞：沿法线方向交换速度分量。
                // impulse 是需要交换的速度量：从 A 减去，加到 B。
                // 切线方向速度不变（只有法线方向交换），这是弹性碰撞的物理特性。
                let impulse = normal * vel_along_normal;
                vel_a.0 -= impulse;
                vel_b.0 += impulse;
            }
        }
    }
}
