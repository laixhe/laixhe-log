//! 游戏主模块（Play 页）：集中注册全部全局资源与系统，
//! 并负责进入游戏时的重置 / 初始化、以及开发调试快捷键。
//! 各子模块分工见 README「代码地图」。

pub mod art;
pub mod car_lights;
pub mod cinematic;
pub mod collision;
pub mod components;
pub mod dispatch;
pub mod facing;
pub mod hud;
pub mod interactions;
pub mod job;
pub mod modal;
pub mod movement;
pub mod npc;
pub mod progression;
pub mod resources;
pub mod save;
pub mod scenes;
pub mod sfx;
pub mod sim;
pub mod street_lights;
pub mod traffic;
pub mod traffic_light;
pub mod transit;

use bevy::prelude::*;

use crate::router::GameState;

use components::*;
use job::JobPipeline;
use rand::RngExt;
use resources::*;

// ==================== 游戏插件 ====================
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // 全局资源
            .init_resource::<PlayerStats>()
            .init_resource::<GameClock>()
            .init_resource::<GameLocation>()
            .init_resource::<WalkState>()
            .init_resource::<PendingAction>()
            .init_resource::<AutoCooldown>()
            .init_resource::<ToastLog>()
            .init_resource::<Modal>()
            .init_resource::<DialogueState>()
            .init_resource::<DialogSettle>()
            .init_resource::<QuizState>()
            .init_resource::<EventState>()
            .init_resource::<WorkBonus>()
            .init_resource::<FreeUse>()
            .init_resource::<JobAdvanceStamp>()
            .init_resource::<GameFlags>()
            .init_resource::<OverInfo>()
            .init_resource::<Ending>()
            .init_resource::<SceneForce>()
            .init_resource::<TransitState>()
            .init_resource::<BikeMode>()
            .init_resource::<SceneResume>()
            .init_resource::<collision::CollisionMap>()
            .init_resource::<JobPipeline>()
            .init_resource::<Cinematic>()
            .init_resource::<save::PendingLoad>()
            .insert_resource(GlobalAmbientLight {
                color: Color::WHITE,
                brightness: 380.0,
                ..default()
            })
            // 启动时生成纸张纹理与程序化音效
            .add_systems(Startup, (hud::generate_paper, sfx::setup_sfx))
            // 进入 / 退出 Playing 页
            // `.chain()` 让括号里的系统严格按书写顺序执行（默认系统间无顺序保证）：
            // 先重置全部资源，再读档覆盖，最后才生成主角 / 相机 / HUD。
            .add_systems(
                OnEnter(GameState::Playing),
                (
                    reset_playing,
                    reset_misc_flags,
                    reset_cinematic,
                    reset_scene_resume,
                    save::apply_save,
                    setup_playing,
                    collision::build_map,
                    scenes::spawn_world_labels,
                )
                    .chain(),
            )
            .add_systems(OnExit(GameState::Playing), cleanup_playing)
            // 游戏主循环
            .add_systems(
                Update,
                (
                    (
                        movement::handle_click,
                        movement::move_player,
                        movement::animate_limbs,
                        interactions::dispatch_action,
                        collision::resolve_player,
                    )
                        .chain(),
                    movement::camera_follow,
                    movement::auto_behaviors,
                    // scene_manager 先处理「重置/读档」的落点，update_location 再按新坐标刷新区域
                    (hud::scene_manager, hud::update_location).chain(),
                    transit::transit_tick,
                    traffic_light::traffic_tick,
                    // 车辆先更新转向状态与位移，车灯系统再据此刻画刹车灯/大灯/转向灯
                    (traffic::vehicle_tick, car_lights::car_lights_tick).chain(),
                    traffic::crossing_tick,
                    // NPC 浮动 / 巡逻 / UI 名牌投影（合并为链以控制顶层系统元组数量）
                    // 名牌投影排在 camera_follow 之后：保证用的是本帧相机位置
                    (scenes::npc_bob, scenes::wander_npcs, scenes::update_world_labels)
                        .chain()
                        .after(movement::camera_follow),
                    // 白天/夜晚光照切换后，街道路灯按同一时段亮灭
                    (scenes::update_daylight, street_lights::street_lights_tick).chain(),
                    progression::phase_tick,
                    cinematic::cinematic_ui,
                    save::auto_save,
                    hud::toast_tick,
                    hud::update_banner,
                    hud::update_money,
                    hud::update_seals,
                    hud::update_skills,
                    hud::update_toast,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (
                    modal::modal_ui,
                    modal::handle_modal_buttons,
                    modal::handle_event_buttons,
                    modal::handle_quiz_buttons,
                    // 先推进求职（等待 +1 / 触发笔试），再让 auto_save 保存当日结果，
                    // 避免读档后求职推进重复执行（笔试重弹、等待天数虚增）
                    job::job_advance_system.before(save::auto_save),
                    debug_shortcuts,
                    debug_save,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

// ==================== 调试快捷键 ====================
// 1-5 传送到五个区域；N 推进一天；J 投简历面板；T 通勤面板；U 随机事件弹窗（S 存档见 debug_save）。
// 方便开发与试玩时快速验证核心循环。弹窗打开时 J/T/U 被禁用，避免顶替当前弹窗。
#[allow(clippy::too_many_arguments)]
pub fn debug_shortcuts(
    keys: Res<ButtonInput<KeyCode>>,
    mut location: ResMut<GameLocation>,
    mut clock: ResMut<GameClock>,
    mut toast: ResMut<ToastLog>,
    mut over: ResMut<OverInfo>,
    mut ending: ResMut<Ending>,
    mut flags: ResMut<GameFlags>,
    mut stats: ResMut<PlayerStats>,
    mut bonus: ResMut<WorkBonus>,
    mut modal: ResMut<Modal>,
    mut next_state: ResMut<NextState<GameState>>,
    mut cinematic: ResMut<Cinematic>,
    mut walk: ResMut<WalkState>,
    mut player: Single<&mut Transform, With<PlayerRoot>>,
    mut event_state: ResMut<EventState>,
) {
    // 调试键 1-5：直接传送到对应区域（统一地图上没有"切换场景"）
    // 传送同时清空行走目标/路径，避免角色继续朝旧目标走。
    if keys.just_pressed(KeyCode::Digit1) {
        location.0 = Location::Home;
        let sp = scenes::spawn_pos(Location::Home);
        player.translation = sp;
        player.rotation = default();
        walk.target = None;
        walk.path.clear();
        toast.push("[调试] 传送到 家");
    }
    if keys.just_pressed(KeyCode::Digit2) {
        location.0 = Location::Campus;
        let sp = scenes::spawn_pos(Location::Campus);
        player.translation = sp;
        player.rotation = default();
        walk.target = None;
        walk.path.clear();
        toast.push("[调试] 传送到 校园");
    }
    if keys.just_pressed(KeyCode::Digit3) {
        location.0 = Location::Cafeteria;
        let sp = scenes::spawn_pos(Location::Cafeteria);
        player.translation = sp;
        player.rotation = default();
        walk.target = None;
        walk.path.clear();
        toast.push("[调试] 传送到 食堂");
    }
    if keys.just_pressed(KeyCode::Digit4) {
        location.0 = Location::Office;
        let sp = scenes::spawn_pos(Location::Office);
        player.translation = sp;
        player.rotation = default();
        walk.target = None;
        walk.path.clear();
        toast.push("[调试] 传送到 办公室");
    }
    if keys.just_pressed(KeyCode::Digit5) {
        location.0 = Location::Park;
        let sp = scenes::spawn_pos(Location::Park);
        player.translation = sp;
        player.rotation = default();
        walk.target = None;
        walk.path.clear();
        toast.push("[调试] 传送到 公园");
    }
    if keys.just_pressed(KeyCode::KeyN) {
        let over_now = progression::advance_day(
            &mut clock,
            &mut stats,
            &mut flags,
            &mut toast,
            &mut bonus,
            &mut over,
            &mut ending,
            &mut next_state,
            &mut cinematic,
        );
        if !over_now {
            info!(
                "[调试] 推进到 第{}周 第{}天（{}）",
                clock.week,
                clock.day,
                day_label(clock.day)
            );
        }
    }
    // 已有弹窗打开时禁用 J/T/U：它们会直接把当前弹窗顶掉（例如对话中途误触）
    if modal.kind.is_none() {
        if keys.just_pressed(KeyCode::KeyJ) {
            toast.push("[调试] 打开投简历面板");
            modal.open(ModalKind::Company);
        }
        if keys.just_pressed(KeyCode::KeyT) {
            toast.push("[调试] 打开通勤选择");
            modal.open(ModalKind::Commute);
        }
        if keys.just_pressed(KeyCode::KeyU) {
            let mut rng = rand::rng();
            event_state.idx = rng.random_range(0..progression::EVENTS.len());
            toast.push("[调试] 打开随机事件弹窗");
            modal.open(ModalKind::Event);
        }
    }
}

// S 手动存档（独立系统：debug_shortcuts 已到 Bevy 系统参数 16 个上限，存档再添 3 个参数会超限）
pub fn debug_save(
    keys: Res<ButtonInput<KeyCode>>,
    stats: Res<PlayerStats>,
    clock: Res<GameClock>,
    flags: Res<GameFlags>,
    pipeline: Res<JobPipeline>,
    location: Res<GameLocation>,
    bonus: Res<WorkBonus>,
    transit: Res<TransitState>,
    stamp: Res<JobAdvanceStamp>,
    modal: Res<Modal>,
    event: Res<EventState>,
    quiz: Res<QuizState>,
    dialog: Res<DialogueState>,
    settle: Res<DialogSettle>,
    mut toast: ResMut<ToastLog>,
    player: Single<&Transform, With<PlayerRoot>>,
) {
    if !keys.just_pressed(KeyCode::KeyS) {
        return;
    }
    let transit = transit.active.then(|| transit.clone());
    if save::save_game(
        &stats,
        &clock,
        &flags,
        &pipeline,
        location.0,
        &bonus,
        player.translation,
        transit,
        &stamp,
        &modal,
        &event,
        &quiz,
        &dialog,
        &settle,
    ) {
        toast.push("💾 已保存进度");
        info!("[存档] 手动保存 第{}周 第{}天", clock.week, clock.day);
    } else {
        toast.push("存档失败！");
        warn!("[存档] 手动保存失败（写入 {} 出错）", save::SAVE_PATH);
    }
}

// ==================== 进入游戏页 ====================
// 第一步：重置全部资源（参数较多，单独拆成一个系统）。
fn reset_cinematic(mut cinematic: ResMut<Cinematic>) {
    *cinematic = Cinematic::default();
}

#[allow(clippy::too_many_arguments)]
fn reset_playing(
    mut stats: ResMut<PlayerStats>,
    mut clock: ResMut<GameClock>,
    mut flags: ResMut<GameFlags>,
    mut pipeline: ResMut<JobPipeline>,
    mut modal: ResMut<Modal>,
    mut dialog: ResMut<DialogueState>,
    mut quiz: ResMut<QuizState>,
    mut bonus: ResMut<WorkBonus>,
    mut over: ResMut<OverInfo>,
    mut ending: ResMut<Ending>,
    mut walk: ResMut<WalkState>,
    mut pending: ResMut<PendingAction>,
    mut location: ResMut<GameLocation>,
    mut force: ResMut<SceneForce>,
    mut toast: ResMut<ToastLog>,
    mut transit: ResMut<TransitState>,
) {
    *stats = PlayerStats::default();
    *clock = GameClock::default();
    *flags = GameFlags::default();
    *pipeline = JobPipeline::default();
    *modal = Modal::default();
    *dialog = DialogueState::default();
    *quiz = QuizState::default();
    *bonus = WorkBonus::default();
    *over = OverInfo::default();
    *ending = Ending::default();
    *walk = WalkState::default();
    *pending = PendingAction::default();
    *transit = TransitState::default();
    toast.items.clear();
    location.0 = Location::Home;
    force.0 += 1;
    info!("[重置] 进入游戏：全部资源已初始化");
}

// 重开新档时一并清零的小资源（独立系统，避免 reset_playing 参数超限）。
// 这些资源若不重置会残留旧档状态：DialogSettle 防刷记录（否则新档第 1 天对话不给效果）、
// BikeMode 骑行开关（否则新档自带 2.5 倍速）、AutoCooldown 自主行为冷却时间戳（否则开局
// 立即触发觅食/补觉）、JobAdvanceStamp 求职去重标记（否则新档当天求职不推进）、EventState。
fn reset_misc_flags(
    mut stamp: ResMut<JobAdvanceStamp>,
    mut settle: ResMut<DialogSettle>,
    mut bike: ResMut<BikeMode>,
    mut cooldown: ResMut<AutoCooldown>,
    mut event_state: ResMut<EventState>,
    mut free_use: ResMut<FreeUse>,
) {
    *stamp = JobAdvanceStamp::default();
    *settle = DialogSettle::default();
    *bike = BikeMode::default();
    *cooldown = AutoCooldown::default();
    *event_state = EventState::default();
    *free_use = FreeUse::default();
}

// 新档/重开时清除「行驶中存档恢复」标记（独立小系统，避免 reset_playing 参数超限）
fn reset_scene_resume(mut resume: ResMut<SceneResume>) {
    resume.0 = None;
}

// 第二步：生成整座城市地图 / 主角 / 3D 相机 / HUD；地图常驻不重建。
fn setup_playing(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    paper: Res<PaperTexture>,
    mut toast: ResMut<ToastLog>,
) {
    // 一次性构建城市（五个区域 + 校园周边探索点 + 主路 + 交通站点）
    scenes::build_city(&mut commands, &mut meshes, &mut materials, &assets, &paper);

    // 低模主角：多段式方块人（腿/躯干/手臂/头/发/背包），
    // 四肢挂在肩/髋枢轴点上，行走时由 movement 系统程序化摆动。
    // 父实体带 Visibility：子网格/标签才能正确继承可见性（否则 Bevy 报 B0004）。
    commands
        .spawn((
            GameRoot,
            PlayerRoot,
            Visibility::default(),
            Transform::from_xyz(0.0, 0.0, 7.0),
        ))
        .with_children(|p| {
            let cloth = materials.add(StandardMaterial {
                base_color: Color::srgb(0.30, 0.55, 0.85),
                perceptual_roughness: 0.7,
                ..default()
            });
            let cloth_dark = materials.add(StandardMaterial {
                base_color: Color::srgb(0.22, 0.42, 0.68),
                perceptual_roughness: 0.7,
                ..default()
            });
            let pants = materials.add(StandardMaterial {
                base_color: Color::srgb(0.24, 0.30, 0.45),
                perceptual_roughness: 0.7,
                ..default()
            });
            let skin = materials.add(StandardMaterial {
                base_color: Color::srgb(0.95, 0.78, 0.62),
                perceptual_roughness: 0.7,
                ..default()
            });
            let hair = materials.add(StandardMaterial {
                base_color: Color::srgb(0.22, 0.18, 0.16),
                perceptual_roughness: 0.7,
                ..default()
            });
            let shoe = materials.add(StandardMaterial {
                base_color: Color::srgb(0.18, 0.16, 0.15),
                perceptual_roughness: 0.7,
                ..default()
            });
            let pack = materials.add(StandardMaterial {
                base_color: Color::srgb(0.85, 0.55, 0.25),
                perceptual_roughness: 0.8,
                ..default()
            });
            let shadow = materials.add(StandardMaterial {
                base_color: Color::srgba(0.15, 0.12, 0.08, 0.4),
                unlit: true,
                ..default()
            });

            // 脚下阴影圆片
            p.spawn((
                Mesh3d(meshes.add(Cylinder::new(0.42, 0.02))),
                MeshMaterial3d(shadow),
                Transform::from_xyz(0.0, 0.01, 0.0),
            ));

            // 躯干（带微倾枢轴）
            p.spawn((
                Visibility::default(),
                LimbKind::Body,
                Transform::from_xyz(0.0, 0.78, 0.0),
            ))
            .with_children(|body| {
                body.spawn((
                    Mesh3d(meshes.add(Cuboid::new(0.52, 0.55, 0.3))),
                    MeshMaterial3d(cloth.clone()),
                    Transform::from_xyz(0.0, 0.0, 0.0),
                ));
                // 头 + 发（头本身也是父实体，带 Visibility 供其下头发网格继承）
                body.spawn((
                    Visibility::default(),
                    Mesh3d(meshes.add(Sphere::new(0.22))),
                    MeshMaterial3d(skin.clone()),
                    Transform::from_xyz(0.0, 0.42, 0.0),
                ))
                .with_children(|head| {
                    head.spawn((
                        Mesh3d(meshes.add(Cuboid::new(0.3, 0.12, 0.3))),
                        MeshMaterial3d(hair.clone()),
                        Transform::from_xyz(0.0, 0.14, 0.0),
                    ));
                });
                // 背包
                body.spawn((
                    Mesh3d(meshes.add(Cuboid::new(0.28, 0.38, 0.14))),
                    MeshMaterial3d(pack.clone()),
                    Transform::from_xyz(0.0, 0.0, -0.22),
                ));
            });

            // 手臂（枢轴在肩部，mesh 向下偏移）
            let arm_mesh = meshes.add(Cuboid::new(0.13, 0.5, 0.13));
            let arm_mat = cloth_dark.clone();
            p.spawn((
                Visibility::default(),
                LimbKind::ArmL,
                Transform::from_xyz(-0.34, 1.02, 0.0),
            ))
            .with_children(|arm| {
                arm.spawn((
                    Mesh3d(arm_mesh.clone()),
                    MeshMaterial3d(arm_mat.clone()),
                    Transform::from_xyz(0.0, -0.25, 0.0),
                ));
            });
            p.spawn((
                Visibility::default(),
                LimbKind::ArmR,
                Transform::from_xyz(0.34, 1.02, 0.0),
            ))
            .with_children(|arm| {
                arm.spawn((
                    Mesh3d(arm_mesh.clone()),
                    MeshMaterial3d(arm_mat.clone()),
                    Transform::from_xyz(0.0, -0.25, 0.0),
                ));
            });

            // 腿（枢轴在髋部，mesh 向下偏移，脚部加深色鞋）
            let leg_mesh = meshes.add(Cuboid::new(0.16, 0.5, 0.16));
            let shoe_mesh = meshes.add(Cuboid::new(0.18, 0.1, 0.24));
            p.spawn((
                Visibility::default(),
                LimbKind::LegL,
                Transform::from_xyz(-0.14, 0.42, 0.0),
            ))
            .with_children(|leg| {
                leg.spawn((
                    Mesh3d(leg_mesh.clone()),
                    MeshMaterial3d(pants.clone()),
                    Transform::from_xyz(0.0, -0.25, 0.0),
                ));
                leg.spawn((
                    Mesh3d(shoe_mesh.clone()),
                    MeshMaterial3d(shoe.clone()),
                    Transform::from_xyz(0.0, -0.52, 0.03),
                ));
            });
            p.spawn((
                Visibility::default(),
                LimbKind::LegR,
                Transform::from_xyz(0.14, 0.42, 0.0),
            ))
            .with_children(|leg| {
                leg.spawn((
                    Mesh3d(leg_mesh.clone()),
                    MeshMaterial3d(pants.clone()),
                    Transform::from_xyz(0.0, -0.25, 0.0),
                ));
                leg.spawn((
                    Mesh3d(shoe_mesh.clone()),
                    MeshMaterial3d(shoe.clone()),
                    Transform::from_xyz(0.0, -0.52, 0.03),
                ));
            });
        });

    // 3D 相机（45° 俯视角，每帧跟随主角；order 0，清理为纸张底色）
    commands.spawn((
        GameRoot,
        Camera3d::default(),
        Camera {
            order: 0,
            ..default()
        },
        Transform::from_xyz(0.0, 15.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // HUD
    hud::spawn_hud(&mut commands, &assets);

    toast.push("欢迎来到程序员求职生存模拟！点击地面移动，走近热点互动");
}

// ==================== 退出游戏页 ====================
// 场景/HUD/弹窗的父子实体都带 GameRoot：只杀树根即可，子实体随父级连带删除。
// try_despawn 让已经被父级连带删除的重复删除静默跳过，避免 "Entity despawned" 警告。
fn cleanup_playing(mut commands: Commands, roots: Query<Entity, With<GameRoot>>) {
    for e in &roots {
        commands.entity(e).try_despawn();
    }
}
