//! Bevy 0.19 入门示例：演示延迟操作（Deferred / Commands 的延迟应用）。
//!
//! Bevy 的 Commands（spawn / despawn / insert 等）是「延迟」执行的：
//! 命令先排队，等系统结束后的 sync point（ApplyDeferred）才真正应用到世界。
//! 所以同一系统内刚 spawn 的实体，用 Query 是查不到的。
//!
//! 学习重点：
//! - Commands 是延迟的：spawn 后不能立即查询到
//! - .chain() 会在有依赖的系统间自动插入 ApplyDeferred，刷新命令
//! - 因此「链式运行」的下一个系统能看到上一个系统刚生成的实体

use bevy::{prelude::*, text::FontSourceTemplate};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

#[derive(Component)]
struct Enemy;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        // .chain() 让 count_enemies 在 spawn_enemy 之后运行。
        // 因为 spawn_enemy 用了 Commands（延迟），Bevy 会自动在两者之间插入
        // ApplyDeferred 刷新命令，所以 count_enemies 能看到本帧刚生成的敌人。
        .add_systems(Update, (spawn_enemy, count_enemies).chain())
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // 初始生成 1 个敌人
    commands.spawn((
        Enemy,
        Mesh2d(meshes.add(Circle::new(30.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.8, 0.2, 0.2)))),
        Transform::from_xyz(-300.0, 0.0, 0.0),
    ));

    commands.spawn_scene(bsn! {
        Text2d::new("按空格生成敌人（观察 Commands 延迟应用）")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(24.0),
        }
        Transform::from_xyz(0.0, -260.0, 0.0)
    });
}

// 空格：生成新敌人。命令是延迟的，本系统内查询还看不到它。
fn spawn_enemy(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    enemies: Query<&Enemy>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        // 用当前敌人数作为 x 偏移，避免重叠（此刻 count 还没包含新敌人）
        let count = enemies.iter().count();
        commands.spawn((
            Enemy,
            Mesh2d(meshes.add(Circle::new(30.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.2, 0.8, 0.4)))),
            Transform::from_xyz(-300.0 + count as f32 * 70.0, 0.0, 0.0),
        ));

        // 关键：此刻查询仍只有旧敌人，新敌人要等命令刷新后才可见
        info!(
            "[延迟] 生成命令已排队，本系统内查询到 {} 个敌人",
            enemies.iter().count()
        );
    }
}

// 统计敌人：因为 .chain() 自动插入 ApplyDeferred，这里能看到上一系统刚生成的敌人
fn count_enemies(enemies: Query<&Enemy>) {
    info!("[延迟] 命令刷新后，共 {} 个敌人", enemies.iter().count());
}
