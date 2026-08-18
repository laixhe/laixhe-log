//! Bevy 0.19 入门示例：演示多字段动画（一个 AnimationClip 同时动画多个字段）。
//!
//! 学习重点：
//! - 一个 AnimationClip 可以包含多条曲线
//! - animated_field!(Transform::translation / rotation / scale) 分别动画不同字段
//! - 同一个目标实体同时做位置、旋转、缩放动画
//!
//! 观察：方块沿 x 轴往返移动、绕自身旋转、同时做缩放脉冲。

use bevy::animation::prelude::{
    AnimatableCurve, AnimatableKeyframeCurve, AnimationClip, AnimationGraph, AnimationGraphHandle,
    AnimationPlayer,
};
use bevy::animation::{AnimatedBy, AnimationTargetId, animated_field};
use bevy::prelude::*;
use bevy::text::FontSource;

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        .run()
}

fn setup(
    mut commands: Commands,
    mut clips: ResMut<Assets<AnimationClip>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);

    let target_id = AnimationTargetId::from_name(&Name::new("Cube"));

    let mut clip = AnimationClip::default();

    // 曲线 1：位置沿 x 轴往返
    clip.add_curve_to_target(
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

    // 曲线 2：旋转绕 Z 轴转一圈
    clip.add_curve_to_target(
        target_id,
        AnimatableCurve::new(
            animated_field!(Transform::rotation),
            AnimatableKeyframeCurve::new([
                (0.0, Quat::IDENTITY),
                (2.0, Quat::from_rotation_z(std::f32::consts::TAU)),
            ])
            .unwrap(),
        ),
    );

    // 曲线 3：缩放脉冲
    clip.add_curve_to_target(
        target_id,
        AnimatableCurve::new(
            animated_field!(Transform::scale),
            AnimatableKeyframeCurve::new([
                (0.0, Vec3::ONE),
                (1.0, Vec3::splat(1.5)),
                (2.0, Vec3::ONE),
            ])
            .unwrap(),
        ),
    );

    let clip_handle = clips.add(clip);
    let (graph, node_index) = AnimationGraph::from_clip(clip_handle);
    let graph_handle = graphs.add(graph);

    let mut player = AnimationPlayer::default();
    player.start(node_index).repeat();
    let player_entity = commands
        .spawn((player, AnimationGraphHandle(graph_handle)))
        .id();

    commands.spawn((
        target_id,
        AnimatedBy(player_entity),
        Mesh2d(meshes.add(Rectangle::new(60.0, 60.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.3, 0.7, 0.9)))),
        Transform::default(),
    ));

    commands.spawn((
        Text2d::new("多字段动画：位置 + 旋转 + 缩放"),
        TextColor(Color::WHITE),
        TextFont {
            font: FontSource::Handle(asset_server.load(FONT_PATH)),
            font_size: FontSize::Px(22.0),
            ..default()
        },
        Transform::from_xyz(0.0, -260.0, 0.0),
    ));
}
