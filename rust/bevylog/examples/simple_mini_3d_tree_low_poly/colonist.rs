//! 殖民者 AI：每个角色的完整行动循环——
//! 需求（饥饿 → 回家吃饭）→ 住房/岗位分配 → 寻路移动 → 执行任务（采集 / 搬运 / 建造 / 劈柴）。
//! 任务与路径都通过 `navigation` 模块异步规划，避免每帧大量 A* 阻塞主线程。

use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    building::{
        Blueprint, BuildingEntrance, CompletedBuilding, Housing, Profession, Workplace,
        WorldGeometry,
    },
    math::xz_distance,
    navigation::{
        NavGrid, PathPlanner, PathRequestId, poll_path_planner, submit_path_planner, sync_nav_grid,
    },
    resources::{
        COLONIST_CARRY_CAPACITY, HOME_FOOD_PER_RESIDENT, Inventory, PublicInventory, carried_amount,
    },
    simulation::SimulationClock,
    terrain::{TerrainSeed, terrain_height},
    types::{BuildingKind, ResourceKind},
    world::{GameAssets, ResourceNode},
};

const GATHER_SECONDS: f32 = 1.4;
const WOOD_SPLIT_SECONDS: f32 = 2.0;
const WOOD_SPLIT_INPUT: i32 = 1;
const WOOD_SPLIT_OUTPUT: i32 = 5;
const BUILD_RATE: f32 = 1.0;
const COLONIST_HALF_HEIGHT: f32 = 0.32;
const SATIETY_LOSS_SECONDS: f32 = 10.0;
const HUNGER_THRESHOLD: f32 = 50.0;
const EAT_RESTORE: f32 = 50.0;
const EAT_REST_SECONDS: f32 = 2.0;
const PATH_FAILURE_RETRY_SECONDS: f32 = 0.2;
const GATHER_PATH_CANDIDATE_LIMIT: usize = 12;

pub struct ColonistPlugin;

impl Plugin for ColonistPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NavGrid>()
            .init_resource::<PathPlanner>()
            .init_resource::<GoalState>()
            .init_resource::<PopulationGrowth>()
            .add_systems(
                Update,
                (
                    grow_population,
                    sync_nav_grid,
                    poll_path_planner,
                    tick_colonist_needs,
                    assign_housing,
                    assign_workplaces,
                    assign_idle_colonists,
                    update_colonists,
                    check_goal,
                    submit_path_planner,
                )
                    .chain(),
            );
    }
}

// 简单游戏目标：人口达到 target 即达成（用调试台加殖民者、造房屋提供容量）。
// reached 表示已达成（不会回退），dismissed 表示玩家已关闭胜利提示。
#[derive(Resource)]
pub struct GoalState {
    pub target: i32,
    pub reached: bool,
    pub dismissed: bool,
}

impl Default for GoalState {
    fn default() -> Self {
        Self {
            target: 10,
            reached: false,
            dismissed: false,
        }
    }
}

// 人口达到目标后点亮胜利标记，UI 据此显示胜利提示。
pub fn check_goal(colonists: Query<&Colonist>, mut goal: ResMut<GoalState>) {
    if !goal.reached && colonists.iter().count() as i32 >= goal.target {
        info!("[目标] 人口达到 {}，目标达成！", goal.target);
        goal.reached = true;
        goal.dismissed = false;
    }
}

// 自动人口增长：每隔 GROWTH_INTERVAL 秒，若公共仓库食物充足且有空房，
// 就在该房屋附近出生一名新定居者（人口上限由住房容量决定，食物是成本）。
const GROWTH_INTERVAL: f32 = 10.0;
const GROWTH_FOOD_COST: i32 = 5;

#[derive(Resource, Default)]
struct PopulationGrowth {
    timer: f32,
}

fn grow_population(
    mut commands: Commands,
    time: Res<Time>,
    clock: Res<SimulationClock>,
    terrain_seed: Res<TerrainSeed>,
    assets: Res<GameAssets>,
    mut growth: ResMut<PopulationGrowth>,
    mut public_inventories: Query<&mut Inventory, With<PublicInventory>>,
    homes: Query<(Entity, &Housing, &Transform), With<CompletedBuilding>>,
    colonists: Query<&Colonist>,
) {
    growth.timer += clock.scaled_delta(&time);
    if growth.timer < GROWTH_INTERVAL {
        return;
    }
    growth.timer = 0.0;

    let available_food: i32 = public_inventories
        .iter()
        .map(|inventory| inventory.amount(ResourceKind::Food))
        .sum();
    if available_food < GROWTH_FOOD_COST {
        info!("[人口增长] 跳过：公共食物 {available_food} 不足 {GROWTH_FOOD_COST}，本次无新生人口");
        return;
    }

    // 找一个有空位的房屋，新殖民者从它附近出生。
    let Some((home, housing, transform)) = homes.iter().find(|(_, housing, _)| housing.has_space())
    else {
        info!("[人口增长] 跳过：没有任何有空位的房屋（人口上限受住房容量约束）");
        return;
    };

    // 从公共仓库扣除「生育成本」食物。
    let mut remaining = GROWTH_FOOD_COST;
    for mut inventory in &mut public_inventories {
        if remaining <= 0 {
            break;
        }
        let removed = inventory.amount(ResourceKind::Food).min(remaining);
        if inventory.remove(ResourceKind::Food, removed) {
            remaining -= removed;
        }
    }
    if remaining > 0 {
        warn!("[人口增长] 扣食物失败（剩余 {remaining}），取消本次出生");
        return;
    }

    let count = colonists.iter().count() as u32;
    let x = transform.translation.x + 1.0;
    let z = transform.translation.z + 1.0;
    let y = terrain_height(terrain_seed.0, x, z) + 0.32;
    info!(
        "[人口增长] 出生定居者 {}（家候选 {home:?}，已住 {}/{}），出生点 ({x:.1}, {y:.1}, {z:.1})，扣除食物 {}",
        count + 1,
        housing.resident_count(),
        Housing::CAPACITY,
        GROWTH_FOOD_COST
    );
    // 出生后无需指定家 / 岗位：assign_housing / assign_workplaces 下一帧会自动分配。
    commands.spawn((
        Mesh3d(assets.colonist_mesh.clone()),
        MeshMaterial3d(assets.colonist_material.clone()),
        Transform::from_translation(Vec3::new(x, y, z)),
        Colonist {
            name: format!("定居者 {}", count + 1),
            state: ColonistState::Idle,
            profession: Profession::Unemployed,
            workplace: None,
            speed: 2.2,
            home: None,
            satiety: 100.0,
            carry_capacity: COLONIST_CARRY_CAPACITY,
        },
    ));
}

#[derive(Component, Debug)]
pub struct Colonist {
    pub name: String,
    pub state: ColonistState,
    pub profession: Profession,
    pub workplace: Option<Entity>,
    pub speed: f32,
    pub home: Option<Entity>,
    pub satiety: f32,
    pub carry_capacity: i32,
}

impl Colonist {
    pub fn status_label(&self) -> String {
        self.state.label()
    }

    pub fn is_hungry(&self) -> bool {
        self.satiety < HUNGER_THRESHOLD
    }
}

#[derive(Debug, Clone)]
pub enum ColonistState {
    Idle,
    Moving {
        target: Vec3,
        path: Vec<Vec3>,
        task: Task,
        nav_revision: u64,
    },
    PlanningPath {
        request_id: PathRequestId,
        target: Vec3,
        task: Task,
    },
    Gathering {
        resource: Entity,
        kind: ResourceKind,
        amount: i32,
        timer: f32,
    },
    SplittingWood {
        workplace: Entity,
        output_amount: i32,
        timer: f32,
    },
    Building {
        blueprint: Entity,
    },
    Eating {
        timer: f32,
    },
    WaitingForPathRetry {
        timer: f32,
    },
}

// 单个任务：描述殖民者「要去做的一件事」。寻路状态（Moving/PlanningPath）
// 会携带当前任务，走到目标后由 complete_movement_task 按任务类型收尾。
#[derive(Debug, Clone)]
pub enum Task {
    PickupMaterial {
        source: Entity,
        blueprint: Entity,
        kind: ResourceKind,
        amount: i32,
        delivery_target: Vec3,
    },
    DeliverMaterial {
        blueprint: Entity,
        kind: ResourceKind,
        amount: i32,
    },
    Build {
        blueprint: Entity,
    },
    Gather {
        resource: Entity,
        kind: ResourceKind,
        amount: i32,
    },
    PickupWoodForSplitting {
        source: Entity,
        workplace: Entity,
        amount: i32,
        work_target: Vec3,
    },
    StartWoodSplitting {
        workplace: Entity,
        input_amount: i32,
    },
    Deposit {
        inventory: Entity,
        kind: ResourceKind,
        amount: i32,
    },
    Eat {
        inventory: Entity,
    },
    PickupHomeFood {
        source: Entity,
        home: Entity,
        amount: i32,
    },
    DeliverHomeFood {
        home: Entity,
        amount: i32,
    },
}

impl ColonistState {
    pub fn label(&self) -> String {
        match self {
            Self::Idle => "空闲".to_string(),
            Self::Moving { task, .. } => format!("移动中：{}", task.label()),
            Self::PlanningPath { task, .. } => format!("规划路径：{}", task.label()),
            Self::Gathering { kind, .. } => format!("采集{}", kind.label()),
            Self::SplittingWood { .. } => "劈柴中".to_string(),
            Self::Building { .. } => "正在建造".to_string(),
            Self::Eating { .. } => "进食中".to_string(),
            Self::WaitingForPathRetry { .. } => "等待路径".to_string(),
        }
    }
}

impl Task {
    pub fn label(&self) -> String {
        match self {
            Self::PickupMaterial { kind, amount, .. } => {
                format!("搬运 {} {}", amount, kind.label())
            }
            Self::DeliverMaterial { kind, amount, .. } => {
                format!("运送 {} {}", amount, kind.label())
            }
            Self::Build { .. } => "前往建造".to_string(),
            Self::Gather { kind, .. } => format!("前往采集{}", kind.label()),
            Self::PickupWoodForSplitting { amount, .. } => {
                format!("搬运 {} 木材去劈柴", amount)
            }
            Self::StartWoodSplitting { .. } => "前往劈柴".to_string(),
            Self::Deposit { kind, amount, .. } => {
                format!("存放 {} {}", amount, kind.label())
            }
            Self::Eat { .. } => "前去进食".to_string(),
            Self::PickupHomeFood { amount, .. } => format!("为家里取 {} 份食物", amount),
            Self::DeliverHomeFood { amount, .. } => {
                format!("送 {} 份食物回家", amount)
            }
        }
    }
}

// 需求系统：饱食度随时间衰减；饿到 0 的殖民者会被移出住房并移除。
pub fn tick_colonist_needs(
    mut commands: Commands,
    time: Res<Time>,
    clock: Res<SimulationClock>,
    mut colonists: Query<(Entity, &mut Colonist)>,
    mut homes: Query<&mut Housing>,
) {
    let dt = clock.scaled_delta(&time);
    if dt == 0.0 {
        return;
    }

    for (entity, mut colonist) in &mut colonists {
        colonist.satiety = decayed_satiety(colonist.satiety, dt);
        if colonist.satiety > 0.0 {
            continue;
        }

        // 饿死：从住房移除并消失（人口下降的唯一来源，配合增长日志排查人口曲线）。
        warn!(
            "[生存] 定居者 {} 饱食度归零，饿死并从地图移除",
            colonist.name
        );
        if let Some(home) = colonist.home.take() {
            if let Ok(mut housing) = homes.get_mut(home) {
                housing.remove_resident(entity);
            }
        }
        commands.entity(entity).despawn();
    }
}

// 住房分配：为没有家（或家已被拆）的殖民者，就近找一个未住满的房屋搬进去。
pub fn assign_housing(
    mut colonists: Query<(Entity, &mut Colonist, &Transform)>,
    mut homes: Query<(Entity, &mut Housing, &Transform, &CompletedBuilding), Without<Colonist>>,
) {
    let mut snapshots: Vec<(Entity, usize, Vec3)> = homes
        .iter()
        .filter(|(_, _, _, building)| building.kind == BuildingKind::House)
        .map(|(entity, housing, transform, _)| {
            (entity, housing.resident_count(), transform.translation)
        })
        .collect();

    for (colonist_entity, mut colonist, transform) in &mut colonists {
        if colonist.satiety <= 0.0 {
            continue;
        }

        if let Some(home) = colonist.home {
            if snapshots.iter().any(|(entity, _, _)| *entity == home) {
                continue;
            }
            // 家已被拆除：清除居住关系，下一轮再尝试分配新房。
            warn!(
                "[住房分配] 定居者 {} 的家 {home:?} 已不存在，清除居住关系",
                colonist.name
            );
            colonist.home = None;
        }

        let Some(index) = best_home_candidate(&snapshots, transform.translation) else {
            // 没有可分配的空房（debug 级别：系统每帧运行，避免刷屏）。
            debug!("[住房分配] 定居者 {} 无家且没有可用的空房", colonist.name);
            continue;
        };
        let home = snapshots[index].0;

        if let Ok((_, mut housing, _, _)) = homes.get_mut(home) {
            if housing.add_resident(colonist_entity) {
                colonist.home = Some(home);
                snapshots[index].1 += 1;
                info!(
                    "[住房分配] 定居者 {} 入住 {home:?}（当前居民 {}/{}）",
                    colonist.name,
                    snapshots[index].1,
                    Housing::CAPACITY
                );
            } else {
                warn!(
                    "[住房分配] 定居者 {} 申请入住 {home:?} 失败（房屋已满）",
                    colonist.name
                );
            }
        }
    }
}

// 岗位分配：
// 1. 先校验已有工作是否仍然有效（岗位还在、职业匹配、没超员），否则把人置为失业；
// 2. 再让失业者就近申请有空位的岗位（每个岗位最多 desired_slots 人，先到先得）。
pub fn assign_workplaces(
    mut colonists: Query<(Entity, &mut Colonist, &Transform)>,
    mut workplaces: Query<(Entity, &mut Workplace, &Transform), Without<Colonist>>,
) {
    let snapshots: Vec<WorkplaceSnapshot> = workplaces
        .iter_mut()
        .map(|(entity, mut workplace, transform)| {
            workplace.clamp_desired_slots();
            WorkplaceSnapshot {
                entity,
                profession: workplace.profession,
                desired_slots: workplace.desired_slots,
                position: transform.translation,
            }
        })
        .collect();

    let mut assigned_counts: HashMap<Entity, u8> = HashMap::new();
    for (_, mut colonist, _) in &mut colonists {
        let Some(workplace_entity) = colonist.workplace else {
            colonist.profession = Profession::Unemployed;
            continue;
        };

        let Some(snapshot) = snapshots
            .iter()
            .find(|snapshot| snapshot.entity == workplace_entity)
        else {
            // 岗位建筑被拆除：把人置为失业，等待重新分配。
            warn!(
                "[岗位分配] 定居者 {} 的岗位 {workplace_entity:?} 已不存在，转为失业",
                colonist.name
            );
            colonist.profession = Profession::Unemployed;
            colonist.workplace = None;
            continue;
        };

        let count = assigned_counts.entry(workplace_entity).or_insert(0);
        if colonist.profession != snapshot.profession || *count >= snapshot.desired_slots {
            // 职业与岗位不匹配，或岗位已超员：被挤出岗位。
            warn!(
                "[岗位分配] 定居者 {} 被挤出岗位 {workplace_entity:?}（职业不匹配或超员），转为失业",
                colonist.name
            );
            colonist.profession = Profession::Unemployed;
            colonist.workplace = None;
        } else {
            *count += 1;
        }
    }

    for (_, mut colonist, transform) in &mut colonists {
        if colonist.profession != Profession::Unemployed || colonist.workplace.is_some() {
            continue;
        }

        let Some(snapshot) =
            best_workplace_candidate(&snapshots, &assigned_counts, transform.translation)
        else {
            // 没有可申请的岗位（debug 级别：系统每帧运行）。
            debug!(
                "[岗位分配] 定居者 {} 处于失业且没有可用的岗位",
                colonist.name
            );
            continue;
        };

        colonist.profession = snapshot.profession;
        colonist.workplace = Some(snapshot.entity);
        *assigned_counts.entry(snapshot.entity).or_insert(0) += 1;
        info!(
            "[岗位分配] 定居者 {} 入职 {}（岗位 {:?}，名额 {}）",
            colonist.name,
            snapshot.profession.label(),
            snapshot.entity,
            snapshot.desired_slots
        );
    }
}

#[derive(Clone, Copy, Debug)]
struct WorkplaceSnapshot {
    entity: Entity,
    profession: Profession,
    desired_slots: u8,
    position: Vec3,
}

fn best_workplace_candidate(
    workplaces: &[WorkplaceSnapshot],
    assigned_counts: &HashMap<Entity, u8>,
    start: Vec3,
) -> Option<WorkplaceSnapshot> {
    workplaces
        .iter()
        .copied()
        .filter(|snapshot| {
            assigned_counts.get(&snapshot.entity).copied().unwrap_or(0) < snapshot.desired_slots
        })
        .min_by(|left, right| {
            xz_distance(start, left.position).total_cmp(&xz_distance(start, right.position))
        })
}

// 空闲殖民者决策：按优先级给自己派活，每帧只接一个任务——
// 饿了先回家吃饭 → 有职业就去干活（采集 / 劈柴）→ 给蓝图送材料 → 去建造 → 往家里囤粮。
// AssignmentReservations 记录已派出的任务，避免多名殖民者抢同一个资源点 / 同一份材料。
pub fn assign_idle_colonists(
    nav_grid: Res<NavGrid>,
    terrain_seed: Res<TerrainSeed>,
    mut planner: ResMut<PathPlanner>,
    mut colonists: Query<(&mut Colonist, &Transform)>,
    blueprints: Query<(Entity, &Blueprint, &Transform, Option<&BuildingEntrance>)>,
    resources: Query<(Entity, &ResourceNode, &Transform)>,
    workplaces: Query<(Entity, &Workplace, &Transform, Option<&BuildingEntrance>)>,
    inventories: Query<
        (
            Entity,
            &Inventory,
            &Transform,
            Option<&BuildingEntrance>,
            Option<&PublicInventory>,
            Option<&Housing>,
            Option<&CompletedBuilding>,
        ),
        Without<Colonist>,
    >,
) {
    let seed = terrain_seed.0;
    let mut reservations = AssignmentReservations::from_colonists(&colonists);

    for (mut colonist, transform) in &mut colonists {
        if colonist.satiety <= 0.0 {
            continue;
        }

        if !matches!(colonist.state, ColonistState::Idle) {
            continue;
        }

        let start = transform.translation;

        if colonist.is_hungry() {
            if let Some(state) = eat_candidate(
                &nav_grid,
                &mut planner,
                start,
                &colonist,
                &inventories,
                seed,
            ) {
                colonist.state = state;
                continue;
            }
        }

        if let Some((reservation, state)) = profession_task_candidate(
            &nav_grid,
            &mut planner,
            start,
            &colonist,
            &resources,
            &workplaces,
            &inventories,
            &reservations,
            seed,
        ) {
            reservations.reserve_production_assignment(reservation);
            colonist.state = state;
            continue;
        }

        let mut assigned = false;
        if let Some((blueprint, amount, state)) = material_delivery_candidate(
            &nav_grid,
            &mut planner,
            start,
            &colonist,
            &blueprints,
            &inventories,
            &reservations,
            seed,
        ) {
            reservations.reserve_material_delivery(blueprint, amount);
            colonist.state = state;
            assigned = true;
        }
        if assigned {
            continue;
        }

        if let Some(state) = build_candidate(&nav_grid, &mut planner, start, &blueprints, seed) {
            colonist.state = state;
            continue;
        }

        if let Some(state) = home_restock_candidate(
            &nav_grid,
            &mut planner,
            start,
            &colonist,
            &inventories,
            seed,
        ) {
            colonist.state = state;
            continue;
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum ProductionReservation {
    Gather {
        resource: Entity,
        kind: ResourceKind,
        amount: i32,
    },
    SplitWood {
        source: Entity,
        input_amount: i32,
        output_amount: i32,
    },
}

#[derive(Default)]
struct AssignmentReservations {
    material_deliveries: Vec<(Entity, i32)>,
    gathered_resources: Vec<(Entity, i32)>,
    inventory_withdrawals: Vec<(Entity, ResourceKind, i32)>,
    public_deposits: Vec<(ResourceKind, i32)>,
}

impl AssignmentReservations {
    fn from_colonists(colonists: &Query<(&mut Colonist, &Transform)>) -> Self {
        let mut reservations = Self::default();
        for (colonist, _) in colonists.iter() {
            reservations.reserve_state(&colonist.state);
        }
        reservations
    }

    fn reserve_state(&mut self, state: &ColonistState) {
        match state {
            ColonistState::Moving { task, .. } => self.reserve_task(task),
            ColonistState::PlanningPath { task, .. } => self.reserve_task(task),
            ColonistState::Gathering {
                resource,
                kind,
                amount,
                ..
            } => {
                self.reserve_gather(*resource, *kind, *amount);
            }
            ColonistState::SplittingWood { output_amount, .. } => {
                self.reserve_public_deposit(ResourceKind::Firewood, *output_amount);
            }
            ColonistState::Idle
            | ColonistState::Building { .. }
            | ColonistState::Eating { .. }
            | ColonistState::WaitingForPathRetry { .. } => {}
        }
    }

    fn reserve_task(&mut self, task: &Task) {
        match task {
            Task::PickupMaterial {
                blueprint,
                kind: ResourceKind::Wood,
                amount,
                ..
            }
            | Task::DeliverMaterial {
                blueprint,
                kind: ResourceKind::Wood,
                amount,
            } => self.reserve_material_delivery(*blueprint, *amount),
            Task::Gather {
                resource,
                kind,
                amount,
            } => self.reserve_gather(*resource, *kind, *amount),
            Task::PickupWoodForSplitting { source, amount, .. } => {
                self.reserve_inventory_withdrawal(*source, ResourceKind::Wood, *amount);
                self.reserve_public_deposit(ResourceKind::Firewood, *amount * WOOD_SPLIT_OUTPUT);
            }
            Task::StartWoodSplitting { input_amount, .. } => {
                self.reserve_public_deposit(
                    ResourceKind::Firewood,
                    *input_amount * WOOD_SPLIT_OUTPUT,
                );
            }
            Task::Deposit { kind, amount, .. } => self.reserve_public_deposit(*kind, *amount),
            Task::PickupMaterial { .. }
            | Task::DeliverMaterial { .. }
            | Task::Build { .. }
            | Task::Eat { .. }
            | Task::PickupHomeFood { .. }
            | Task::DeliverHomeFood { .. } => {}
        }
    }

    fn reserve_material_delivery(&mut self, blueprint: Entity, amount: i32) {
        self.material_deliveries.push((blueprint, amount.max(0)));
    }

    fn reserved_material_delivery(&self, blueprint: Entity) -> i32 {
        self.material_deliveries
            .iter()
            .filter(|(entity, _)| *entity == blueprint)
            .map(|(_, amount)| *amount)
            .sum()
    }

    fn reserve_gather(&mut self, resource: Entity, kind: ResourceKind, amount: i32) {
        let amount = amount.max(0);
        self.gathered_resources.push((resource, amount));
        self.reserve_public_deposit(kind, amount);
    }

    fn reserved_gather(&self, resource: Entity) -> i32 {
        self.gathered_resources
            .iter()
            .filter(|(entity, _)| *entity == resource)
            .map(|(_, amount)| *amount)
            .sum()
    }

    fn reserve_public_deposit(&mut self, kind: ResourceKind, amount: i32) {
        self.public_deposits.push((kind, amount.max(0)));
    }

    fn reserve_inventory_withdrawal(&mut self, inventory: Entity, kind: ResourceKind, amount: i32) {
        self.inventory_withdrawals
            .push((inventory, kind, amount.max(0)));
    }

    fn reserved_inventory_withdrawal(&self, inventory: Entity, kind: ResourceKind) -> i32 {
        self.inventory_withdrawals
            .iter()
            .filter(|(entity, resource_kind, _)| *entity == inventory && *resource_kind == kind)
            .map(|(_, _, amount)| *amount)
            .sum()
    }

    fn reserve_production_assignment(&mut self, reservation: ProductionReservation) {
        match reservation {
            ProductionReservation::Gather {
                resource,
                kind,
                amount,
            } => self.reserve_gather(resource, kind, amount),
            ProductionReservation::SplitWood {
                source,
                input_amount,
                output_amount,
            } => {
                self.reserve_inventory_withdrawal(source, ResourceKind::Wood, input_amount);
                self.reserve_public_deposit(ResourceKind::Firewood, output_amount);
            }
        }
    }

    fn reserved_public_capacity(&self) -> i32 {
        self.public_deposits
            .iter()
            .map(|(kind, amount)| *amount * kind.unit_size())
            .sum()
    }
}

fn profession_task_candidate(
    nav_grid: &NavGrid,
    planner: &mut PathPlanner,
    start: Vec3,
    colonist: &Colonist,
    resources: &Query<(Entity, &ResourceNode, &Transform)>,
    workplaces: &Query<(Entity, &Workplace, &Transform, Option<&BuildingEntrance>)>,
    inventories: &Query<
        (
            Entity,
            &Inventory,
            &Transform,
            Option<&BuildingEntrance>,
            Option<&PublicInventory>,
            Option<&Housing>,
            Option<&CompletedBuilding>,
        ),
        Without<Colonist>,
    >,
    reservations: &AssignmentReservations,
    seed: u64,
) -> Option<(ProductionReservation, ColonistState)> {
    if colonist.profession != Profession::Unemployed && colonist.workplace.is_none() {
        return None;
    }

    match colonist.profession {
        Profession::Lumberjack => {
            let (_, resource, amount, state) = gather_candidate(
                nav_grid,
                planner,
                start,
                ResourceKind::Wood,
                resources,
                inventories,
                reservations,
                colonist.carry_capacity,
                seed,
            )?;
            Some((
                ProductionReservation::Gather {
                    resource,
                    kind: ResourceKind::Wood,
                    amount,
                },
                state,
            ))
        }
        Profession::Gatherer => {
            let (_, resource, amount, state) = gather_candidate(
                nav_grid,
                planner,
                start,
                ResourceKind::Food,
                resources,
                inventories,
                reservations,
                colonist.carry_capacity,
                seed,
            )?;
            Some((
                ProductionReservation::Gather {
                    resource,
                    kind: ResourceKind::Food,
                    amount,
                },
                state,
            ))
        }
        Profession::WoodSplitter => wood_splitting_candidate(
            nav_grid,
            planner,
            start,
            colonist,
            workplaces,
            inventories,
            reservations,
            seed,
        ),
        Profession::Unemployed => None,
    }
}

fn wood_splitting_candidate(
    nav_grid: &NavGrid,
    planner: &mut PathPlanner,
    start: Vec3,
    colonist: &Colonist,
    workplaces: &Query<(Entity, &Workplace, &Transform, Option<&BuildingEntrance>)>,
    inventories: &Query<
        (
            Entity,
            &Inventory,
            &Transform,
            Option<&BuildingEntrance>,
            Option<&PublicInventory>,
            Option<&Housing>,
            Option<&CompletedBuilding>,
        ),
        Without<Colonist>,
    >,
    reservations: &AssignmentReservations,
    seed: u64,
) -> Option<(ProductionReservation, ColonistState)> {
    let workplace_entity = colonist.workplace?;
    let (_, workplace, workplace_transform, workplace_entrance) =
        workplaces.get(workplace_entity).ok()?;
    if workplace.profession != Profession::WoodSplitter {
        return None;
    }

    let firewood_capacity =
        total_public_addable_after_reserved(inventories, reservations, ResourceKind::Firewood);
    if firewood_capacity < WOOD_SPLIT_OUTPUT {
        return None;
    }

    let work_target = building_interaction_position(workplace_transform, workplace_entrance);
    let mut candidates: Vec<(f32, Entity, Vec3)> = inventories
        .iter()
        .filter_map(|(source, inventory, transform, entrance, public, _, _)| {
            if public.is_none() {
                return None;
            }
            let available = inventory.amount(ResourceKind::Wood)
                - reservations.reserved_inventory_withdrawal(source, ResourceKind::Wood);
            if available < WOOD_SPLIT_INPUT {
                return None;
            }

            let source_target = building_interaction_position(transform, entrance);
            Some((
                xz_distance(start, source_target) + xz_distance(source_target, work_target),
                source,
                source_target,
            ))
        })
        .collect();
    candidates.sort_by(|(dist_a, ..), (dist_b, ..)| dist_a.total_cmp(dist_b));

    for (_, source, source_target) in candidates {
        let task = Task::PickupWoodForSplitting {
            source,
            workplace: workplace_entity,
            amount: WOOD_SPLIT_INPUT,
            work_target,
        };
        if let Some((state, _)) =
            moving_state_to_target(nav_grid, planner, start, source_target, task, seed)
        {
            return Some((
                ProductionReservation::SplitWood {
                    source,
                    input_amount: WOOD_SPLIT_INPUT,
                    output_amount: WOOD_SPLIT_OUTPUT,
                },
                state,
            ));
        }
    }

    None
}

fn eat_candidate(
    nav_grid: &NavGrid,
    planner: &mut PathPlanner,
    start: Vec3,
    colonist: &Colonist,
    inventories: &Query<
        (
            Entity,
            &Inventory,
            &Transform,
            Option<&BuildingEntrance>,
            Option<&PublicInventory>,
            Option<&Housing>,
            Option<&CompletedBuilding>,
        ),
        Without<Colonist>,
    >,
    seed: u64,
) -> Option<ColonistState> {
    if let Some(home) = colonist.home {
        if let Ok((_, inventory, transform, entrance, _, _, _)) = inventories.get(home) {
            if inventory.amount(ResourceKind::Food) > 0 {
                let target = building_interaction_position(transform, entrance);
                if let Some((state, _)) = moving_state_to_target(
                    nav_grid,
                    planner,
                    start,
                    target,
                    Task::Eat { inventory: home },
                    seed,
                ) {
                    return Some(state);
                }
            }
        }
    }

    let mut candidates: Vec<(f32, Entity, Vec3)> = inventories
        .iter()
        .filter(|(_, inventory, _, _, public, _, _)| {
            public.is_some() && inventory.amount(ResourceKind::Food) > 0
        })
        .map(|(entity, _, transform, entrance, _, _, _)| {
            let target = building_interaction_position(transform, entrance);
            (xz_distance(start, target), entity, target)
        })
        .collect();
    candidates.sort_by(|(dist_a, ..), (dist_b, ..)| dist_a.total_cmp(dist_b));

    for (_, inventory, target) in candidates {
        if let Some((state, _)) = moving_state_to_target(
            nav_grid,
            planner,
            start,
            target,
            Task::Eat { inventory },
            seed,
        ) {
            return Some(state);
        }
    }

    None
}

fn material_delivery_candidate(
    nav_grid: &NavGrid,
    planner: &mut PathPlanner,
    start: Vec3,
    colonist: &Colonist,
    blueprints: &Query<(Entity, &Blueprint, &Transform, Option<&BuildingEntrance>)>,
    inventories: &Query<
        (
            Entity,
            &Inventory,
            &Transform,
            Option<&BuildingEntrance>,
            Option<&PublicInventory>,
            Option<&Housing>,
            Option<&CompletedBuilding>,
        ),
        Without<Colonist>,
    >,
    reservations: &AssignmentReservations,
    seed: u64,
) -> Option<(Entity, i32, ColonistState)> {
    let mut candidates: Vec<(i32, f32, Entity, i32, Vec3, Task)> = Vec::new();

    for (blueprint_entity, blueprint, blueprint_transform, blueprint_entrance) in blueprints {
        let reserved = reservations.reserved_material_delivery(blueprint_entity);
        let remaining = blueprint.needs_wood() - reserved;
        if remaining <= 0 {
            continue;
        }

        let amount = carried_amount(ResourceKind::Wood, colonist.carry_capacity).min(remaining);
        let blueprint_target =
            building_interaction_position(blueprint_transform, blueprint_entrance);

        for (source, inventory, source_transform, source_entrance, public, _, _) in inventories {
            if public.is_none() || inventory.amount(ResourceKind::Wood) < amount {
                continue;
            }

            let source_target = building_interaction_position(source_transform, source_entrance);
            let distance =
                xz_distance(start, source_target) + xz_distance(source_target, blueprint_target);
            candidates.push((
                remaining,
                distance,
                blueprint_entity,
                amount,
                source_target,
                Task::PickupMaterial {
                    source,
                    blueprint: blueprint_entity,
                    kind: ResourceKind::Wood,
                    amount,
                    delivery_target: blueprint_target,
                },
            ));
        }
    }

    candidates.sort_by(|(rem_a, dist_a, ..), (rem_b, dist_b, ..)| {
        rem_b.cmp(rem_a).then_with(|| dist_a.total_cmp(dist_b))
    });

    for (_, _, blueprint, amount, target, task) in candidates {
        if let Some((state, _)) =
            moving_state_to_target(nav_grid, planner, start, target, task, seed)
        {
            return Some((blueprint, amount, state));
        }
    }

    None
}

fn build_candidate(
    nav_grid: &NavGrid,
    planner: &mut PathPlanner,
    start: Vec3,
    blueprints: &Query<(Entity, &Blueprint, &Transform, Option<&BuildingEntrance>)>,
    seed: u64,
) -> Option<ColonistState> {
    let mut candidates: Vec<(f32, Vec3, Task)> = blueprints
        .iter()
        .filter(|(_, blueprint, _, _)| {
            blueprint.has_materials() && blueprint.progress < blueprint.build_seconds
        })
        .map(|(entity, _, blueprint_transform, entrance)| {
            let target = building_interaction_position(blueprint_transform, entrance);
            (
                xz_distance(start, target),
                target,
                Task::Build { blueprint: entity },
            )
        })
        .collect();

    candidates.sort_by(|(dist_a, ..), (dist_b, ..)| dist_a.total_cmp(dist_b));

    for (_, target, task) in candidates {
        if let Some((state, _)) =
            moving_state_to_target(nav_grid, planner, start, target, task, seed)
        {
            return Some(state);
        }
    }

    None
}

fn home_restock_candidate(
    nav_grid: &NavGrid,
    planner: &mut PathPlanner,
    start: Vec3,
    colonist: &Colonist,
    inventories: &Query<
        (
            Entity,
            &Inventory,
            &Transform,
            Option<&BuildingEntrance>,
            Option<&PublicInventory>,
            Option<&Housing>,
            Option<&CompletedBuilding>,
        ),
        Without<Colonist>,
    >,
    seed: u64,
) -> Option<ColonistState> {
    let home = colonist.home?;
    let Ok((_, home_inventory, _, _, _, home_housing, _)) = inventories.get(home) else {
        return None;
    };
    let housing = home_housing?;
    let target_food = (housing.resident_count() as i32 * HOME_FOOD_PER_RESIDENT).min(
        home_inventory.max_addable(ResourceKind::Food) + home_inventory.amount(ResourceKind::Food),
    );
    let need = (target_food - home_inventory.amount(ResourceKind::Food))
        .min(home_inventory.max_addable(ResourceKind::Food));
    if need <= 0 {
        return None;
    }

    let mut candidates: Vec<(f32, Entity, i32, Vec3)> = inventories
        .iter()
        .filter_map(|(source, inventory, transform, entrance, public, _, _)| {
            if public.is_none() {
                return None;
            }

            let amount = carried_amount(ResourceKind::Food, colonist.carry_capacity)
                .min(need)
                .min(inventory.amount(ResourceKind::Food));
            if amount <= 0 {
                return None;
            }

            let target = building_interaction_position(transform, entrance);
            Some((xz_distance(start, target), source, amount, target))
        })
        .collect();
    candidates.sort_by(|(dist_a, ..), (dist_b, ..)| dist_a.total_cmp(dist_b));

    for (_, source, amount, target) in candidates {
        let task = Task::PickupHomeFood {
            source,
            home,
            amount,
        };
        if let Some((state, _)) =
            moving_state_to_target(nav_grid, planner, start, target, task, seed)
        {
            return Some(state);
        }
    }

    None
}

fn gather_candidate(
    nav_grid: &NavGrid,
    planner: &mut PathPlanner,
    start: Vec3,
    kind: ResourceKind,
    resources: &Query<(Entity, &ResourceNode, &Transform)>,
    inventories: &Query<
        (
            Entity,
            &Inventory,
            &Transform,
            Option<&BuildingEntrance>,
            Option<&PublicInventory>,
            Option<&Housing>,
            Option<&CompletedBuilding>,
        ),
        Without<Colonist>,
    >,
    reservations: &AssignmentReservations,
    carry_capacity: i32,
    seed: u64,
) -> Option<(usize, Entity, i32, ColonistState)> {
    let public_capacity = total_public_addable_after_reserved(inventories, reservations, kind);
    if public_capacity <= 0 {
        return None;
    }

    let candidates = gather_candidate_snapshots(
        start,
        kind,
        carry_capacity,
        public_capacity,
        resources.iter().map(|(entity, node, transform)| {
            (entity, node.kind, node.amount, transform.translation)
        }),
        reservations,
    );

    for candidate in candidates.into_iter().take(GATHER_PATH_CANDIDATE_LIMIT) {
        if let Some(result) = movement_to_resource(
            nav_grid,
            planner,
            start,
            candidate.position,
            Task::Gather {
                resource: candidate.resource,
                kind,
                amount: candidate.amount,
            },
            seed,
        ) {
            return Some((result.0, candidate.resource, candidate.amount, result.1));
        }
    }
    None
}

#[derive(Clone, Copy, Debug)]
struct GatherCandidateSnapshot {
    distance: f32,
    resource: Entity,
    position: Vec3,
    amount: i32,
}

fn gather_candidate_snapshots(
    start: Vec3,
    kind: ResourceKind,
    carry_capacity: i32,
    public_capacity: i32,
    resources: impl IntoIterator<Item = (Entity, ResourceKind, i32, Vec3)>,
    reservations: &AssignmentReservations,
) -> Vec<GatherCandidateSnapshot> {
    let mut candidates: Vec<_> = resources
        .into_iter()
        .filter_map(|(resource, resource_kind, node_amount, position)| {
            if resource_kind != kind {
                return None;
            }

            let available = (node_amount - reservations.reserved_gather(resource)).max(0);
            let amount = carried_amount(kind, carry_capacity)
                .min(available)
                .min(public_capacity);
            if amount <= 0 {
                return None;
            }

            Some(GatherCandidateSnapshot {
                distance: xz_distance(start, position),
                resource,
                position,
                amount,
            })
        })
        .collect();
    candidates.sort_by(|left, right| left.distance.total_cmp(&right.distance));
    candidates
}

fn total_public_addable(
    inventories: &Query<
        (
            Entity,
            &Inventory,
            &Transform,
            Option<&BuildingEntrance>,
            Option<&PublicInventory>,
            Option<&Housing>,
            Option<&CompletedBuilding>,
        ),
        Without<Colonist>,
    >,
    kind: ResourceKind,
) -> i32 {
    inventories
        .iter()
        .filter(|(_, _, _, _, public, _, _)| public.is_some())
        .map(|(_, inventory, _, _, _, _, _)| inventory.max_addable(kind))
        .sum()
}

fn total_public_addable_after_reserved(
    inventories: &Query<
        (
            Entity,
            &Inventory,
            &Transform,
            Option<&BuildingEntrance>,
            Option<&PublicInventory>,
            Option<&Housing>,
            Option<&CompletedBuilding>,
        ),
        Without<Colonist>,
    >,
    reservations: &AssignmentReservations,
    kind: ResourceKind,
) -> i32 {
    let capacity = inventories
        .iter()
        .filter(|(_, _, _, _, public, _, _)| public.is_some())
        .map(|(_, inventory, _, _, _, _, _)| inventory.max_addable(kind) * kind.unit_size())
        .sum::<i32>();
    (capacity - reservations.reserved_public_capacity()).max(0) / kind.unit_size()
}

fn building_interaction_position(
    transform: &Transform,
    entrance: Option<&BuildingEntrance>,
) -> Vec3 {
    entrance
        .map(|entrance| entrance.world_position)
        .unwrap_or(transform.translation)
}

fn valid_workplace(
    workplaces: &Query<&Workplace, Without<Colonist>>,
    entity: Entity,
    profession: Profession,
) -> bool {
    workplaces
        .get(entity)
        .map(|workplace| workplace.profession == profession)
        .unwrap_or(false)
}

fn moving_state_to_target(
    nav_grid: &NavGrid,
    planner: &mut PathPlanner,
    start: Vec3,
    target: Vec3,
    task: Task,
    _seed: u64,
) -> Option<(ColonistState, usize)> {
    if nav_grid.endpoint_chunks_clean(start, target) && !nav_grid.maybe_reachable(start, target) {
        return None;
    }
    let request_id = planner.request_path(nav_grid, start, target);

    Some((
        ColonistState::PlanningPath {
            request_id,
            target,
            task,
        },
        0,
    ))
}

fn movement_to_resource(
    nav_grid: &NavGrid,
    planner: &mut PathPlanner,
    start: Vec3,
    resource_position: Vec3,
    task: Task,
    seed: u64,
) -> Option<(usize, ColonistState)> {
    let mut targets: Vec<(f32, Vec3)> = resource_interaction_targets(resource_position, seed)
        .into_iter()
        .map(|target| (xz_distance(start, target), target))
        .collect();

    targets.sort_by(|(dist_a, ..), (dist_b, ..)| dist_a.total_cmp(dist_b));

    for (_, target) in targets {
        if let Some((state, path_len)) =
            moving_state_to_target(nav_grid, planner, start, target, task.clone(), seed)
        {
            return Some((path_len, state));
        }
    }
    None
}

fn resource_interaction_targets(resource_position: Vec3, seed: u64) -> [Vec3; 8] {
    const RADIUS: f32 = 0.82;
    const DIAGONAL: f32 = RADIUS * 0.70710677;
    let base_x = resource_position.x;
    let base_z = resource_position.z;
    [
        Vec3::new(
            base_x + RADIUS,
            terrain_height(seed, base_x + RADIUS, base_z),
            base_z,
        ),
        Vec3::new(
            base_x - RADIUS,
            terrain_height(seed, base_x - RADIUS, base_z),
            base_z,
        ),
        Vec3::new(
            base_x,
            terrain_height(seed, base_x, base_z + RADIUS),
            base_z + RADIUS,
        ),
        Vec3::new(
            base_x,
            terrain_height(seed, base_x, base_z - RADIUS),
            base_z - RADIUS,
        ),
        Vec3::new(
            base_x + DIAGONAL,
            terrain_height(seed, base_x + DIAGONAL, base_z + DIAGONAL),
            base_z + DIAGONAL,
        ),
        Vec3::new(
            base_x - DIAGONAL,
            terrain_height(seed, base_x - DIAGONAL, base_z + DIAGONAL),
            base_z + DIAGONAL,
        ),
        Vec3::new(
            base_x + DIAGONAL,
            terrain_height(seed, base_x + DIAGONAL, base_z - DIAGONAL),
            base_z - DIAGONAL,
        ),
        Vec3::new(
            base_x - DIAGONAL,
            terrain_height(seed, base_x - DIAGONAL, base_z - DIAGONAL),
            base_z - DIAGONAL,
        ),
    ]
}

fn deposit_moving_state(
    nav_grid: &NavGrid,
    planner: &mut PathPlanner,
    start: Vec3,
    inventories: &Query<
        (
            Entity,
            &Inventory,
            &Transform,
            Option<&BuildingEntrance>,
            Option<&PublicInventory>,
            Option<&Housing>,
            Option<&CompletedBuilding>,
        ),
        Without<Colonist>,
    >,
    kind: ResourceKind,
    amount: i32,
    seed: u64,
    exclude: Option<Entity>,
) -> Option<ColonistState> {
    let mut candidates: Vec<(bool, f32, Entity, Vec3)> = inventories
        .iter()
        .filter(|(entity, inventory, _, _, public, _, _)| {
            public.is_some() && Some(*entity) != exclude && inventory.max_addable(kind) > 0
        })
        .map(|(entity, inventory, transform, entrance, _, _, _)| {
            let target = building_interaction_position(transform, entrance);
            (
                inventory.max_addable(kind) >= amount,
                xz_distance(start, target),
                entity,
                target,
            )
        })
        .collect();
    candidates.sort_by(|(full_a, dist_a, ..), (full_b, dist_b, ..)| {
        full_b.cmp(full_a).then_with(|| dist_a.total_cmp(dist_b))
    });

    for (_, _, inventory, target) in candidates {
        let task = Task::Deposit {
            inventory,
            kind,
            amount,
        };
        if let Some((state, _)) =
            moving_state_to_target(nav_grid, planner, start, target, task, seed)
        {
            return Some(state);
        }
    }

    None
}

// 殖民者状态机：每帧取出当前状态推进一步，再把新状态写回。
// - Idle：什么都不做（等 assign_idle_colonists 派活）
// - Moving / PlanningPath：寻路移动；中途世界变化会自动重规划，到达后交给
//   complete_movement_task 按任务类型收尾
// - Gathering / SplittingWood：原地计时干活，干完把产出送往公共仓库
// - Building：给蓝图累加建造进度（材料齐了才推进）
// - Eating / WaitingForPathRetry：短倒计时状态，结束后回到 Idle
pub fn update_colonists(
    mut commands: Commands,
    time: Res<Time>,
    clock: Res<SimulationClock>,
    terrain_seed: Res<TerrainSeed>,
    mut geometry: ResMut<WorldGeometry>,
    nav_grid: Res<NavGrid>,
    mut planner: ResMut<PathPlanner>,
    mut colonists: Query<(&mut Colonist, &mut Transform)>,
    mut blueprints: Query<(Entity, &mut Blueprint)>,
    mut resources: Query<&mut ResourceNode>,
    workplaces: Query<&Workplace, Without<Colonist>>,
    mut inventory_access: ParamSet<(
        Query<
            (
                Entity,
                &Inventory,
                &Transform,
                Option<&BuildingEntrance>,
                Option<&PublicInventory>,
                Option<&Housing>,
                Option<&CompletedBuilding>,
            ),
            Without<Colonist>,
        >,
        Query<&mut Inventory>,
    )>,
) {
    let seed = terrain_seed.0;
    let dt = clock.scaled_delta(&time);
    if dt == 0.0 {
        return;
    }

    for (mut colonist, mut transform) in &mut colonists {
        if colonist.satiety <= 0.0 {
            colonist.state = ColonistState::Idle;
            continue;
        }

        let current_state = std::mem::replace(&mut colonist.state, ColonistState::Idle);
        match current_state {
            ColonistState::Idle => {}
            ColonistState::Eating { mut timer } => {
                timer -= dt;
                if timer > 0.0 {
                    colonist.state = ColonistState::Eating { timer };
                } else {
                    colonist.state = ColonistState::Idle;
                }
            }
            ColonistState::WaitingForPathRetry { mut timer } => {
                timer -= dt;
                if timer > 0.0 {
                    colonist.state = ColonistState::WaitingForPathRetry { timer };
                } else {
                    colonist.state = ColonistState::Idle;
                }
            }
            ColonistState::Moving {
                target,
                mut path,
                task,
                nav_revision,
            } => {
                let task_valid = {
                    let inventories = inventory_access.p0();
                    moving_task_is_valid(&task, &mut resources, &workplaces, &inventories)
                };
                if !task_valid {
                    debug!(
                        "[殖民者] {} 任务失效（目标已消失），回到空闲",
                        colonist.name
                    );
                    colonist.state = ColonistState::Idle;
                    continue;
                }

                if nav_grid.path_needs_replan(&path, nav_revision) {
                    colonist.state = moving_state_to_target(
                        &nav_grid,
                        &mut planner,
                        transform.translation,
                        target,
                        task.clone(),
                        seed,
                    )
                    .map(|(state, _)| state)
                    .unwrap_or_else(|| {
                        unreachable_moving_state(target, task.clone(), &mut inventory_access)
                    });
                    continue;
                }

                if move_along_path(&mut transform, target, &mut path, colonist.speed, dt, seed) {
                    colonist.state = complete_movement_task(
                        &mut geometry,
                        &nav_grid,
                        &mut planner,
                        &mut inventory_access,
                        &mut blueprints,
                        &workplaces,
                        &mut colonist,
                        transform.translation,
                        task,
                        seed,
                    );
                } else {
                    colonist.state = ColonistState::Moving {
                        target,
                        path,
                        task,
                        nav_revision: nav_grid.revision(),
                    };
                }
            }
            ColonistState::PlanningPath {
                request_id,
                target,
                task,
            } => {
                let task_valid = {
                    let inventories = inventory_access.p0();
                    moving_task_is_valid(&task, &mut resources, &workplaces, &inventories)
                };
                if !task_valid {
                    debug!(
                        "[殖民者] {} 任务失效（目标已消失），回到空闲",
                        colonist.name
                    );
                    colonist.state = ColonistState::Idle;
                    continue;
                }

                let Some(result) = planner.take_result(request_id) else {
                    colonist.state = ColonistState::PlanningPath {
                        request_id,
                        target,
                        task,
                    };
                    continue;
                };

                let path_stale = match &result.path {
                    Some(path) => nav_grid.path_needs_replan(path, result.revision),
                    // Conservative: if no path was found and geometry changed, retry
                    None => result.revision != nav_grid.revision(),
                };
                if path_stale {
                    colonist.state = moving_state_to_target(
                        &nav_grid,
                        &mut planner,
                        transform.translation,
                        target,
                        task.clone(),
                        seed,
                    )
                    .map(|(state, _)| state)
                    .unwrap_or_else(|| {
                        unreachable_moving_state(target, task.clone(), &mut inventory_access)
                    });
                    continue;
                }

                if let Some(path) = result.path {
                    colonist.state = ColonistState::Moving {
                        target,
                        path,
                        task,
                        nav_revision: result.revision,
                    };
                } else {
                    colonist.state = unreachable_moving_state(target, task, &mut inventory_access);
                }
            }
            ColonistState::Gathering {
                resource,
                kind,
                amount: planned_amount,
                mut timer,
            } => {
                let work_seconds = colonist
                    .workplace
                    .and_then(|wp| workplaces.get(wp).ok())
                    .map(|w| w.work_seconds)
                    .unwrap_or(GATHER_SECONDS);
                timer += dt;
                if timer < work_seconds {
                    colonist.state = ColonistState::Gathering {
                        resource,
                        kind,
                        amount: planned_amount,
                        timer,
                    };
                    continue;
                }

                let public_capacity = {
                    let inventories = inventory_access.p0();
                    total_public_addable(&inventories, kind)
                };
                if public_capacity <= 0 {
                    colonist.state = ColonistState::Idle;
                    continue;
                }

                let mut amount = 0;
                if let Ok(mut node) = resources.get_mut(resource) {
                    amount = planned_amount.min(node.amount).min(public_capacity);
                    node.amount -= amount;
                    if amount > 0 && node.amount <= 0 {
                        geometry.release_entity(resource);
                        commands.entity(resource).despawn();
                    }
                }

                if amount > 0 {
                    let inventories = inventory_access.p0();
                    colonist.state = deposit_moving_state(
                        &nav_grid,
                        &mut planner,
                        transform.translation,
                        &inventories,
                        kind,
                        amount,
                        seed,
                        None,
                    )
                    .unwrap_or(ColonistState::Idle);
                } else {
                    colonist.state = ColonistState::Idle;
                }
            }
            ColonistState::SplittingWood {
                workplace,
                output_amount,
                mut timer,
            } => {
                if !valid_workplace(&workplaces, workplace, Profession::WoodSplitter) {
                    colonist.state = ColonistState::Idle;
                    continue;
                }

                let work_seconds = workplaces
                    .get(workplace)
                    .map(|w| w.work_seconds)
                    .unwrap_or(WOOD_SPLIT_SECONDS);
                timer += dt;
                if timer < work_seconds {
                    colonist.state = ColonistState::SplittingWood {
                        workplace,
                        output_amount,
                        timer,
                    };
                    continue;
                }

                let inventories = inventory_access.p0();
                colonist.state = deposit_moving_state(
                    &nav_grid,
                    &mut planner,
                    transform.translation,
                    &inventories,
                    ResourceKind::Firewood,
                    output_amount,
                    seed,
                    None,
                )
                .unwrap_or(ColonistState::Idle);
            }
            ColonistState::Building {
                blueprint: blueprint_entity,
            } => {
                if let Ok((_, mut blueprint)) = blueprints.get_mut(blueprint_entity) {
                    if blueprint.has_materials() {
                        blueprint.progress =
                            (blueprint.progress + BUILD_RATE * dt).min(blueprint.build_seconds);
                    }
                    if blueprint.is_complete() {
                        colonist.state = ColonistState::Idle;
                    } else {
                        colonist.state = ColonistState::Building {
                            blueprint: blueprint_entity,
                        };
                    }
                } else {
                    colonist.state = ColonistState::Idle;
                }
            }
        }
    }
}

// 任务收尾：殖民者走到目标点后，按 Task 类型执行实际操作——
// 从仓库取 / 送建材、开始采集或劈柴、进食、给家里囤粮。
// 任一步失败（目标没了、库存不足）就把物资放回最近的公共仓库，人回到空闲。
fn complete_movement_task(
    _geometry: &mut WorldGeometry,
    nav_grid: &NavGrid,
    planner: &mut PathPlanner,
    inventory_access: &mut ParamSet<(
        Query<
            (
                Entity,
                &Inventory,
                &Transform,
                Option<&BuildingEntrance>,
                Option<&PublicInventory>,
                Option<&Housing>,
                Option<&CompletedBuilding>,
            ),
            Without<Colonist>,
        >,
        Query<&mut Inventory>,
    )>,
    blueprints: &mut Query<(Entity, &mut Blueprint)>,
    workplaces: &Query<&Workplace, Without<Colonist>>,
    colonist: &mut Colonist,
    position: Vec3,
    task: Task,
    seed: u64,
) -> ColonistState {
    match task {
        Task::PickupMaterial {
            source,
            blueprint,
            kind,
            amount,
            delivery_target,
        } => {
            let removed = remove_from_inventory(inventory_access, source, kind, amount);
            if !removed {
                return ColonistState::Idle;
            }

            if blueprints.contains(blueprint) {
                moving_state_to_target(
                    nav_grid,
                    planner,
                    position,
                    delivery_target,
                    Task::DeliverMaterial {
                        blueprint,
                        kind,
                        amount,
                    },
                    seed,
                )
                .map(|(state, _)| state)
                .unwrap_or_else(|| {
                    add_to_nearest_public(inventory_access, position, kind, amount);
                    path_retry_state()
                })
            } else {
                add_to_nearest_public(inventory_access, position, kind, amount);
                ColonistState::Idle
            }
        }
        Task::DeliverMaterial {
            blueprint,
            kind,
            amount,
        } => {
            if let Ok((_, mut blueprint)) = blueprints.get_mut(blueprint) {
                if kind == ResourceKind::Wood {
                    blueprint.delivered_wood =
                        (blueprint.delivered_wood + amount).min(blueprint.required_wood);
                }
            } else {
                add_to_nearest_public(inventory_access, position, kind, amount);
            }
            ColonistState::Idle
        }
        Task::Build { blueprint } => ColonistState::Building { blueprint },
        Task::Gather {
            resource,
            kind,
            amount,
        } => ColonistState::Gathering {
            resource,
            kind,
            amount,
            timer: 0.0,
        },
        Task::PickupWoodForSplitting {
            source,
            workplace,
            amount,
            work_target,
        } => {
            let removed =
                remove_from_inventory(inventory_access, source, ResourceKind::Wood, amount);
            if !removed {
                return ColonistState::Idle;
            }

            if valid_workplace(workplaces, workplace, Profession::WoodSplitter) {
                moving_state_to_target(
                    nav_grid,
                    planner,
                    position,
                    work_target,
                    Task::StartWoodSplitting {
                        workplace,
                        input_amount: amount,
                    },
                    seed,
                )
                .map(|(state, _)| state)
                .unwrap_or_else(|| {
                    add_to_nearest_public(inventory_access, position, ResourceKind::Wood, amount);
                    path_retry_state()
                })
            } else {
                add_to_nearest_public(inventory_access, position, ResourceKind::Wood, amount);
                ColonistState::Idle
            }
        }
        Task::StartWoodSplitting {
            workplace,
            input_amount,
        } => {
            if valid_workplace(workplaces, workplace, Profession::WoodSplitter) {
                ColonistState::SplittingWood {
                    workplace,
                    output_amount: input_amount * WOOD_SPLIT_OUTPUT,
                    timer: 0.0,
                }
            } else {
                add_to_nearest_public(inventory_access, position, ResourceKind::Wood, input_amount);
                ColonistState::Idle
            }
        }
        Task::Deposit {
            inventory,
            kind,
            amount,
        } => {
            let leftover = add_to_inventory(inventory_access, inventory, kind, amount);
            if leftover > 0 {
                let inventories = inventory_access.p0();
                deposit_moving_state(
                    nav_grid,
                    planner,
                    position,
                    &inventories,
                    kind,
                    leftover,
                    seed,
                    Some(inventory),
                )
                .unwrap_or_else(path_retry_state)
            } else {
                ColonistState::Idle
            }
        }
        Task::Eat { inventory } => {
            if remove_from_inventory(inventory_access, inventory, ResourceKind::Food, 1) {
                colonist.satiety = restored_satiety(colonist.satiety);
                ColonistState::Eating {
                    timer: EAT_REST_SECONDS,
                }
            } else {
                ColonistState::Idle
            }
        }
        Task::PickupHomeFood {
            source,
            home,
            amount,
        } => {
            if !remove_from_inventory(inventory_access, source, ResourceKind::Food, amount) {
                return ColonistState::Idle;
            }

            let target = {
                let inventories = inventory_access.p0();
                inventories
                    .get(home)
                    .ok()
                    .map(|(_, _, transform, entrance, _, _, _)| {
                        building_interaction_position(transform, entrance)
                    })
            };

            if let Some(target) = target {
                moving_state_to_target(
                    nav_grid,
                    planner,
                    position,
                    target,
                    Task::DeliverHomeFood { home, amount },
                    seed,
                )
                .map(|(state, _)| state)
                .unwrap_or_else(|| {
                    add_to_nearest_public(inventory_access, position, ResourceKind::Food, amount);
                    path_retry_state()
                })
            } else {
                add_to_nearest_public(inventory_access, position, ResourceKind::Food, amount);
                ColonistState::Idle
            }
        }
        Task::DeliverHomeFood { home, amount } => {
            let leftover = add_to_inventory(inventory_access, home, ResourceKind::Food, amount);
            if leftover > 0 {
                add_to_nearest_public(inventory_access, position, ResourceKind::Food, leftover);
            }
            ColonistState::Idle
        }
    }
}

fn remove_from_inventory(
    inventory_access: &mut ParamSet<(
        Query<
            (
                Entity,
                &Inventory,
                &Transform,
                Option<&BuildingEntrance>,
                Option<&PublicInventory>,
                Option<&Housing>,
                Option<&CompletedBuilding>,
            ),
            Without<Colonist>,
        >,
        Query<&mut Inventory>,
    )>,
    inventory: Entity,
    kind: ResourceKind,
    amount: i32,
) -> bool {
    inventory_access
        .p1()
        .get_mut(inventory)
        .map(|mut inventory| inventory.remove(kind, amount))
        .unwrap_or(false)
}

fn add_to_inventory(
    inventory_access: &mut ParamSet<(
        Query<
            (
                Entity,
                &Inventory,
                &Transform,
                Option<&BuildingEntrance>,
                Option<&PublicInventory>,
                Option<&Housing>,
                Option<&CompletedBuilding>,
            ),
            Without<Colonist>,
        >,
        Query<&mut Inventory>,
    )>,
    inventory: Entity,
    kind: ResourceKind,
    amount: i32,
) -> i32 {
    let accepted = inventory_access
        .p1()
        .get_mut(inventory)
        .map(|mut inventory| inventory.add_partial(kind, amount))
        .unwrap_or(0);
    amount - accepted
}

fn add_to_nearest_public(
    inventory_access: &mut ParamSet<(
        Query<
            (
                Entity,
                &Inventory,
                &Transform,
                Option<&BuildingEntrance>,
                Option<&PublicInventory>,
                Option<&Housing>,
                Option<&CompletedBuilding>,
            ),
            Without<Colonist>,
        >,
        Query<&mut Inventory>,
    )>,
    position: Vec3,
    kind: ResourceKind,
    amount: i32,
) -> i32 {
    let mut targets: Vec<(f32, Entity)> = {
        let inventories = inventory_access.p0();
        inventories
            .iter()
            .filter(|(_, inventory, _, _, public, _, _)| {
                public.is_some() && inventory.max_addable(kind) > 0
            })
            .map(|(entity, _, transform, entrance, _, _, _)| {
                (
                    xz_distance(position, building_interaction_position(transform, entrance)),
                    entity,
                )
            })
            .collect()
    };
    targets.sort_by(|(dist_a, _), (dist_b, _)| dist_a.total_cmp(dist_b));

    let mut remaining = amount;
    for (_, target) in targets {
        if remaining <= 0 {
            break;
        }

        let accepted = inventory_access
            .p1()
            .get_mut(target)
            .map(|mut inventory| inventory.add_partial(kind, remaining))
            .unwrap_or(0);
        remaining -= accepted;
    }

    remaining
}

fn moving_task_is_valid(
    task: &Task,
    resources: &mut Query<&mut ResourceNode>,
    workplaces: &Query<&Workplace, Without<Colonist>>,
    inventories: &Query<
        (
            Entity,
            &Inventory,
            &Transform,
            Option<&BuildingEntrance>,
            Option<&PublicInventory>,
            Option<&Housing>,
            Option<&CompletedBuilding>,
        ),
        Without<Colonist>,
    >,
) -> bool {
    match task {
        Task::Gather {
            resource,
            kind,
            amount,
        } => resources
            .get_mut(*resource)
            .map(|node| node.kind == *kind && node.amount >= *amount && *amount > 0)
            .unwrap_or(false),
        Task::PickupWoodForSplitting {
            source,
            workplace,
            amount,
            ..
        } => {
            valid_workplace(workplaces, *workplace, Profession::WoodSplitter)
                && inventories
                    .get(*source)
                    .map(|(_, inventory, _, _, public, _, _)| {
                        public.is_some() && inventory.amount(ResourceKind::Wood) >= *amount
                    })
                    .unwrap_or(false)
        }
        Task::StartWoodSplitting { .. } => true,
        Task::PickupMaterial { .. }
        | Task::DeliverMaterial { .. }
        | Task::Build { .. }
        | Task::Deposit { .. }
        | Task::Eat { .. }
        | Task::PickupHomeFood { .. }
        | Task::DeliverHomeFood { .. } => true,
    }
}

fn unreachable_moving_state(
    target: Vec3,
    task: Task,
    inventory_access: &mut ParamSet<(
        Query<
            (
                Entity,
                &Inventory,
                &Transform,
                Option<&BuildingEntrance>,
                Option<&PublicInventory>,
                Option<&Housing>,
                Option<&CompletedBuilding>,
            ),
            Without<Colonist>,
        >,
        Query<&mut Inventory>,
    )>,
) -> ColonistState {
    match task {
        Task::Deposit { .. } => path_retry_state(),
        Task::DeliverMaterial { kind, amount, .. } => {
            add_to_nearest_public(inventory_access, target, kind, amount);
            path_retry_state()
        }
        Task::DeliverHomeFood { amount, .. } => {
            add_to_nearest_public(inventory_access, target, ResourceKind::Food, amount);
            path_retry_state()
        }
        Task::StartWoodSplitting { input_amount, .. } => {
            add_to_nearest_public(inventory_access, target, ResourceKind::Wood, input_amount);
            path_retry_state()
        }
        Task::PickupMaterial { .. }
        | Task::Build { .. }
        | Task::Gather { .. }
        | Task::Eat { .. }
        | Task::PickupHomeFood { .. }
        | Task::PickupWoodForSplitting { .. } => path_retry_state(),
    }
}

fn path_retry_state() -> ColonistState {
    ColonistState::WaitingForPathRetry {
        timer: PATH_FAILURE_RETRY_SECONDS,
    }
}

fn move_along_path(
    transform: &mut Transform,
    target: Vec3,
    path: &mut Vec<Vec3>,
    speed: f32,
    dt: f32,
    seed: u64,
) -> bool {
    let waypoint = path.first().copied().unwrap_or(target);
    if move_toward(transform, waypoint, speed, dt, seed) {
        if !path.is_empty() {
            path.remove(0);
        }
        return path.is_empty();
    }

    false
}

fn move_toward(transform: &mut Transform, target: Vec3, speed: f32, dt: f32, seed: u64) -> bool {
    let to_target = target - transform.translation;
    let xz_dist = (to_target.x * to_target.x + to_target.z * to_target.z).sqrt();
    if xz_dist <= 0.05 {
        let ground_y = terrain_height(seed, target.x, target.z) + COLONIST_HALF_HEIGHT;
        transform.translation = Vec3::new(target.x, ground_y, target.z);
        return true;
    }

    let step = speed * dt;
    if step >= xz_dist {
        let ground_y = terrain_height(seed, target.x, target.z) + COLONIST_HALF_HEIGHT;
        transform.translation = Vec3::new(target.x, ground_y, target.z);
        true
    } else {
        let dir_x = to_target.x / xz_dist;
        let dir_z = to_target.z / xz_dist;
        let new_x = transform.translation.x + dir_x * step;
        let new_z = transform.translation.z + dir_z * step;
        let ground_y = terrain_height(seed, new_x, new_z) + COLONIST_HALF_HEIGHT;
        transform.translation = Vec3::new(new_x, ground_y, new_z);
        let yaw = dir_x.atan2(dir_z);
        transform.rotation = Quat::from_rotation_y(yaw);
        false
    }
}

fn decayed_satiety(satiety: f32, dt: f32) -> f32 {
    (satiety - dt / SATIETY_LOSS_SECONDS).max(0.0)
}

fn restored_satiety(satiety: f32) -> f32 {
    (satiety + EAT_RESTORE).min(100.0)
}

fn best_home_candidate(homes: &[(Entity, usize, Vec3)], start: Vec3) -> Option<usize> {
    homes
        .iter()
        .enumerate()
        .filter(|(_, (_, residents, _))| *residents < Housing::CAPACITY)
        .min_by(|(_, (_, count_a, pos_a)), (_, (_, count_b, pos_b))| {
            count_a
                .cmp(count_b)
                .then_with(|| xz_distance(start, *pos_a).total_cmp(&xz_distance(start, *pos_b)))
        })
        .map(|(index, _)| index)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::{COLONIST_CARRY_CAPACITY, Inventory, PublicInventory};
    use crate::terrain::DEFAULT_TERRAIN_SEED;

    const SEED: u64 = DEFAULT_TERRAIN_SEED;

    fn test_entity(index: u32) -> Entity {
        Entity::from_raw_u32(index).unwrap()
    }

    fn test_colonist(name: &str, profession: Profession, workplace: Option<Entity>) -> Colonist {
        Colonist {
            name: name.to_string(),
            state: ColonistState::Idle,
            profession,
            workplace,
            speed: 2.2,
            home: None,
            satiety: 100.0,
            carry_capacity: COLONIST_CARRY_CAPACITY,
        }
    }

    #[test]
    fn move_along_path_advances_one_waypoint_at_a_time() {
        let mut transform = Transform::from_translation(Vec3::new(
            0.0,
            terrain_height(SEED, 0.0, 0.0) + COLONIST_HALF_HEIGHT,
            0.0,
        ));
        let ty = terrain_height(SEED, 2.0, 0.0) + COLONIST_HALF_HEIGHT;
        let target = Vec3::new(2.0, ty, 0.0);
        let wy = terrain_height(SEED, 1.0, 0.0) + COLONIST_HALF_HEIGHT;
        let mut path = vec![Vec3::new(1.0, wy, 0.0), target];

        assert!(!move_along_path(
            &mut transform,
            target,
            &mut path,
            10.0,
            0.1,
            SEED,
        ));
        assert!((transform.translation.x - 1.0).abs() < 0.01);
        assert_eq!(path, vec![target]);

        assert!(move_along_path(
            &mut transform,
            target,
            &mut path,
            10.0,
            0.1,
            SEED,
        ));
        assert!((transform.translation.x - target.x).abs() < 0.01);
        assert!(path.is_empty());
    }

    #[test]
    fn carried_amount_uses_resource_unit_size() {
        assert_eq!(
            carried_amount(ResourceKind::Wood, COLONIST_CARRY_CAPACITY),
            1
        );
        assert_eq!(
            carried_amount(ResourceKind::Food, COLONIST_CARRY_CAPACITY),
            10
        );
    }

    #[test]
    fn hungry_threshold_starts_below_fifty() {
        let colonist = Colonist {
            name: "Test".to_string(),
            state: ColonistState::Idle,
            profession: Profession::Unemployed,
            workplace: None,
            speed: 1.0,
            home: None,
            satiety: 49.9,
            carry_capacity: COLONIST_CARRY_CAPACITY,
        };

        assert!(colonist.is_hungry());
    }

    #[test]
    fn satiety_decays_with_simulation_time_and_caps_at_zero() {
        assert_eq!(decayed_satiety(100.0, 10.0), 99.0);
        assert_eq!(decayed_satiety(0.5, 10.0), 0.0);
    }

    #[test]
    fn eating_restores_satiety_without_exceeding_one_hundred() {
        assert_eq!(restored_satiety(40.0), 90.0);
        assert_eq!(restored_satiety(80.0), 100.0);
    }

    #[test]
    fn housing_assignment_balances_before_distance() {
        let near_full = Entity::from_raw_u32(1).unwrap();
        let far_empty = Entity::from_raw_u32(2).unwrap();
        let homes = vec![
            (near_full, 4, Vec3::new(0.1, 0.0, 0.0)),
            (far_empty, 0, Vec3::new(100.0, 0.0, 0.0)),
        ];

        let selected = best_home_candidate(&homes, Vec3::ZERO).unwrap();

        assert_eq!(homes[selected].0, far_empty);
    }

    #[test]
    fn workplace_candidate_uses_open_slots_by_distance() {
        let near = test_entity(1);
        let far = test_entity(2);
        let snapshots = vec![
            WorkplaceSnapshot {
                entity: near,
                profession: Profession::Lumberjack,
                desired_slots: 1,
                position: Vec3::new(1.0, 0.0, 0.0),
            },
            WorkplaceSnapshot {
                entity: far,
                profession: Profession::Gatherer,
                desired_slots: 1,
                position: Vec3::new(5.0, 0.0, 0.0),
            },
        ];
        let mut assigned_counts = HashMap::new();
        assigned_counts.insert(near, 1);

        let selected = best_workplace_candidate(&snapshots, &assigned_counts, Vec3::ZERO).unwrap();

        assert_eq!(selected.entity, far);
        assert_eq!(selected.profession, Profession::Gatherer);
    }

    #[test]
    fn assign_workplaces_fills_and_releases_slots() {
        let mut app = App::new();
        app.add_systems(Update, assign_workplaces);
        let workplace = app
            .world_mut()
            .spawn((
                Transform::from_translation(Vec3::ZERO),
                Workplace {
                    profession: Profession::WoodSplitter,
                    desired_slots: 2,
                    max_slots: 2,
                    work_seconds: 2.0,
                },
            ))
            .id();
        let first = app
            .world_mut()
            .spawn((
                Transform::from_translation(Vec3::new(1.0, 0.0, 0.0)),
                test_colonist("First", Profession::Unemployed, None),
            ))
            .id();
        let second = app
            .world_mut()
            .spawn((
                Transform::from_translation(Vec3::new(2.0, 0.0, 0.0)),
                test_colonist("Second", Profession::Unemployed, None),
            ))
            .id();

        app.update();

        assert_eq!(
            app.world()
                .entity(first)
                .get::<Colonist>()
                .unwrap()
                .workplace,
            Some(workplace)
        );
        assert_eq!(
            app.world()
                .entity(second)
                .get::<Colonist>()
                .unwrap()
                .profession,
            Profession::WoodSplitter
        );

        app.world_mut()
            .entity_mut(workplace)
            .get_mut::<Workplace>()
            .unwrap()
            .desired_slots = 1;
        app.update();

        let assigned = [first, second]
            .into_iter()
            .filter(|entity| {
                app.world()
                    .entity(*entity)
                    .get::<Colonist>()
                    .unwrap()
                    .workplace
                    == Some(workplace)
            })
            .count();

        assert_eq!(assigned, 1);
    }

    #[test]
    fn gathering_assignment_requires_matching_profession() {
        let mut app = App::new();
        app.insert_resource(TerrainSeed(SEED));
        let mut geometry = WorldGeometry::default();
        let mut nav_grid = NavGrid::default();
        nav_grid.sync_for_test(&mut geometry, SEED);
        app.insert_resource(nav_grid);
        app.init_resource::<PathPlanner>();
        app.add_systems(Update, assign_idle_colonists);

        let workplace = app
            .world_mut()
            .spawn((
                Transform::from_translation(Vec3::ZERO),
                Workplace {
                    profession: Profession::Lumberjack,
                    desired_slots: 1,
                    max_slots: 2,
                    work_seconds: 2.0,
                },
            ))
            .id();
        app.world_mut().spawn((
            Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ResourceNode {
                kind: ResourceKind::Wood,
                amount: 10,
            },
        ));
        app.world_mut().spawn((
            Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
            Inventory::public(100),
            PublicInventory,
        ));
        let lumberjack = app
            .world_mut()
            .spawn((
                Transform::from_translation(Vec3::ZERO),
                test_colonist("Lumberjack", Profession::Lumberjack, Some(workplace)),
            ))
            .id();
        let unemployed = app
            .world_mut()
            .spawn((
                Transform::from_translation(Vec3::new(0.5, 0.0, 0.0)),
                test_colonist("Unemployed", Profession::Unemployed, None),
            ))
            .id();

        app.update();

        assert!(matches!(
            app.world()
                .entity(lumberjack)
                .get::<Colonist>()
                .unwrap()
                .state,
            ColonistState::PlanningPath {
                task: Task::Gather {
                    kind: ResourceKind::Wood,
                    ..
                },
                ..
            }
        ));
        assert!(matches!(
            app.world()
                .entity(unemployed)
                .get::<Colonist>()
                .unwrap()
                .state,
            ColonistState::Idle
        ));
    }

    #[test]
    fn wood_splitting_consumes_wood_and_deposits_firewood() {
        let mut app = App::new();
        let mut time = Time::<()>::default();
        time.advance_by(std::time::Duration::from_secs_f32(0.1));
        app.insert_resource(time);
        app.insert_resource(SimulationClock::default());
        app.insert_resource(TerrainSeed(SEED));
        let mut geometry = WorldGeometry::default();
        let mut nav_grid = NavGrid::default();
        nav_grid.sync_for_test(&mut geometry, SEED);
        app.insert_resource(geometry);
        app.insert_resource(nav_grid);
        app.init_resource::<PathPlanner>();
        app.add_systems(Update, update_colonists);

        let mut inventory = Inventory::public(100);
        inventory.add(ResourceKind::Wood, 1);
        let source = app
            .world_mut()
            .spawn((
                Transform::from_translation(Vec3::ZERO),
                inventory,
                PublicInventory,
            ))
            .id();
        let workplace = app
            .world_mut()
            .spawn((
                Transform::from_translation(Vec3::ZERO),
                Workplace {
                    profession: Profession::WoodSplitter,
                    desired_slots: 1,
                    max_slots: 2,
                    work_seconds: 2.0,
                },
            ))
            .id();
        let colonist = app
            .world_mut()
            .spawn((
                Transform::from_translation(Vec3::ZERO),
                Colonist {
                    state: ColonistState::Moving {
                        target: Vec3::ZERO,
                        path: Vec::new(),
                        task: Task::PickupWoodForSplitting {
                            source,
                            workplace,
                            amount: WOOD_SPLIT_INPUT,
                            work_target: Vec3::ZERO,
                        },
                        nav_revision: 0,
                    },
                    ..test_colonist("Splitter", Profession::WoodSplitter, Some(workplace))
                },
            ))
            .id();

        app.update();

        assert_eq!(
            app.world()
                .entity(source)
                .get::<Inventory>()
                .unwrap()
                .amount(ResourceKind::Wood),
            0
        );

        app.world_mut()
            .entity_mut(colonist)
            .get_mut::<Colonist>()
            .unwrap()
            .state = ColonistState::SplittingWood {
            workplace,
            output_amount: WOOD_SPLIT_OUTPUT,
            timer: WOOD_SPLIT_SECONDS,
        };

        app.update();

        app.world_mut()
            .entity_mut(colonist)
            .get_mut::<Colonist>()
            .unwrap()
            .state = ColonistState::Moving {
            target: Vec3::ZERO,
            path: Vec::new(),
            task: Task::Deposit {
                inventory: source,
                kind: ResourceKind::Firewood,
                amount: WOOD_SPLIT_OUTPUT,
            },
            nav_revision: 0,
        };

        app.update();

        assert_eq!(
            app.world()
                .entity(source)
                .get::<Inventory>()
                .unwrap()
                .amount(ResourceKind::Firewood),
            WOOD_SPLIT_OUTPUT
        );
    }

    #[test]
    fn gather_reservation_exhausts_single_node_for_next_worker() {
        let resource = test_entity(1);
        let mut reservations = AssignmentReservations::default();
        let resources = vec![(resource, ResourceKind::Food, 10, Vec3::new(1.0, 0.0, 0.0))];

        let candidates = gather_candidate_snapshots(
            Vec3::ZERO,
            ResourceKind::Food,
            COLONIST_CARRY_CAPACITY,
            20,
            resources.clone(),
            &reservations,
        );
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].amount, 10);

        reservations.reserve_gather(resource, ResourceKind::Food, candidates[0].amount);
        let candidates = gather_candidate_snapshots(
            Vec3::ZERO,
            ResourceKind::Food,
            COLONIST_CARRY_CAPACITY,
            20,
            resources,
            &reservations,
        );

        assert!(candidates.is_empty());
    }

    #[test]
    fn gather_reservation_moves_next_worker_to_next_node() {
        let near = test_entity(1);
        let far = test_entity(2);
        let mut reservations = AssignmentReservations::default();
        reservations.reserve_gather(near, ResourceKind::Food, 10);

        let candidates = gather_candidate_snapshots(
            Vec3::ZERO,
            ResourceKind::Food,
            COLONIST_CARRY_CAPACITY,
            20,
            vec![
                (near, ResourceKind::Food, 10, Vec3::new(1.0, 0.0, 0.0)),
                (far, ResourceKind::Food, 10, Vec3::new(3.0, 0.0, 0.0)),
            ],
            &reservations,
        );

        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].resource, far);
    }

    #[test]
    fn public_capacity_reservation_blocks_extra_gathering() {
        let resource = test_entity(1);
        let mut reservations = AssignmentReservations::default();
        reservations.reserve_public_deposit(ResourceKind::Food, 10);
        let public_capacity = ((10 * ResourceKind::Food.unit_size())
            - reservations.reserved_public_capacity())
        .max(0)
            / ResourceKind::Food.unit_size();

        let candidates = gather_candidate_snapshots(
            Vec3::ZERO,
            ResourceKind::Food,
            COLONIST_CARRY_CAPACITY,
            public_capacity,
            vec![(resource, ResourceKind::Food, 20, Vec3::new(1.0, 0.0, 0.0))],
            &reservations,
        );

        assert!(candidates.is_empty());
    }

    #[test]
    fn moving_state_requests_async_path_before_moving() {
        let mut geometry = WorldGeometry::default();
        let mut nav_grid = NavGrid::default();
        nav_grid.sync_for_test(&mut geometry, SEED);
        let mut planner = PathPlanner::default();
        let task = Task::Build {
            blueprint: test_entity(20),
        };

        let (state, path_len) = moving_state_to_target(
            &nav_grid,
            &mut planner,
            Vec3::ZERO,
            Vec3::new(2.0, 0.0, 0.0),
            task,
            SEED,
        )
        .unwrap();

        assert_eq!(path_len, 0);
        assert!(matches!(
            state,
            ColonistState::PlanningPath { request_id: 1, .. }
        ));
        assert!(planner.take_result(1).is_none());
    }

    #[test]
    fn planning_path_reserves_its_task() {
        let resource = test_entity(1);
        let state = ColonistState::PlanningPath {
            request_id: 7,
            target: Vec3::new(1.0, 0.0, 0.0),
            task: Task::Gather {
                resource,
                kind: ResourceKind::Food,
                amount: 10,
            },
        };
        let mut reservations = AssignmentReservations::default();

        reservations.reserve_state(&state);

        assert_eq!(reservations.reserved_gather(resource), 10);
    }
}
