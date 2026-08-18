use bevy::prelude::*;
use bevy::text::FontSource;

use crate::game::resources::{Ending, FONT_PATH, PlayerStats, SKILL_NAMES};
use crate::router::GameState;

#[derive(Component, Clone, Default)]
pub struct OverRoot;

#[derive(Component, Clone, Default)]
pub struct RestartButton;

// ==================== 结局页（演出版） ====================
// 背景色随结局类型变化；标题加金框奖章样式；附技能结算条。
pub fn setup_game_over(
    mut commands: Commands,
    assets: Res<AssetServer>,
    ending: Res<Ending>,
    stats: Res<PlayerStats>,
) {
    let font = |size: f32| TextFont {
        font: FontSource::Handle(assets.load(FONT_PATH)),
        font_size: FontSize::Px(size),
        ..default()
    };
    let title = ending.title.clone();
    // 按结局类型配色
    let (bg, accent, badge) = if title.contains("心态") || title.contains("崩溃") {
        (
            Color::srgb(0.80, 0.83, 0.87),
            Color::srgb(0.35, 0.42, 0.52),
            "心态崩了",
        )
    } else if title.contains("街头") || title.contains("失利") || title.contains("回炉") {
        (
            Color::srgb(0.83, 0.81, 0.77),
            Color::srgb(0.48, 0.46, 0.42),
            "出局",
        )
    } else if title.contains("大厂") || title.contains("冲浪") || title.contains("SSP") {
        (
            Color::srgb(0.98, 0.93, 0.82),
            Color::srgb(0.80, 0.55, 0.16),
            "高光",
        )
    } else if title.contains("创业") {
        (
            Color::srgb(0.90, 0.95, 0.87),
            Color::srgb(0.35, 0.62, 0.30),
            "搞事业",
        )
    } else {
        (
            Color::srgb(0.93, 0.90, 0.84),
            Color::srgb(0.55, 0.42, 0.30),
            "上岸",
        )
    };
    let summary = format!(
        "存款 ¥{:.0} · 精力 {:.0} · 心态 {:.0} · 健康 {:.0}",
        stats.money, stats.energy, stats.mentality, stats.health,
    );

    commands
        .spawn((
            OverRoot,
            Node {
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: px(14),
                ..default()
            },
            BackgroundColor(bg),
        ))
        .with_children(|root| {
            root.spawn((
                OverRoot,
                Text::new("—— 结局 ——"),
                TextColor(Color::srgb(0.5, 0.42, 0.3)),
                font(20.0),
            ));
            // 奖章式标题：金框 + 徽章
            root.spawn((
                OverRoot,
                Node {
                    padding: UiRect::axes(px(28), px(12)),
                    border: UiRect::all(px(3)),
                    border_radius: BorderRadius::all(px(10)),
                    ..default()
                },
                BorderColor::all(accent),
                BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.35)),
            ))
            .with_children(|medal| {
                medal.spawn((
                    OverRoot,
                    Text::new(format!("🏅 {} · {}", badge, title)),
                    TextColor(accent),
                    font(42.0),
                ));
            });
            root.spawn((
                OverRoot,
                Text::new(ending.desc.clone()),
                TextColor(Color::srgb(0.35, 0.28, 0.2)),
                font(20.0),
            ));
            root.spawn((
                OverRoot,
                Text::new(summary),
                TextColor(Color::srgb(0.45, 0.36, 0.26)),
                font(17.0),
            ));
            // 技能结算条
            root.spawn((
                OverRoot,
                Node {
                    width: percent(46),
                    flex_direction: FlexDirection::Column,
                    row_gap: px(5),
                    ..default()
                },
            ))
            .with_children(|bars| {
                for (i, name) in SKILL_NAMES.iter().enumerate() {
                    let v = stats.skills[i];
                    bars.spawn((
                        OverRoot,
                        Node {
                            width: percent(100),
                            flex_direction: FlexDirection::Row,
                            column_gap: px(8),
                            align_items: AlignItems::Center,
                            ..default()
                        },
                    ))
                    .with_children(|row| {
                        row.spawn((
                            OverRoot,
                            Text::new(format!("{name} {v:.0}")),
                            TextColor(Color::srgb(0.35, 0.28, 0.2)),
                            TextFont {
                                font: FontSource::Handle(assets.load(FONT_PATH)),
                                font_size: FontSize::Px(14.0),
                                ..default()
                            },
                        ));
                        // 底条
                        row.spawn((
                            OverRoot,
                            Node {
                                width: percent(62),
                                height: px(10),
                                border_radius: BorderRadius::all(px(5)),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.4, 0.36, 0.3, 0.25)),
                        ))
                        .with_children(|track| {
                            // 填充
                            track.spawn((
                                OverRoot,
                                Node {
                                    width: percent(v),
                                    height: percent(100),
                                    border_radius: BorderRadius::all(px(5)),
                                    ..default()
                                },
                                BackgroundColor(accent),
                            ));
                        });
                    });
                }
            });
            root.spawn((
                OverRoot,
                RestartButton,
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
                    OverRoot,
                    Text::new("再玩一次"),
                    TextColor(Color::WHITE),
                    font(30.0),
                ));
            });
        });
}

pub fn cleanup_game_over(mut commands: Commands, query: Query<Entity, With<OverRoot>>) {
    // 结算面板与其子实体都带 OverRoot：try_despawn 容忍随父级连带删除的重复删除
    for entity in &query {
        commands.entity(entity).try_despawn();
    }
}

pub fn handle_buttons(
    mut next_state: ResMut<NextState<GameState>>,
    mut query: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
) {
    for interaction in &mut query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Playing);
        }
    }
}
