//! Bevy 0.19 入门示例：演示把 UI 渲染到指定相机（UiTargetCamera）。
//!
//! 学习重点：
//! - UiTargetCamera：放在 UI 根节点上，指定该 UI 渲染到哪个相机
//! - 默认 UI 渲染到「默认 UI 相机」（通常是主窗口相机）
//! - 用 UiTargetCamera 可以把 UI 渲染到第二个窗口的相机
//!
//! 观察：主窗口显示「主窗口 UI」，第二个窗口显示「第二窗口 UI」。

use bevy::camera::RenderTarget;
use bevy::prelude::*;
use bevy::text::FontSource;
use bevy::window::WindowRef;

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = FontSource::Handle(asset_server.load(FONT_PATH));

    // 主窗口相机 + 默认 UI（渲染到主窗口）
    commands.spawn(Camera2d);

    commands
        .spawn(Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new("主窗口 UI（默认）"),
                TextColor(Color::srgb(0.9, 0.4, 0.4)),
                TextFont {
                    font: font.clone(),
                    font_size: FontSize::Px(26.0),
                    ..default()
                },
            ));
        });

    // 第二个窗口
    let second_window = commands
        .spawn(Window {
            title: "第二窗口".to_string(),
            resolution: (420, 300).into(),
            position: WindowPosition::At(IVec2::new(900, 200)),
            ..default()
        })
        .id();

    // 第二个窗口的相机
    let second_camera = commands
        .spawn((
            Camera2d,
            RenderTarget::Window(WindowRef::Entity(second_window)),
        ))
        .id();

    // 第二窗口的 UI：用 UiTargetCamera 指定渲染到第二个相机
    commands
        .spawn((
            UiTargetCamera(second_camera),
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("第二窗口 UI（UiTargetCamera）"),
                TextColor(Color::srgb(0.4, 0.6, 0.9)),
                TextFont {
                    font: font.clone(),
                    font_size: FontSize::Px(24.0),
                    ..default()
                },
            ));
        });

    info!("[UI相机] 主窗口与第二窗口各自渲染对应 UI");
}
