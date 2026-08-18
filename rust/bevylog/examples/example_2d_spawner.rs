//! Bevy 0.19 入门示例：演示定时生成物体（Timer + Spawn + Despawn）。
//!
//! 学习重点：
//! - Timer：每隔固定时间触发一次（TimerMode::Repeating）
//! - 生成物体：commands.spawn 在运行时动态创建实体
//! - 销毁物体：超出屏幕后 commands.entity(...).despawn() 回收
//!
//! 观察：圆点从顶部不断下落，到底部后自动销毁。

use bevy::prelude::*;

#[derive(Resource)]
struct SpawnTimer(Timer);

#[derive(Component)]
struct Falling {
    speed: f32,
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.06, 0.06, 0.08)))
        .insert_resource(SpawnTimer(Timer::from_seconds(0.3, TimerMode::Repeating)))
        .add_systems(Startup, setup)
        .add_systems(Update, (spawn, fall))
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

// 定时生成下落的圆点（x 位置随时间摆动）
fn spawn(
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        let x = (time.elapsed_secs() * 13.7).sin() * 280.0;
        let speed = 160.0 + (time.elapsed_secs() * 7.0).sin() * 60.0;

        commands.spawn((
            Falling { speed },
            Mesh2d(meshes.add(Circle::new(14.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.9, 0.6, 0.3)))),
            Transform::from_xyz(x, 250.0, 0.0),
        ));
    }
}

// 圆点下落，超出屏幕后销毁
fn fall(time: Res<Time>, mut commands: Commands, mut q: Query<(Entity, &mut Transform, &Falling)>) {
    for (entity, mut tf, falling) in &mut q {
        tf.translation.y -= falling.speed * time.delta_secs();
        if tf.translation.y < -260.0 {
            commands.entity(entity).despawn();
        }
    }
}
