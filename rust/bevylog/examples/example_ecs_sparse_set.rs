//! Bevy 0.19 入门示例：演示 SparseSet 存储的实战场景（频繁增删的状态）。
//!
//! 学习重点：
//! - 用 #[component(storage = "SparseSet")] 声明一个「中毒」状态组件
//! - 中毒状态频繁增删，SparseSet 增删不移动 archetype，代价低
//! - Commands 是延迟执行的，PostUpdate 系统能看到本帧 Update 应用的结果
//!
//! 观察：每秒随机让部分圆中毒（变绿）/恢复（变灰），日志打印中毒数量。

use bevy::prelude::*;
use rand::RngExt;

// 可被中毒的实体标记
#[derive(Component)]
struct Enemy;

// 中毒状态：SparseSet 存储（频繁增删）
#[derive(Component)]
#[component(storage = "SparseSet")]
struct Poisoned;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.08)))
        .add_systems(Startup, setup)
        .add_systems(Update, toggle_poison)
        .add_systems(PostUpdate, update_colors)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // 20 个可中毒的圆，排成 5x4 网格
    for i in 0..20 {
        let x = (i as f32 % 5.0 - 2.0) * 80.0;
        let y = (i as f32 / 5.0 - 1.5) * 80.0;
        commands.spawn((
            Enemy,
            Mesh2d(meshes.add(Circle::new(20.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.4, 0.4, 0.45)))),
            Transform::from_xyz(x, y, 0.0),
        ));
    }
}

// 每秒随机给部分实体添加/移除中毒状态
fn toggle_poison(
    time: Res<Time>,
    mut commands: Commands,
    enemies: Query<Entity, With<Enemy>>,
    mut last_tick: Local<f32>,
) {
    if time.elapsed_secs() - *last_tick < 1.0 {
        return;
    }
    *last_tick = time.elapsed_secs();

    let total = enemies.iter().count();
    let mut poisoned = 0;
    let mut rng = rand::rng();
    for e in &enemies {
        if rng.random::<f32>() < 0.3 {
            commands.entity(e).insert(Poisoned);
            poisoned += 1;
        } else {
            commands.entity(e).remove::<Poisoned>();
        }
    }
    info!("[SparseSet] 本次中毒 {poisoned} / {total} 个");
}

// 根据是否中毒更新颜色（PostUpdate 能看到 Update 里 Commands 应用的结果）
fn update_colors(
    mut materials: ResMut<Assets<ColorMaterial>>,
    poisoned: Query<&MeshMaterial2d<ColorMaterial>, (With<Enemy>, With<Poisoned>)>,
    healthy: Query<&MeshMaterial2d<ColorMaterial>, (With<Enemy>, Without<Poisoned>)>,
) {
    for mat in &poisoned {
        if let Some(mut m) = materials.get_mut(&mat.0) {
            m.color = Color::srgb(0.3, 0.9, 0.4); // 中毒：绿
        }
    }
    for mat in &healthy {
        if let Some(mut m) = materials.get_mut(&mat.0) {
            m.color = Color::srgb(0.4, 0.4, 0.45); // 健康：灰
        }
    }
}
