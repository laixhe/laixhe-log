//! Bevy 0.19 入门示例：演示 UI 焦点导航（Tab 键在控件间切换焦点）。
//!
//! 通过 TabGroup + TabIndex 组件，配合 TabNavigationPlugin / InputDispatchPlugin，
//! 让键盘的 Tab / Shift+Tab 在按钮之间循环切换焦点。
//!
//! 学习重点：
//! - TabGroup：标记一个「可 Tab 导航」的容器（可设置 order / modal）
//! - TabIndex(i32)：标记某个控件在 Tab 顺序中的位置（>=0 才参与顺序导航）
//! - TabNavigationPlugin：自动处理 Tab 键焦点切换（InputDispatchPlugin 已由 DefaultPlugins 默认添加）
//! - FocusGained / FocusLost 事件（EntityEvent）：监听焦点变化并高亮控件
//!
//! 操作：按 Tab 键在三个按钮之间循环切换焦点（Shift+Tab 反向）。

use bevy::input_focus::tab_navigation::{TabGroup, TabIndex, TabNavigationPlugin};
use bevy::input_focus::{FocusGained, FocusLost};
use bevy::prelude::*;
use bevy::text::FontSource;

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        // 启用键盘 Tab 焦点导航（InputDispatchPlugin 已由 DefaultPlugins 默认添加）
        .add_plugins(TabNavigationPlugin)
        .add_systems(Startup, setup)
        // 监听焦点变化：获得焦点高亮边框，失去焦点恢复
        .add_observer(on_focus_gained)
        .add_observer(on_focus_lost)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // TabGroup 容器：三个按钮是它的后代，按 TabIndex 顺序参与 Tab 导航。
    commands
        .spawn((
            TabGroup::default(),
            Node {
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: px(20),
                ..default()
            },
        ))
        .with_children(|parent| {
            // 三个按钮，TabIndex 分别为 0 / 1 / 2，Tab 键按此顺序循环
            for i in 0..3 {
                parent
                    .spawn((
                        Button,
                        TabIndex(i),
                        Node {
                            width: px(200),
                            height: px(60),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::all(px(3)),
                            border_radius: BorderRadius::all(px(10)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                        BorderColor::all(Color::WHITE),
                    ))
                    .with_children(|button| {
                        button.spawn((
                            Text::new(format!("按钮 {}", i + 1)),
                            TextColor(Color::WHITE),
                            TextFont {
                                font: FontSource::Handle(asset_server.load(FONT_PATH)),
                                font_size: FontSize::Px(20.0),
                                ..default()
                            },
                        ));
                    });
            }
        });
}

// 获得焦点：边框变绿（只处理事件最初目标，忽略冒泡到父级的重复事件）
fn on_focus_gained(trigger: On<FocusGained>, mut query: Query<&mut BorderColor>) {
    if trigger.entity != trigger.original_event_target() {
        return;
    }
    if let Ok(mut border) = query.get_mut(trigger.entity) {
        *border = BorderColor::all(Color::srgb(0.3, 0.9, 0.4));
    }
    info!("[焦点] 控件获得焦点: {:?}", trigger.entity);
}

// 失去焦点：边框恢复白色
fn on_focus_lost(trigger: On<FocusLost>, mut query: Query<&mut BorderColor>) {
    if trigger.entity != trigger.original_event_target() {
        return;
    }
    if let Ok(mut border) = query.get_mut(trigger.entity) {
        *border = BorderColor::all(Color::WHITE);
    }
    info!("[焦点] 控件失去焦点: {:?}", trigger.entity);
}
