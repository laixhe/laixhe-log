//! Bevy 0.19 入门示例：演示查询（Query）系统的几种常用形式。
//!
//! Query 是 ECS 的核心：按组件组合匹配实体，遍历并读写组件。
//!
//! 学习重点：
//! - Query<(&A, &mut B)>：遍历所有匹配实体（iter / iter_mut）
//! - Single<...>：期望恰好一个匹配实体（0 个或多个会 panic）
//! - Query::get(entity)：按实体精确访问（只读）
//! - With<T> / Without<T> 过滤：缩小匹配范围，也让调度器能判定系统间互不冲突

use bevy::{prelude::*, text::FontSourceTemplate};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

// 速度组件：玩家和敌人都挂，但只有部分系统会用到
#[derive(Component)]
struct Speed(f32);

// 保存玩家实体的资源：演示用 Query::get 按实体精确访问
#[derive(Resource)]
struct PlayerEntity(Entity);

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_player,         // Single：恰好一个玩家
                move_enemies,        // iter_mut + Without 过滤：只遍历敌人
                sum_speed,           // iter 只读：统计所有速度
                report_player_speed, // get：按实体精确读取
            ),
        )
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // 生成 1 个玩家（绿色）
    let player = commands
        .spawn((
            Player,
            Speed(120.0),
            Mesh2d(meshes.add(Circle::new(30.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.2, 0.8, 0.4)))),
            Transform::from_xyz(-200.0, 150.0, 0.0),
        ))
        .id();

    // 把玩家实体存进资源，供 report_player_speed 用 get 精确访问
    commands.insert_resource(PlayerEntity(player));

    // 生成 5 个敌人（红色）
    for i in 0..5 {
        commands.spawn((
            Enemy,
            Speed(40.0 + i as f32 * 20.0),
            Mesh2d(meshes.add(Circle::new(20.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.8, 0.2, 0.2)))),
            Transform::from_xyz((i as f32 - 2.0) * 80.0, -150.0, 0.0),
        ));
    }

    commands.spawn_scene(bsn! {
        Text2d::new("Single / iter_mut / get / iter 四种查询形式")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(24.0),
        }
        Transform::from_xyz(0.0, 260.0, 0.0)
    });
}

// 形式1：Single —— 期望恰好一个带 Player 的实体。
// 若玩家数量不是 1（0 个或多个），Single 会 panic，用于快速暴露配置错误。
fn move_player(time: Res<Time>, mut player: Single<&mut Transform, With<Player>>) {
    // 上下往返：用 sin 让玩家在 y 轴来回摆动
    player.translation.y = (time.elapsed_secs() * 2.0).sin() * 120.0;
}

// 形式2：iter_mut + With/Without 过滤 —— 遍历所有敌人（排除玩家）。
// Without<Player> 让调度器知道此系统不碰玩家，从而能与 move_player 并行执行。
fn move_enemies(
    time: Res<Time>,
    mut query: Query<(&Speed, &mut Transform), (With<Enemy>, Without<Player>)>,
) {
    for (speed, mut transform) in &mut query {
        transform.translation.x += speed.0 * time.delta_secs();
        // 越界回绕到左侧
        if transform.translation.x > 400.0 {
            transform.translation.x = -400.0;
        }
    }
}

// 形式3：iter 只读 —— 统计所有 Speed 之和（不修改任何组件）。
fn sum_speed(query: Query<&Speed>, time: Res<Time>, mut last: Local<f32>) {
    // 每秒打印一次，避免刷屏
    if time.elapsed_secs() - *last > 1.0 {
        *last = time.elapsed_secs();
        let mut sum = 0.0;
        let mut count = 0;
        for speed in &query {
            sum += speed.0;
            count += 1;
        }
        info!("[查询] 共 {} 个实体带 Speed，速度总和 = {:.1}", count, sum);
    }
}

// 形式4：get —— 按实体精确访问（只读）。这里读取之前保存的玩家实体的 Speed。
fn report_player_speed(
    player: Res<PlayerEntity>,
    query: Query<&Speed>,
    time: Res<Time>,
    mut last: Local<f32>,
) {
    if time.elapsed_secs() - *last > 1.0 {
        *last = time.elapsed_secs();
        match query.get(player.0) {
            Ok(speed) => info!("[查询] 玩家速度 = {:.1}", speed.0),
            Err(_) => info!("[查询] 玩家实体没有 Speed 组件"),
        }
    }
}
