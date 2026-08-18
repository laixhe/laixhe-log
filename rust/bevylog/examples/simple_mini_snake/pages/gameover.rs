use bevy::{
    prelude::*,
    text::{FontSize, FontSourceTemplate},
};

use crate::pages::game::Score;
use crate::pages::router::GameState;

// 中文字体路径（与其它示例共用同一资产）
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

#[derive(Component, Clone, Default)]
pub struct GameOverRoot;

#[derive(Component, Clone, Default)]
pub struct RestartButton;

#[derive(Component, Clone, Default)]
pub struct MainMenuButton;

/// 创建 GameOver 界面（游戏结束 + 分数 + 按钮）
pub fn setup_game_over(mut commands: Commands, score: Res<Score>) {
    let score_value = score.0;
    let score_text = format!("最终分数: {}", score_value);

    commands.spawn_scene(bsn! {
        #GameOverRoot
        GameOverRoot
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
        }
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85))
        Children [
            // 标题
            (
                Text("游戏结束")
                TextFont {
                    font: FontSourceTemplate::Handle(FONT_PATH),
                    font_size: FontSize::Px(60.0),
                }
                TextColor(Color::srgb(0.9, 0.2, 0.2))
            ),
            // 最终分数
            (
                Text(score_text)
                TextFont {
                    font: FontSourceTemplate::Handle(FONT_PATH),
                    font_size: FontSize::Px(40.0),
                }
                TextColor(Color::WHITE)
                Node {
                    margin: UiRect::all(Val::Px(20.0)),
                }
            ),
            // 重新开始按钮
            (
                RestartButton
                Button
                Node {
                    margin: UiRect::all(Val::Px(10.0)),
                    padding: UiRect::all(Val::Px(15.0)),
                    border: UiRect::all(Val::Px(2.0)),
                }
                BorderColor::all(Color::srgb(0.2, 0.8, 0.2))
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15))
                Children [
                    (
                        Text("重新开始")
                        TextFont {
                            font: FontSourceTemplate::Handle(FONT_PATH),
                            font_size: FontSize::Px(35.0),
                        }
                        TextColor(Color::WHITE)
                    )
                ]
            ),
            // 返回主菜单按钮
            (
                MainMenuButton
                Button
                Node {
                    margin: UiRect::all(Val::Px(10.0)),
                    padding: UiRect::all(Val::Px(15.0)),
                    border: UiRect::all(Val::Px(2.0)),
                }
                BorderColor::all(Color::srgb(0.6, 0.6, 0.6))
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15))
                Children [
                    (
                        Text("返回菜单")
                        TextFont {
                            font: FontSourceTemplate::Handle(FONT_PATH),
                            font_size: FontSize::Px(35.0),
                        }
                        TextColor(Color::WHITE)
                    )
                ]
            )
        ]
    });
}

pub fn cleanup_game_over(mut commands: Commands, query: Query<Entity, With<GameOverRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

/// 处理重新开始和返回菜单按钮点击
pub fn handle_gameover_buttons(
    mut next_state: ResMut<NextState<GameState>>,
    restart_query: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
    menu_query: Query<&Interaction, (Changed<Interaction>, With<MainMenuButton>)>,
) {
    for interaction in &restart_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Playing);
            return;
        }
    }
    for interaction in &menu_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Menu);
            return;
        }
    }
}

/// 按钮悬停效果
pub fn button_hover_effects(
    mut query: Query<
        (&Interaction, &mut BackgroundColor),
        Or<(With<RestartButton>, With<MainMenuButton>)>,
    >,
) {
    for (interaction, mut background) in &mut query {
        *background = BackgroundColor(match *interaction {
            Interaction::Pressed => Color::srgb(0.25, 0.25, 0.25),
            Interaction::Hovered => Color::srgb(0.2, 0.2, 0.2),
            Interaction::None => Color::srgb(0.15, 0.15, 0.15),
        });
    }
}
