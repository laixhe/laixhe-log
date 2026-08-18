//! Bevy 0.19 入门示例：演示观察者 / 触发器（Observer / EntityEvent）。
//!
//! Observer 是「即时事件」：commands.trigger 触发后，当帧立即由观察者处理
//! （区别于 example_ecs_events 的缓冲式 Event，后者要到下一帧才被 EventReader 读到）。
//!
//! 学习重点：
//! - #[derive(EntityEvent)] 自定义实体事件
//! - commands.trigger() 触发事件
//! - .observe() 挂到实体上的观察者（按 entity 字段路由）
//! - .add_observer() 全局观察者（所有事件都触发）

use bevy::{prelude::*, text::FontSourceTemplate};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Health(f32);

// 实体事件：entity 字段决定路由，挂在该实体上的观察者会收到
#[derive(EntityEvent)]
struct DamageEvent {
    entity: Entity,
    amount: f32,
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        .add_systems(Update, attack_on_space)
        // 全局观察者：所有 DamageEvent 都触发（统一日志）
        .add_observer(log_damage)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    for i in 0..5 {
        let x = (i as f32 - 2.0) * 90.0;
        commands
            .spawn((
                Enemy,
                Health(100.0),
                Mesh2d(meshes.add(Circle::new(35.0))),
                MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.8, 0.2, 0.2)))),
                Transform::from_xyz(x, 0.0, 0.0),
            ))
            // 每个敌人挂一个观察者，只有「路由到自己」的伤害事件才会触发
            .observe(on_enemy_damaged);
    }

    commands.spawn_scene(bsn! {
        Text2d::new("按空格随机攻击一个敌人（Observer 当帧处理）")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(24.0),
        }
        Transform::from_xyz(0.0, -260.0, 0.0)
    });
}

// 按空格触发伤害事件
fn attack_on_space(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    enemies: Query<Entity, With<Enemy>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        // 简化：取第一个敌人（实际项目可用 rand 随机挑一个）
        if let Some(target) = enemies.iter().next() {
            commands.trigger(DamageEvent {
                entity: target,
                amount: 25.0,
            });
        }
    }
}

// 全局观察者：任何伤害事件都触发，负责统一日志
fn log_damage(event: On<DamageEvent>) {
    info!(
        "[观察者] 全局日志：实体 {} 受到 {:.0} 点伤害",
        event.entity.index(),
        event.amount
    );
}

// 实体观察者：只处理路由到自己实体上的伤害，扣血并判断是否摧毁
fn on_enemy_damaged(
    event: On<DamageEvent>,
    mut commands: Commands,
    mut healths: Query<&mut Health, With<Enemy>>,
) {
    if let Ok(mut health) = healths.get_mut(event.entity) {
        health.0 -= event.amount;
        info!(
            "[观察者] 实体 {} 生命值 = {:.0}",
            event.entity.index(),
            health.0
        );

        if health.0 <= 0.0 {
            commands.entity(event.entity).despawn();
            info!("[观察者] 实体 {} 已被摧毁", event.entity.index());
        }
    }
}
