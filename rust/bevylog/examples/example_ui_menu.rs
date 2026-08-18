//! Bevy 0.19 入门示例：演示 Menu 弹出菜单。
//!
//! Menu 是 Bevy 0.19 UI 控件中最复杂的一个，涉及按钮、弹窗、焦点管理。
//!
//! 学习重点：
//! - MenuButton：菜单按钮（require Button），点击触发 MenuEvent::Toggle
//! - MenuPopup：弹窗容器（require TabGroup::modal + MenuFocusState）
//! - MenuItem：菜单项，点击触发 Activate 事件并关闭菜单
//! - MenuEvent：冒泡到 menu 实体，由 app 写的 observer 处理（打开/关闭弹窗）
//! - MenuPlugin 自动处理焦点（ESC 关闭、方向键导航、点击外部关闭）
//!
//! 操作：点击「菜单」按钮打开/关闭弹窗，点击菜单项触发日志。

use bevy::input_focus::tab_navigation::NavAction;
use bevy::input_focus::{FocusCause, InputFocus};
use bevy::prelude::*;
use bevy::text::FontSource;
use bevy::ui_widgets::{
    Activate, MenuAction, MenuButton, MenuEvent, MenuFocusState, MenuItem, MenuPopup,
};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        // 监听菜单项激活（用户点击了某个菜单项）
        .add_observer(on_menu_item_activate)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // menu 实体：包含按钮 + 弹窗，观察 MenuEvent 处理打开/关闭
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            top: px(60),
            left: percent(50),
            ..default()
        })
        .observe(menu_event_handler)
        .with_children(|menu| {
            // 菜单按钮
            menu.spawn((
                MenuButton,
                Node {
                    padding: UiRect::axes(px(20), px(10)),
                    border_radius: BorderRadius::all(px(6)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.4, 0.7)),
            ))
            .with_children(|button| {
                button.spawn((
                    Text::new("菜单"),
                    TextColor(Color::WHITE),
                    TextFont {
                        font: FontSource::Handle(asset_server.load(FONT_PATH)),
                        font_size: FontSize::Px(20.0),
                        ..default()
                    },
                ));
            });

            // 弹窗（默认隐藏，打开时显示在按钮下方）
            menu.spawn((
                MenuPopup::default(),
                Node {
                    position_type: PositionType::Absolute,
                    top: px(44),
                    width: px(160),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(px(4)),
                    row_gap: px(2),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.15, 0.18)),
                Visibility::Hidden,
            ))
            .with_children(|popup| {
                for label in ["菜单项 1", "菜单项 2", "菜单项 3"] {
                    popup
                        .spawn((
                            MenuItem,
                            Node {
                                width: percent(100),
                                padding: UiRect::axes(px(12), px(8)),
                                border_radius: BorderRadius::all(px(4)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.2, 0.2, 0.25)),
                        ))
                        .with_children(|item| {
                            item.spawn((
                                Text::new(label),
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

// 处理 MenuEvent：切换弹窗的可见性和焦点状态
fn menu_event_handler(
    trigger: On<MenuEvent>,
    q_children: Query<&Children>,
    mut q_popup: Query<(&mut Visibility, &mut MenuFocusState), With<MenuPopup>>,
    q_button: Query<Entity, With<MenuButton>>,
    mut focus: ResMut<InputFocus>,
) {
    // 找到 menu 实体下的弹窗（menu 是 observer 所在实体）
    let menu = trigger.observer();
    let Ok(children) = q_children.get(menu) else {
        return;
    };
    for child in children.iter() {
        let Ok((mut vis, mut state)) = q_popup.get_mut(child) else {
            continue;
        };
        match trigger.action {
            MenuAction::Open(nav) => {
                *vis = Visibility::Visible;
                *state = MenuFocusState::Opening(nav);
            }
            MenuAction::Toggle => {
                if *vis == Visibility::Visible {
                    *vis = Visibility::Hidden;
                    *state = MenuFocusState::Closed;
                } else {
                    *vis = Visibility::Visible;
                    *state = MenuFocusState::Opening(NavAction::First);
                }
            }
            MenuAction::CloseAll => {
                *vis = Visibility::Hidden;
                *state = MenuFocusState::Closed;
            }
            MenuAction::FocusRoot => {
                // 关闭时把焦点还给按钮
                if let Some(button) = q_button.iter().next() {
                    focus.set(button, FocusCause::Navigated);
                }
            }
        }
    }
}

// 用户点击菜单项：打印日志
fn on_menu_item_activate(trigger: On<Activate>) {
    info!("[UI组件] 激活菜单项: {:?}", trigger.entity);
}
