//! Bevy 0.19 入门示例：演示多窗口（多个 Window + 多相机渲染到不同窗口）。
//!
//! 创建第二个窗口，并让第二个相机渲染到它；用 RenderLayers 让两个窗口显示不同内容。
//!
//! 学习重点：
//! - spawn(Window { ... })：创建额外的窗口（默认只创建主窗口）
//! - RenderTarget::Window(WindowRef::Entity(...))：把相机渲染到指定窗口
//! - RenderLayers：给实体和相机打「层」标签，让不同相机只渲染特定层的实体
//!
//! 效果：主窗口显示红色圆，第二个窗口显示蓝色圆（各自独立旋转）。

use bevy::camera::RenderTarget;
use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use bevy::window::WindowRef;

// 红色圆标记（主窗口显示）
#[derive(Component)]
struct RedCircle;

// 蓝色圆标记（第二个窗口显示）
#[derive(Component)]
struct BlueCircle;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, spin)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 主窗口相机：只渲染 layer 0
    commands.spawn((Camera2d, RenderLayers::layer(0)));

    // 主窗口内容：红色圆（layer 0）
    commands.spawn((
        RedCircle,
        Mesh2d(meshes.add(Circle::new(80.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.9, 0.2, 0.2)))),
        RenderLayers::layer(0),
    ));

    // 创建第二个窗口（独立于主窗口）
    let second_window = commands
        .spawn(Window {
            title: "第二个窗口".to_string(),
            resolution: (400, 300).into(),
            position: WindowPosition::At(IVec2::new(900, 200)),
            ..default()
        })
        .id();

    // 第二个相机：只渲染 layer 1，输出到第二个窗口
    commands.spawn((
        Camera2d,
        RenderLayers::layer(1),
        RenderTarget::Window(WindowRef::Entity(second_window)),
    ));

    // 第二个窗口内容：蓝色圆（layer 1）
    commands.spawn((
        BlueCircle,
        Mesh2d(meshes.add(Circle::new(80.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.2, 0.4, 0.9)))),
        RenderLayers::layer(1),
    ));

    info!("[多窗口] 已创建第二个窗口，两个窗口分别渲染不同图层");
}

// 让两个圆以相反方向旋转，展示两个窗口都在独立实时渲染
fn spin(
    time: Res<Time>,
    mut red: Single<&mut Transform, (With<RedCircle>, Without<BlueCircle>)>,
    mut blue: Single<&mut Transform, (With<BlueCircle>, Without<RedCircle>)>,
) {
    red.rotation = Quat::from_rotation_z(time.elapsed_secs());
    blue.rotation = Quat::from_rotation_z(-time.elapsed_secs());
}
