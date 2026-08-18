//! Bevy 0.19 入门示例：演示动画播放控制（暂停 / 调速 / 重播）。
//!
//! 学习重点：
//! - AnimationPlayer::pause_all / resume_all：暂停 / 继续全部动画
//! - AnimationPlayer::all_paused：判断是否全部暂停
//! - AnimationPlayer::adjust_speeds：按倍数调整播放速度
//!
//! 操作：空格暂停/继续；↑ 加速；↓ 减速；R 重播。

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
        .add_systems(Update, control)
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
        Text2d::new("空格暂停/继续，↑加速 ↓减速，R重播"),
        TextColor(Color::WHITE),
        TextFont {
            font: FontSource::Handle(asset_server.load(FONT_PATH)),
            font_size: FontSize::Px(20.0),
            ..default()
        },
        Transform::from_xyz(0.0, -260.0, 0.0),
    ));
}

fn control(keys: Res<ButtonInput<KeyCode>>, mut q_player: Query<&mut AnimationPlayer>) {
    for mut player in &mut q_player {
        if keys.just_pressed(KeyCode::Space) {
            if player.all_paused() {
                player.resume_all();
                info!("[动画] 继续播放");
            } else {
                player.pause_all();
                info!("[动画] 暂停");
            }
        }
        if keys.just_pressed(KeyCode::ArrowUp) {
            player.adjust_speeds(1.5);
            info!("[动画] 加速");
        }
        if keys.just_pressed(KeyCode::ArrowDown) {
            player.adjust_speeds(0.5);
            info!("[动画] 减速");
        }
        if keys.just_pressed(KeyCode::KeyR) {
            player.rewind_all();
            player.resume_all();
            info!("[动画] 重播");
        }
    }
}
