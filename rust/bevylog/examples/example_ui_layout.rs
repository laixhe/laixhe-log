//! Bevy 0.19 入门示例：演示 UI 布局单位与盒模型。
//!
//! 学习重点：
//! - `Val` 的各类单位：`px` / `percent` / `vw` / `vh` / `auto`
//! - `UiRect`：margin（外边距）/ padding（内边距）/ border（边框）
//! - `PositionType::Absolute`：绝对定位（相对父节点）
//! - `BorderRadius`：圆角、`Outline`：外描边（不占布局空间）
//!
//! 观察：调整窗口大小，`vw`/`vh`/`percent` 的盒子会随视口变化，而 `px` 固定。

use bevy::prelude::*;
use bevy::text::FontSource;

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // 根容器：居中纵向排列
    commands
        .spawn(Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: px(24),
            ..default()
        })
        .with_children(|parent| {
            let font = |size: f32| TextFont {
                font: FontSource::Handle(asset_server.load(FONT_PATH)),
                font_size: FontSize::Px(size),
                ..default()
            };

            // 标题
            parent.spawn((
                Text::new("布局单位与盒模型"),
                TextColor(Color::WHITE),
                font(24.0),
            ));

            // ---- 盒模型：margin(外) / border(边) / padding(内) ----
            parent
                .spawn((
                    Node {
                        // 外边距：把整个盒子从周围推离
                        margin: UiRect::all(px(16)),
                        // 边框：占据布局空间，落在 padding 之外
                        border: UiRect::all(px(6)),
                        // 内边距：边框到内容之间的留白
                        padding: UiRect::all(px(24)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.25, 0.25, 0.30)), // padding 区域颜色
                    BorderColor::all(Color::srgb(0.90, 0.60, 0.20)), // 边框颜色
                    // 外描边：不占布局空间，绘制在边框之外
                    Outline::new(px(4), px(4), Color::srgb(0.90, 0.20, 0.20)),
                ))
                .with_children(|box_| {
                    // 内容区域（受 padding 约束）
                    box_.spawn((
                        Node {
                            width: px(120),
                            height: px(60),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.30, 0.60, 0.90)),
                    ));
                });

            // ---- 绝对定位 ----
            parent
                .spawn((
                    Node {
                        width: px(260),
                        height: px(160),
                        border_radius: BorderRadius::all(px(10)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.20)),
                ))
                .with_children(|container| {
                    // 相对定位的普通子节点（默认 PositionType::Relative）
                    container.spawn((
                        Node {
                            width: px(60),
                            height: px(60),
                            border_radius: BorderRadius::all(px(8)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.30, 0.50, 0.90)),
                    ));

                    // 绝对定位到容器右下角（right/bottom 相对父节点边界）
                    container.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            right: px(8),
                            bottom: px(8),
                            width: px(60),
                            height: px(60),
                            border_radius: BorderRadius::all(px(8)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.90, 0.50, 0.30)),
                    ));
                });

            // ---- 视口单位 vw / vh（相对窗口尺寸）----
            parent
                .spawn(Node {
                    column_gap: px(16),
                    ..default()
                })
                .with_children(|row| {
                    row.spawn((
                        Node {
                            width: vw(10), // 窗口宽度的 10%
                            height: px(40),
                            border_radius: BorderRadius::all(px(6)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.20, 0.70, 0.50)),
                    ));
                    row.spawn((
                        Node {
                            width: vh(20), // 窗口高度的 20%
                            height: px(40),
                            border_radius: BorderRadius::all(px(6)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.80, 0.40, 0.60)),
                    ));
                    row.spawn((
                        Node {
                            width: px(80), // 固定 80 逻辑像素
                            height: px(40),
                            border_radius: BorderRadius::all(px(6)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.50, 0.50, 0.90)),
                    ));
                });
        });
}
