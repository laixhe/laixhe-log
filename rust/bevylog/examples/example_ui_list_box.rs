//! Bevy 0.19 入门示例：演示 ListBox 列表框。
//!
//! ListBox 把多个 ListItem 组成一个可选列表：点击某一行选中它。
//!
//! 学习重点：
//! - ListBox：列表容器（require ActiveDescendant）
//! - ListItem：列表项（require Selectable）
//! - 外部状态管理：ListBox 发出 ValueChange<Entity>（选中的行实体），
//!   app 自己维护 Selected 组件和视觉
//!
//! 操作：点击某一行选中它（高亮）。

use bevy::prelude::*;
use bevy::text::FontSource;
use bevy::ui::Selected;
use bevy::ui_widgets::{ListBox, ListItem, ValueChange};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_observer(on_list_change)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            // 列表框：垂直排列多行
            parent
                .spawn((
                    ListBox,
                    Node {
                        width: px(240),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(4),
                        ..default()
                    },
                ))
                .with_children(|list| {
                    for (i, label) in ["第一行", "第二行", "第三行", "第四行"].iter().enumerate()
                    {
                        // 列表项：行背景 + 文字
                        list.spawn((
                            ListItem,
                            Node {
                                width: percent(100),
                                height: px(40),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border_radius: BorderRadius::all(px(6)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.18, 0.18, 0.22)),
                        ))
                        .with_children(|row| {
                            row.spawn((
                                Text::new(format!("{} {}", i + 1, label)),
                                TextColor(Color::WHITE),
                                TextFont {
                                    font: FontSource::Handle(asset_server.load(FONT_PATH)),
                                    font_size: FontSize::Px(18.0),
                                    ..default()
                                },
                            ));
                        });
                    }
                });
        });
}

// 列表选择变化：维护 Selected 组件 + 更新行背景色
fn on_list_change(
    trigger: On<ValueChange<Entity>>,
    q_items: Query<Entity, With<ListItem>>,
    mut q_bg: Query<&mut BackgroundColor>,
    mut commands: Commands,
) {
    let selected = trigger.value;
    for entity in &q_items {
        let is_selected = entity == selected;
        if is_selected {
            commands.entity(entity).insert(Selected);
        } else {
            commands.entity(entity).remove::<Selected>();
        }
        // 更新行背景色
        if let Ok(mut bg) = q_bg.get_mut(entity) {
            *bg = BackgroundColor(if is_selected {
                Color::srgb(0.3, 0.6, 0.9)
            } else {
                Color::srgb(0.18, 0.18, 0.22)
            });
        }
    }
    info!("[UI组件] 选中列表项: {:?}", selected);
}
