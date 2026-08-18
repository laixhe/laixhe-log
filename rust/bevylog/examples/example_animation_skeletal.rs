//! Bevy 0.19 入门示例：演示 3D 骨骼动画（加载带骨骼的 gltf 模型）。
//!
//! 加载 assets/models/skeletal_arm.gltf（由 generate_skeletal_gltf 生成）：
//! 一条「手臂」网格，由 2 根骨骼驱动，动画让第二根骨骼绕 Z 轴来回摆动。
//!
//! 学习重点：
//! - AssetServer 加载 gltf，得到 Handle<Gltf>
//! - WorldAssetRoot 生成 gltf 里的场景
//! - Gltf.animations 拿到动画 clip，用 AnimationGraph::from_clips 建立动画图
//! - AnimationGraphHandle + AnimationPlayer.play().repeat() 播放动画

use bevy::prelude::*;

// 持有 gltf 句柄，供后续系统读取
#[derive(Resource)]
struct SkeletalModel(Handle<Gltf>);

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.1, 0.12, 0.18)))
        .add_systems(Startup, setup)
        .add_systems(Update, (spawn_gltf, play_animation))
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 3D 相机：从 (1, 0, 3) 看向手臂中间
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(1.0, 0.0, 3.0).looking_at(Vec3::new(1.0, 0.0, 0.0), Vec3::Y),
    ));

    // 方向光
    commands.spawn((
        DirectionalLight {
            illuminance: 5000.0,
            ..default()
        },
        Transform::from_xyz(2.0, 3.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 加载 gltf（异步，加载完成后 spawn_gltf 会生成场景）
    let gltf = asset_server.load("models/skeletal_arm.gltf");
    commands.insert_resource(SkeletalModel(gltf));
}

// 等 gltf 加载完成后，生成它的第一个场景
fn spawn_gltf(
    mut commands: Commands,
    model: Res<SkeletalModel>,
    gltf_assets: Res<Assets<Gltf>>,
    mut done: Local<bool>,
) {
    if *done {
        return;
    }
    let Some(gltf) = gltf_assets.get(&model.0) else {
        return;
    };
    *done = true;

    commands.spawn(WorldAssetRoot(gltf.scenes[0].clone()));
    info!("[骨骼动画] gltf 场景已生成");
}

// 建立动画图并循环播放第一个动画
fn play_animation(
    mut commands: Commands,
    model: Res<SkeletalModel>,
    gltf_assets: Res<Assets<Gltf>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    player: Single<(Entity, &mut AnimationPlayer)>,
    mut done: Local<bool>,
) {
    if *done {
        return;
    }
    let Some(gltf) = gltf_assets.get(&model.0) else {
        return;
    };

    let (entity, mut player) = player.into_inner();

    // 把 gltf 里的所有动画 clip 加入动画图
    let (graph, indices) = AnimationGraph::from_clips(gltf.animations.clone());
    let graph_handle = graphs.add(graph);

    // 把动画图挂到播放器实体上
    commands
        .entity(entity)
        .insert(AnimationGraphHandle(graph_handle));

    // 循环播放第一个动画
    if let Some(&node) = indices.first() {
        player.play(node).repeat();
        info!("[骨骼动画] 开始循环播放动画");
    }

    *done = true;
}
