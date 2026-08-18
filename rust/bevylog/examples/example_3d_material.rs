//! Bevy 0.19 入门示例：演示 3D PBR 材质属性（StandardMaterial）。
//!
//! 学习重点：
//! - base_color：基础颜色
//! - metallic：金属度（1 = 纯金属，0 = 非金属）
//! - perceptual_roughness：感知粗糙度（0 光滑，1 粗糙）
//! - emissive：自发光颜色（LinearRgba，可 >1 产生 HDR 光晕）
//! - unlit：无光照（忽略灯光，直接显示 base_color）
//! - alpha_mode + srgba：半透明混合
//!
//! 观察：一排球体展示不同材质，从左到右依次是普通/金属/粗糙/光滑/发光/无光照/半透明。

use bevy::{prelude::*, text::FontSourceTemplate};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.06, 0.06, 0.08)))
        .add_systems(Startup, setup)
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
        Transform::from_xyz(0.0, 1.8, 8.0).looking_at(Vec3::new(0.0, 0.5, 0.0), Vec3::Y),
    ));

    // 2D 覆盖层相机（显示底部文字）
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
    ));

    // 方向光（金属/粗糙度差异需要光照才能体现）
    commands.spawn((
        DirectionalLight {
            illuminance: 8000.0,
            ..default()
        },
        Transform::from_xyz(2.0, 5.0, 3.0).looking_at(Vec3::new(0.0, 0.5, 0.0), Vec3::Y),
    ));

    // 地面：接住球体，也能展示材质对光照的反应
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(8.0)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.3, 0.35),
            perceptual_roughness: 0.9,
            ..default()
        })),
    ));

    // 一排球体，展示不同材质属性
    let spheres: [(&str, StandardMaterial); 7] = [
        (
            "普通",
            StandardMaterial {
                base_color: Color::srgb(0.3, 0.6, 0.9),
                perceptual_roughness: 0.5,
                ..default()
            },
        ),
        (
            "金属",
            StandardMaterial {
                base_color: Color::srgb(0.8, 0.8, 0.8),
                metallic: 1.0,
                perceptual_roughness: 0.2,
                ..default()
            },
        ),
        (
            "粗糙",
            StandardMaterial {
                base_color: Color::srgb(0.8, 0.4, 0.2),
                perceptual_roughness: 1.0,
                ..default()
            },
        ),
        (
            "光滑",
            StandardMaterial {
                base_color: Color::srgb(0.2, 0.7, 0.4),
                perceptual_roughness: 0.05,
                ..default()
            },
        ),
        (
            "发光",
            StandardMaterial {
                base_color: Color::srgb(0.9, 0.2, 0.1),
                emissive: LinearRgba::rgb(2.0, 0.4, 0.1),
                ..default()
            },
        ),
        (
            "无光照",
            StandardMaterial {
                base_color: Color::srgb(0.6, 0.3, 0.9),
                unlit: true,
                ..default()
            },
        ),
        (
            "半透明",
            StandardMaterial {
                base_color: Color::srgba(0.2, 0.6, 0.9, 0.5),
                alpha_mode: AlphaMode::Blend,
                ..default()
            },
        ),
    ];

    let label = spheres
        .iter()
        .map(|(n, _)| *n)
        .collect::<Vec<_>>()
        .join("  ");

    for (i, (_, material)) in spheres.into_iter().enumerate() {
        let x = (i as f32 - 3.0) * 1.5;
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.5))),
            MeshMaterial3d(materials.add(material)),
            Transform::from_xyz(x, 0.5, 0.0),
        ));
    }

    // 底部说明文字
    commands.spawn_scene(bsn! {
        Text2d::new(label)
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(26.0),
        }
        Transform::from_xyz(0.0, -300.0, 0.0)
    });
}
