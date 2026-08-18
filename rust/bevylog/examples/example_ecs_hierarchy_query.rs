//! Bevy 0.19 入门示例：演示层级遍历（关系组件 ChildOf / Children）。
//!
//! Bevy 0.19 把父子关系改成了「关系组件」（旧版叫 Parent / Children）：
//! - `ChildOf`：挂在子实体上、指向父实体（关系，一对一）
//! - `Children`：挂在父实体上、存所有子实体（关系目标，一对多）
//!
//! 学习重点：
//! - iter_descendants(entity)：遍历某实体的所有后代（广度优先）
//! - iter_descendants_depth_first(entity)：深度优先遍历后代
//! - iter_ancestors(entity)：遍历某实体的所有祖先（从父到根）
//! - related(entity)：拿到实体的直接父实体

use bevy::prelude::*;

// 根实体标记
#[derive(Component)]
struct Root;

// 名字组件：用于打印层级关系
#[derive(Component)]
struct Named(&'static str);

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        .add_systems(Update, traverse)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // 构建三层结构：
    //   root
    //   ├── A
    //   │   ├── A1
    //   │   └── A2
    //   └── B
    commands.spawn((Named("root"), Root)).with_children(|root| {
        root.spawn(Named("A")).with_children(|a| {
            a.spawn(Named("A1"));
            a.spawn(Named("A2"));
        });
        root.spawn(Named("B"));
    });
}

// 每秒遍历一次层级，打印后代和祖先链
fn traverse(
    root: Single<Entity, With<Root>>,
    children_q: Query<&Children>,
    parent_q: Query<&ChildOf>,
    names: Query<&Named>,
    time: Res<Time>,
    mut last: Local<f32>,
) {
    if time.elapsed_secs() - *last < 1.0 {
        return;
    }
    *last = time.elapsed_secs();

    let root_entity = *root;

    // 1. 遍历 root 的所有后代（广度优先）
    let descendants: Vec<String> = children_q
        .iter_descendants(root_entity)
        .map(|e| names.get(e).map(|n| n.0).unwrap_or("?").to_string())
        .collect();
    info!(
        "[层级] root 的后代（广度优先）: {}",
        descendants.join(" → ")
    );

    // 2. 找一个叶子（深度优先里第一个没有子实体的），向上遍历祖先
    if let Some(leaf) = children_q
        .iter_descendants_depth_first(root_entity)
        .find(|e| children_q.get(*e).map(|c| c.is_empty()).unwrap_or(true))
    {
        let ancestors: Vec<String> = parent_q
            .iter_ancestors(leaf)
            .map(|e| names.get(e).map(|n| n.0).unwrap_or("?").to_string())
            .collect();
        info!(
            "[层级] 叶子「{}」的祖先链: {}",
            names.get(leaf).map(|n| n.0).unwrap_or("?"),
            ancestors.join(" ← "),
        );
    }

    // 3. 直接父实体：related
    if let Some(leaf) = children_q
        .iter_descendants_depth_first(root_entity)
        .find(|e| children_q.get(*e).map(|c| c.is_empty()).unwrap_or(true))
    {
        if let Some(parent) = parent_q.related(leaf) {
            info!(
                "[层级] 叶子「{}」的直接父实体是「{}」",
                names.get(leaf).map(|n| n.0).unwrap_or("?"),
                names.get(parent).map(|n| n.0).unwrap_or("?"),
            );
        }
    }
}
