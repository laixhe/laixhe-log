//! Bevy 0.19 入门示例：演示多关节人形骨骼动画（加载带 15 根骨骼的 gltf）。
//!
//! 加载 assets/models/humanoid.gltf（由 generate_humanoid_gltf 生成）：
//! 一个「火柴人」，躯干 / 头 / 双臂 / 双腿由 15 根骨骼驱动，
//! 关节处顶点绑「上下两根骨骼各 50% 权重」，弯曲时平滑过渡。
//!
//! 含三个动画 clip：Walk（走）、Run（跑）、Idle（静止），按「空格键」循环切换。
//! 切换通过 `AnimationTransitions` 实现动画混合（blend），
//! 让走 / 跑 / 静止之间平滑衔接，而不是瞬间跳变（例如「跑动→静止」的过渡）。
//!
//! 学习重点：
//! - 多骨骼蒙皮（skinning）：关节处顶点受多根骨骼权重影响
//! - Gltf.animations 拿到多个动画 clip，AnimationGraph::from_clips 建立动画图
//! - AnimationTransitions::play 实现动画过渡混合（fade out 旧动画 + fade in 新动画）

use bevy::prelude::*;
use std::time::Duration;

// 持有 gltf 句柄，供后续系统读取
#[derive(Resource)]
struct HumanoidModel(Handle<Gltf>);

// 保存三个动画 clip 在图中的节点索引，供切换系统使用
#[derive(Resource, Default)]
struct AnimNodes {
    walk: Option<AnimationNodeIndex>,
    run: Option<AnimationNodeIndex>,
    idle: Option<AnimationNodeIndex>,
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.1, 0.12, 0.18)))
        .init_resource::<AnimNodes>()
        .add_systems(Startup, setup)
        .add_systems(Update, (spawn_gltf, setup_animation, control_animation))
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 3D 相机：从斜前方看向人形中心（约 y=0.85）
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(1.4, 1.0, 3.0).looking_at(Vec3::new(0.0, 0.85, 0.0), Vec3::Y),
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
    let gltf = asset_server.load("models/humanoid.gltf");
    commands.insert_resource(HumanoidModel(gltf));
}

// 等 gltf 加载完成后，生成它的第一个场景
fn spawn_gltf(
    mut commands: Commands,
    model: Res<HumanoidModel>,
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
    info!("[人形骨骼动画] gltf 场景已生成");
}

// 建立动画图，给播放器实体挂上 AnimationGraphHandle 和 AnimationTransitions，
// 并保存两个动画的节点索引。
fn setup_animation(
    mut commands: Commands,
    model: Res<HumanoidModel>,
    gltf_assets: Res<Assets<Gltf>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    player: Single<(Entity, &AnimationPlayer)>,
    mut done: Local<bool>,
) {
    if *done {
        return;
    }
    let Some(gltf) = gltf_assets.get(&model.0) else {
        return;
    };

    let (entity, _) = player.into_inner();

    // 把 gltf 里的所有动画 clip 加入动画图（顺序：Walk, Run, Idle）
    let (graph, indices) = AnimationGraph::from_clips(gltf.animations.clone());
    let graph_handle = graphs.add(graph);

    // 动画图 + 过渡组件挂在同一个实体上（与 AnimationPlayer 同一实体）
    commands.entity(entity).insert((
        AnimationGraphHandle(graph_handle),
        AnimationTransitions::default(),
    ));

    // 保存三个动画的节点索引
    let walk = indices.first().copied();
    let run = indices.get(1).copied();
    let idle = indices.get(2).copied();
    commands.insert_resource(AnimNodes { walk, run, idle });

    *done = true;
}

// 初始播放 Walk，并监听空格键在 Walk → Run → Idle 之间循环平滑切换。
fn control_animation(
    keys: Res<ButtonInput<KeyCode>>,
    nodes: Res<AnimNodes>,
    mut anim: Query<(&mut AnimationTransitions, &mut AnimationPlayer)>,
    mut current: Local<usize>,
    mut started: Local<bool>,
) {
    // 组件尚未就绪（动画图还没建立）时跳过
    let Ok((mut transitions, mut player)) = anim.single_mut() else {
        return;
    };

    // 首次运行：播放 Walk（无过渡，立即开始）
    if !*started {
        if let Some(walk) = nodes.walk {
            transitions.play(&mut player, walk, Duration::ZERO).repeat();
            info!("[人形骨骼动画] 开始循环播放 Walk（按空格循环切换 Walk/Run/Idle）");
        }
        *started = true;
        return;
    }

    if !keys.just_pressed(KeyCode::Space) {
        return;
    }

    // 0=Walk, 1=Run, 2=Idle，空格按下后循环切换到下一个
    const NAMES: [&str; 3] = ["Walk", "Run", "Idle"];
    let next = (*current + 1) % 3;
    let target = match next {
        0 => nodes.walk,
        1 => nodes.run,
        _ => nodes.idle,
    };
    if let Some(node) = target {
        // 0.3 秒内淡出旧动画、淡入新动画，实现平滑衔接
        transitions
            .play(&mut player, node, Duration::from_millis(300))
            .repeat();
        *current = next;
        info!(
            "[人形骨骼动画] 切换到 {name} 动画（0.3s 平滑过渡）",
            name = NAMES[next]
        );
    }
}
