use crate::pages::game::{Score, WinFlag};
use crate::pages::router::GameState;
use bevy::{prelude::*, text::FontSourceTemplate};

// 中文字体路径（与其它示例共用同一资产）
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

#[derive(Component, Clone, Default)]
pub struct GameOverRoot;
#[derive(Component, Clone, Default)]
pub struct RestartButton;
#[derive(Component, Clone, Default)]
pub struct MenuButton;

// ==================== 游戏结束界面系统 ====================
/// 创建游戏结束界面（胜利 / 超时文案 + 最终得分 + 重新开始 / 返回主菜单）
pub fn setup_game_over(mut commands: Commands, score: Res<Score>, win_flag: Res<WinFlag>) {
    let (title, title_color, bg) = if win_flag.0 {
        (
            "胜利！",
            Color::srgb(0.95, 0.85, 0.3),
            Color::srgb(0.05, 0.12, 0.05),
        )
    } else {
        (
            "时间到，未集满金币",
            Color::srgb(0.95, 0.4, 0.35),
            Color::srgb(0.12, 0.05, 0.05),
        )
    };

    commands.spawn_scene(bsn! {
        GameOverRoot
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(20.0),
        }
        BackgroundColor(bg)
        Children [
            (
                Text(title)
                TextFont {
                    font: FontSourceTemplate::Handle(FONT_PATH),
                    font_size: FontSize::Px(52.0),
                }
                TextColor(title_color)
            ),
            (
                Text(format!("收集金币：{} / {}", score.0, 10))
                TextFont {
                    font: FontSourceTemplate::Handle(FONT_PATH),
                    font_size: FontSize::Px(30.0),
                }
                TextColor(Color::WHITE)
            ),
            // 重新开始
            (
                RestartButton
                Button
                Node {
                    padding: UiRect::all(Val::Px(14.0)),
                    border: UiRect::all(Val::Px(2.0)),
                }
                BorderColor::all(Color::srgb(0.4, 0.85, 0.4))
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15))
                Children [
                    (
                        Text("再来一局")
                        TextFont {
                            font: FontSourceTemplate::Handle(FONT_PATH),
                            font_size: FontSize::Px(30.0),
                        }
                        TextColor(Color::WHITE)
                    )
                ]
            ),
            // 返回主菜单
            (
                MenuButton
                Button
                Node {
                    padding: UiRect::all(Val::Px(14.0)),
                    border: UiRect::all(Val::Px(2.0)),
                }
                BorderColor::all(Color::srgb(0.4, 0.6, 0.9))
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15))
                Children [
                    (
                        Text("返回主菜单")
                        TextFont {
                            font: FontSourceTemplate::Handle(FONT_PATH),
                            font_size: FontSize::Px(30.0),
                        }
                        TextColor(Color::WHITE)
                    )
                ]
            ),
        ]
    });
}

pub fn cleanup_game_over(mut commands: Commands, query: Query<Entity, With<GameOverRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn handle_buttons(
    mut next_state: ResMut<NextState<GameState>>,
    mut restart_query: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
    mut menu_query: Query<&Interaction, (Changed<Interaction>, With<MenuButton>)>,
) {
    for interaction in &mut restart_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Playing);
        }
    }
    for interaction in &mut menu_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Menu);
        }
    }
}

pub fn button_hover_effects(
    restart_markers: Query<(), With<RestartButton>>,
    // 合并成单个可变查询，避免两个查询同时写 BackgroundColor / BorderColor（Bevy B0001）
    mut query: Query<
        (Entity, &Interaction, &mut BackgroundColor, &mut BorderColor),
        Or<(With<RestartButton>, With<MenuButton>)>,
    >,
) {
    for (entity, interaction, mut background, mut border) in &mut query {
        // 按按钮类型取不同的配色（重新开始=绿色系，返回菜单=蓝色系）
        let (base, accent) = if restart_markers.contains(entity) {
            ((0.2, 0.45, 0.2), (0.4, 0.85, 0.4))
        } else {
            ((0.2, 0.3, 0.5), (0.4, 0.6, 0.9))
        };
        let (bg, bd) = hover_colors(*interaction, base, accent);
        *background = BackgroundColor(bg);
        *border = BorderColor::all(bd);
    }
}

// 根据交互状态返回 背景色 / 边框色（hovered 加深、pressed 更深）
fn hover_colors(
    interaction: Interaction,
    base: (f32, f32, f32),
    accent: (f32, f32, f32),
) -> (Color, Color) {
    match interaction {
        Interaction::Pressed => (
            Color::srgb(base.0 + 0.1, base.1 + 0.1, base.2 + 0.1),
            Color::WHITE,
        ),
        Interaction::Hovered => (
            Color::srgb(base.0 + 0.05, base.1 + 0.05, base.2 + 0.05),
            Color::srgb(accent.0 + 0.1, accent.1 + 0.1, accent.2 + 0.1),
        ),
        Interaction::None => (
            Color::srgb(0.15, 0.15, 0.15),
            Color::srgb(accent.0, accent.1, accent.2),
        ),
    }
}
