//! Bevy 0.19 入门示例：演示 3D 图元集合（Mesh3d + 各种形状）。
//!
//! 学习重点：
//! - Cuboid / Sphere / Cylinder / Cone / Capsule3d / Torus 等内置 3D 图元
//! - Mesh3d + MeshMaterial3d + StandardMaterial 渲染 3D 网格
//! - 图元统一「中心在原点」，用 Transform 摆放位置
//! - Camera2d 覆盖层 + Text2d 在 3D 画面上叠加文字
//!
//! 观察：一排 3D 图元缓慢自转，下方文字标注名称。

use bevy::{prelude::*, text::FontSourceTemplate};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// 自转标记
#[derive(Component)]
struct Spin;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        .add_systems(Update, spin)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 3D 相机
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 2D 覆盖层相机：渲染在 3D 之上，不清屏，用于显示文字
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
    ));

    // 方向光
    commands.spawn((
        DirectionalLight {
            illuminance: 6000.0,
            ..default()
        },
        Transform::from_xyz(2.0, 5.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 各种 3D 图元（名字 + 网格句柄）
    let shapes: Vec<(&str, Handle<Mesh>)> = vec![
        ("立方体", meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        ("球", meshes.add(Sphere::new(0.6))),
        ("圆柱", meshes.add(Cylinder::new(0.5, 1.4))),
        ("圆锥", meshes.add(Cone::new(0.5, 1.4))),
        ("胶囊", meshes.add(Capsule3d::new(0.35, 0.8))),
        ("圆环", meshes.add(Torus::new(0.3, 0.8))),
    ];

    let colors = [
        Color::srgb(0.9, 0.4, 0.4),
        Color::srgb(0.4, 0.7, 0.4),
        Color::srgb(0.4, 0.5, 0.9),
        Color::srgb(0.9, 0.7, 0.3),
        Color::srgb(0.7, 0.4, 0.9),
        Color::srgb(0.4, 0.9, 0.7),
    ];

    for (i, (_, mesh)) in shapes.iter().enumerate() {
        let x = (i as f32 - 2.5) * 1.7;
        commands.spawn((
            Spin,
            Mesh3d(mesh.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: colors[i],
                perceptual_roughness: 0.6,
                ..default()
            })),
            Transform::from_xyz(x, 0.0, 0.0),
        ));
    }

    // 底部说明文字（由 2D 覆盖层相机渲染）
    let label = shapes
        .iter()
        .map(|(n, _)| *n)
        .collect::<Vec<_>>()
        .join("  ");
    commands.spawn_scene(bsn! {
        Text2d::new(label)
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(28.0),
        }
        Transform::from_xyz(0.0, -300.0, 0.0)
    });
}

// 让所有图元绕 Y 轴自转，展示 3D 形态
fn spin(time: Res<Time>, mut q: Query<&mut Transform, With<Spin>>) {
    for mut tf in &mut q {
        tf.rotate_y(time.delta_secs() * 0.8);
    }
}
