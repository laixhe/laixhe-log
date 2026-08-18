//! Bevy 0.19 入门示例：演示 ECS 性能优化的核心技巧（查询过滤 + 变更检测）。
//!
//! 学习重点：
//! 1. With<T> / Without<T> 查询过滤：只遍历需要的实体，减少每帧工作量
//! 2. Changed<T> / Added<T> 变更检测：只在数据变化时运行，避免每帧空转
//! 3. 系统并行：Bevy 按数据访问自动并行调度；访问的数据越精确，越容易并行
//!
//! 场景：500 个敌人，只有 5 个在移动。演示「只更新移动的 5 个」「只在受伤时报警」。

use bevy::{prelude::*, text::FontSourceTemplate};
// 随机数：用 random_range 随机挑一个存活敌人（rand 0.10 的方法在 RngExt trait 上）
use rand::RngExt;

// 敌人标记
#[derive(Component)]
struct Enemy;

// 移动标记（只有少数敌人有）
#[derive(Component)]
struct Moving;

// 移动速度
#[derive(Component)]
struct Speed(f32);

// 生命值
#[derive(Component)]
struct Health(f32);

// 攻击者标记：挂这个组件的实体是「攻击者」（本示例里是玩家）
#[derive(Component)]
struct Attacker;

// 伤害来源：区分不同攻击方式，日志里用来显示「伤害来自哪」
#[derive(Component, Clone, Copy)]
enum DamageSource {
    Melee, // 近战攻击
}

impl DamageSource {
    fn label(self) -> &'static str {
        match self {
            DamageSource::Melee => "近战攻击",
        }
    }
}

// 中文字体路径（bsn! 里用 FontSourceTemplate 自动加载，否则中文会显示为方块）
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

const ENEMY_COUNT: usize = 500;
const MOVING_COUNT: usize = 5;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.08)))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_enemies,
                // 先造成伤害，再触发警报（chain 保证 Changed<Health> 能在同一帧被观察到）
                (damage_random_enemy, alert_low_health).chain(),
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

    // 攻击者实体：一个「玩家」，负责造成伤害。这里没有给它加渲染组件，只作为伤害来源标识。
    commands.spawn((Attacker, DamageSource::Melee, Name::new("玩家")));

    // 生成 500 个敌人，其中前 5 个带 Moving（会移动）
    for i in 0..ENEMY_COUNT {
        let moving = i < MOVING_COUNT;
        let color = if moving {
            Color::srgb(0.2, 0.8, 0.4) // 移动的：绿色
        } else {
            Color::srgb(0.4, 0.4, 0.4) // 静止的：灰色
        };

        let x = (i as f32 % 25.0 - 12.0) * 22.0;
        let y = (i as f32 / 25.0 - 10.0) * 22.0;

        let mut entity = commands.spawn((
            Enemy,
            Health(100.0),
            Name::new(format!("敌人{i}")),
            Mesh2d(meshes.add(Circle::new(7.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(color))),
            Transform::from_xyz(x, y, 0.0),
        ));

        // 只有移动的敌人才挂 Moving + Speed（这样 With<Moving> 才能过滤）
        if moving {
            entity.insert((Moving, Speed(60.0)));
        }
    }

    info!("[ECS] 生成了 {ENEMY_COUNT} 个敌人，其中 {MOVING_COUNT} 个在移动");

    // 提示文本
    commands.spawn_scene(bsn! {
        Text2d::new("按空格给随机敌人造成伤害（观察 Changed 过滤）")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(24.0),
        }
        Transform::from_xyz(0.0, -260.0, 0.0)
    });
}

// 技巧1：With<Moving> 过滤 —— 只遍历带 Moving 的 5 个实体，而不是全部 500 个。
// 没有这个过滤，move_enemies 每帧都要遍历 500 个敌人再判断「这个要不要动」。
fn move_enemies(
    time: Res<Time>,
    mut query: Query<(&Speed, &mut Transform), With<Moving>>,
    // 节流：记录上次打印时间，每秒打印一次遍历量
    mut last_report: Local<f32>,
) {
    let mut moved = 0;
    for (speed, mut transform) in &mut query {
        transform.translation.x += speed.0 * time.delta_secs();
        moved += 1;
    }

    // 每秒打印一次实际遍历量，直观对比 With 过滤的效果
    if time.elapsed_secs() - *last_report > 1.0 {
        *last_report = time.elapsed_secs();
        info!(
            "[统计] move_enemies 每帧遍历 {} 个实体（共 {} 个，With<Moving> 过滤掉了 {} 个）",
            moved,
            ENEMY_COUNT,
            ENEMY_COUNT - moved
        );
    }
}

// 按空格随机挑一个存活敌人造成伤害（触发 Health 变化）
fn damage_random_enemy(
    keyboard: Res<ButtonInput<KeyCode>>,
    // 读取攻击者信息（Name = 攻击者标识，DamageSource = 伤害来源）
    attacker: Single<(&Name, &DamageSource), With<Attacker>>,
    // 敌人查询带上 Entity 和 Name，方便随机挑选并追踪具体受伤对象
    mut enemies: Query<(Entity, &Name, &mut Health), With<Enemy>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        let (attacker_name, source) = *attacker;

        // 记录本次攻击的开始时间，用于统计耗时
        let attack_start = std::time::Instant::now();

        // 先收集所有存活（生命值 > 0）的敌人实体，避免抽到已阵亡的敌人
        let alive: Vec<Entity> = enemies
            .iter()
            .filter(|(_, _, health)| health.0 > 0.0)
            .map(|(entity, _, _)| entity)
            .collect();

        info!(
            "[攻击] 开始：攻击者={} 来源={} 存活敌人={} 个",
            attacker_name,
            source.label(),
            alive.len()
        );

        if alive.is_empty() {
            info!("[攻击] 结束：没有存活敌人，跳过本次攻击");
            return;
        }

        // 随机挑一个存活敌人：random_range(0..len) 生成 [0, len) 的随机下标
        let mut rng = rand::rng();
        let target = alive[rng.random_range(0..alive.len())];

        if let Ok((_, victim_name, mut health)) = enemies.get_mut(target) {
            // 只有生命值大于 0 才继续扣血，且用 max(0.0) 夹紧，避免掉到负数反复触发警报
            let before = health.0;
            health.0 = (health.0 - 20.0).max(0.0);

            // 攻击耗时：从开始到扣血完成，单位微秒（µs）
            let elapsed = attack_start.elapsed();
            info!(
                "[攻击] 结束：目标ID={} 目标={} 生命值 {} -> {} 耗时={:.2}µs",
                target.index(),
                victim_name,
                before,
                health.0,
                elapsed.as_secs_f64() * 1_000_000.0,
            );
        }
    }
}

// 技巧2：Changed<Health> 过滤 —— 只在生命值变化时运行，平时零开销。
// 对比：如果去掉 Changed，这个系统每帧都要遍历全部 500 个敌人做空检查。
fn alert_low_health(query: Query<&Health, (With<Enemy>, Changed<Health>)>) {
    let mut checked = 0;
    for health in &query {
        checked += 1;
        if health.0 <= 0.0 {
            info!("[警报] 敌人生命值归零！");
        }
    }

    // 只在有生命值变化时打印遍历量；平时这个系统遍历 0 个（Changed 过滤生效）
    if checked > 0 {
        info!(
            "[统计] alert_low_health 本帧遍历 {} 个实体（Changed<Health> 过滤，平时为 0；朴素写法每帧遍历 {} 个）",
            checked, ENEMY_COUNT
        );
    }
}
