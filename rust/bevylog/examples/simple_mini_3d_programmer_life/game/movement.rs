//! 主角移动：点击地面寻路、四肢摆动走路动画、相机跟随，
//! 以及自主行为（饱食 < 30 自动觅食、精力 < 28 自动补觉）。

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::*;
use super::resources::*;
use super::traffic::Vehicle;

// ==================== 点击寻路 ====================
// 点击地面 → 走到点击处；点击热点/NPC 附近 → 走过去并自动触发。
// 这里用 Bevy 0.19 的 `Single` 拿唯一窗口 / 相机（等价于 Query 但保证恰好一个），
// 再用 camera.viewport_to_world_2d 把屏幕坐标转成世界坐标。
#[allow(clippy::too_many_arguments)]
pub fn handle_click(
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera3d>>,
    mouse: Res<ButtonInput<MouseButton>>,
    hotspots: Query<(&Hotspot, &Transform)>,
    npcs: Query<(&NpcMarker, &Transform)>,
    ui_interaction: Query<&Interaction>,
    modal: Res<Modal>,
    cinematic: Res<Cinematic>,
    transit: Res<TransitState>,
    bank: Res<super::sfx::SoundBank>,
    mut commands: Commands,
    mut walk: ResMut<WalkState>,
    mut pending: ResMut<PendingAction>,
    player: Single<&Transform, With<PlayerRoot>>,
) {
    if modal.kind.is_some() || !mouse.just_pressed(MouseButton::Left) {
        return;
    }
    // 过场演出期间锁操作
    if cinematic.active {
        return;
    }
    super::sfx::play(&mut commands, &bank, super::sfx::Sfx::Click);
    // 乘车自动行驶中不能手动走动
    if transit.active {
        return;
    }
    // 鼠标悬停在 UI 元素上时不触发行走
    if ui_interaction.iter().any(|i| *i != Interaction::None) {
        return;
    }
    let Some(cursor) = window.cursor_position() else {
        return;
    };
    let (cam, cam_gt) = *camera;
    let Ok(ray) = cam.viewport_to_world(cam_gt, cursor) else {
        return;
    };
    // 与地面 y=0 求交
    let Some(t) = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y)) else {
        return;
    };
    let p = ray.get_point(t);
    let gp = Vec3::new(
        p.x.clamp(-WORLD_HALF, WORLD_HALF),
        0.0,
        p.z.clamp(-WORLD_HALF, WORLD_HALF),
    );

    // 玩家发起新点击 = 新意图：清掉弹窗期间滞留的待执行行为，
    // 避免弹窗关闭后自动触发上一个热点（如弹窗开着时走到床边，关弹窗后被误判睡觉）
    pending.0 = None;

    // 找最近的、半径 2.2 以内的 NPC（优先于热点，方便点人对话）
    let mut nearest_npc: Option<(usize, f32, Vec3)> = None;
    for (npc, tf) in &npcs {
        let d = tf.translation.xz().distance(gp.xz());
        if d < 2.2 && nearest_npc.is_none_or(|(_, nd, _)| d < nd) {
            nearest_npc = Some((npc.idx, d, tf.translation));
        }
    }
    if let Some((idx, _, pos)) = nearest_npc {
        // 停在 NPC 面前约 1.2m，不走到 NPC 正中心——
        // 否则玩家与 NPC 重合时，NPC 头顶的名牌正好悬在玩家头顶上方
        let dir = pos - player.translation;
        let dist = Vec2::new(dir.x, dir.z).length();
        let stand = if dist > 0.001 {
            pos - Vec3::new(dir.x, 0.0, dir.z).normalize() * 1.2
        } else {
            pos - Vec3::new(0.0, 0.0, 1.2)
        };
        walk.target = Some(Vec3::new(stand.x, 0.0, stand.z));
        walk.cmd = WalkCmd::Npc(idx);
        return;
    }

    // 找最近的、半径 2.0 以内的热点
    let mut nearest: Option<(HotspotKind, f32, Vec3)> = None;
    for (hp, tf) in &hotspots {
        let d = tf.translation.xz().distance(gp.xz());
        if d < 2.0 && nearest.is_none_or(|(_, nd, _)| d < nd) {
            nearest = Some((hp.kind, d, tf.translation));
        }
    }

    debug!("[寻路] 点击 ({:.1}, {:.1})", gp.x, gp.z);
    match nearest {
        Some((kind, _, pos)) => {
            debug!("[寻路] 走向热点 {kind:?}");
            walk.target = Some(Vec3::new(pos.x, 0.0, pos.z));
            walk.cmd = WalkCmd::Interact(kind);
        }
        None => {
            debug!("[寻路] 走向空地");
            walk.target = Some(gp);
            walk.cmd = WalkCmd::Move;
        }
    }
}

// ==================== 行走 ====================
// 直线走向目标；走路起伏 + 自动面向移动方向；到达后写入待执行行为。
// 会先算一条绕开建筑的 A* 路径（walk.path），沿路径点走，每帧再做碰撞推出：
// 贴墙能滑动，穿不过墙/建筑。目标不可达（被墙围住）走约 0.7 秒后自动放弃。
// 行走时四肢程序化摆动（摆臂摆腿 + 身体微倾）。
#[allow(clippy::too_many_arguments)]
pub fn move_player(
    time: Res<Time>,
    mut walk: ResMut<WalkState>,
    mut pending: ResMut<PendingAction>,
    hotspots: Query<(&Hotspot, &Transform), Without<PlayerRoot>>,
    bank: Res<super::sfx::SoundBank>,
    bike: Res<BikeMode>,
    map: Res<super::collision::CollisionMap>,
    vehicles: Query<&Transform, (With<Vehicle>, Without<PlayerRoot>)>,
    mut commands: Commands,
    mut step_acc: Local<f32>,
    mut stuck: Local<(f32, u32)>, // (离最终目标的距离, 连续停滞帧数)
    mut toast: ResMut<ToastLog>,
    mut player: Single<&mut Transform, With<PlayerRoot>>,
) {
    let dt = time.delta_secs();
    let Some(target) = walk.target else {
        // 静止时让起伏归零
        walk.bob = 0.0;
        walk.path.clear();
        walk.path_target = None;
        player.translation.y = 0.0;
        return;
    };

    // 目标变化 → 用 A* 重算绕行路径
    if walk.path_target != Some(target) {
        walk.path_target = Some(target);
        walk.path = super::collision::find_path(&map, player.translation.xz(), target.xz())
            .unwrap_or_default();
        stuck.0 = f32::MAX;
        stuck.1 = 0;
        debug!(
            "[寻路] 目标 ({:.1},{:.1}) 路径 {} 段",
            target.x,
            target.z,
            walk.path.len()
        );
    }

    let pos = player.translation;
    let to = target - pos;
    let final_dist = Vec2::new(to.x, to.z).length();

    // 到达最终目标：写入待执行行为
    if final_dist < 0.4 {
        walk.target = None;
        walk.path.clear();
        walk.path_target = None;
        player.translation = Vec3::new(target.x, 0.0, target.z);
        let resolved: Option<PendingKind> = match walk.cmd {
            WalkCmd::Interact(kind) => Some(PendingKind::Hotspot(kind)),
            WalkCmd::Npc(idx) => Some(PendingKind::Npc(idx)),
            WalkCmd::Food => nearest_food(hotspots, player.translation).map(PendingKind::Hotspot),
            WalkCmd::Sleep => Some(PendingKind::Hotspot(HotspotKind::Bed)),
            WalkCmd::Move => None,
        };
        if let Some(kind) = resolved {
            debug!("[寻路] 到达目标，待执行行为 {kind:?}");
            pending.0 = Some(kind);
        }
        walk.cmd = WalkCmd::Move;
        walk.bob = 0.0;
        return;
    }

    // 直线（无绕行路径）时做停滞检测：约 0.7 秒没接近目标就放弃，避免对着墙一直走
    if walk.path.is_empty() {
        if final_dist < stuck.0 - 0.001 {
            stuck.0 = final_dist;
            stuck.1 = 0;
        } else {
            stuck.1 += 1;
            if stuck.1 > 42 {
                debug!("[寻路] 目标不可达，放弃 ({:.1},{:.1})", target.x, target.z);
                toast.push("走不过去…");
                walk.target = None;
                walk.cmd = WalkCmd::Move;
                walk.path.clear();
                walk.path_target = None;
                walk.bob = 0.0;
                return;
            }
        }
    }

    // 弹掉已到达的中间路径点，取当前子目标
    while let Some(wp) = walk.path.first() {
        let wv = Vec3::new(wp.x, 0.0, wp.y);
        if Vec2::new(wv.x - pos.x, wv.z - pos.z).length() < 0.4 {
            walk.path.remove(0);
        } else {
            break;
        }
    }
    let goal = walk
        .path
        .first()
        .map(|wp| Vec3::new(wp.x, 0.0, wp.y))
        .unwrap_or(target);
    let to_goal = goal - pos;
    let dist_xz = Vec2::new(to_goal.x, to_goal.z).length();
    if dist_xz < 0.4 {
        // 正好踩在中间点上：落下即可，下一帧会弹出并走向下一个点
        player.translation = Vec3::new(goal.x, 0.0, goal.z);
        walk.bob = 0.0;
        return;
    }

    // 面向移动方向（绕 Y 轴）
    let yaw = to_goal.x.atan2(to_goal.z);
    player.rotation = Quat::from_rotation_y(yaw);

    // 直线移动 + 走路起伏
    let dir = Vec2::new(to_goal.x, to_goal.z).normalize();
    walk.bob += dt * 11.0;
    *step_acc += dt;
    if *step_acc > 0.38 {
        *step_acc = 0.0;
        super::sfx::play(&mut commands, &bank, super::sfx::Sfx::Step);
    }
    let bob_y = walk.bob.sin().abs() * 0.12;
    // 共享单车骑行：速度按倍率提升
    let speed = if bike.0 {
        PLAYER_SPEED * BIKE_SPEED_MULT
    } else {
        PLAYER_SPEED
    };
    let step = speed * dt;
    let mv = if step >= dist_xz {
        to_goal
    } else {
        Vec3::new(dir.x * step, 0.0, dir.y * step)
    };
    player.translation.x += mv.x;
    player.translation.z += mv.z;
    player.translation.y = bob_y;

    // 碰撞推出：贴着墙滑动，不穿建筑
    super::collision::resolve(&mut player.translation, &map.boxes);

    // 动态车辆推挤：玩家不能直接穿过行驶中的车（车也会让行，见 traffic::vehicle_tick）
    let car_radius = 1.0 + super::collision::PLAYER_RADIUS;
    for v in &vehicles {
        let d = player.translation.xz() - v.translation.xz();
        let dist = d.length();
        if dist < car_radius && dist > 1e-5 {
            let push = d / dist * (car_radius - dist);
            player.translation.x += push.x;
            player.translation.z += push.y;
        }
    }
}

// ==================== 程序化行走动画 ====================
// 独立系统（不与主角 Transform 同系统，避免查询冲突）：
// 行走时摆臂摆腿（相位相反）+ 身体微倾，静止时归零。
pub fn animate_limbs(walk: Res<WalkState>, mut limbs: Query<(&mut Transform, &LimbKind)>) {
    let swing = walk.bob.sin();
    for (mut t, kind) in &mut limbs {
        let (rx, rz) = match kind {
            LimbKind::ArmL => (swing * 0.55, 0.0),
            LimbKind::ArmR => (-swing * 0.55, 0.0),
            LimbKind::LegL => (-swing * 0.5, 0.0),
            LimbKind::LegR => (swing * 0.5, 0.0),
            LimbKind::Body => (-swing.abs() * 0.05, swing * 0.03),
        };
        t.rotation = Quat::from_rotation_x(rx) * Quat::from_rotation_z(rz);
    }
}

// 找当前场景里最近的觅食热点
fn nearest_food(
    hotspots: Query<(&Hotspot, &Transform), Without<PlayerRoot>>,
    from: Vec3,
) -> Option<HotspotKind> {
    let mut best: Option<(f32, HotspotKind)> = None;
    for (hp, tf) in &hotspots {
        if hp.kind.is_food() {
            let d = tf.translation.distance(from);
            if best.is_none_or(|(nd, _)| d < nd) {
                best = Some((d, hp.kind));
            }
        }
    }
    best.map(|(_, k)| k)
}

// ==================== 45° 俯视相机 ====================
// 相机固定在主角「上方 + 后方」，俯角 45°，形成 2.5D 斜俯视视角。
// 通勤场景用更近更平视的角度，让车厢/街道充满画面。
pub fn camera_follow(
    player: Single<&Transform, With<PlayerRoot>>,
    mut camera: Single<&mut Transform, (With<Camera3d>, Without<PlayerRoot>)>,
) {
    let target = player.translation;
    camera.translation = target + Vec3::new(0.0, 15.0, 15.0);
    camera.look_at(target + Vec3::Y * 0.5, Vec3::Y);
}

// ==================== 自主行为（觅食 / 补觉） ====================
// 饱食 < 30 → 冒气泡「肚子饿了…」自动走向最近觅食点；
// 精力 < 28 → 自动走向床补觉（不在家则提示回家）。
#[allow(clippy::too_many_arguments)]
pub fn auto_behaviors(
    time: Res<Time>,
    stats: Res<PlayerStats>,
    location: Res<GameLocation>,
    modal: Res<Modal>,
    cinematic: Res<Cinematic>,
    hotspots: Query<(&Hotspot, &Transform)>,
    player: Single<&Transform, With<PlayerRoot>>,
    transit: Res<TransitState>,
    mut walk: ResMut<WalkState>,
    mut toast: ResMut<ToastLog>,
    mut cooldown: ResMut<AutoCooldown>,
) {
    if modal.kind.is_some() || walk.target.is_some() || cinematic.active {
        return;
    }
    // 乘车行驶中不触发自主行为
    if transit.active {
        return;
    }
    let now = time.elapsed_secs();

    // 觅食
    if stats.satiety < 30.0 && now - cooldown.food > 8.0 {
        info!("[自动行为] 触发觅食（饱食 {:.0}）", stats.satiety);
        cooldown.food = now;
        // 找当前场景最近的觅食热点
        let mut best: Option<(f32, Vec3)> = None;
        for (hp, tf) in &hotspots {
            if hp.kind.is_food() {
                let d = tf.translation.distance(player.translation);
                if best.is_none_or(|(nd, _)| d < nd) {
                    best = Some((d, tf.translation));
                }
            }
        }
        if let Some((_, pos)) = best {
            toast.push("肚子饿了…");
            walk.target = Some(Vec3::new(pos.x, 0.0, pos.z));
            walk.cmd = WalkCmd::Food;
        }
        return;
    }

    // 补觉
    if stats.energy < 28.0 && now - cooldown.sleep > 12.0 {
        info!("[自动行为] 触发补觉（精力 {:.0}）", stats.energy);
        cooldown.sleep = now;
        if location.0 == Location::Home {
            toast.push("好困……去补个觉");
            // 家的床热点世界坐标 = 家的中心 + 床相对坐标（-6.0, 4.5）
            walk.target = Some(HOME_CENTER + Vec3::new(-6.0, 0.0, 4.5));
            walk.cmd = WalkCmd::Sleep;
        } else {
            toast.push("精力不足……先回家睡一觉吧");
        }
    }
}
