//! 菜单模块：主菜单（开始/设置）+ 设置页（调整砖块行列数）。
//!
//! 学习重点：
//! - 嵌套 MenuState(Main/Settings) 子状态机，在 GlobalGameState::Menu 下再分页
//! - Button + Interaction(Pressed/Hovered/None) 三态交互与配色
//! - Single<Entity, With<Menu>> 取父节点实体，用 ChildOf 挂载子菜单
//! - SettingLabel + Res<GameSettings>::is_changed() 在设置变更时刷新数字标签
//! - DespawnOnExit 在切页/退菜单时自动清理对应 UI

use super::{GameSettings, GlobalGameState};
// FontSourceTemplate：bsn! 宏中 TextFont 的 font 字段类型
//   FontSourceTemplate::Handle("路径") 让 bsn 内部自动加载字体资产，无需手动 AssetServer
use bevy::{prelude::*, text::FontSourceTemplate};

// 中文字体路径：所有 UI 文本共享，bsn! 宏内部会缓存加载的资产
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// 菜单子状态机：仅在 GlobalGameState::Menu 活跃时才有意义
// 嵌套状态机让菜单分页互不干扰，切页时各自 OnEnter/DespawnOnExit
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MenuState {
    #[default]
    Menu,     // 占位初始态：App 启动后立刻切到 Main，本身不渲染任何 UI（勿与 GlobalGameState::Menu 混淆）
    Main,     // 主菜单页（Play/Settings 按钮）
    Settings, // 设置页（调整行列数）
}

// Marker 组件：无字段的「标签」，用来在 Query 中筛选「带 Menu 标签的实体」
#[derive(Component, Clone, Copy, Default)]
struct Menu;

// 用一个枚举标记每个按钮的「行为」，button_system 据此分发逻辑
// 比给每个按钮写一个独立组件更紧凑
#[derive(Clone, Copy, Debug, PartialEq, Default)]
enum ButtonAction {
    RowsInc,
    RowsDec,
    ColsInc,
    ColsDec,
    Back,
    #[default]
    Play,
    Settings,
}

// Newtype 组件包装 ButtonAction；pub 字段让外部可直接读 .0
// 挂在按钮实体上，点击时 button_system 读它决定做什么
#[derive(Component, Clone, Copy, Debug, Default)]
struct SettingButton(pub ButtonAction);

// 标记一个 Text 是「行数标签」还是「列数标签」，便于 update_settings_labels 定向刷新
#[derive(Clone, Copy, Debug, PartialEq, Default)]
enum LabelType {
    #[default]
    Rows,
    Cols,
}

#[derive(Component, Clone, Copy, Debug, Default)]
struct SettingLabel(pub LabelType);

pub fn menu_plugin(app: &mut App) {
    app // 进入顶层 Menu 态时建菜单根节点
        .add_systems(OnEnter(GlobalGameState::Menu), menu_setup)
        // 进入子状态时各自建对应页面的 UI
        .add_systems(OnEnter(MenuState::Main), main_menu_setup)
        .add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
        // 两个 Update 系统只在 Menu 态运行：处理按钮点击、刷新数字标签
        .add_systems(
            Update,
            (
                button_system.run_if(in_state(GlobalGameState::Menu)),
                update_settings_labels.run_if(in_state(GlobalGameState::Menu)),
            ),
        )
        // 注册子状态机（init_state 必须在用 in_state(MenuState) 之前调用）
        .init_state::<MenuState>();
}

fn menu_setup(mut commands: Commands, mut menu_state: ResMut<NextState<MenuState>>) {
    commands.spawn_scene(bsn! {
        Menu // 打上 Menu 标签，后续 Single<Entity, With<Menu>> 能查到它作为父节点
        DespawnOnExit::<GlobalGameState>(GlobalGameState::Menu)
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15))
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column, // 子元素纵向排列
        }
        Children [
            // 元组语法：一个实体挂载多个组件（Text 内容 + TextFont 字体字号 + TextColor 颜色）
            (
                Text::new("Bevy 打砖块")
                TextFont {
                    font: FontSourceTemplate::Handle(FONT_PATH),
                    font_size: FontSize::Px(80.0),
                }
                TextColor(Color::WHITE)
            )
        ]
    });

    // 根节点建好后立刻切到 Main 子页，触发 main_menu_setup
    menu_state.set(MenuState::Main);
}

// 按钮工厂：返回 impl Scene（一段可被 spawn_scene 嵌入的节点描述）
// 把按钮的「外观 + 行为」封装起来，调用处只需传文字和 action
fn menu_button(text: &'static str, width: f32, height: f32, action: ButtonAction) -> impl Scene {
    bsn! {
        SettingButton(action) // 记录行为，点击时 button_system 读它
        Button                // Button 组件自带 Interaction，引擎自动追踪鼠标悬停/按下
        BackgroundColor(Color::srgb(0.3, 0.3, 0.3))
        Node {
            width: Val::Px(width),  // Val::Px 是像素绝对值（Percent 是百分比）
            height: Val::Px(height),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
        }
        Children [
            (
                Text::new(text)
                TextFont {
                    font: FontSourceTemplate::Handle(FONT_PATH),
                    font_size: FontSize::Px(24.0),
                }
                TextColor(Color::WHITE)
            )
        ]
    }
}

fn main_menu_setup(mut commands: Commands, menu: Single<Entity, With<Menu>>) {
    // Single 要求查询结果唯一：多于一个或为零都会 panic；menu.entity() 解引用拿到内部的 Entity id
    let parent = menu.entity();
    commands.spawn_scene(bsn! {
        ChildOf(parent)
        DespawnOnExit::<MenuState>(MenuState::Main)
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(20.0),
        }
        Children [
            (
                Text::new("主菜单")
                TextFont {
                    font: FontSourceTemplate::Handle(FONT_PATH),
                    font_size: FontSize::Px(60.0),
                }
                TextColor(Color::WHITE)
            ),
            menu_button("开始游戏", 150.0, 30.0, ButtonAction::Play),
            menu_button("游戏设置", 150.0, 30.0, ButtonAction::Settings)
        ]
    });
}

fn setting_row(
    label: &'static str,
    value: usize,
    inc_btn: ButtonAction,
    dec_btn: ButtonAction,
    label_component: LabelType,
) -> impl Scene {
    bsn! {
        Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(16.0),
        }
        Children [
            (
                Text::new(label)
                TextFont {
                    font: FontSourceTemplate::Handle(FONT_PATH),
                    font_size: FontSize::Px(26.0),
                }
                TextColor(Color::srgb(0.8, 0.8, 0.8))
                Node {
                    width: Val::Px(100.0),
                }
            ),
            menu_button("−", 36.0, 36.0, dec_btn),
            (
                SettingLabel(label_component)
                Text::new(value.to_string())
                TextFont {
                    font: FontSourceTemplate::Handle(FONT_PATH),
                    font_size: FontSize::Px(26.0),
                }
                TextColor(Color::WHITE)
                Node {
                    width: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                }
            ),
            menu_button("+", 36.0, 36.0, inc_btn)
        ]
    }
}

fn settings_menu_setup(
    mut commands: Commands,
    menu: Single<Entity, With<Menu>>,
    settings: Res<GameSettings>,
) {
    let parent = menu.entity();
    commands.spawn_scene(bsn! {
        ChildOf(parent)
        DespawnOnExit::<MenuState>(MenuState::Settings)
        Node {
            flex_direction: FlexDirection::Column,
            align_content: AlignContent::Center,
            row_gap: Val::Px(20.0),
            margin: UiRect::top(Val::Px(23.0)),
        }
        Children [
            (
                Text::new("设置")
                TextFont {
                    font: FontSourceTemplate::Handle(FONT_PATH),
                    font_size: FontSize::Px(36.0),
                }
                TextColor(Color::WHITE)
            ),
            setting_row(
                "行数",
                settings.brick_rows,
                ButtonAction::RowsInc,
                ButtonAction::RowsDec,
                LabelType::Rows,
            ),
            setting_row(
                "列数",
                settings.brick_columns,
                ButtonAction::ColsInc,
                ButtonAction::ColsDec,
                LabelType::Cols,
            ),
            menu_button("返回", 100.0, 30.0, ButtonAction::Back)
        ]
    });
}

fn button_system(
    // Changed<Interaction>：只在交互态变化的那一帧才把实体纳入查询，避免每帧扫描所有按钮（性能优化）
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &SettingButton),
        Changed<Interaction>,
    >,
    mut settings: ResMut<GameSettings>,
    // 通过两个 NextState 句柄分别切换顶层（Game）和子层（Main/Settings）状态
    mut game_state: ResMut<NextState<GlobalGameState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    // Interaction 由引擎根据鼠标状态自动更新：Pressed 按下、Hovered 悬停、None 默认
    for (interaction, mut color, button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.4, 0.4, 0.4)); // 按下变亮
                match button.0 {
                    ButtonAction::Play => {
                        info!("[状态] Menu → Game（点击 Play）");
                        game_state.set(GlobalGameState::Game);
                    }
                    ButtonAction::Back => {
                        info!("[状态] Settings → Main（点击 Back）");
                        menu_state.set(MenuState::Main);
                    }
                    ButtonAction::Settings => {
                        info!("[状态] Main → Settings（点击 Settings）");
                        menu_state.set(MenuState::Settings);
                    }
                    ButtonAction::RowsInc => {
                        settings.brick_rows = (settings.brick_rows + 1).min(10);
                        info!("[设置] 砖块行数 +1 → {}", settings.brick_rows);
                    }
                    ButtonAction::RowsDec => {
                        settings.brick_rows = (settings.brick_rows - 1).max(1);
                        info!("[设置] 砖块行数 -1 → {}", settings.brick_rows);
                    }
                    ButtonAction::ColsInc => {
                        settings.brick_columns = (settings.brick_columns + 1).min(20);
                        info!("[设置] 砖块列数 +1 → {}", settings.brick_columns);
                    }
                    ButtonAction::ColsDec => {
                        settings.brick_columns = (settings.brick_columns - 1).max(1);
                        info!("[设置] 砖块列数 -1 → {}", settings.brick_columns);
                    }
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.5, 0.5, 0.5));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.3, 0.3, 0.3));
            }
        }
    }
}

// 设置页数字标签刷新：仅在 GameSettings 变化时才重写 Text 内容（避免每帧无谓写入）
fn update_settings_labels(
    settings: Res<GameSettings>,
    mut label_query: Query<(&SettingLabel, &mut Text)>,
) {
    // is_changed()：引擎自动追踪资源是否被修改，未变则提前返回
    if !settings.is_changed() {
        return;
    }
    for (label, mut text) in &mut label_query {
        match label.0 {
            // text.0 访问 Text 内部的 String 字段（Text 是 newtype Text(String)）
            LabelType::Rows => text.0 = settings.brick_rows.to_string(),
            LabelType::Cols => text.0 = settings.brick_columns.to_string(),
        }
    }
}
