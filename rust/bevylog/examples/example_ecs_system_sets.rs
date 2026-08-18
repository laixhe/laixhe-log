//! Bevy 0.19 入门示例：演示 SystemSet 系统集合（显式管理系统执行顺序）。
//! 三个集合按 Input → Logic → Render 顺序执行，用移动的圆演示「顺序为何重要」。
//!
//! 学习重点：
//! - #[derive(SystemSet)] 定义系统集合（给一组系统打分组标签）
//! - configure_sets(...).chain() 声明集合之间的执行顺序
//! - in_set(...) 把系统放进指定集合
//! - 相比逐个 .chain()，SystemSet 更结构化，代码变多后仍能清晰管理顺序

use bevy::{prelude::*, text::FontSourceTemplate};

// 中文字体路径
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// 系统集合：把 Update 里的系统分成三个阶段，按顺序执行
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum AppSet {
    Input,  // 阶段1：读取输入
    Logic,  // 阶段2：更新逻辑
    Render, // 阶段3：更新显示
}

// 移动方向资源：Input 系统写入，Logic 系统读取
#[derive(Resource, Default)]
struct MoveDir(Vec2);

// 可移动实体标记
#[derive(Component)]
struct Mover;

// 位置显示文字标记
#[derive(Component, Clone, Default)]
struct PosText;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<MoveDir>()
        // 关键：声明三个集合按 Input → Logic → Render 顺序执行。
        // 如果不声明顺序，Bevy 可能以任意顺序运行它们（尽管会因为资源冲突而串行化）。
        .configure_sets(
            Update,
            (AppSet::Input, AppSet::Logic, AppSet::Render).chain(),
        )
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                read_input.in_set(AppSet::Input),
                update_position.in_set(AppSet::Logic),
                update_display.in_set(AppSet::Render),
            ),
        )
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // 可移动的圆
    commands.spawn((
        Mover,
        Mesh2d(meshes.add(Circle::new(30.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.2, 0.6, 1.0)))),
        Transform::default(),
    ));

    // 位置显示文字
    commands.spawn_scene(bsn! {
        PosText
        Text2d::new("位置：")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(24.0),
        }
        Transform::from_xyz(0.0, -250.0, 0.0)
    });
}

// 阶段1（Input）：读取键盘，写入移动方向资源
fn read_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut dir: ResMut<MoveDir>,
    // 系统本地状态：只在第一次运行时打印一次，演示三阶段按顺序执行
    mut logged: Local<bool>,
) {
    if !*logged {
        *logged = true;
        info!("[顺序] 1. Input 阶段（读取输入）");
    }
    dir.0 = Vec2::ZERO;
    if keyboard.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        dir.0.x -= 1.0;
    }
    if keyboard.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        dir.0.x += 1.0;
    }
    if keyboard.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
        dir.0.y += 1.0;
    }
    if keyboard.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
        dir.0.y -= 1.0;
    }
}

// 阶段2（Logic）：根据方向移动圆
fn update_position(
    time: Res<Time>,
    dir: Res<MoveDir>,
    mut mover: Single<&mut Transform, With<Mover>>,
    // 系统本地状态：只在第一次运行时打印一次，演示三阶段按顺序执行
    mut logged: Local<bool>,
) {
    if !*logged {
        *logged = true;
        info!("[顺序] 2. Logic 阶段（更新位置）");
    }
    if dir.0 != Vec2::ZERO {
        let delta = dir.0.normalize() * 300.0 * time.delta_secs();
        mover.translation += delta.extend(0.0);
    }
}

// 阶段3（Render）：显示最新位置
fn update_display(
    time: Res<Time>,
    dir: Res<MoveDir>,
    mover: Single<&Transform, With<Mover>>,
    mut text: Single<&mut Text2d, With<PosText>>,
    // 系统本地状态：logged 用于打印一次顺序；last_update 用于节流文字更新
    mut logged: Local<bool>,
    mut last_update: Local<f32>,
) {
    if !*logged {
        *logged = true;
        info!("[顺序] 3. Render 阶段（更新显示）");
    }
    // 每 1 秒才更新一次文字，避免 CJK 文本每帧重排刷屏 ICU4X 警告
    if time.elapsed_secs() - *last_update > 1.0 {
        *last_update = time.elapsed_secs();
        let pos = mover.translation.truncate();
        text.0 = format!(
            "位置 = ({:.0}, {:.0}) | 方向 = ({}, {})",
            pos.x, pos.y, dir.0.x, dir.0.y
        );
    }
}
