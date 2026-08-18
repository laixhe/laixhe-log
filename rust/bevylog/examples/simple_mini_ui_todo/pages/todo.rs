use bevy::input_focus::InputFocus;
use bevy::prelude::*;
use bevy::text::{EditableText, FontSource, TextCursorStyle};
use bevy::ui::Selected;
use bevy::ui_widgets::{ListBox, ListItem, ScrollArea, ValueChange};

use crate::pages::router::AppPage;
use crate::state::{AppData, ListDirty, SaveSlot, SelectedIndex, theme_color};

// 中文字体路径（与其它示例共用同一资产）
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// ==================== 组件标记 ====================
#[derive(Component, Clone, Default)]
pub struct TodoRoot; // 整页根标记：OnExit 时一键清理

#[derive(Component)]
pub struct TodoInput; // 可编辑文本框（新增待办）

#[derive(Component)]
pub struct AddButton; // 添加按钮
#[derive(Component)]
pub struct DeleteButton; // 删除选中按钮
#[derive(Component)]
pub struct SaveButton; // 保存按钮
#[derive(Component)]
pub struct LoadButton; // 读档按钮
#[derive(Component)]
pub struct SettingsButton; // 去设置页按钮

#[derive(Component)]
pub struct TodoListBox; // ListBox 容器（列表项重建时用它定位）

#[derive(Component)]
pub struct TodoItemIndex(pub usize); // 列表项在 AppData.todos 中的索引

// ==================== 进入清单页 ====================
pub fn setup_todo(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    data: Res<AppData>,
    mut dirty: ResMut<ListDirty>,
) {
    let font = FontSource::Handle(asset_server.load(FONT_PATH));

    commands
        .spawn((
            TodoRoot,
            Node {
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: px(16),
                ..default()
            },
            BackgroundColor(theme_color(data.theme)),
        ))
        .with_children(|parent| {
            // 标题
            parent.spawn((
                Text::new("待办清单"),
                TextColor(Color::srgb(0.6, 0.8, 1.0)),
                TextFont {
                    font: font.clone(),
                    font_size: FontSize::Px(42.0),
                    ..default()
                },
            ));

            // 输入行：文本框 + 添加按钮
            parent
                .spawn(Node {
                    column_gap: px(10),
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|row| {
                    // 可编辑文本框（点击聚焦输入，回车提交）
                    row.spawn((
                        TodoInput,
                        Node {
                            width: px(300),
                            height: px(42),
                            border: UiRect::all(px(2)),
                            padding: UiRect::all(px(8)),
                            ..default()
                        },
                        BorderColor::all(Color::srgb(0.4, 0.5, 0.6)),
                        BackgroundColor(Color::srgb(0.12, 0.12, 0.16)),
                        EditableText::default(),
                        TextFont {
                            font: font.clone(),
                            font_size: FontSize::Px(20.0),
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        TextCursorStyle::default(),
                    ));
                    // 添加按钮
                    row.spawn((
                        AddButton,
                        Button,
                        Node {
                            padding: UiRect::all(px(10)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.4, 0.3)),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new("添加"),
                            TextColor(Color::WHITE),
                            TextFont {
                                font: font.clone(),
                                font_size: FontSize::Px(20.0),
                                ..default()
                            },
                        ));
                    });
                });

            // 待办列表：ListBox + ScrollArea，固定高度超出可滚动
            parent
                .spawn((
                    TodoListBox,
                    ListBox,
                    ScrollArea,
                    Node {
                        width: px(420),
                        height: px(240),
                        overflow: Overflow::scroll_y(),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(4),
                        padding: UiRect::all(px(8)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.1, 0.1, 0.13)),
                ))
                .with_children(|list| {
                    // 初始列表项由 rebuild_list_system 生成（setup 结束置脏标记）
                    let _ = list;
                });

            // 操作行：删除 / 保存 / 读档 / 设置
            parent
                .spawn(Node {
                    column_gap: px(10),
                    ..default()
                })
                .with_children(|row| {
                    row.spawn((
                        DeleteButton,
                        Button,
                        Node {
                            padding: UiRect::all(px(8)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.4, 0.2, 0.2)),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new("删除选中"),
                            TextColor(Color::WHITE),
                            TextFont {
                                font: font.clone(),
                                font_size: FontSize::Px(18.0),
                                ..default()
                            },
                        ));
                    });
                    row.spawn((
                        SaveButton,
                        Button,
                        Node {
                            padding: UiRect::all(px(8)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.3, 0.4)),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new("保存"),
                            TextColor(Color::WHITE),
                            TextFont {
                                font: font.clone(),
                                font_size: FontSize::Px(18.0),
                                ..default()
                            },
                        ));
                    });
                    row.spawn((
                        LoadButton,
                        Button,
                        Node {
                            padding: UiRect::all(px(8)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.3, 0.4)),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new("读档"),
                            TextColor(Color::WHITE),
                            TextFont {
                                font: font.clone(),
                                font_size: FontSize::Px(18.0),
                                ..default()
                            },
                        ));
                    });
                    row.spawn((
                        SettingsButton,
                        Button,
                        Node {
                            padding: UiRect::all(px(8)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.3, 0.3, 0.4)),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new("设置"),
                            TextColor(Color::WHITE),
                            TextFont {
                                font: font.clone(),
                                font_size: FontSize::Px(18.0),
                                ..default()
                            },
                        ));
                    });
                });

            // 操作提示
            parent.spawn((
                Text::new(
                    "输入内容回车 / 点击「添加」 | 点击列表选中 | S 保存 / L 读档 | 设置页可调主题",
                ),
                TextColor(Color::srgb(0.5, 0.55, 0.65)),
                TextFont {
                    font: font.clone(),
                    font_size: FontSize::Px(16.0),
                    ..default()
                },
            ));
        });

    // 置脏：首帧由 rebuild_list_system 生成列表项
    dirty.0 = true;
    info!("[页面] 进入待办清单页（{} 条待办）", data.todos.len());
}

// ==================== 退出清单页 ====================
pub fn cleanup_todo(mut commands: Commands, query: Query<Entity, With<TodoRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

// ==================== 列表重建 ====================
/// 数据 → UI 单向同步：dirty 时清空旧列表项，按 AppData.todos 重建 ListItem。
/// 数据变化（添加/删除/读档）只需改 AppData + 置脏，渲染自动跟随。
pub fn rebuild_list_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    data: Res<AppData>,
    mut dirty: ResMut<ListDirty>,
    selected: Res<SelectedIndex>,
    items: Query<Entity, With<TodoItemIndex>>,
    list_boxes: Query<Entity, With<TodoListBox>>,
) {
    if !dirty.0 {
        return;
    }
    let font = FontSource::Handle(asset_server.load(FONT_PATH));

    // 清理旧列表项
    for entity in &items {
        commands.entity(entity).despawn();
    }

    // 重建列表项
    if let Ok(list_entity) = list_boxes.single() {
        commands.entity(list_entity).with_children(|parent| {
            for (i, todo) in data.todos.iter().enumerate() {
                let is_selected = selected.0 == Some(i);
                let mut item = parent.spawn((
                    TodoItemIndex(i),
                    ListItem,
                    Node {
                        width: percent(100),
                        height: px(38),
                        align_items: AlignItems::Center,
                        padding: UiRect::horizontal(px(12)),
                        border_radius: BorderRadius::all(px(6)),
                        ..default()
                    },
                    BackgroundColor(if is_selected {
                        Color::srgb(0.3, 0.6, 0.9)
                    } else {
                        Color::srgb(0.18, 0.18, 0.22)
                    }),
                ));
                if is_selected {
                    item.insert(Selected);
                }
                item.with_children(|row| {
                    row.spawn((
                        Text::new(format!("{}. {}", i + 1, todo)),
                        TextColor(Color::WHITE),
                        TextFont {
                            font: font.clone(),
                            font_size: FontSize::Px(18.0),
                            ..default()
                        },
                    ));
                });
            }
        });
    }

    dirty.0 = false;
}

// ==================== 添加待办 ====================
pub fn on_add(
    keyboard: Res<ButtonInput<KeyCode>>,
    add_query: Query<&Interaction, (Changed<Interaction>, With<AddButton>)>,
    mut inputs: Query<&mut EditableText, With<TodoInput>>,
    mut data: ResMut<AppData>,
    mut dirty: ResMut<ListDirty>,
) {
    // 回车 或 点击「添加」按钮都触发
    let add_pressed = add_query.iter().any(|i| *i == Interaction::Pressed)
        || keyboard.just_pressed(KeyCode::Enter);
    if !add_pressed {
        return;
    }
    let Ok(mut input) = inputs.single_mut() else {
        return;
    };
    // input.value() 返回 SplitString，先转成 String 再 trim 判空
    let text = input.value().to_string().trim().to_string();
    if text.is_empty() {
        return;
    }
    data.todos.push(text.clone());
    input.clear();
    dirty.0 = true;
    info!("[待办] 添加：{text}");
}

// ==================== 列表选择（观察者） ====================
/// 监听 ListBox 的 ValueChange<Entity>：维护 Selected 组件 + 高亮 + 记录索引。
/// 注意：RadioGroup 也发 ValueChange<Entity>，这里用 With<ListBox> 过滤来源。
pub fn on_list_change(
    trigger: On<ValueChange<Entity>>,
    list_boxes: Query<(), With<ListBox>>,
    q_items: Query<(Entity, &TodoItemIndex), With<ListItem>>,
    mut q_bg: Query<&mut BackgroundColor>,
    mut commands: Commands,
    mut selected: ResMut<SelectedIndex>,
) {
    if list_boxes.get(trigger.source).is_err() {
        return; // 不是列表产生的选择事件，跳过
    }
    let selected_entity = trigger.value;
    let mut selected_idx = None;
    for (entity, index) in &q_items {
        let is_selected = entity == selected_entity;
        if is_selected {
            selected_idx = Some(index.0);
            commands.entity(entity).insert(Selected);
        } else {
            commands.entity(entity).remove::<Selected>();
        }
        if let Ok(mut bg) = q_bg.get_mut(entity) {
            *bg = BackgroundColor(if is_selected {
                Color::srgb(0.3, 0.6, 0.9)
            } else {
                Color::srgb(0.18, 0.18, 0.22)
            });
        }
    }
    selected.0 = selected_idx;
    info!("[待办] 选中索引 {:?}", selected_idx);
}

// ==================== 删除选中 ====================
pub fn on_delete(
    delete_query: Query<&Interaction, (Changed<Interaction>, With<DeleteButton>)>,
    mut data: ResMut<AppData>,
    mut dirty: ResMut<ListDirty>,
    mut selected: ResMut<SelectedIndex>,
) {
    for interaction in &delete_query {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if let Some(i) = selected.0 {
            if i < data.todos.len() {
                let removed = data.todos.remove(i);
                info!("[待办] 删除：{removed}");
                dirty.0 = true;
                selected.0 = None;
            }
        }
    }
}

// ==================== 保存 / 读档 ====================
/// 键盘快捷键是否被输入框占用：输入框聚焦时 S/L 不触发存档/读档，
/// 否则在输入框里打字（如 "load"）会意外读档覆盖未保存的修改。
fn typing_in_input(
    input_focus: Res<InputFocus>,
    todo_inputs: Query<Entity, With<TodoInput>>,
) -> bool {
    let Some(focused) = input_focus.get() else {
        return false;
    };
    todo_inputs.contains(focused)
}

pub fn on_save(
    save_query: Query<&Interaction, (Changed<Interaction>, With<SaveButton>)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    input_focus: Res<InputFocus>,
    todo_inputs: Query<Entity, With<TodoInput>>,
    data: Res<AppData>,
    mut slot: ResMut<SaveSlot>,
) {
    if !(save_query.iter().any(|i| *i == Interaction::Pressed)
        || (keyboard.just_pressed(KeyCode::KeyS) && !typing_in_input(input_focus, todo_inputs)))
    {
        return;
    }
    match ron::to_string(&*data) {
        Ok(s) => {
            slot.0 = s;
            info!("[存档] 已保存 {} 条待办 + 设置", data.todos.len());
        }
        Err(e) => info!("[存档] 保存失败：{e}"),
    }
}

pub fn on_load(
    load_query: Query<&Interaction, (Changed<Interaction>, With<LoadButton>)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    input_focus: Res<InputFocus>,
    todo_inputs: Query<Entity, With<TodoInput>>,
    mut data: ResMut<AppData>,
    slot: Res<SaveSlot>,
    mut dirty: ResMut<ListDirty>,
) {
    if !(load_query.iter().any(|i| *i == Interaction::Pressed)
        || (keyboard.just_pressed(KeyCode::KeyL) && !typing_in_input(input_focus, todo_inputs)))
    {
        return;
    }
    if slot.0.is_empty() {
        info!("[存档] 还没有存档，先点击「保存」");
        return;
    }
    match ron::from_str::<AppData>(&slot.0) {
        Ok(loaded) => {
            info!("[存档] 已读档：{} 条待办", loaded.todos.len());
            *data = loaded;
            dirty.0 = true;
        }
        Err(e) => info!("[存档] 读档失败：{e}"),
    }
}

// ==================== 跳转设置页 ====================
pub fn on_settings(
    settings_query: Query<&Interaction, (Changed<Interaction>, With<SettingsButton>)>,
    mut next_state: ResMut<NextState<AppPage>>,
) {
    for interaction in &settings_query {
        if *interaction == Interaction::Pressed {
            next_state.set(AppPage::Settings);
        }
    }
}
