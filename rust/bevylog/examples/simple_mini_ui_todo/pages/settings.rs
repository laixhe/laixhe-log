use bevy::prelude::*;
use bevy::text::FontSource;
use bevy::ui::Checked;
use bevy::ui_widgets::{
    Checkbox, RadioButton, RadioGroup, Slider, SliderRange, SliderThumb, SliderValue, ValueChange,
};

use crate::pages::router::AppPage;
use crate::state::{AppData, theme_color};

// 中文字体路径（与其它示例共用同一资产）
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";
// 滑块轨道宽度（px）与 thumb 宽度：thumb 可移动范围 = 轨道宽 - thumb 宽
const TRACK_WIDTH: f32 = 240.0;
const THUMB_WIDTH: f32 = 20.0;

// ==================== 组件标记 ====================
#[derive(Component, Clone, Default)]
pub struct SettingsRoot; // 整页根标记：OnExit 时一键清理

#[derive(Component)]
pub struct BackButton; // 返回清单页

#[derive(Component)]
pub struct SoundCheckbox; // 音效复选框（Checkbox 实体）
#[derive(Component)]
pub struct CheckMark; // 复选框内的勾选标记

#[derive(Component)]
pub struct VolumeSlider; // 音量滑块（Slider 实体）
#[derive(Component)]
pub struct VolumeText; // 音量数值文本（拖动滑块时同步更新）

#[derive(Component)]
pub struct ThemeIndex(pub usize); // 单选按钮对应的主题索引
#[derive(Component)]
pub struct RadioDot; // 单选按钮的圆圈指示器

// ==================== 进入设置页 ====================
pub fn setup_settings(mut commands: Commands, asset_server: Res<AssetServer>, data: Res<AppData>) {
    let font = FontSource::Handle(asset_server.load(FONT_PATH));

    commands
        .spawn((
            SettingsRoot,
            Node {
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: px(24),
                ..default()
            },
            BackgroundColor(theme_color(data.theme)),
        ))
        .with_children(|parent| {
            // 标题
            parent.spawn((
                Text::new("设置"),
                TextColor(Color::srgb(0.6, 0.8, 1.0)),
                TextFont {
                    font: font.clone(),
                    font_size: FontSize::Px(40.0),
                    ..default()
                },
            ));

            // ---- 音效开关（Checkbox）----
            parent
                .spawn(Node {
                    align_items: AlignItems::Center,
                    column_gap: px(12),
                    ..default()
                })
                .with_children(|row| {
                    // 复选框：spawn 后按存档状态补插 Checked（Option 不能直接放 bundle）
                    let mut checkbox = row.spawn((
                        SoundCheckbox,
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
                    ));
                    if data.sound_on {
                        checkbox.insert(Checked);
                    }
                    checkbox.with_children(|box_| {
                        // 勾选标记：默认按存档状态显示/隐藏
                        box_.spawn((
                            CheckMark,
                            Node {
                                width: px(16),
                                height: px(16),
                                border_radius: BorderRadius::all(px(4)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.3, 0.9, 0.4)),
                            if data.sound_on {
                                Visibility::Visible
                            } else {
                                Visibility::Hidden
                            },
                        ));
                    });
                    row.spawn((
                        Text::new("启用音效"),
                        TextColor(Color::WHITE),
                        TextFont {
                            font: font.clone(),
                            font_size: FontSize::Px(22.0),
                            ..default()
                        },
                    ));
                });

            // ---- 音量滑块（Slider）----
            parent
                .spawn(Node {
                    align_items: AlignItems::Center,
                    column_gap: px(12),
                    ..default()
                })
                .with_children(|row| {
                    row.spawn((
                        VolumeSlider,
                        Slider::default(),
                        SliderValue(data.volume),
                        SliderRange::new(0.0, 100.0),
                        Node {
                            width: px(TRACK_WIDTH),
                            height: px(20),
                            border_radius: BorderRadius::all(px(10)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    ))
                    .with_children(|track| {
                        // thumb：绝对定位，初始位置按存档音量计算
                        let t = SliderRange::new(0.0, 100.0).thumb_position(data.volume);
                        track.spawn((
                            SliderThumb,
                            Node {
                                position_type: PositionType::Absolute,
                                width: px(THUMB_WIDTH),
                                height: px(THUMB_WIDTH),
                                left: px(t * (TRACK_WIDTH - THUMB_WIDTH)),
                                border_radius: BorderRadius::all(px(10)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.3, 0.5, 0.9)),
                        ));
                    });
                    row.spawn((
                        VolumeText,
                        Text::new(format!("音量 {:.0}", data.volume)),
                        TextColor(Color::WHITE),
                        TextFont {
                            font: font.clone(),
                            font_size: FontSize::Px(22.0),
                            ..default()
                        },
                    ));
                });

            // ---- 主题色（RadioGroup 单选组）----
            parent
                .spawn((
                    RadioGroup,
                    Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: px(10),
                        ..default()
                    },
                ))
                .with_children(|group| {
                    for (i, label) in ["蓝色主题", "紫色主题", "青色主题"].iter().enumerate()
                    {
                        let is_selected = data.theme == i;
                        // 单选按钮：spawn 后按选中状态补插 Checked
                        let mut button = group.spawn((
                            RadioButton,
                            ThemeIndex(i),
                            Node {
                                align_items: AlignItems::Center,
                                column_gap: px(10),
                                ..default()
                            },
                        ));
                        if is_selected {
                            button.insert(Checked);
                        }
                        button.with_children(|row| {
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
                                BackgroundColor(if is_selected {
                                    Color::srgb(0.3, 0.8, 0.4)
                                } else {
                                    Color::srgb(0.15, 0.15, 0.15)
                                }),
                            ));
                            row.spawn((
                                Text::new(*label),
                                TextColor(Color::WHITE),
                                TextFont {
                                    font: font.clone(),
                                    font_size: FontSize::Px(20.0),
                                    ..default()
                                },
                            ));
                        });
                    }
                });

            // 返回按钮
            parent
                .spawn((
                    BackButton,
                    Button,
                    Node {
                        padding: UiRect::all(px(12)),
                        border: UiRect::all(px(2)),
                        ..default()
                    },
                    BorderColor::all(Color::srgb(0.4, 0.6, 0.9)),
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new("返回清单"),
                        TextColor(Color::WHITE),
                        TextFont {
                            font: font.clone(),
                            font_size: FontSize::Px(24.0),
                            ..default()
                        },
                    ));
                });
        });

    info!(
        "[页面] 进入设置页（音效 {}，音量 {:.0}，主题 {}）",
        data.sound_on, data.volume, data.theme
    );
}

// ==================== 退出设置页 ====================
pub fn cleanup_settings(mut commands: Commands, query: Query<Entity, With<SettingsRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

// ==================== 观察者：控件值变化 → 更新数据 ====================
/// Checkbox 值变化：更新 AppData.sound_on + 勾选标记可见性
pub fn on_checkbox_change(
    trigger: On<ValueChange<bool>>,
    q_children: Query<&Children>,
    mut q_mark: Query<&mut Visibility, With<CheckMark>>,
    mut data: ResMut<AppData>,
) {
    data.sound_on = trigger.value;
    if let Ok(children) = q_children.get(trigger.source) {
        for child in children.iter() {
            if let Ok(mut vis) = q_mark.get_mut(child) {
                *vis = if trigger.value {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                };
            }
        }
    }
    info!("[设置] 音效 = {}", trigger.value);
}

/// Slider 值变化：更新 AppData.volume + thumb 位置 + 音量文本
pub fn on_slider_change(
    trigger: On<ValueChange<f32>>,
    q_slider: Query<(&SliderRange, &Children), With<VolumeSlider>>,
    mut q_thumb: Query<&mut Node, With<SliderThumb>>,
    mut q_volume_text: Query<&mut Text, With<VolumeText>>,
    mut data: ResMut<AppData>,
) {
    let value = trigger.value;
    data.volume = value;
    if let Ok((range, children)) = q_slider.get(trigger.source) {
        let t = range.thumb_position(value);
        if let Some(thumb) = children.first() {
            if let Ok(mut node) = q_thumb.get_mut(*thumb) {
                node.left = px(t * (TRACK_WIDTH - THUMB_WIDTH));
            }
        }
    }
    // 同步更新音量数字，避免停留在初始值让新手误以为没生效
    if let Ok(mut text) = q_volume_text.single_mut() {
        text.0 = format!("音量 {:.0}", value);
    }
    info!("[设置] 音量 = {:.0}", value);
}

/// RadioGroup 值变化：更新 AppData.theme + 圆圈视觉。
/// 注意：ListBox 也发 ValueChange<Entity>，这里用 With<RadioGroup> 过滤来源。
pub fn on_radio_change(
    trigger: On<ValueChange<Entity>>,
    q_groups: Query<(), With<RadioGroup>>,
    q_buttons: Query<(Entity, &ThemeIndex), With<RadioButton>>,
    q_children: Query<&Children>,
    mut q_dot: Query<&mut BackgroundColor, With<RadioDot>>,
    mut commands: Commands,
    mut data: ResMut<AppData>,
) {
    if q_groups.get(trigger.source).is_err() {
        return; // 不是单选组产生的选择事件，跳过
    }
    let selected = trigger.value;
    let mut theme = None;
    for (entity, index) in &q_buttons {
        let is_selected = entity == selected;
        if is_selected {
            theme = Some(index.0);
            commands.entity(entity).insert(Checked);
        } else {
            commands.entity(entity).remove::<Checked>();
        }
        if let Ok(children) = q_children.get(entity) {
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
    }
    if let Some(t) = theme {
        data.theme = t;
        info!("[设置] 主题 = {}", t);
    }
}

// ==================== 返回清单页 ====================
pub fn on_back(
    back_query: Query<&Interaction, (Changed<Interaction>, With<BackButton>)>,
    mut next_state: ResMut<NextState<AppPage>>,
) {
    for interaction in &back_query {
        if *interaction == Interaction::Pressed {
            next_state.set(AppPage::Todo);
        }
    }
}
