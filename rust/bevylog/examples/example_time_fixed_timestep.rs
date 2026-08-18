//! Bevy 0.19 入门示例：演示 FixedUpdate 固定时间步长。
//! 一个圆在底部反弹，物理逻辑用 FixedUpdate（固定 dt）驱动，与渲染帧率解耦。
//!
//! 学习重点：
//! - Time::<Fixed>::from_hz 设置固定步长（如 60Hz，即每 1/60 秒一步）
//! - FixedUpdate 调度：以固定时间步长运行系统，物理 / 游戏逻辑应与帧率解耦
//! - Res<Time>（Update 里是真实可变时间）vs Res<Time<Fixed>>（固定步长时间）
//! - 在 FixedUpdate 里 time.delta_secs() 恒为固定步长（1/60），不随帧率变化

use bevy::window::PrimaryWindow;
use bevy::{prelude::*, text::FontSourceTemplate};

// 中文字体路径
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// 球标记
#[derive(Component)]
struct Ball;

// 速度组件（像素/秒）
#[derive(Component)]
struct Velocity(Vec2);

// 信息文字标记
#[derive(Component, Clone, Default)]
struct InfoText;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK))
        // 固定时间步长：60Hz（每 1/60 秒推进一次 FixedUpdate）
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_systems(Startup, setup)
        // 物理逻辑放 FixedUpdate，用固定步长推进（与渲染帧率解耦）
        .add_systems(FixedUpdate, apply_physics)
        // 显示更新放 Update（每帧跟随渲染）
        .add_systems(Update, update_display)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // 球：初始在屏幕中上方，带向下速度
    commands.spawn((
        Ball,
        Mesh2d(meshes.add(Circle::new(25.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.2, 0.8, 0.4)))),
        Transform::from_xyz(0.0, 200.0, 0.0),
        Velocity(Vec2::new(0.0, -200.0)),
    ));

    // 底部信息文字
    commands.spawn_scene(bsn! {
        InfoText
        Text2d::new("固定步长：")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(24.0),
        }
        Transform::from_xyz(0.0, -280.0, 0.0)
    });
}

// 物理：用固定步长推进（重力 + 底部反弹）
fn apply_physics(
    // FixedUpdate 里的固定步长时间（delta_secs 恒为 1/60）
    time: Res<Time<Fixed>>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    // 系统本地状态：只在第一次运行时打印一次固定步长，便于确认
    mut logged: Local<bool>,
) {
    let dt = time.delta_secs();
    if !*logged {
        *logged = true;
        info!("[物理] 固定步长 dt = {:.6}s", dt);
    }
    for (mut transform, mut velocity) in &mut query {
        // 重力（向下加速，2D 中 +Y 朝上）
        velocity.0.y -= 900.0 * dt;
        // 位移 = 速度 × 时间
        transform.translation += velocity.0.extend(0.0) * dt;

        // 底部反弹（球半径为 25）
        let bottom = -window.height() / 2.0 + 25.0;
        if transform.translation.y < bottom {
            transform.translation.y = bottom;
            velocity.0.y = velocity.0.y.abs(); // 反转方向向上弹
        }
    }
}

// 显示：更新信息文字（真实运行时间 vs 固定步长）。
// 注意：这里每 1 秒才更新一次，而不是每帧更新——既避免 CJK 文本每帧重排刷屏 ICU4X 警告，
// 也示范「不要每帧做不必要的工作」。
fn update_display(
    real_time: Res<Time>,         // Update 里的真实时间（可变，随帧率波动）
    fixed_time: Res<Time<Fixed>>, // 固定步长时间（恒定 1/60）
    mut text: Single<&mut Text2d, With<InfoText>>,
    // 系统本地状态：记录上次更新文字的时间
    mut last_update: Local<f32>,
) {
    if real_time.elapsed_secs() - *last_update > 1.0 {
        *last_update = real_time.elapsed_secs();
        text.0 = format!(
            "固定步长: {:.4}s | 真实运行时间: {:.1}s",
            fixed_time.delta_secs(),
            real_time.elapsed_secs(),
        );
    }
}
