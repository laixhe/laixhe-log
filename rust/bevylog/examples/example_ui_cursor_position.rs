//! Bevy 0.19 入门示例：演示相对光标位置（RelativeCursorPosition）。
//!
//! 学习重点：
//! - RelativeCursorPosition：放在 Node 上，自动记录鼠标相对节点的位置
//! - normalized：归一化坐标，中心为 (0, 0)，右上为 (0.5, -0.5)，左下为 (-0.5, 0.5)
//! - 配合 UiTransform 让子节点跟随鼠标移动
//!
//! 观察：鼠标在灰色面板内移动时，绿色圆点跟随鼠标位置移动。

use bevy::prelude::*;
use bevy::text::FontSource;
use bevy::ui::RelativeCursorPosition;

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// 感应面板标记
#[derive(Component)]
struct HoverPanel;

// 跟随圆点标记
#[derive(Component)]
struct Dot;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_dot)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands
        .spawn(Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: px(16),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new("鼠标在面板内移动，绿点跟随"),
                TextColor(Color::WHITE),
                TextFont {
                    font: FontSource::Handle(asset_server.load(FONT_PATH)),
                    font_size: FontSize::Px(20.0),
                    ..default()
                },
            ));

            // 感应面板：挂 RelativeCursorPosition 自动更新鼠标相对位置
            parent
                .spawn((
                    HoverPanel,
                    RelativeCursorPosition::default(),
                    Node {
                        width: px(320),
                        height: px(200),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        border_radius: BorderRadius::all(px(10)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.20)),
                ))
                .with_children(|panel| {
                    // 跟随圆点：初始在中心，用 UiTransform 平移
                    panel.spawn((
                        Dot,
                        Node {
                            width: px(20),
                            height: px(20),
                            border_radius: BorderRadius::all(px(10)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.3, 0.8, 0.4)),
                    ));
                });
        });
}

// 读取鼠标相对位置，移动圆点（面板尺寸 320 x 200）
fn move_dot(
    q_panel: Query<(&RelativeCursorPosition, &Children), With<HoverPanel>>,
    mut q_dot: Query<&mut UiTransform, With<Dot>>,
) {
    for (cursor, children) in &q_panel {
        let Some(dot) = children.first() else {
            continue;
        };
        let Ok(mut tf) = q_dot.get_mut(*dot) else {
            continue;
        };

        // normalized 中心为 (0,0)，x 向右为正、y 向上为正
        // UI 的 y 轴向上，所以 y 偏移取负
        let normalized = cursor.normalized.unwrap_or(Vec2::ZERO);
        tf.translation = Val2::px(normalized.x * 320.0, -normalized.y * 200.0);
    }
}
