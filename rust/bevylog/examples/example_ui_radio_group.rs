//! Bevy 0.19 入门示例：演示 RadioGroup 单选组。
//!
//! RadioGroup 把多个 RadioButton 组合成一个「互斥」单元：
//! 选中一个时，其余自动取消选中。
//!
//! 学习重点：
//! - RadioGroup：标记单选组容器（require AccessibilityNode）
//! - RadioButton：单选按钮（require Checkable），点击 / 方向键切换
//! - 外部状态管理：RadioGroup 发出 ValueChange<Entity>（选中的按钮实体），
//!   app 自己维护 Checked 组件和视觉
//!
//! 操作：点击某个单选按钮切换选中。

use bevy::prelude::*;
use bevy::text::FontSource;
use bevy::ui::Checked;
use bevy::ui_widgets::{RadioButton, RadioGroup, ValueChange};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// 单选按钮的圆圈指示器标记
#[derive(Component)]
struct RadioDot;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        // 监听单选变化，维护 Checked 状态 + 更新视觉
        .add_observer(on_radio_change)
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
            // 单选组：垂直排列三个选项
            parent
                .spawn((
                    RadioGroup,
                    Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: px(14),
                        ..default()
                    },
                ))
                .with_children(|group| {
                    for label in ["选项 A", "选项 B", "选项 C"].iter() {
                        group
                            .spawn((
                                RadioButton,
                                Node {
                                    align_items: AlignItems::Center,
                                    column_gap: px(10),
                                    ..default()
                                },
                            ))
                            .with_children(|row| {
                                // 圆圈指示器
                                row.spawn((
                                    RadioDot,
                                    Node {
                                        width: px(20),
                                        height: px(20),
                                        border: UiRect::all(px(2)),
                                        border_radius: BorderRadius::MAX,
                                        ..default()
                                    },
                                    BorderColor::all(Color::WHITE),
                                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                                ));

                                row.spawn((
                                    Text::new(*label),
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
        });
}

// 单选变化：维护 Checked 状态 + 更新圆圈颜色
fn on_radio_change(
    trigger: On<ValueChange<Entity>>,
    q_buttons: Query<(Entity, &Children), With<RadioButton>>,
    mut q_dot: Query<&mut BackgroundColor, With<RadioDot>>,
    mut commands: Commands,
) {
    let selected = trigger.value;
    for (entity, children) in &q_buttons {
        let is_selected = entity == selected;
        // 维护 Checked 组件
        if is_selected {
            commands.entity(entity).insert(Checked);
        } else {
            commands.entity(entity).remove::<Checked>();
        }
        // 更新圆圈颜色
        if let Some(dot) = children.first() {
            if let Ok(mut bg) = q_dot.get_mut(*dot) {
                *bg = BackgroundColor(if is_selected {
                    Color::srgb(0.3, 0.8, 0.4)
                } else {
                    Color::srgb(0.15, 0.15, 0.15)
                });
            }
        }
    }
    info!("[UI组件] 选中单选项: {:?}", selected);
}
