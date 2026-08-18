//! Bevy 0.19 入门示例：演示 ScrollArea 滚动区域。
//!
//! ScrollArea 组件让一个 overflow: scroll 的 UI 容器支持滚轮滚动。
//!
//! 学习重点：
//! - ScrollArea：标记组件，放在 overflow: scroll 的 Node 上（require ScrollPosition）
//! - Overflow::scroll_y()：让容器在 y 轴方向滚动
//! - 内容超出容器高度时，用滚轮上下滚动
//!
//! 操作：鼠标悬停在列表上滚动滚轮，上下翻看列表项。

use bevy::prelude::*;
use bevy::text::FontSource;
use bevy::ui_widgets::ScrollArea;

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // 根容器：居中
    commands
        .spawn(Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            // 滚动区域：固定高度 + 垂直滚动
            parent
                .spawn((
                    ScrollArea,
                    Node {
                        width: px(240),
                        height: px(240),
                        overflow: Overflow::scroll_y(),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(6),
                        padding: UiRect::all(px(8)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.12, 0.12, 0.15)),
                ))
                .with_children(|list| {
                    // 20 个列表项，超出容器高度，产生滚动
                    for i in 0..20 {
                        list.spawn((
                            Node {
                                width: percent(100),
                                height: px(36),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border_radius: BorderRadius::all(px(6)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.2, 0.2, 0.25)),
                        ))
                        .with_children(|item| {
                            item.spawn((
                                Text::new(format!("列表项 {}", i + 1)),
                                TextColor(Color::WHITE),
                                TextFont {
                                    font: FontSource::Handle(asset_server.load(FONT_PATH)),
                                    font_size: FontSize::Px(16.0),
                                    ..default()
                                },
                            ));
                        });
                    }
                });
        });
}
