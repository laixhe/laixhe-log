//! Bevy 0.19 入门示例：演示 UI 叠放层级（ZIndex / GlobalZIndex）。
//!
//! 学习重点：
//! - `ZIndex`：控制同一父节点下兄弟节点之间的前后叠放顺序，值越大越靠上
//! - `GlobalZIndex`：让任意层级的节点「逃离」布局树的绘制顺序，跨层级叠放
//! - 没有 ZIndex 的节点默认按层级顺序绘制（后加入的节点覆盖先加入的）
//!
//! 观察：三张卡片相互重叠，绿色卡片被提升到最上层，红色卡片被压到最底层。

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

            parent.spawn((
                Text::new("UI 叠放层级"),
                TextColor(Color::WHITE),
                font(24.0),
            ));

            // 三张卡片在同一个父容器内绝对定位，互相重叠
            parent
                .spawn(Node {
                    width: px(320),
                    height: px(200),
                    ..default()
                })
                .with_children(|container| {
                    // 红色卡片：压到最底层（GlobalZIndex -1 会低于所有默认层级）
                    container.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: px(0),
                            top: px(0),
                            width: px(160),
                            height: px(160),
                            border_radius: BorderRadius::all(px(12)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.85, 0.25, 0.25)),
                        GlobalZIndex(-1),
                    ));

                    // 蓝色卡片：默认层级（绘制顺序中间）
                    container.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: px(80),
                            top: px(20),
                            width: px(160),
                            height: px(160),
                            border_radius: BorderRadius::all(px(12)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.25, 0.40, 0.85)),
                    ));

                    // 绿色卡片：提升到最上层（GlobalZIndex 1 高于所有默认层级）
                    container.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: px(160),
                            top: px(40),
                            width: px(160),
                            height: px(160),
                            border_radius: BorderRadius::all(px(12)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.25, 0.80, 0.45)),
                        GlobalZIndex(1),
                    ));
                });
        });
}
