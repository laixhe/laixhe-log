//! Bevy 0.19 入门示例：演示 UI 视口（ViewportNode，小地图 / 画中画）。
//!
//! 学习重点：
//! - Image::new_target_texture：创建可渲染到的纹理（render target）
//! - RenderTarget::Image：让相机把画面渲染到纹理，而非窗口
//! - ViewportNode：在 UI 节点内显示某个相机渲染的画面
//!
//! 观察：主窗口中央显示旋转的橙圆；右下角 UI 小地图里也实时显示同一个橙圆。

use bevy::camera::RenderTarget;
use bevy::prelude::*;
use bevy::render::render_resource::TextureFormat;
use bevy::text::FontSource;

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// 旋转的橙圆标记
#[derive(Component)]
struct Spin;

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
    mut images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
) {
    // 主相机：渲染到主窗口
    commands.spawn(Camera2d);

    // 场景对象：一个旋转的橙圆（主窗口和小地图都能看到）
    commands.spawn((
        Spin,
        Mesh2d(meshes.add(Circle::new(80.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.9, 0.5, 0.2)))),
    ));

    // 渲染目标纹理：ViewportNode 会自动把它的尺寸调整成节点大小
    let image = Image::new_target_texture(256, 256, TextureFormat::Rgba8UnormSrgb, None);
    let image_handle = images.add(image);

    // 小地图相机：渲染到 image，而非窗口
    let minimap_camera = commands
        .spawn((
            Camera2d,
            Camera {
                order: 1,
                ..default()
            },
            RenderTarget::Image(image_handle.clone().into()),
        ))
        .id();

    // UI：居中标题 + 小地图视口
    commands
        .spawn(Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: px(16),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new("UI 视口（小地图）"),
                TextColor(Color::WHITE),
                TextFont {
                    font: FontSource::Handle(asset_server.load(FONT_PATH)),
                    font_size: FontSize::Px(22.0),
                    ..default()
                },
            ));

            // 小地图：在 UI 里显示 minimap_camera 渲染的画面
            parent.spawn((
                ViewportNode::new(minimap_camera),
                Node {
                    width: px(220),
                    height: px(220),
                    border: UiRect::all(px(3)),
                    border_radius: BorderRadius::all(px(8)),
                    ..default()
                },
                BorderColor::all(Color::srgb(0.4, 0.7, 1.0)),
            ));
        });
}

// 让橙圆旋转，展示主窗口与小地图都在实时渲染
fn spin(time: Res<Time>, mut q: Query<&mut Transform, With<Spin>>) {
    for mut tf in &mut q {
        tf.rotation = Quat::from_rotation_z(time.elapsed_secs());
    }
}
