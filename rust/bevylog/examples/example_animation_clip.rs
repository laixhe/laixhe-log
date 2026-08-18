//! Bevy 0.19 入门示例：演示 2D 动画（AnimationClip）。
//!
//! Bevy 的动画系统用 AnimationClip 定义关键帧曲线，AnimationPlayer 播放，
//! 通过 AnimationTargetId + AnimatedBy 把曲线应用到目标实体。
//!
//! 学习重点：
//! - AnimatableCurve + AnimatableKeyframeCurve 定义关键帧曲线
//! - animated_field!(Transform::translation) 指定要动画的组件字段
//! - AnimationClip::add_curve_to_target 把曲线绑定到目标
//! - AnimationGraph::from_clip 创建单剪辑动画图
//! - AnimationPlayer.start().repeat() 循环播放
//! - AnimationTargetId + AnimatedBy 链接播放器与目标实体

use bevy::animation::prelude::{
    AnimatableCurve, AnimatableKeyframeCurve, AnimationClip, AnimationGraph, AnimationGraphHandle,
    AnimationPlayer,
};
use bevy::animation::{AnimatedBy, AnimationTargetId, animated_field};
use bevy::{prelude::*, text::FontSourceTemplate};

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
) {
    commands.spawn(Camera2d);

    // 动画目标 ID：用名字生成（同一名字始终得到同一 ID）
    let target_id = AnimationTargetId::from_name(&Name::new("Ball"));

    // 1. 创建动画剪辑：小球在 x 轴 -200 ↔ 200 之间往返
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
            .expect("关键帧曲线至少需要两个采样点"),
        ),
    );

    // 2. 加入资产库，得到 Handle
    let clip_handle = clips.add(clip);

    // 3. 创建动画图（单剪辑），得到图和节点索引
    let (graph, node_index) = AnimationGraph::from_clip(clip_handle);
    let graph_handle = graphs.add(graph);

    // 4. 创建播放器，开始循环播放
    let mut player = AnimationPlayer::default();
    player.start(node_index).repeat();

    let player_entity = commands
        .spawn((player, AnimationGraphHandle(graph_handle)))
        .id();

    // 5. 生成被动画的小球实体：AnimationTargetId 匹配剪辑目标，AnimatedBy 指向播放器
    commands.spawn((
        target_id,
        AnimatedBy(player_entity),
        Mesh2d(meshes.add(Circle::new(40.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.2, 0.8, 0.4)))),
        Transform::default(),
    ));

    // 提示文本
    commands.spawn_scene(bsn! {
        Text2d::new("AnimationClip 2D 动画：小球沿 x 轴往返移动（循环）")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(24.0),
        }
        Transform::from_xyz(0.0, -260.0, 0.0)
    });
}
