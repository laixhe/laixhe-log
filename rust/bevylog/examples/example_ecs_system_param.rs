//! Bevy 0.19 入门示例：自定义系统参数（SystemParam derive）。
//!
//! 当多个系统反复声明相同的参数组合时，可以用 #[derive(SystemParam)] 打包成一个参数，
//! 减少签名重复、让代码更清晰。
//!
//! 学习重点：
//! - #[derive(SystemParam)] 自定义参数
//! - 参数内可组合 Query / Res / ResMut / Local 等
//! - 在系统里直接用自定义参数类型

use bevy::ecs::system::SystemParam;
use bevy::{prelude::*, text::FontSourceTemplate};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Health(f32);

#[derive(Resource, Default)]
struct Score(u32);

// 自定义系统参数：把「敌人生命值查询 + 分数资源」打包成一个参数。
// 任何系统声明一个 EnemyHealth 参数，就能同时访问这两个数据。
#[derive(SystemParam)]
struct EnemyHealth<'w, 's> {
    healths: Query<'w, 's, &'static mut Health, With<Enemy>>,
    score: ResMut<'w, Score>,
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .init_resource::<Score>()
        .add_systems(Startup, setup)
        .add_systems(Update, damage_all)
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
        Text2d::new("按空格攻击所有敌人（自定义 SystemParam）")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(24.0),
        }
        Transform::from_xyz(0.0, -260.0, 0.0)
    });
}

// 使用自定义系统参数：一个 EnemyHealth 就同时拿到了敌人查询和分数
fn damage_all(keyboard: Res<ButtonInput<KeyCode>>, mut param: EnemyHealth) {
    if keyboard.just_pressed(KeyCode::Space) {
        let mut hit = 0;
        for mut health in &mut param.healths {
            health.0 -= 10.0;
            hit += 1;
        }
        param.score.0 += 1;
        info!("[系统参数] 攻击了 {} 个敌人，分数 = {}", hit, param.score.0);
    }
}
