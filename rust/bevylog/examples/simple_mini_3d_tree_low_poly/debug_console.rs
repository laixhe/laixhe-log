//! 调试控制台（按反引号 `` ` `` 开合）：加殖民者、加资源（木材/食物/柴火）、
//! 瞬间完成全部或选中的建筑、快速建造开关、线框模式开关。
//! 线框模式会关掉平行光并压暗背景色，方便看清网格结构。

use bevy::pbr::wireframe::WireframeConfig;
use bevy::prelude::*;
use bevy::text::FontSource;

use crate::{
    building::{Blueprint, Profession},
    colonist::{Colonist, ColonistState},
    resources::{COLONIST_CARRY_CAPACITY, CentralStorage, Inventory, PublicInventory},
    selection::{SelectedTarget, SelectionState},
    types::ResourceKind,
    ui,
    world::{GameAssets, MainLight},
};

#[derive(Resource, Default)]
pub struct DebugConsoleState {
    pub visible: bool,
    pub fast_build: bool,
    pub wireframe_mode: bool,
}

#[derive(Component)]
pub struct DebugConsoleRoot;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum DebugButton {
    AddColonist,
    AddFiveColonists,
    AddWood100,
    AddWood1000,
    AddFood100,
    AddFood1000,
    AddFirewood100,
    AddFirewood1000,
    InstantFinishAll,
    InstantFinishSelected,
    ToggleFastBuild,
    ToggleWireframe,
}

#[derive(Component)]
struct FastBuildLabel;

#[derive(Component)]
struct WireframeLabel;

pub struct DebugConsolePlugin;

impl Plugin for DebugConsolePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugConsoleState>()
            .add_systems(
                Startup,
                spawn_debug_console.after(crate::world::setup_scene),
            )
            .add_systems(
                Update,
                (
                    toggle_debug_console,
                    update_debug_visibility,
                    handle_debug_buttons,
                    fast_build_blueprints,
                    update_fast_build_label,
                    apply_wireframe_mode,
                    update_wireframe_label,
                ),
            );
    }
}

fn spawn_debug_console(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 与 UI 一致，所有文本使用中文字体（默认字体不含中文）。
    let font_handle = asset_server.load("fonts/Yozai-Regular.ttf");
    // 返回 (字体, 排版) 两个组件。换行用 AnyCharacter：中文按字换行，且不触发 ICU4X 报错。
    let font = |size: f32| {
        (
            TextFont {
                font: FontSource::Handle(font_handle.clone()),
                font_size: FontSize::Px(size),
                ..default()
            },
            TextLayout {
                linebreak: LineBreak::AnyCharacter,
                ..default()
            },
        )
    };

    commands
        .spawn((
            DebugConsoleRoot,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(12.0),
                top: Val::Px(118.0),
                min_width: Val::Px(480.0),
                display: Display::None,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                padding: UiRect::all(Val::Px(12.0)),
                ..default()
            },
            BackgroundColor(ui::PANEL),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("调试控制台"),
                font(14.0),
                TextColor(Color::srgb(0.86, 0.9, 0.92)),
            ));

            // Row 1: Colonists
            parent
                .spawn((Node {
                    display: Display::Flex,
                    column_gap: Val::Px(8.0),
                    ..default()
                },))
                .with_children(|row| {
                    row.spawn(ui::utility_button(
                        &font,
                        "+1 殖民者",
                        DebugButton::AddColonist,
                    ));
                    row.spawn(ui::utility_button(
                        &font,
                        "+5 殖民者",
                        DebugButton::AddFiveColonists,
                    ));
                });

            // Row 2: Wood
            parent
                .spawn((Node {
                    display: Display::Flex,
                    column_gap: Val::Px(8.0),
                    ..default()
                },))
                .with_children(|row| {
                    row.spawn(ui::utility_button(
                        &font,
                        "+100 木材",
                        DebugButton::AddWood100,
                    ));
                    row.spawn(ui::utility_button(
                        &font,
                        "+1000 木材",
                        DebugButton::AddWood1000,
                    ));
                });

            // Row 3: Food
            parent
                .spawn((Node {
                    display: Display::Flex,
                    column_gap: Val::Px(8.0),
                    ..default()
                },))
                .with_children(|row| {
                    row.spawn(ui::utility_button(
                        &font,
                        "+100 食物",
                        DebugButton::AddFood100,
                    ));
                    row.spawn(ui::utility_button(
                        &font,
                        "+1000 食物",
                        DebugButton::AddFood1000,
                    ));
                });

            // Row 4: Firewood
            parent
                .spawn((Node {
                    display: Display::Flex,
                    column_gap: Val::Px(8.0),
                    ..default()
                },))
                .with_children(|row| {
                    row.spawn(ui::utility_button(
                        &font,
                        "+100 柴火",
                        DebugButton::AddFirewood100,
                    ));
                    row.spawn(ui::utility_button(
                        &font,
                        "+1000 柴火",
                        DebugButton::AddFirewood1000,
                    ));
                });

            // Row 5: Building
            parent
                .spawn((Node {
                    display: Display::Flex,
                    column_gap: Val::Px(8.0),
                    ..default()
                },))
                .with_children(|row| {
                    row.spawn(ui::utility_button(
                        &font,
                        "全部完成",
                        DebugButton::InstantFinishAll,
                    ));
                    row.spawn(ui::utility_button(
                        &font,
                        "完成选中",
                        DebugButton::InstantFinishSelected,
                    ));
                });

            // Row 6: Fast Build toggle (built manually for FastBuildLabel marker)
            parent
                .spawn((
                    Button,
                    DebugButton::ToggleFastBuild,
                    ui::button_node(),
                    BackgroundColor(ui::BUTTON),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        FastBuildLabel,
                        Text::new("快速建造：关"),
                        font(12.0),
                        TextColor(Color::WHITE),
                    ));
                });

            // Row 7: Wireframe toggle
            parent
                .spawn((
                    Button,
                    DebugButton::ToggleWireframe,
                    ui::button_node(),
                    BackgroundColor(ui::BUTTON),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        WireframeLabel,
                        Text::new("线框：关"),
                        font(12.0),
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

fn toggle_debug_console(keyboard: Res<ButtonInput<KeyCode>>, mut state: ResMut<DebugConsoleState>) {
    if keyboard.just_pressed(KeyCode::Backquote) {
        state.visible = !state.visible;
    }
}

fn update_debug_visibility(
    state: Res<DebugConsoleState>,
    mut panel: Query<&mut Node, With<DebugConsoleRoot>>,
) {
    if let Ok(mut node) = panel.single_mut() {
        node.display = if state.visible {
            Display::Flex
        } else {
            Display::None
        };
    }
}

fn handle_debug_buttons(
    mut commands: Commands,
    mut debug_state: ResMut<DebugConsoleState>,
    assets: Res<GameAssets>,
    selection: Res<SelectionState>,
    colonists: Query<&Colonist>,
    mut blueprints: Query<&mut Blueprint>,
    mut central_inventories: Query<&mut Inventory, With<CentralStorage>>,
    mut public_inventories: Query<&mut Inventory, (With<PublicInventory>, Without<CentralStorage>)>,
    mut buttons: Query<
        (&Interaction, &DebugButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, button, mut color) in &mut buttons {
        match *interaction {
            Interaction::Pressed => {
                match button {
                    DebugButton::AddColonist => {
                        let count = colonists.iter().count() as u32;
                        spawn_debug_colonists(&mut commands, &assets, 1, count);
                    }
                    DebugButton::AddFiveColonists => {
                        let count = colonists.iter().count() as u32;
                        spawn_debug_colonists(&mut commands, &assets, 5, count);
                    }
                    DebugButton::AddWood100 => add_debug_resource(
                        &mut central_inventories,
                        &mut public_inventories,
                        ResourceKind::Wood,
                        100,
                    ),
                    DebugButton::AddWood1000 => add_debug_resource(
                        &mut central_inventories,
                        &mut public_inventories,
                        ResourceKind::Wood,
                        1000,
                    ),
                    DebugButton::AddFood100 => add_debug_resource(
                        &mut central_inventories,
                        &mut public_inventories,
                        ResourceKind::Food,
                        100,
                    ),
                    DebugButton::AddFood1000 => add_debug_resource(
                        &mut central_inventories,
                        &mut public_inventories,
                        ResourceKind::Food,
                        1000,
                    ),
                    DebugButton::AddFirewood100 => add_debug_resource(
                        &mut central_inventories,
                        &mut public_inventories,
                        ResourceKind::Firewood,
                        100,
                    ),
                    DebugButton::AddFirewood1000 => add_debug_resource(
                        &mut central_inventories,
                        &mut public_inventories,
                        ResourceKind::Firewood,
                        1000,
                    ),
                    DebugButton::InstantFinishAll => {
                        for mut bp in &mut blueprints {
                            bp.delivered_wood = bp.required_wood;
                            bp.progress = bp.build_seconds;
                        }
                    }
                    DebugButton::InstantFinishSelected => {
                        if let Some(SelectedTarget::Blueprint(entity)) = selection.selected
                            && let Ok(mut bp) = blueprints.get_mut(entity)
                        {
                            bp.delivered_wood = bp.required_wood;
                            bp.progress = bp.build_seconds;
                        }
                    }
                    DebugButton::ToggleFastBuild => {
                        debug_state.fast_build = !debug_state.fast_build;
                    }
                    DebugButton::ToggleWireframe => {
                        debug_state.wireframe_mode = !debug_state.wireframe_mode;
                    }
                }
                *color = BackgroundColor(ui::BUTTON_ACTIVE);
            }
            Interaction::Hovered => *color = BackgroundColor(ui::BUTTON_HOVER),
            Interaction::None => *color = BackgroundColor(ui::BUTTON),
        }
    }
}

fn add_debug_resource(
    central_inventories: &mut Query<&mut Inventory, With<CentralStorage>>,
    public_inventories: &mut Query<
        &mut Inventory,
        (With<PublicInventory>, Without<CentralStorage>),
    >,
    kind: ResourceKind,
    amount: i32,
) {
    let mut remaining = amount;
    for mut inventory in central_inventories.iter_mut() {
        remaining -= inventory.add_partial(kind, remaining);
        if remaining <= 0 {
            return;
        }
    }

    for mut inventory in public_inventories.iter_mut() {
        remaining -= inventory.add_partial(kind, remaining);
        if remaining <= 0 {
            return;
        }
    }
}

fn fast_build_blueprints(state: Res<DebugConsoleState>, mut blueprints: Query<&mut Blueprint>) {
    if !state.fast_build {
        return;
    }
    for mut bp in &mut blueprints {
        bp.delivered_wood = bp.required_wood;
        bp.progress = bp.build_seconds;
    }
}

fn update_fast_build_label(
    state: Res<DebugConsoleState>,
    mut texts: Query<&mut Text, With<FastBuildLabel>>,
) {
    if let Ok(mut text) = texts.single_mut() {
        let new = if state.fast_build {
            "快速建造：开".to_string()
        } else {
            "快速建造：关".to_string()
        };
        if text.0 != new {
            text.0 = new;
        }
    }
}

fn apply_wireframe_mode(
    state: Res<DebugConsoleState>,
    mut wireframe_config: ResMut<WireframeConfig>,
    mut clear_color: ResMut<ClearColor>,
    mut lights: Query<&mut DirectionalLight, With<MainLight>>,
) {
    if !state.is_changed() {
        return;
    }
    wireframe_config.global = state.wireframe_mode;
    if state.wireframe_mode {
        wireframe_config.default_color = Color::srgb(0.25, 0.25, 0.25);
        clear_color.0 = Color::srgb(0.05, 0.05, 0.08);
        if let Ok(mut light) = lights.single_mut() {
            light.illuminance = 0.0;
            light.shadow_maps_enabled = false;
        }
    } else {
        wireframe_config.default_color = Color::WHITE;
        clear_color.0 = Color::srgb(0.76, 0.8, 0.86);
        if let Ok(mut light) = lights.single_mut() {
            light.illuminance = 12_000.0;
            light.shadow_maps_enabled = true;
        }
    }
}

fn update_wireframe_label(
    state: Res<DebugConsoleState>,
    mut texts: Query<&mut Text, With<WireframeLabel>>,
) {
    if let Ok(mut text) = texts.single_mut() {
        let new = if state.wireframe_mode {
            "线框：开".to_string()
        } else {
            "线框：关".to_string()
        };
        if text.0 != new {
            text.0 = new;
        }
    }
}

fn spawn_debug_colonists(
    commands: &mut Commands,
    assets: &GameAssets,
    count: u32,
    existing_count: u32,
) {
    for i in 0..count {
        let index = existing_count + i;
        let x_off = (i % 5) as f32 * 0.8 - 1.6;
        let z_off = 2.0 + (i / 5) as f32 * 0.8;
        commands.spawn((
            Mesh3d(assets.colonist_mesh.clone()),
            MeshMaterial3d(assets.colonist_material.clone()),
            Transform::from_translation(Vec3::new(x_off, 0.32, z_off)),
            Colonist {
                name: format!("定居者 {}", index + 1),
                state: ColonistState::Idle,
                profession: Profession::Unemployed,
                workplace: None,
                speed: 2.2,
                home: None,
                satiety: 100.0,
                carry_capacity: COLONIST_CARRY_CAPACITY,
            },
        ));
    }
}
