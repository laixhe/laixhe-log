//! Bevy 0.19 入门示例：演示动画事件（AnimationEvent）。
//!
//! 学习重点：
//! - #[derive(AnimationEvent)]：定义动画事件类型
//! - AnimationClip::add_event：在动画播放到某个时间点时触发事件
//! - On<Event>：用全局 observer 监听动画事件
//!
//! 观察：小球动画每循环一次，会在中点位置触发事件并打印日志。

use bevy::animation::prelude::{
    AnimatableCurve, AnimatableKeyframeCurve, AnimationClip, AnimationGraph, AnimationGraphHandle,
    AnimationPlayer,
};
use bevy::animation::{AnimatedBy, AnimationEvent, AnimationTargetId, animated_field};
use bevy::prelude::*;
use bevy::text::FontSource;

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// 动画事件：动画播放到指定时间点触发
#[derive(AnimationEvent, Clone, Debug)]
struct Halfway;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_observer(on_halfway)
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

    let target_id = AnimationTargetId::from_name(&Name::new("Ball"));

    let mut clip = AnimationClip::default();
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

    // 在动画播放到 1.0 秒（中点）时触发事件
    clip.add_event(1.0, Halfway);

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
        Mesh2d(meshes.add(Circle::new(40.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.2, 0.8, 0.4)))),
        Transform::default(),
    ));

    commands.spawn((
        Text2d::new("动画事件：每次循环到中点触发事件"),
        TextColor(Color::WHITE),
        TextFont {
            font: FontSource::Handle(asset_server.load(FONT_PATH)),
            font_size: FontSize::Px(20.0),
            ..default()
        },
        Transform::from_xyz(0.0, -260.0, 0.0),
    ));
}

fn on_halfway(_event: On<Halfway>) {
    info!("[动画事件] 动画播放到中点!");
}
