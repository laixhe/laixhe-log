//! Bevy 0.19 入门示例：动画混合（AnimationTransitions 平滑过渡）。
//! 用两个程序化动画 clip，通过 AnimationTransitions 在它们之间平滑混合（交叉淡入淡出）。
//!
//! 学习重点：
//! - AnimationClip::add_curve_to_target 程序化定义动画（无需外部模型）
//! - AnimationGraph::from_clips 把多个 clip 加入同一个动画图
//! - AnimationTransitions::play(player, node, duration) 平滑切换到新动画
//!   - duration > 0 时旧动画逐渐淡出、新动画淡入（混合）
//!   - Duration::ZERO 则瞬间切换（不混合）
//! - 混合期间两个动画的权重相加，位置会「中和」——观察对角线过渡
//!
//! 操作：1 切到左右移动；2 切到上下移动（观察平滑过渡）。

use bevy::animation::prelude::{
    AnimatableCurve, AnimatableKeyframeCurve, AnimationClip, AnimationGraph, AnimationGraphHandle,
    AnimationNodeIndex, AnimationPlayer, AnimationTransitions,
};
use bevy::animation::{AnimatedBy, AnimationTargetId, animated_field};
use bevy::prelude::*;
use std::time::Duration;

// 保存两个动画节点，供切换时引用
#[derive(Resource)]
struct BlendNodes(AnimationNodeIndex, AnimationNodeIndex);

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        .add_systems(Update, control)
        .run()
}

fn setup(
    mut commands: Commands,
    mut clips: ResMut<Assets<AnimationClip>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let target_id = AnimationTargetId::from_name(&Name::new("Ball"));

    // 动画 A：左右移动（x 变化）
    let mut clip_a = AnimationClip::default();
    clip_a.add_curve_to_target(
        target_id,
        AnimatableCurve::new(
            animated_field!(Transform::translation),
            AnimatableKeyframeCurve::new([
                (0.0, Vec3::new(-200.0, 0.0, 0.0)),
                (1.0, Vec3::new(200.0, 0.0, 0.0)),
                (2.0, Vec3::new(-200.0, 0.0, 0.0)),
            ])
            .unwrap(),
        ),
    );
    let handle_a = clips.add(clip_a);

    // 动画 B：上下移动（y 变化）
    let mut clip_b = AnimationClip::default();
    clip_b.add_curve_to_target(
        target_id,
        AnimatableCurve::new(
            animated_field!(Transform::translation),
            AnimatableKeyframeCurve::new([
                (0.0, Vec3::new(0.0, -200.0, 0.0)),
                (1.0, Vec3::new(0.0, 200.0, 0.0)),
                (2.0, Vec3::new(0.0, -200.0, 0.0)),
            ])
            .unwrap(),
        ),
    );
    let handle_b = clips.add(clip_b);

    // 把两个 clip 加入同一个动画图
    let (graph, indices) = AnimationGraph::from_clips([handle_a, handle_b]);
    let graph_handle = graphs.add(graph);

    // 播放器 + 过渡管理器
    let mut player = AnimationPlayer::default();
    let mut transitions = AnimationTransitions::new();
    // 初始播放动画 A（瞬间开始，无需过渡）
    transitions
        .play(&mut player, indices[0], Duration::ZERO)
        .repeat();

    let player_entity = commands
        .spawn((player, transitions, AnimationGraphHandle(graph_handle)))
        .id();

    // 小球（动画作用对象）
    commands.spawn((
        target_id,
        AnimatedBy(player_entity),
        Mesh2d(meshes.add(Circle::new(40.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.2, 0.8, 0.4)))),
        Transform::default(),
    ));

    commands.insert_resource(BlendNodes(indices[0], indices[1]));
}

// 按键切换动画：1 → 左右，2 → 上下（1 秒平滑过渡）。
fn control(
    keys: Res<ButtonInput<KeyCode>>,
    nodes: Res<BlendNodes>,
    mut query: Query<(&mut AnimationTransitions, &mut AnimationPlayer)>,
) {
    for (mut transitions, mut player) in &mut query {
        if keys.just_pressed(KeyCode::Digit1) {
            transitions
                .play(&mut player, nodes.0, Duration::from_secs(1))
                .repeat();
            info!("[动画混合] 切换到左右移动");
        }
        if keys.just_pressed(KeyCode::Digit2) {
            transitions
                .play(&mut player, nodes.1, Duration::from_secs(1))
                .repeat();
            info!("[动画混合] 切换到上下移动");
        }
    }
}
