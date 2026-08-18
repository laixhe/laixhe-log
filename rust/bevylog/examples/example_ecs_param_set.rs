//! Bevy 0.19 入门示例：演示 ParamSet（系统参数集合）。
//!
//! 一个系统里如果要用多个「可变访问同一组件」的 Query，会因为借用冲突无法直接写。
//! ParamSet 把多个参数打包，允许按顺序（p0 / p1 / ...）逐个访问，避免冲突。
//!
//! 学习重点：
//! - ParamSet<(Query<..>, Query<..>)> 打包多个参数
//! - param.p0() / p1() 按索引逐个访问
//! - 解决同一系统内多次可变访问同一组件的问题

use bevy::{prelude::*, text::FontSourceTemplate};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        .add_systems(Update, move_entities)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // 玩家（绿色）
    commands.spawn((
        Player,
        Mesh2d(meshes.add(Circle::new(30.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.2, 0.8, 0.4)))),
        Transform::from_xyz(-200.0, 0.0, 0.0),
    ));

    // 敌人（红色）
    for i in 0..4 {
        commands.spawn((
            Enemy,
            Mesh2d(meshes.add(Circle::new(25.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.8, 0.2, 0.2)))),
            Transform::from_xyz(-300.0 + i as f32 * 70.0, 100.0, 0.0),
        ));
    }

    commands.spawn_scene(bsn! {
        Text2d::new("ParamSet：一个系统内分别移动玩家和敌人")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(24.0),
        }
        Transform::from_xyz(0.0, -260.0, 0.0)
    });
}

// 一个系统里用两个 Query 都可变访问 Transform，直接写会冲突；
// 用 ParamSet 打包后按 p0 / p1 逐个访问即可。
fn move_entities(
    time: Res<Time>,
    mut transforms: ParamSet<(
        Query<&mut Transform, With<Player>>,
        Query<&mut Transform, With<Enemy>>,
    )>,
) {
    // p0：玩家上下往返
    for mut transform in &mut transforms.p0() {
        transform.translation.y = (time.elapsed_secs() * 2.0).sin() * 120.0;
    }

    // p1：敌人水平移动
    for mut transform in &mut transforms.p1() {
        transform.translation.x += 100.0 * time.delta_secs();
        if transform.translation.x > 400.0 {
            transform.translation.x = -300.0;
        }
    }
}
