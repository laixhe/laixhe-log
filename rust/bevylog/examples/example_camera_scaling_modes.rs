//! Bevy 0.19 入门示例：演示正交投影缩放模式（ScalingMode）。
//!
//! 学习重点：
//! - ScalingMode 决定「世界坐标如何映射到屏幕」，与 scale 共同决定可见范围
//! - WindowSize 跟随窗口；Fixed 固定世界尺寸；FixedVertical/FixedHorizontal 固定单边
//! - AutoMin/AutoMax 保持宽高比，限制最小/最大可见范围
//!
//! 操作：按数字键 1~6 切换缩放模式，观察固定 800x600 边框在屏幕上的变化。

use bevy::camera::ScalingMode;
use bevy::prelude::*;
use bevy::text::FontSource;

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// 提示标签标记
#[derive(Component)]
struct ModeLabel;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.08)))
        .add_systems(Startup, setup)
        .add_systems(Update, switch_scaling_mode)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // 2D 相机 + 自定义正交投影（默认 WindowSize）
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection::default_2d()),
    ));

    // 固定世界尺寸 800x600 的边框，用来观察不同缩放模式
    let border = Color::srgb(0.5, 0.5, 0.6);
    let thickness = 6.0;
    let w = 800.0;
    let h = 600.0;
    let edges = [
        (Vec2::new(0.0, h / 2.0), Vec2::new(w, thickness)),
        (Vec2::new(0.0, -h / 2.0), Vec2::new(w, thickness)),
        (Vec2::new(w / 2.0, 0.0), Vec2::new(thickness, h)),
        (Vec2::new(-w / 2.0, 0.0), Vec2::new(thickness, h)),
    ];
    for (pos, size) in edges {
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(size.x, size.y))),
            MeshMaterial2d(materials.add(ColorMaterial::from(border))),
            Transform::from_xyz(pos.x, pos.y, 0.0),
        ));
    }

    // 中心参照圆
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(30.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.2, 0.7, 0.9)))),
        Transform::default(),
    ));

    // 提示标签
    commands.spawn((
        ModeLabel,
        Text2d::new("按 1~6 切换缩放模式"),
        TextColor(Color::WHITE),
        TextFont {
            font: FontSource::Handle(asset_server.load(FONT_PATH)),
            font_size: FontSize::Px(22.0),
            ..default()
        },
        Transform::from_xyz(0.0, h / 2.0 + 60.0, 0.0),
    ));
}

fn switch_scaling_mode(
    keys: Res<ButtonInput<KeyCode>>,
    projection: Single<&mut Projection, With<Camera2d>>,
    mut label: Single<&mut Text2d, With<ModeLabel>>,
) {
    let Projection::Orthographic(ortho) = &mut *projection.into_inner() else {
        return;
    };

    let (mode, name) = if keys.just_pressed(KeyCode::Digit1) {
        (ScalingMode::WindowSize, "WindowSize 窗口大小")
    } else if keys.just_pressed(KeyCode::Digit2) {
        (
            ScalingMode::Fixed {
                width: 800.0,
                height: 600.0,
            },
            "Fixed 固定 800x600",
        )
    } else if keys.just_pressed(KeyCode::Digit3) {
        (
            ScalingMode::FixedVertical {
                viewport_height: 600.0,
            },
            "FixedVertical 高度固定 600",
        )
    } else if keys.just_pressed(KeyCode::Digit4) {
        (
            ScalingMode::FixedHorizontal {
                viewport_width: 800.0,
            },
            "FixedHorizontal 宽度固定 800",
        )
    } else if keys.just_pressed(KeyCode::Digit5) {
        (
            ScalingMode::AutoMin {
                min_width: 800.0,
                min_height: 600.0,
            },
            "AutoMin 最小 800x600",
        )
    } else if keys.just_pressed(KeyCode::Digit6) {
        (
            ScalingMode::AutoMax {
                max_width: 800.0,
                max_height: 600.0,
            },
            "AutoMax 最大 800x600",
        )
    } else {
        return;
    };

    ortho.scaling_mode = mode;
    label.0 = format!("缩放模式：{name}");
    info!("[相机] 切换缩放模式：{name}");
}
