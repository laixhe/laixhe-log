//! Bevy 0.19 入门示例：演示粒子系统（烟花爆炸效果）。
//! 鼠标左键点击在点击位置生成烟花爆炸，粒子向外辐射并受重力下落，
//! 颜色和大小随生命衰减，生命耗尽自动销毁。
//!
//! 学习重点：
//! - 粒子作为实体：每个粒子是独立实体，带 Particle + Velocity + Transform 组件
//! - 生命周期管理：Particle 组件存储 lifetime，每帧递减，归零时 despawn
//! - 年龄比例：1 - lifetime / max_lifetime 计算 0.0~1.0 的年龄，驱动颜色淡出和缩放
//! - 重力模拟：每帧给速度 y 分量叠加重力加速度，实现抛物线轨迹
//! - 共享网格资源：所有粒子复用同一个 Circle mesh（clone Handle 不复制底层数据）
//! - rand crate 随机数：用 rand::rng() 生成伪随机角度、颜色、速度和生命
//! - Assets 资源修改：通过 Handle 从 Assets<T> 中获取可变引用，修改材质属性

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
// rand 0.10 的 trait 结构：random::<T>() / random_range(a..b) 这两个方法
// 都在 RngExt trait 上（不在 Rng trait 上），所以只 import RngExt。
// 额外 use rand::Rng 会触发 unused import 警告。
use rand::RngExt;

// 重力加速度（像素/秒²）：正值表示向下加速度。
// 注意 Bevy 2D 默认坐标系 +Y 朝上，所以"向下"加速度会减小速度的 y 分量，
// 代码中用 velocity.y -= GRAVITY * dt 方式更新（见 update_particles 的重力更新步）。
const GRAVITY: f32 = 300.0;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        // 背景色设为黑色，让彩色粒子更醒目
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        // 两个系统都请求 ResMut<Assets<ColorMaterial>> 独占写访问，
        // Bevy 检测到冲突会自动串行化（不并行运行），但执行顺序不确定。
        // 不需要 .chain()：新粒子通过 Commands 生成（帧结束才生效），
        // 本帧 spawn 的粒子本帧不会被 update_particles 的 Query 看到，
        // 所以谁先执行结果都一致。
        .add_systems(Update, (spawn_firework, update_particles))
        .run()
}

// 粒子组件：存储生命信息，用于控制粒子的存续和衰减。
#[derive(Component)]
struct Particle {
    // 剩余生命（秒）：每帧递减，归零时实体被销毁
    lifetime: f32,
    // 总生命（秒）：记录初始生命值，用于计算年龄比例（0.0 = 刚生成, 1.0 = 即将消失）
    max_lifetime: f32,
}

// 速度组件：每帧移动的方向和速度（像素/秒）。
#[derive(Component)]
struct Velocity(Vec2);

// 共享网格资源：所有粒子复用同一个 Circle mesh，避免每个粒子都创建一份网格数据。
#[derive(Resource)]
struct ParticleMesh(Handle<Mesh>);

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    // 生成 2D 相机（必须有一个 Camera2d 才能看到画面）
    commands.spawn(Camera2d);

    // 创建共享粒子网格：半径 4 像素的小圆点。
    // 后续每个粒子 clone 这个 Handle（引用计数，不复制底层网格数据）。
    let mesh = meshes.add(Circle::new(4.0));
    commands.insert_resource(ParticleMesh(mesh));

    // 底部提示文本
    commands.spawn((
        Text2d::new("粒子系统：左键点击放烟花"),
        TextFont {
            font_size: FontSize::Px(30.0),
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, -300.0, 0.0),
    ));
}

// 烟花生成系统：鼠标左键点击时在鼠标位置生成一束粒子。
fn spawn_firework(
    // 鼠标按键状态
    mouse: Res<ButtonInput<MouseButton>>,
    // 主窗口：用于获取鼠标坐标和窗口尺寸
    window: Single<&Window, With<PrimaryWindow>>,
    // 共享粒子网格：所有粒子复用这个 mesh
    mesh: Res<ParticleMesh>,
    mut commands: Commands,
    // 材质资源：每个粒子需要独立的 ColorMaterial（颜色和透明度不同）
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // just_pressed 检测「本帧刚按下」，避免按住时连续生成
    // let 链（edition 2024）：if 条件中可以用 && 连接 let 模式匹配
    if mouse.just_pressed(MouseButton::Left)
        && let Some(cursor) = window.cursor_position()
    {
        // 窗口坐标 → 世界坐标（注意 y 轴方向相反！）：
        // - 窗口坐标：原点左上角，+y 朝下
        // - 2D 世界坐标：原点屏幕中心（Camera2d 默认在原点），+y 朝上
        // 所以 x 方向直接减半宽，y 方向要用半高减 cursor.y（翻转方向）。
        let half = Vec2::new(window.width() / 2.0, window.height() / 2.0);
        let world_pos = Vec2::new(cursor.x - half.x, half.y - cursor.y);

        // 随机数生成器：rand::rng() 返回线程本地的 RNG，每次调用都不同
        let mut rng = rand::rng();

        // 生成 24 个粒子，均匀分布在圆周上，速度和颜色有随机变化
        const COUNT: usize = 24;
        for i in 0..COUNT {
            // 基础角度：均匀分布在整个圆周（0 ~ 2π，TAU = 2π）
            let base_angle = (i as f32 / COUNT as f32) * std::f32::consts::TAU;
            // 随机偏移：让分布不完全规则，更自然。rng.random::<f32>() 返回 [0, 1)
            let angle = base_angle + (rng.random::<f32>() - 0.5) * 0.4;
            // 随机速度：150~280 像素/秒。random_range(a..b) 是左闭右开区间 [a, b)：
            //   - 最小值 150 可达，最大值 280 不可达（最大约 279.999...）
            //   - Rust Range 语法 `a..b` 天生是左闭右开（不含右端点）
            let speed = rng.random_range(150.0..280.0);
            let velocity = Vec2::new(angle.cos(), angle.sin()) * speed;
            // 随机颜色：HSL 色相 0~360°，饱和度 0.8，亮度 0.6
            //   Range [0, 360) 最大值 360 不可达（HSL 360° = 0°，所以也没问题）
            let color = Color::hsl(rng.random_range(0.0..360.0), 0.8, 0.6);
            // 随机生命：0.8~1.5 秒。左闭右开 [0.8, 1.5)，最大约 1.4999...s
            let lifetime = rng.random_range(0.8..1.5);

            commands.spawn((
                // clone Handle（引用计数，不复制网格数据）
                Mesh2d(mesh.0.clone()),
                // 每个粒子独立的材质（颜色和透明度会随时间变化）
                MeshMaterial2d(materials.add(ColorMaterial::from(color))),
                Transform::from_xyz(world_pos.x, world_pos.y, 0.0),
                Particle {
                    lifetime,
                    max_lifetime: lifetime,
                },
                Velocity(velocity),
            ));
        }

        info!("[粒子] 烟花爆炸 at ({:.0}, {:.0})", world_pos.x, world_pos.y);
    }
}

// 粒子更新系统：移动粒子、应用重力、衰减生命、淡出颜色、缩小尺寸、销毁过期粒子。
fn update_particles(
    time: Res<Time>,
    mut commands: Commands,
    // 材质资源：用于修改粒子的颜色透明度
    mut materials: ResMut<Assets<ColorMaterial>>,
    // 查询所有粒子实体：需要 Entity（销毁）+ Particle（生命）+ Velocity（速度）
    // + Transform（位置/缩放）+ MeshMaterial2d（材质句柄，用于修改颜色）
    mut particles: Query<(
        Entity,
        &mut Particle,
        &mut Velocity,
        &mut Transform,
        &MeshMaterial2d<ColorMaterial>,
    )>,
) {
    let dt = time.delta_secs();

    for (entity, mut particle, mut velocity, mut transform, material_handle) in &mut particles {
        // 1. 衰减生命
        particle.lifetime -= dt;

        // 2. 生命耗尽 → 销毁实体
        //    commands 在帧结束时执行，不会立即从查询中移除，所以用 continue 跳过后续更新
        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }

        // 3. 应用重力：Bevy 2D +Y 朝上，向下加速度会减小 y 速度分量，所以用 -=
        velocity.0.y -= GRAVITY * dt;

        // 4. 更新位置：位移 = 速度 × 时间
        transform.translation += velocity.0.extend(0.0) * dt;

        // 5. 计算年龄比例（0.0 = 刚生成, 1.0 = 即将消失）
        let age = 1.0 - (particle.lifetime / particle.max_lifetime);

        // 6. 颜色淡出：alpha 随年龄从 1.0 衰减到 0.0
        //    通过材质句柄从 Assets 中获取可变引用，修改 color 的 alpha 通道
        if let Some(mut material) = materials.get_mut(material_handle.0.id()) {
            material.color = material.color.with_alpha(1.0 - age);
        }

        // 7. 缩小尺寸：scale 随年龄从 1.0 缩小到 0.2
        let scale = 1.0 - age * 0.8;
        transform.scale = Vec3::splat(scale);
    }
}
