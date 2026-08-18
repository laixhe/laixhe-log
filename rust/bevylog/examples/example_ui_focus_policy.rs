//! Bevy 0.19 入门示例：演示交互穿透（FocusPolicy）。
//!
//! 学习重点：
//! - FocusPolicy::Block：节点会拦截交互，下层节点收不到鼠标事件
//! - FocusPolicy::Pass：节点不拦截，交互穿透到下层节点
//!
//! 操作：空格切换遮罩的 FocusPolicy；Block 时点击按钮无反应，Pass 时按钮可点击。

use bevy::prelude::*;
use bevy::text::FontSource;
use bevy::ui::FocusPolicy;

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// 遮罩标记
#[derive(Component)]
struct Mask;

// 记录遮罩是否拦截（true = Block，false = Pass）
#[derive(Resource)]
struct Blocking(bool);

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Blocking>()
        .add_systems(Startup, setup)
        .add_systems(Update, (toggle_mask, report_clicks))
        .run()
}

impl Default for Blocking {
    fn default() -> Self {
        Self(true)
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands
        .spawn(Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            // 底层按钮
            parent
                .spawn((
                    Button,
                    Node {
                        width: px(200),
                        height: px(80),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        border_radius: BorderRadius::all(px(10)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.3, 0.6, 0.9)),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new("点击我"),
                        TextColor(Color::WHITE),
                        TextFont {
                            font: FontSource::Handle(asset_server.load(FONT_PATH)),
                            font_size: FontSize::Px(22.0),
                            ..default()
                        },
                    ));
                });

            // 覆盖在按钮上的半透明遮罩（初始 Block，拦截点击）
            parent.spawn((
                Mask,
                FocusPolicy::Block,
                Node {
                    position_type: PositionType::Absolute,
                    width: px(200),
                    height: px(80),
                    border_radius: BorderRadius::all(px(10)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.4)),
            ));
        });
}

// 空格切换遮罩：Block <-> Pass
fn toggle_mask(
    keys: Res<ButtonInput<KeyCode>>,
    mut blocking: ResMut<Blocking>,
    mut q_mask: Query<&mut FocusPolicy, With<Mask>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        blocking.0 = !blocking.0;
        for mut policy in &mut q_mask {
            *policy = if blocking.0 {
                FocusPolicy::Block
            } else {
                FocusPolicy::Pass
            };
        }
        info!(
            "[UI焦点] 遮罩 = {}",
            if blocking.0 {
                "Block（拦截）"
            } else {
                "Pass（穿透）"
            }
        );
    }
}

// 检测按钮点击：只有遮罩为 Pass 时按钮才能被点中
fn report_clicks(
    blocking: Res<Blocking>,
    q_button: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
) {
    for interaction in &q_button {
        if *interaction == Interaction::Pressed {
            info!(
                "[UI焦点] 按钮被点击（当前遮罩 = {}）",
                if blocking.0 { "Block" } else { "Pass" }
            );
        }
    }
}
