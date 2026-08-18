use bevy::prelude::*;
use bevy::text::FontSource;

use crate::game::save::{self, PendingLoad};
use crate::router::GameState;

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

#[derive(Component, Clone, Default)]
pub struct MenuRoot;

#[derive(Component, Clone, Default)]
pub struct StartButton;

#[derive(Component, Clone, Default)]
pub struct ContinueButton;

// ==================== 主菜单（命令式构建，支持条件按钮） ====================
pub fn setup_menu(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut pending: ResMut<PendingLoad>,
) {
    // 清掉残留的读档请求
    pending.0 = None;
    // 字体加载：FontSource::Handle 把 assets/fonts/Yozai-Regular.ttf 异步加载成句柄，
    // 之后每个 Text 都带上 TextFont（字号）+ TextColor（颜色）组合。这是本示例
    // 所有中文 UI 的标准字体写法，后续弹窗/菜单都复用同样的 Yozai 字体。
    let font = |size: f32| TextFont {
        font: FontSource::Handle(assets.load(FONT_PATH)),
        font_size: FontSize::Px(size),
        ..default()
    };
    let can_continue = save::has_save();

    commands
        .spawn((
            MenuRoot,
            Node {
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: px(24),
                ..default()
            },
            BackgroundColor(Color::srgb(0.93, 0.88, 0.76)),
        ))
        .with_children(|root| {
            root.spawn((
                MenuRoot,
                Text::new("程序员求职生存模拟"),
                TextColor(Color::srgb(0.72, 0.15, 0.12)),
                font(56.0),
            ));
            root.spawn((
                MenuRoot,
                Text::new("3D 45° 俯视 · 低模 · Rust + Bevy 0.19"),
                TextColor(Color::srgb(0.45, 0.35, 0.25)),
                font(20.0),
            ));
            root.spawn((
                MenuRoot,
                StartButton,
                Button,
                Node {
                    padding: UiRect::all(px(14)),
                    border: UiRect::all(px(3)),
                    ..default()
                },
                BorderColor::all(Color::srgb(0.85, 0.65, 0.2)),
                BackgroundColor(Color::srgb(0.72, 0.15, 0.12)),
            ))
            .with_children(|btn| {
                btn.spawn((
                    MenuRoot,
                    Text::new("开始新游戏"),
                    TextColor(Color::WHITE),
                    font(34.0),
                ));
            });
            if can_continue {
                root.spawn((
                    MenuRoot,
                    ContinueButton,
                    Button,
                    Node {
                        padding: UiRect::all(px(10)),
                        border: UiRect::all(px(2)),
                        ..default()
                    },
                    BorderColor::all(Color::srgb(0.72, 0.55, 0.25)),
                    BackgroundColor(Color::srgb(0.62, 0.50, 0.34)),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        MenuRoot,
                        Text::new("继续游戏（读档）"),
                        TextColor(Color::WHITE),
                        font(22.0),
                    ));
                });
            }
            root.spawn((
                MenuRoot,
                Text::new(
                    "点击地面移动 · 走近热点/NPC 自动触发 · 晚上睡觉推进天数\n工作日上班赚钱 · 电脑投简历 · 周一交房租 · 心态归零就崩",
                ),
                TextColor(Color::srgb(0.5, 0.42, 0.3)),
                font(18.0),
            ));
        });
}

pub fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuRoot>>) {
    // 菜单根与其子实体都带 MenuRoot：try_despawn 容忍随父级连带删除的重复删除
    for entity in &query {
        commands.entity(entity).try_despawn();
    }
}

pub fn handle_menu_buttons(
    mut next_state: ResMut<NextState<GameState>>,
    mut pending: ResMut<PendingLoad>,
    start: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>,
    cont: Query<&Interaction, (Changed<Interaction>, With<ContinueButton>)>,
) {
    for interaction in start {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Playing);
        }
    }
    for interaction in cont {
        if *interaction == Interaction::Pressed {
            if let Some(data) = save::load_game() {
                pending.0 = Some(data);
                next_state.set(GameState::Playing);
            }
        }
    }
}
