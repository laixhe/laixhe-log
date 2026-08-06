use bevy::prelude::*;

use crate::resources::Score;

// 分数显示标记组件：用于 Query 找到分数文本实体，更新其显示内容。
#[derive(Component)]
pub struct ScoreDisplay;

// 分数 UI 初始化：在屏幕左上角创建一个文本节点，初始显示 "Score: 0"。
// 用 UI 系统的 Text + Node（绝对定位），而非 Text2d（世界文本），
// 因为分数是 HUD（抬头显示），不随相机移动，适合用 UI 节点。
pub fn setup_score_display(mut commands: Commands) {
    commands.spawn((
        Text::new("Score: 0"),
        TextFont {
            font_size: FontSize::Px(30.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 1.0)),
        Node {
            // 绝对定位：相对于屏幕左上角
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        ScoreDisplay,
    ));
}

// 分数刷新：每帧从 Score 资源读取最新总分，更新文本内容。
// 用 Res<Score> 只读访问分数（不需要 ResMut，因为不修改分数）。
pub fn update_score_display(
    score: Res<Score>,
    // Single 查询：期望恰好一个 ScoreDisplay 实体（与其他示例风格一致）
    mut text: Single<&mut Text, With<ScoreDisplay>>,
) {
    text.0 = format!("Score: {}", score.total);
}
