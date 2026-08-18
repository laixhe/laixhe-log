//! Bevy 0.19 入门示例：关系查询进阶（related / relationship_sources / root_ancestor）。
//!
//! 前面 example_ecs_relationship 演示了「定义自定义关系」，example_ecs_hierarchy_query
//! 演示了内置 ChildOf/Children 的层级遍历。本例进阶：用 Query 的关系查询方法
//! 直接查自定义关系的目标、来源、根节点。
//!
//! 学习重点：
//! - `query.related::<R>(entity)`：拿到实体指向的关系目标（一对一方向）
//! - `query.relationship_sources::<S>(entity)`：拿到指向该实体的所有来源（一对多反向）
//! - `query.root_ancestor::<R>(entity)`：沿关系链一路向上找到「根」
//!
//! 关系结构（Follows：跟随者 → 被跟随者）：
//!   队长 ← A ← B（A 跟随队长，B 跟随 A，形成链）

use bevy::prelude::*;

// 自定义关系：跟随者指向被跟随者
#[derive(Component)]
#[relationship(relationship_target = Followers)]
struct Follows(pub Entity);

// 关系目标：被跟随者上自动维护的跟随者列表
#[derive(Component)]
#[relationship_target(relationship = Follows)]
struct Followers(Vec<Entity>);

// 名字组件
#[derive(Component)]
struct Named(&'static str);

// 保存需要查询的实体
#[derive(Resource)]
struct Entities {
    leader: Entity,
    b: Entity,
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, query_relationships)
        .run()
}

fn setup(mut commands: Commands) {
    // 队长 ← A ← B
    let leader = commands.spawn(Named("队长")).id();
    let a = commands.spawn((Named("A"), Follows(leader))).id();
    let b = commands.spawn((Named("B"), Follows(a))).id();

    commands.insert_resource(Entities { leader, b });
    info!("[关系查询] 已建立关系链：队长 ← A ← B");
}

// 每秒查询一次，演示关系查询方法。
fn query_relationships(
    follows: Query<&Follows>,
    followers: Query<&Followers>,
    names: Query<&Named>,
    entities: Res<Entities>,
    time: Res<Time>,
    mut last: Local<f32>,
) {
    if time.elapsed_secs() - *last < 1.0 {
        return;
    }
    *last = time.elapsed_secs();

    let name = |e: Entity| names.get(e).map(|n| n.0).unwrap_or("?").to_string();

    // 1. related：B 的直接被跟随者
    if let Some(parent) = follows.related::<Follows>(entities.b) {
        info!("[关系查询] B 的直接被跟随者 = {}", name(parent));
    }

    // 2. relationship_sources：队长的所有直接跟随者（反查）
    let sources: Vec<String> = followers
        .relationship_sources::<Followers>(entities.leader)
        .map(name)
        .collect();
    info!("[关系查询] 队长的直接跟随者 = {}", sources.join(", "));

    // 3. root_ancestor：B 沿链向上找到的根（队长）
    let root = follows.root_ancestor::<Follows>(entities.b);
    info!("[关系查询] B 的关系根节点 = {}", name(root));
}
