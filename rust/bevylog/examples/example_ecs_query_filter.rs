//! Bevy 0.19 入门示例：演示 Query 的高级过滤器（组合过滤 + 变更/新增检测）。
//!
//! Query 除了 `With<T>` / `Without<T>`，还支持更灵活的过滤器：
//! - `Or<(...)>`：匹配「满足任意一个条件」的实体（逻辑或）
//! - `Changed<T>`：只匹配「T 组件自上次读取后被修改过」的实体
//! - `Added<T>`：只匹配「T 组件刚被添加」的实体（常用于检测新生成的实体）
//!
//! 学习重点：
//! - Or<(With<A>, With<B>)> 组合过滤
//! - Changed<T> 变更检测（避免每帧处理未变化的实体）
//! - Added<T> 新增检测（一次性的初始化逻辑）
//! - .chain() 显式控制系统顺序（让「修改」先于「检测」）

use bevy::{prelude::*, text::FontSourceTemplate};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

#[derive(Component)]
struct Red;

#[derive(Component)]
struct Blue;

#[derive(Component)]
struct Health(i32);

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                // 先扣血，再检测变化，保证 detect 能拿到本帧的修改
                damage_periodically,
                detect_health_change,
                count_red_or_blue,
                detect_new_red,
            )
                .chain(),
        )
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // 2 个红色实体（正方形）
    for i in 0..2 {
        commands.spawn((
            Red,
            Health(100),
            Mesh2d(meshes.add(Rectangle::new(40.0, 40.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.9, 0.2, 0.2)))),
            Transform::from_xyz(-200.0 + i as f32 * 80.0, 60.0, 0.0),
        ));
    }

    // 2 个蓝色实体（正方形）
    for i in 0..2 {
        commands.spawn((
            Blue,
            Health(80),
            Mesh2d(meshes.add(Rectangle::new(40.0, 40.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.2, 0.4, 0.9)))),
            Transform::from_xyz(-200.0 + i as f32 * 80.0, -60.0, 0.0),
        ));
    }

    commands.spawn_scene(bsn! {
        Text2d::new("Or / Changed / Added 过滤器")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(26.0),
        }
        Transform::from_xyz(0.0, 180.0, 0.0)
    });
}

// 过滤器1：Or 组合过滤 —— 匹配「红色 或 蓝色」的实体（逻辑或）。
// 用 Query<(), Or<...>> 只做实体匹配、不读取组件，配合 iter().count() 统计数量。
fn count_red_or_blue(
    red_or_blue: Query<(), Or<(With<Red>, With<Blue>)>>,
    red: Query<(), With<Red>>,
    blue: Query<(), With<Blue>>,
    time: Res<Time>,
    mut last: Local<f32>,
) {
    // 每秒统计一次，避免刷屏
    if time.elapsed_secs() - *last < 1.0 {
        return;
    }
    *last = time.elapsed_secs();

    info!(
        "[过滤器] Or 匹配 {} 个实体（红 {} + 蓝 {}）",
        red_or_blue.iter().count(),
        red.iter().count(),
        blue.iter().count(),
    );
}

// 过滤器2：Changed —— 只匹配 Health 被修改过的实体（扣血后那一帧才触发）。
fn detect_health_change(query: Query<&Health, Changed<Health>>) {
    for health in &query {
        info!("[过滤器] 检测到血量变化：当前 = {}", health.0);
    }
}

// 过滤器3：Added —— 只匹配刚添加 Red 组件的实体（一次性初始化）。
fn detect_new_red(query: Query<&Transform, Added<Red>>) {
    for transform in &query {
        info!(
            "[过滤器] 检测到新红色实体，位置 = ({:.0}, {:.0})",
            transform.translation.x, transform.translation.y
        );
    }
}

// 每 2 秒给所有实体扣 10 点血，触发 Changed 检测。
fn damage_periodically(mut query: Query<&mut Health>, time: Res<Time>, mut last: Local<f32>) {
    if time.elapsed_secs() - *last < 2.0 {
        return;
    }
    *last = time.elapsed_secs();

    for mut health in &mut query {
        health.0 = (health.0 - 10).max(0);
    }
    info!("[过滤器] 已扣血 10 点");
}
