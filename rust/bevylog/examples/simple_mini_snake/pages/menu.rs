use crate::pages::router::GameState;
use bevy::{prelude::*, text::FontSourceTemplate};

// 中文字体路径（与其它示例共用同一资产）
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

#[derive(Component, Clone, Default)]
pub struct MenuRoot;
#[derive(Component, Clone, Default)]
pub struct StartGameButton;

// ==================== 菜单界面系统 ====================
/// 创建主菜单界面（标题 + 开始游戏按钮）
pub fn setup_menu(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        // 创建菜单根节点（全屏居中布局）
        #MenuRoot
        MenuRoot
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
        }
        BackgroundColor(Color::srgb(0.1, 0.1, 0.1))
        Children [
            // 标题
            (
                Text("贪吃蛇")
                TextFont {
                    font: FontSourceTemplate::Handle(FONT_PATH),
                    font_size: FontSize::Px(60.0),
                }
                TextColor(Color::srgb(0.2, 0.8, 0.2))
            ),
            // 按钮容器
            (
                Node {
                    margin: UiRect::all(Val::Px(30.0)),
                    padding: UiRect::all(Val::Px(20.0)),
                }
                Children [
                    (
                        StartGameButton
                        Button
                        Node {
                            padding: UiRect::all(Val::Px(15.0)),
                            border: UiRect::all(Val::Px(2.0)),
                        }
                        BorderColor::all(Color::srgb(0.2, 0.8, 0.2))
                        BackgroundColor(Color::srgb(0.15, 0.15, 0.15))
                        Children [
                            (
                                Text("开始游戏")
                                TextFont {
                                    font: FontSourceTemplate::Handle(FONT_PATH),
                                    font_size: FontSize::Px(40.0),
                                }
                                TextColor(Color::WHITE)
                            )
                        ]
                    )
                ]
            )
        ]
    });
}
pub fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn handle_menu_buttons(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<StartGameButton>)>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Playing);
        }
    }
}

pub fn button_hover_effects(
    mut query: Query<(&Interaction, &mut BackgroundColor, &mut BorderColor), With<StartGameButton>>,
) {
    for (interaction, mut background, mut border) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                *background = BackgroundColor(Color::srgb(0.25, 0.25, 0.25));
                *border = BorderColor::all(Color::srgb(0.3, 0.9, 0.3));
            }
            Interaction::Hovered => {
                *background = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));
                *border = BorderColor::all(Color::srgb(0.25, 0.85, 0.25));
            }
            Interaction::None => {
                *background = BackgroundColor(Color::srgb(0.15, 0.15, 0.15));
                *border = BorderColor::all(Color::srgb(0.2, 0.8, 0.2));
            }
        }
    }
}
