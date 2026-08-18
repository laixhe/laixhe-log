//! Bevy 0.19 入门示例：演示消息系统（Message / MessageWriter / MessageReader）。
//!
//! Message 是「双缓冲队列」事件：写入方用 MessageWriter 写入，读取方用 MessageReader 消费。
//! 与 Event（观察者模式，触发即执行）不同，Message 需要显式读取，且可能跨帧延迟。
//!
//! 学习重点：
//! - #[derive(Message)] 自定义消息
//! - MessageWriter<M>::write 写入消息
//! - MessageReader<M>::read 读取并消费消息
//! - 用 .chain() 保证「写入先于读取」，当帧就能读到

use bevy::{prelude::*, text::FontSourceTemplate};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Health(f32);

// 伤害消息：携带目标实体和伤害值
#[derive(Message)]
struct DamageMessage {
    entity: Entity,
    amount: f32,
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        // .chain() 保证发送先于接收，当帧就能读到消息
        .add_systems(Update, (send_damage, apply_damage).chain())
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    for i in 0..5 {
        commands.spawn((
            Enemy,
            Health(100.0),
            Mesh2d(meshes.add(Circle::new(30.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.8, 0.2, 0.2)))),
            Transform::from_xyz((i as f32 - 2.0) * 90.0, 0.0, 0.0),
        ));
    }

    commands.spawn_scene(bsn! {
        Text2d::new("按空格发送伤害消息（Message 双缓冲队列）")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(24.0),
        }
        Transform::from_xyz(0.0, -260.0, 0.0)
    });
}

// 发送方：按空格写入一条伤害消息
fn send_damage(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut writer: MessageWriter<DamageMessage>,
    enemies: Query<Entity, With<Enemy>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        if let Some(target) = enemies.iter().next() {
            writer.write(DamageMessage {
                entity: target,
                amount: 25.0,
            });
            info!("[消息] 写入伤害消息 -> 实体 {}", target.index());
        }
    }
}

// 接收方：读取并应用伤害
fn apply_damage(
    mut reader: MessageReader<DamageMessage>,
    mut healths: Query<&mut Health, With<Enemy>>,
) {
    for msg in reader.read() {
        if let Ok(mut health) = healths.get_mut(msg.entity) {
            health.0 -= msg.amount;
            info!(
                "[消息] 实体 {} 生命值 = {:.0}",
                msg.entity.index(),
                health.0
            );
        }
    }
}
