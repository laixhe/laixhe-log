use crate::pages::router::GameState;
use bevy::{prelude::*, text::FontSourceTemplate};

// 中文字体路径（与其它示例共用同一资产）
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

#[derive(Component, Clone, Default)]
pub struct MenuRoot;
#[derive(Component, Clone, Default)]
pub struct StartGameButton;

// ==================== 菜单界面系统 ====================
/// 创建主菜单界面（标题 + 开始游戏按钮 + 玩法提示）
pub fn setup_menu(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        MenuRoot
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(24.0),
        }
        BackgroundColor(Color::srgb(0.08, 0.09, 0.14))
        Children [
            // 标题
            (
                Text("3D 收集金币")
                TextFont {
                    font: FontSourceTemplate::Handle(FONT_PATH),
                    font_size: FontSize::Px(60.0),
                }
                TextColor(Color::srgb(1.0, 0.85, 0.3))
            ),
            // 玩法说明
            (
                Text("控制角色收集场地上的金币，在倒计时结束前集满即可获胜")
                TextFont {
                    font: FontSourceTemplate::Handle(FONT_PATH),
                    font_size: FontSize::Px(22.0),
                }
                TextColor(Color::srgb(0.6, 0.7, 0.85))
            ),
            // 开始按钮
            (
                StartGameButton
                Button
                Node {
                    padding: UiRect::all(Val::Px(15.0)),
                    border: UiRect::all(Val::Px(2.0)),
                }
                BorderColor::all(Color::srgb(1.0, 0.85, 0.3))
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15))
                Children [
                    (
                        Text("开始游戏")
                        TextFont {
                            font: FontSourceTemplate::Handle(FONT_PATH),
                            font_size: FontSize::Px(36.0),
                        }
                        TextColor(Color::WHITE)
                    )
                ]
            ),
            // 操作提示
            (
                Text("WASD 移动 | 空格跳跃 | Q/E 转身 | ESC 返回菜单")
                TextFont {
                    font: FontSourceTemplate::Handle(FONT_PATH),
                    font_size: FontSize::Px(18.0),
                }
                TextColor(Color::srgb(0.5, 0.55, 0.65))
            ),
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
                *border = BorderColor::all(Color::srgb(1.0, 0.9, 0.4));
            }
            Interaction::Hovered => {
                *background = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));
                *border = BorderColor::all(Color::srgb(1.0, 0.85, 0.35));
            }
            Interaction::None => {
                *background = BackgroundColor(Color::srgb(0.15, 0.15, 0.15));
                *border = BorderColor::all(Color::srgb(0.7, 0.6, 0.25));
            }
        }
    }
}
