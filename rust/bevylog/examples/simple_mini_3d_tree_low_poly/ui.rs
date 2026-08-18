//! HUD 界面（纯代码生成，无 .ron 场景文件）：资源栏、建造按钮、选中对象信息面板、
//! 岗位数量调节（+ / -）、时间控制（暂停 / 倍率）、网格吸附开关、FPS。
//! `F1` 可整体隐藏 / 显示。

use bevy::prelude::*;
use bevy::text::FontSource;

use crate::{
    building::{
        Blueprint, BuildState, CompletedBuilding, Footprint, Housing, Workplace, WorldGeometry,
    },
    colonist::{Colonist, GoalState},
    farm::{CompletedFarmPlot, FarmPlot},
    resources::{CentralStorage, Inventory, PublicInventory, public_stock},
    selection::{SelectedTarget, SelectionState},
    simulation::SimulationClock,
    terrain::TerrainGenerationConfig,
    types::{BuildingKind, CONSTRUCTION_KINDS, ConstructionKind, MAP_GRID_CELLS, ResourceKind},
    world::ResourceNode,
};

pub(crate) const PANEL: Color = Color::srgba(0.08, 0.09, 0.1, 0.82);
pub(crate) const BUTTON: Color = Color::srgb(0.18, 0.2, 0.22);
pub(crate) const BUTTON_HOVER: Color = Color::srgb(0.26, 0.29, 0.31);
pub(crate) const BUTTON_ACTIVE: Color = Color::srgb(0.26, 0.42, 0.28);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiVisibility>()
            .init_resource::<UiRefreshTimer>()
            .add_systems(Startup, spawn_ui.after(crate::world::setup_scene))
            .add_systems(
                Startup,
                (spawn_help_panel, spawn_victory_overlay, spawn_tutorial),
            )
            .add_systems(
                Update,
                (
                    toggle_ui_visibility,
                    update_ui_visibility,
                    handle_ui_buttons,
                    update_ui_text,
                    toggle_help,
                    update_victory_overlay,
                    handle_victory_button,
                    update_tutorial_text,
                ),
            );
    }
}

// FPS 文字节流计时器：bevy 0.19 的文本布局引擎对中文做分词缺少 ICU4X 数据，
// 每次布局都会打一条警告日志，所以动态文本只在实际变化时才更新，
// FPS 则限频刷新（每秒约 2 次），避免日志刷屏。
#[derive(Resource)]
struct UiRefreshTimer(Timer);

impl Default for UiRefreshTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Repeating))
    }
}

#[derive(Resource)]
pub struct UiVisibility {
    pub visible: bool,
}

impl Default for UiVisibility {
    fn default() -> Self {
        Self { visible: true }
    }
}

#[derive(Component)]
struct UiRoot;

fn toggle_ui_visibility(keyboard: Res<ButtonInput<KeyCode>>, mut visibility: ResMut<UiVisibility>) {
    if keyboard.just_pressed(KeyCode::F1) {
        visibility.visible = !visibility.visible;
    }
}

fn update_ui_visibility(visibility: Res<UiVisibility>, mut panels: Query<&mut Node, With<UiRoot>>) {
    if !visibility.is_changed() {
        return;
    }
    for mut node in &mut panels {
        node.display = if visibility.visible {
            Display::Flex
        } else {
            Display::None
        };
    }
}

#[derive(Component)]
pub struct ResourceText;

#[derive(Component)]
pub struct StatusText;

#[derive(Component)]
pub struct TerrainDebugText;

#[derive(Component)]
pub struct SelectionTitle;

#[derive(Component)]
pub struct SelectionBody;

#[derive(Component)]
pub struct JobControlsRoot;

#[derive(Component)]
pub struct JobSlotsText;

#[derive(Component)]
pub struct JobSlotButton(pub i8);

#[derive(Component)]
pub struct BuildButton(pub ConstructionKind);

#[derive(Component)]
pub enum TimeButton {
    Pause,
    Speed(f32),
}

#[derive(Component)]
pub struct SnapButton;

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct HelpRoot;

#[derive(Component)]
pub struct VictoryOverlay;

#[derive(Component)]
pub struct VictoryContinueButton;

#[derive(Component)]
pub struct TutorialRoot;

#[derive(Component)]
pub struct TutorialText;

pub fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 中文字体：Bevy 默认字体不含中文，必须显式加载项目自带的字体文件。
    // 所有文本节点都要带上 TextFont（用这个字体），否则中文会显示为方块。
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

    // 顶栏：资源 + 状态。宽度按内容自适应（不设 right），把右上角让给独立的 FPS 面板。
    commands.spawn((
        UiRoot,
        Node {
            position_type: PositionType::Absolute,
            left: px(12),
            top: px(12),
            height: px(44),
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            padding: UiRect::axes(px(14), px(8)),
            ..default()
        },
        BackgroundColor(PANEL),
        children![
            (
                ResourceText,
                Text::new("木材：0  食物：0  柴火：0  人口：0/0  时间：1x"),
                font(14.0),
                TextColor(Color::WHITE),
            ),
            (
                StatusText,
                Text::new("选择一个建筑。"),
                font(14.0),
                TextColor(Color::srgb(0.86, 0.9, 0.92)),
            )
        ],
    ));

    // 独立 FPS 面板：固定在右上角，与顶栏分开，便于随时观察帧率。
    commands.spawn((
        UiRoot,
        Node {
            position_type: PositionType::Absolute,
            right: px(12),
            top: px(12),
            min_width: px(96),
            height: px(44),
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            padding: UiRect::axes(px(12), px(8)),
            ..default()
        },
        BackgroundColor(PANEL),
        children![(
            FpsText,
            Text::new("FPS: --"),
            font(14.0),
            TextColor(Color::srgb(0.86, 0.9, 0.92)),
        )],
    ));

    commands.spawn((
        UiRoot,
        Node {
            position_type: PositionType::Absolute,
            left: px(12),
            top: px(68),
            width: px(470),
            min_height: px(38),
            display: Display::Flex,
            align_items: AlignItems::Center,
            padding: UiRect::axes(px(12), px(6)),
            ..default()
        },
        BackgroundColor(PANEL),
        children![(
            TerrainDebugText,
            Text::new("地图：432x432  种子：0x0000000000000000  资源点：木材 0 / 食物 0"),
            font(13.0),
            TextColor(Color::srgb(0.84, 0.88, 0.9)),
        )],
    ));

    commands.spawn((
        UiRoot,
        Node {
            position_type: PositionType::Absolute,
            right: px(12),
            top: px(68),
            width: px(330),
            min_height: px(220),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: px(10),
            padding: UiRect::all(px(14)),
            ..default()
        },
        BackgroundColor(PANEL),
        children![
            (
                SelectionTitle,
                Text::new("未选择"),
                font(16.0),
                TextColor(Color::WHITE),
            ),
            (
                SelectionBody,
                Text::new("点击定居者、建筑、蓝图或资源节点。"),
                font(13.0),
                TextColor(Color::srgb(0.84, 0.88, 0.9)),
            ),
            (
                JobControlsRoot,
                Node {
                    display: Display::None,
                    align_items: AlignItems::Center,
                    column_gap: px(8),
                    ..default()
                },
                children![
                    job_slot_button(&font, "-", -1),
                    (
                        JobSlotsText,
                        Text::new("工人：0/0"),
                        font(13.0),
                        TextColor(Color::srgb(0.9, 0.92, 0.86)),
                    ),
                    job_slot_button(&font, "+", 1),
                ],
            ),
        ],
    ));

    commands.spawn((
        UiRoot,
        Node {
            position_type: PositionType::Absolute,
            left: px(12),
            bottom: px(12),
            display: Display::Flex,
            column_gap: px(8),
            align_items: AlignItems::Center,
            padding: UiRect::all(px(8)),
            ..default()
        },
        BackgroundColor(PANEL),
        children![
            build_button(&font, CONSTRUCTION_KINDS[0]),
            build_button(&font, CONSTRUCTION_KINDS[1]),
            build_button(&font, CONSTRUCTION_KINDS[2]),
            build_button(&font, CONSTRUCTION_KINDS[3]),
            build_button(&font, CONSTRUCTION_KINDS[4]),
            build_button(&font, CONSTRUCTION_KINDS[5]),
            build_button(&font, CONSTRUCTION_KINDS[6]),
            utility_button(&font, "网格吸附 G", SnapButton),
            time_button(&font, "暂停", TimeButton::Pause),
            time_button(&font, "1x", TimeButton::Speed(1.0)),
            time_button(&font, "2x", TimeButton::Speed(2.0)),
            time_button(&font, "4x", TimeButton::Speed(4.0)),
        ],
    ));
}

// 帮助面板（按 H 开合）：集中列出所有操作键位，方便新手随时查看。
fn spawn_help_panel(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load("fonts/Yozai-Regular.ttf");
    let font = |size: f32| TextFont {
        font: FontSource::Handle(font_handle.clone()),
        font_size: FontSize::Px(size),
        ..default()
    };

    commands.spawn((
        HelpRoot,
        Node {
            position_type: PositionType::Absolute,
            left: px(12),
            top: px(120),
            min_width: px(470),
            display: Display::None, // 默认隐藏，按 H 打开
            flex_direction: FlexDirection::Column,
            row_gap: px(8),
            padding: UiRect::all(px(14)),
            ..default()
        },
        BackgroundColor(PANEL),
        children![
            (Text::new("操作说明"), font(16.0), TextColor(Color::WHITE),),
            (
                Text::new(
                    "选建筑：1 房屋  2 仓库  3 伐木屋  4 采集屋  5 道路  6 农场  7 劈柴场\n\
                     放置 / 选择：鼠标左键      取消：右键 / Esc\n\
                     旋转建筑：R（网格模式短按 90°，自由模式长按连续旋转）\n\
                     网格吸附：G      暂停 / 继续：空格      隐藏 UI：F1\n\
                     拆除建筑：选中后按 Delete（中央仓库不可拆）\n\
                     农场：左键点角点，右键撤销，回车闭合\n\
                     调试台：~（Esc 键下方）\n\
                     视角：右键旋转 / 中键平移 / 滚轮缩放\n\
                     人口：食物充足且有房时自动增长；目标人口 10\n\
                     再次按 H 关闭本面板",
                ),
                font(13.0),
                TextColor(Color::srgb(0.86, 0.9, 0.92)),
            )
        ],
    ));
}

fn toggle_help(keyboard: Res<ButtonInput<KeyCode>>, mut panels: Query<&mut Node, With<HelpRoot>>) {
    if keyboard.just_pressed(KeyCode::KeyH) {
        for mut node in &mut panels {
            node.display = if node.display == Display::None {
                Display::Flex
            } else {
                Display::None
            };
        }
    }
}

// 胜利提示：人口目标达成后弹出全屏遮罩 + 说明 + 「继续游戏」按钮。
fn spawn_victory_overlay(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load("fonts/Yozai-Regular.ttf");
    let font = |size: f32| TextFont {
        font: FontSource::Handle(font_handle.clone()),
        font_size: FontSize::Px(size),
        ..default()
    };

    commands.spawn((
        VictoryOverlay,
        Node {
            position_type: PositionType::Absolute,
            left: px(0),
            right: px(0),
            top: px(0),
            bottom: px(0),
            display: Display::None, // 达成目标后才显示
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: px(16),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.55)),
        children![(
            Node {
                min_width: px(360),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: px(14),
                padding: UiRect::all(px(28)),
                ..default()
            },
            BackgroundColor(PANEL),
            children![
                (
                    Text::new("目标达成！"),
                    font(32.0),
                    TextColor(Color::srgb(0.95, 0.8, 0.3)),
                ),
                (
                    Text::new("聚落人口达到 10，兴旺繁荣！"),
                    font(16.0),
                    TextColor(Color::WHITE),
                ),
                (
                    Button,
                    VictoryContinueButton,
                    button_node(),
                    BackgroundColor(BUTTON),
                    children![(Text::new("继续游戏"), font(14.0), TextColor(Color::WHITE))],
                ),
            ],
        )],
    ));
}

fn update_victory_overlay(
    goal: Res<GoalState>,
    mut overlay: Query<&mut Node, With<VictoryOverlay>>,
) {
    let Ok(mut node) = overlay.single_mut() else {
        return;
    };
    let display = if goal.reached && !goal.dismissed {
        Display::Flex
    } else {
        Display::None
    };
    if node.display != display {
        node.display = display;
    }
}

fn handle_victory_button(
    mut goal: ResMut<GoalState>,
    mut buttons: Query<(&Interaction, &VictoryContinueButton), Changed<Interaction>>,
) {
    for (interaction, _) in &mut buttons {
        if *interaction == Interaction::Pressed {
            goal.dismissed = true;
        }
    }
}

// 开局教程提示条：固定在顶部居中，随聚落进度给出「下一步该做什么」。
fn spawn_tutorial(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load("fonts/Yozai-Regular.ttf");
    commands.spawn((
        TutorialRoot,
        Node {
            position_type: PositionType::Absolute,
            top: px(60),
            left: px(0),
            right: px(0),
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Node {
                padding: UiRect::axes(px(12), px(6)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
            children![(
                TutorialText,
                Text::new("第一步：按 3 建造伐木屋，让定居者砍树获取木材。"),
                TextFont {
                    font: FontSource::Handle(font_handle),
                    font_size: FontSize::Px(13.0),
                    ..default()
                },
                TextLayout {
                    linebreak: LineBreak::AnyCharacter,
                    ..default()
                },
                TextColor(Color::WHITE),
            )],
        )],
    ));
}

// 分步引导：按「伐木屋 → 采集屋 → 房屋 → 冲人口」的顺序给出提示，
// 只在实际进度变化时更新文本，避免触发中文布局日志刷屏。
fn update_tutorial_text(
    mut text_query: Query<&mut Text, With<TutorialText>>,
    completed: Query<&CompletedBuilding>,
    goal: Res<GoalState>,
) {
    let has_woodcutter = completed.iter().any(|b| b.kind == BuildingKind::Woodcutter);
    let has_gatherer = completed.iter().any(|b| b.kind == BuildingKind::Gatherer);
    let has_house = completed.iter().any(|b| b.kind == BuildingKind::House);
    let hint = if goal.reached {
        "目标达成！聚落已繁荣，继续自由建设吧。"
    } else if !has_woodcutter {
        "第一步：按 3 建造伐木屋，让定居者砍树获取木材。"
    } else if !has_gatherer {
        "第二步：按 4 建造采集屋，稳定食物供应，防止定居者挨饿。"
    } else if !has_house {
        "第三步：按 1 建造房屋，为人口增长提供容量（每栋 5 人）。"
    } else {
        "第四步：人口会随时间和食物自动增长；也可按 ~ 打开调试台添加殖民者，目标人口 10！"
    };
    if let Ok(mut text) = text_query.single_mut() {
        if text.0 != hint {
            text.0 = hint.to_string();
        }
    }
}

pub fn handle_ui_buttons(
    mut build_state: ResMut<BuildState>,
    mut clock: ResMut<SimulationClock>,
    selection: Res<SelectionState>,
    mut workplaces: Query<&mut Workplace>,
    mut build_buttons: Query<
        (&Interaction, &BuildButton, &mut BackgroundColor),
        (
            Changed<Interaction>,
            With<Button>,
            Without<SnapButton>,
            Without<TimeButton>,
            Without<JobSlotButton>,
        ),
    >,
    mut snap_buttons: Query<
        (&Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            With<Button>,
            With<SnapButton>,
            Without<BuildButton>,
            Without<TimeButton>,
            Without<JobSlotButton>,
        ),
    >,
    mut time_buttons: Query<
        (&Interaction, &TimeButton, &mut BackgroundColor),
        (
            Changed<Interaction>,
            With<Button>,
            Without<BuildButton>,
            Without<SnapButton>,
            Without<JobSlotButton>,
        ),
    >,
    mut job_slot_buttons: Query<
        (&Interaction, &JobSlotButton, &mut BackgroundColor),
        (
            Changed<Interaction>,
            With<Button>,
            Without<BuildButton>,
            Without<SnapButton>,
            Without<TimeButton>,
        ),
    >,
) {
    for (interaction, button, mut color) in &mut build_buttons {
        match *interaction {
            Interaction::Pressed => {
                build_state.select_construction(button.0);
                *color = BackgroundColor(BUTTON_ACTIVE);
            }
            Interaction::Hovered => *color = BackgroundColor(BUTTON_HOVER),
            Interaction::None => *color = BackgroundColor(BUTTON),
        }
    }

    for (interaction, mut color) in &mut snap_buttons {
        match *interaction {
            Interaction::Pressed => {
                build_state.snap_to_grid = !build_state.snap_to_grid;
                *color = BackgroundColor(BUTTON_ACTIVE);
            }
            Interaction::Hovered => *color = BackgroundColor(BUTTON_HOVER),
            Interaction::None => *color = BackgroundColor(BUTTON),
        }
    }

    for (interaction, button, mut color) in &mut time_buttons {
        match *interaction {
            Interaction::Pressed => {
                match button {
                    TimeButton::Pause => clock.paused = !clock.paused,
                    TimeButton::Speed(speed) => {
                        clock.paused = false;
                        clock.speed = *speed;
                    }
                }
                *color = BackgroundColor(BUTTON_ACTIVE);
            }
            Interaction::Hovered => *color = BackgroundColor(BUTTON_HOVER),
            Interaction::None => *color = BackgroundColor(BUTTON),
        }
    }

    for (interaction, button, mut color) in &mut job_slot_buttons {
        match *interaction {
            Interaction::Pressed => {
                if let Some(SelectedTarget::Building(entity)) = selection.selected
                    && let Ok(mut workplace) = workplaces.get_mut(entity)
                {
                    workplace.adjust_desired_slots(button.0);
                }
                *color = BackgroundColor(BUTTON_ACTIVE);
            }
            Interaction::Hovered => *color = BackgroundColor(BUTTON_HOVER),
            Interaction::None => *color = BackgroundColor(BUTTON),
        }
    }
}

fn update_ui_text(
    time: Res<Time>,
    mut fps_timer: ResMut<UiRefreshTimer>,
    clock: Res<SimulationClock>,
    goal: Res<GoalState>,
    diagnostics: Res<bevy::diagnostic::DiagnosticsStore>,
    terrain_config: Res<TerrainGenerationConfig>,
    build_state: Res<BuildState>,
    geometry: Res<WorldGeometry>,
    selection: Res<SelectionState>,
    colonists: Query<(Entity, &Colonist)>,
    completed: Query<(
        Entity,
        &CompletedBuilding,
        Option<&Inventory>,
        Option<&Housing>,
        Option<&CentralStorage>,
        Option<&Workplace>,
    )>,
    farms: Query<(Entity, &CompletedFarmPlot, &Footprint)>,
    blueprints: Query<(Entity, &Blueprint, Option<&FarmPlot>, Option<&Footprint>)>,
    resource_nodes: Query<(Entity, &ResourceNode)>,
    public_inventories: Query<&Inventory, With<PublicInventory>>,
    mut text_queries: ParamSet<(
        Query<&mut Text, With<ResourceText>>,
        Query<&mut Text, With<StatusText>>,
        Query<&mut Text, With<SelectionTitle>>,
        Query<&mut Text, With<SelectionBody>>,
        Query<&mut Text, With<TerrainDebugText>>,
        Query<&mut Text, With<FpsText>>,
        Query<&mut Node, With<JobControlsRoot>>,
        Query<&mut Text, With<JobSlotsText>>,
    )>,
) {
    // 动态文本都只在“内容真的变了”时才写回 Text，避免每帧触发文本重新布局
    // （布局中文会触发 parley/ICU4X 的缺数据警告日志）。
    let stock = public_stock(public_inventories.iter());
    let population = colonists.iter().count() as i32;
    let capacity: i32 = completed
        .iter()
        .map(|(_, building, _, _, _, _)| building.kind.definition().population_capacity)
        .sum();
    let homeless = colonists
        .iter()
        .filter(|(_, colonist)| colonist.home.is_none())
        .count();
    let idle_count = colonists
        .iter()
        .filter(|(_, colonist)| matches!(colonist.state, crate::colonist::ColonistState::Idle))
        .count();
    let (obstacles, road_obstacles, _) = geometry.summary();
    let (wood_nodes, food_nodes) = resource_node_counts(&resource_nodes);

    if let Ok(mut text) = text_queries.p0().single_mut() {
        // 目标显示：未达成时显示 目标：当前/目标，达成后固定为「目标达成！」。
        let goal_label = if goal.reached {
            "目标达成！".to_string()
        } else {
            format!("目标：{}/{}", population, goal.target)
        };
        let new = format!(
            "{}：{}  {}：{}  {}：{}  人口：{}/{}  无家：{}  空闲：{}  {}  时间：{}",
            ResourceKind::Wood.label(),
            stock.wood,
            ResourceKind::Food.label(),
            stock.food,
            ResourceKind::Firewood.label(),
            stock.firewood,
            population,
            capacity,
            homeless,
            idle_count,
            goal_label,
            clock.label()
        );
        if text.0 != new {
            text.0 = new;
        }
    }

    if let Ok(mut text) = text_queries.p1().single_mut() {
        let new = format!(
            "{}  障碍：{}  道路：{}  吸附：{}",
            build_state.status,
            obstacles,
            road_obstacles,
            if build_state.snap_to_grid {
                "开"
            } else {
                "关"
            }
        );
        if text.0 != new {
            text.0 = new;
        }
    }

    let (title, body) = selected_panel_text(
        &selection,
        &colonists,
        &completed,
        &farms,
        &blueprints,
        &resource_nodes,
    );
    if let Ok(mut text) = text_queries.p2().single_mut() {
        if text.0 != title {
            text.0 = title;
        }
    }
    if let Ok(mut text) = text_queries.p3().single_mut() {
        if text.0 != body {
            text.0 = body;
        }
    }
    if let Ok(mut text) = text_queries.p4().single_mut() {
        let new = format!(
            "地图：{}x{}  种子：0x{:016X}  资源点：木材 {} / 食物 {}",
            MAP_GRID_CELLS, MAP_GRID_CELLS, terrain_config.seed, wood_nodes, food_nodes
        );
        if text.0 != new {
            text.0 = new;
        }
    }
    fps_timer.0.tick(time.delta());
    if let Ok(mut text) = text_queries.p5().single_mut() {
        // FPS 限频刷新（0.5 秒一次），避免每秒几十次重新布局。
        if fps_timer.0.just_finished() {
            let fps = diagnostics
                .get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS)
                .and_then(|d| d.smoothed())
                .map(|f| f as i32)
                .unwrap_or(0);
            let new = format!("FPS: {}", fps);
            if text.0 != new {
                text.0 = new;
            }
        }
    }
    let job_status = selected_job_status(&selection, &colonists, &completed);
    if let Ok(mut node) = text_queries.p6().single_mut() {
        let display = if job_status.is_some() {
            Display::Flex
        } else {
            Display::None
        };
        if node.display != display {
            node.display = display;
        }
    }
    if let Ok(mut text) = text_queries.p7().single_mut() {
        let new = if let Some((assigned, desired)) = job_status {
            format!("工人：{}/{}", assigned, desired)
        } else {
            "工人：0/0".to_string()
        };
        if text.0 != new {
            text.0 = new;
        }
    }
}

fn resource_node_counts(resource_nodes: &Query<(Entity, &ResourceNode)>) -> (usize, usize) {
    let mut wood = 0;
    let mut food = 0;
    for (_, node) in resource_nodes {
        match node.kind {
            ResourceKind::Wood => wood += 1,
            ResourceKind::Food => food += 1,
            ResourceKind::Firewood => {}
        }
    }

    (wood, food)
}

fn selected_job_status(
    selection: &SelectionState,
    colonists: &Query<(Entity, &Colonist)>,
    completed: &Query<(
        Entity,
        &CompletedBuilding,
        Option<&Inventory>,
        Option<&Housing>,
        Option<&CentralStorage>,
        Option<&Workplace>,
    )>,
) -> Option<(usize, u8)> {
    let Some(SelectedTarget::Building(entity)) = selection.selected else {
        return None;
    };
    let (_, _, _, _, _, workplace) = completed.get(entity).ok()?;
    let workplace = workplace?;
    Some((
        assigned_worker_count(colonists, entity),
        workplace.desired_slots,
    ))
}

fn assigned_worker_count(colonists: &Query<(Entity, &Colonist)>, workplace: Entity) -> usize {
    colonists
        .iter()
        .filter(|(_, colonist)| colonist.workplace == Some(workplace))
        .count()
}

fn build_button(
    font: &impl Fn(f32) -> (TextFont, TextLayout),
    kind: ConstructionKind,
) -> impl Bundle {
    let label = format!("{} {}", kind.hotkey_label(), kind.label());
    (
        Button,
        BuildButton(kind),
        button_node(),
        BackgroundColor(BUTTON),
        children![(Text::new(label), font(12.0), TextColor(Color::WHITE))],
    )
}

pub(crate) fn utility_button<T: Component>(
    font: &impl Fn(f32) -> (TextFont, TextLayout),
    label: &'static str,
    marker: T,
) -> impl Bundle {
    (
        Button,
        marker,
        button_node(),
        BackgroundColor(BUTTON),
        children![(Text::new(label), font(12.0), TextColor(Color::WHITE))],
    )
}

fn time_button(
    font: &impl Fn(f32) -> (TextFont, TextLayout),
    label: &'static str,
    marker: TimeButton,
) -> impl Bundle {
    (
        Button,
        marker,
        button_node(),
        BackgroundColor(BUTTON),
        children![(Text::new(label), font(12.0), TextColor(Color::WHITE))],
    )
}

fn job_slot_button(
    font: &impl Fn(f32) -> (TextFont, TextLayout),
    label: &'static str,
    delta: i8,
) -> impl Bundle {
    (
        Button,
        JobSlotButton(delta),
        Node {
            min_width: px(34),
            height: px(30),
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            padding: UiRect::axes(px(8), px(4)),
            ..default()
        },
        BackgroundColor(BUTTON),
        children![(Text::new(label), font(12.0), TextColor(Color::WHITE))],
    )
}

pub(crate) fn button_node() -> Node {
    Node {
        min_width: px(72),
        height: px(34),
        display: Display::Flex,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        padding: UiRect::axes(px(10), px(4)),
        ..default()
    }
}

fn selected_panel_text(
    selection: &SelectionState,
    colonists: &Query<(Entity, &Colonist)>,
    completed: &Query<(
        Entity,
        &CompletedBuilding,
        Option<&Inventory>,
        Option<&Housing>,
        Option<&CentralStorage>,
        Option<&Workplace>,
    )>,
    farms: &Query<(Entity, &CompletedFarmPlot, &Footprint)>,
    blueprints: &Query<(Entity, &Blueprint, Option<&FarmPlot>, Option<&Footprint>)>,
    resource_nodes: &Query<(Entity, &ResourceNode)>,
) -> (String, String) {
    let Some(selected) = selection.selected else {
        return (
            "未选择".to_string(),
            "点击定居者、建筑、农场、蓝图或资源节点。".to_string(),
        );
    };

    match selected {
        SelectedTarget::Colonist(entity) => colonists
            .get(entity)
            .map(|(_, colonist)| {
                let home = colonist
                    .home
                    .map(|entity| format!("{entity:?}"))
                    .unwrap_or_else(|| "无".to_string());
                let workplace = colonist
                    .workplace
                    .map(|entity| format!("{entity:?}"))
                    .unwrap_or_else(|| "无".to_string());
                (
                    colonist.name.clone(),
                    format!(
                        "状态：{}\n职业：{}\n工作场所：{}\n饱食度：{:.0}/100\n家：{}\n速度：{:.1}",
                        colonist.status_label(),
                        colonist.profession.label(),
                        workplace,
                        colonist.satiety,
                        home,
                        colonist.speed
                    ),
                )
            })
            .unwrap_or_else(|_| missing_selection()),
        SelectedTarget::Resource(entity) => resource_nodes
            .get(entity)
            .map(|(_, node)| {
                let required_building = match node.kind {
                    ResourceKind::Wood => Some(BuildingKind::Woodcutter),
                    ResourceKind::Food => Some(BuildingKind::Gatherer),
                    ResourceKind::Firewood => None,
                };
                let enabled = required_building
                    .map(|required_building| {
                        completed
                            .iter()
                            .any(|(_, building, _, _, _, _)| building.kind == required_building)
                    })
                    .unwrap_or(false);
                let used_by = required_building
                    .map(|required_building| required_building.definition().label)
                    .unwrap_or("劈柴场");
                (
                    format!("{}节点", node.kind.label()),
                    format!(
                        "剩余：{}\n用途：{}\n可采集：{}",
                        node.amount,
                        used_by,
                        if enabled { "是" } else { "需要建筑" }
                    ),
                )
            })
            .unwrap_or_else(|_| missing_selection()),
        SelectedTarget::Blueprint(entity) => blueprints
            .get(entity)
            .map(|(_, blueprint, farm_plot, footprint)| {
                let label = blueprint.kind.label();
                let area_cells = farm_plot.map(|plot| plot.area_cells).or_else(|| {
                    footprint.map(|footprint| {
                        crate::building::polygon_area(&footprint.polygon)
                            / crate::types::CELL_SIZE.powi(2)
                    })
                });
                let material_line = if blueprint.kind == ConstructionKind::Farm {
                    "无需材料。"
                } else if blueprint.needs_wood() > 0 {
                    "仓库有木材后，定居者会送来。"
                } else {
                    "等待建造者施工。"
                };
                let mut body = format!(
                    "状态：{}\n木材：{}/{}\n施工进度：{:.0}%\n{}",
                    blueprint.status().label(),
                    blueprint.delivered_wood,
                    blueprint.required_wood,
                    blueprint.progress_ratio() * 100.0,
                    material_line
                );
                if let Some(area_cells) = area_cells {
                    body.push_str(&format!("\n面积：{:.1} 格", area_cells));
                }
                (format!("{}蓝图", label), body)
            })
            .unwrap_or_else(|_| missing_selection()),
        SelectedTarget::Building(entity) => completed
            .get(entity)
            .map(|(_, building, inventory, housing, central, workplace)| {
                let definition = building.kind.definition();
                let title = if central.is_some() {
                    "中央仓库".to_string()
                } else {
                    definition.label.to_string()
                };
                let mut body = format!("{}\n状态：运营中", building.kind.description());

                if let Some(housing) = housing {
                    body.push_str(&format!(
                        "\n居民：{}/{}",
                        housing.resident_count(),
                        Housing::CAPACITY
                    ));
                } else if definition.population_capacity > 0 {
                    body.push_str(&format!("\n容量：{}", definition.population_capacity));
                }

                if let Some(inventory) = inventory {
                    body.push_str(&format!(
                        "\n库存：木材 {}  食物 {}  柴火 {}\n容量：{}/{}",
                        inventory.wood,
                        inventory.food,
                        inventory.firewood,
                        inventory.used_capacity(),
                        inventory.capacity
                    ));
                }

                if let Some(workplace) = workplace {
                    let assigned = assigned_worker_count(colonists, entity);
                    body.push_str(&format!(
                        "\n工人：{}/{}  职业：{}",
                        assigned,
                        workplace.desired_slots,
                        workplace.profession.label()
                    ));
                }

                (title, body)
            })
            .unwrap_or_else(|_| missing_selection()),
        SelectedTarget::Farm(entity) => farms
            .get(entity)
            .map(|(_, farm, _)| {
                (
                    "农场地块".to_string(),
                    format!(
                        "{}\n状态：已建成\n面积：{:.1} 格",
                        ConstructionKind::Farm.description(),
                        farm.area_cells
                    ),
                )
            })
            .unwrap_or_else(|_| missing_selection()),
    }
}

fn missing_selection() -> (String, String) {
    ("选择已丢失".to_string(), "所选对象已不存在。".to_string())
}
