//! Bevy 0.19 入门示例：演示 Toggle 开关 和 ProgressBar 进度条。
//!
//! Bevy 0.19 没有独立的 Toggle / ProgressBar 组件，但可以用现有控件组合实现：
//! - Toggle：复用 Checkbox 组件（交互逻辑相同），自定义「胶囊 + 圆点」视觉
//! - ProgressBar：纯自定义（Node 背景 + 填充子节点，动态改宽度）
//!
//! 学习重点：
//! - Toggle = Checkbox 组件 + 自定义视觉（复用 ValueChange<bool> 事件）
//! - ProgressBar = Node + 填充子节点，用 Node.width 百分比动态更新
//! - 无样式控件的组合能力：用基础组件搭建任意 UI 控件
//!
//! 操作：点击开关切换；进度条自动循环（0 → 100%）。

use bevy::prelude::*;
use bevy::text::FontSource;
use bevy::ui_widgets::{Checkbox, ValueChange, checkbox_self_update};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// Toggle 圆点标记（用于切换时更新位置）
#[derive(Component)]
struct ToggleKnob;

// ProgressBar 填充标记（用于更新宽度）
#[derive(Component)]
struct ProgressFill;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update_progress)
        // Toggle 复用 Checkbox 的自更新 observer
        .add_observer(checkbox_self_update)
        // 监听 Toggle 状态变化，移动圆点
        .add_observer(on_toggle_change)
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
            row_gap: px(40),
            ..default()
        })
        .with_children(|parent| {
            // ---- Toggle 开关 ----
            parent
                .spawn(Node {
                    align_items: AlignItems::Center,
                    column_gap: px(12),
                    ..default()
                })
                .with_children(|row| {
                    // 开关主体：复用 Checkbox 组件 + 胶囊视觉
                    row.spawn((
                        Checkbox,
                        Node {
                            width: px(56),
                            height: px(28),
                            border_radius: BorderRadius::MAX,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.2, 0.25)),
                    ))
                    .with_children(|toggle| {
                        // 圆点：绝对定位，勾选时右移
                        toggle.spawn((
                            ToggleKnob,
                            Node {
                                position_type: PositionType::Absolute,
                                width: px(22),
                                height: px(22),
                                left: px(3.0),
                                border_radius: BorderRadius::MAX,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.85, 0.85, 0.85)),
                        ));
                    });

                    row.spawn((
                        Text::new("开关（点击切换）"),
                        TextColor(Color::WHITE),
                        TextFont {
                            font: FontSource::Handle(asset_server.load(FONT_PATH)),
                            font_size: FontSize::Px(20.0),
                            ..default()
                        },
                    ));
                });

            // ---- ProgressBar 进度条 ----
            parent
                .spawn(Node {
                    align_items: AlignItems::Center,
                    column_gap: px(12),
                    ..default()
                })
                .with_children(|row| {
                    // 轨道背景
                    row.spawn((
                        Node {
                            width: px(220),
                            height: px(20),
                            border_radius: BorderRadius::all(px(10)),
                            overflow: Overflow::clip(),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    ))
                    .with_children(|track| {
                        // 填充：宽度用百分比，随进度变化
                        track.spawn((
                            ProgressFill,
                            Node {
                                width: percent(0.0),
                                height: percent(100),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.3, 0.8, 0.4)),
                        ));
                    });

                    row.spawn((
                        Text::new("进度条（自动循环）"),
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

// Toggle 状态变化：移动圆点位置
fn on_toggle_change(
    trigger: On<ValueChange<bool>>,
    q_children: Query<&Children>,
    mut q_knob: Query<&mut Node, With<ToggleKnob>>,
) {
    if let Ok(children) = q_children.get(trigger.source) {
        if let Some(knob) = children.first() {
            if let Ok(mut node) = q_knob.get_mut(*knob) {
                // 勾选时圆点移到右侧（56 - 22 - 3 = 31）
                node.left = px(if trigger.value { 31.0 } else { 3.0 });
            }
        }
    }
    info!("[UI组件] 开关状态 = {}", trigger.value);
}

// 进度条：每 3 秒从 0% 线性涨到 100%，然后循环
fn update_progress(time: Res<Time>, mut q_fill: Query<&mut Node, With<ProgressFill>>) {
    let progress = (time.elapsed_secs() % 3.0) / 3.0;
    for mut node in &mut q_fill {
        node.width = percent(progress * 100.0);
    }
}
