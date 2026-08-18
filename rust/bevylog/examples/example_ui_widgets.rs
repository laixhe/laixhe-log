//! Bevy 0.19 入门示例：演示 UI 组件（Checkbox 复选框 / Slider 滑块）。
//!
//! Bevy 0.19 的 `bevy_ui_widgets` 提供了一组「无样式」标准控件（headless widget），
//! 状态通过外部管理：控件发出 `ValueChange<T>` 事件，由 app 响应。
//!
//! 学习重点：
//! - Checkbox 组件：点击 / 空格切换勾选状态，发出 ValueChange<bool>
//! - checkbox_self_update observer：让 Checkbox 自动维护 Checked 组件
//! - Slider 组件 + SliderValue / SliderRange：滑块，发出 ValueChange<f32>
//! - SliderThumb 标记 thumb 子节点，需自行根据值定位
//!
//! 操作：点击复选框切换勾选；拖动滑块调节数值（终端打印状态）。

use bevy::prelude::*;
use bevy::text::FontSource;
use bevy::ui_widgets::{
    Checkbox, Slider, SliderRange, SliderThumb, SliderValue, ValueChange, checkbox_self_update,
};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        // checkbox_self_update：让 Checkbox 自动加/移除 Checked 组件
        .add_observer(checkbox_self_update)
        // 监听控件值变化，更新视觉 + 打印日志
        .add_observer(on_checkbox_change)
        .add_observer(on_slider_change)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // 根容器：垂直排列
    commands
        .spawn(Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: px(30),
            ..default()
        })
        .with_children(|parent| {
            // ---- 复选框 ----
            parent
                .spawn(Node {
                    align_items: AlignItems::Center,
                    column_gap: px(12),
                    ..default()
                })
                .with_children(|row| {
                    // 复选框主体：无样式的 Checkbox + 手动加 Node 作为可视框
                    row.spawn((
                        Checkbox,
                        Node {
                            width: px(28),
                            height: px(28),
                            border: UiRect::all(px(3)),
                            border_radius: BorderRadius::all(px(6)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                        BorderColor::all(Color::WHITE),
                    ))
                    .with_children(|box_| {
                        // 勾选标记：默认隐藏，Checked 时显示
                        box_.spawn((
                            Node {
                                width: px(16),
                                height: px(16),
                                border_radius: BorderRadius::all(px(4)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.3, 0.9, 0.4)),
                            Visibility::Hidden,
                        ));
                    });

                    // 文字标签
                    row.spawn((
                        Text::new("启用音效（点击切换）"),
                        TextColor(Color::WHITE),
                        TextFont {
                            font: FontSource::Handle(asset_server.load(FONT_PATH)),
                            font_size: FontSize::Px(20.0),
                            ..default()
                        },
                    ));
                });

            // ---- 滑块 ----
            parent
                .spawn(Node {
                    align_items: AlignItems::Center,
                    column_gap: px(12),
                    ..default()
                })
                .with_children(|row| {
                    // 滑块轨道：Slider + SliderValue + SliderRange
                    row.spawn((
                        Slider::default(),
                        SliderValue(50.0),
                        SliderRange::new(0.0, 100.0),
                        Node {
                            width: px(220),
                            height: px(20),
                            border_radius: BorderRadius::all(px(10)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    ))
                    .with_children(|track| {
                        // thumb：标记 SliderThumb，绝对定位（初始 left 对应 value=50）
                        track.spawn((
                            SliderThumb,
                            Node {
                                position_type: PositionType::Absolute,
                                width: px(20),
                                height: px(20),
                                left: px(90.0),
                                border_radius: BorderRadius::all(px(10)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.3, 0.5, 0.9)),
                        ));
                    });

                    row.spawn((
                        Text::new("音量（拖动调节）"),
                        TextColor(Color::WHITE),
                        TextFont {
                            font: FontSource::Handle(asset_server.load(FONT_PATH)),
                            font_size: FontSize::Px(20.0),
                            ..default()
                        },
                    ));
                });
        });
}

// 复选框值变化：切换勾选标记可见性 + 打印日志
fn on_checkbox_change(
    trigger: On<ValueChange<bool>>,
    q_children: Query<&Children>,
    mut commands: Commands,
) {
    // 更新勾选标记的可见性（第一个子节点）
    if let Ok(children) = q_children.get(trigger.source) {
        if let Some(mark) = children.first() {
            commands.entity(*mark).insert(if trigger.value {
                Visibility::Visible
            } else {
                Visibility::Hidden
            });
        }
    }
    info!("[UI组件] 复选框状态 = {}", trigger.value);
}

// 滑块值变化：移动 thumb 位置 + 打印日志
fn on_slider_change(
    trigger: On<ValueChange<f32>>,
    q_slider: Query<(&SliderRange, &Children)>,
    mut q_thumb: Query<&mut Node, With<SliderThumb>>,
) {
    let value = trigger.value;
    if let Ok((range, children)) = q_slider.get(trigger.source) {
        // 轨道 220px，thumb 20px，可移动 200px
        let t = range.thumb_position(value);
        if let Some(thumb) = children.first() {
            if let Ok(mut node) = q_thumb.get_mut(*thumb) {
                node.left = px(t * 200.0);
            }
        }
    }
    info!("[UI组件] 滑块值 = {:.1}", value);
}
