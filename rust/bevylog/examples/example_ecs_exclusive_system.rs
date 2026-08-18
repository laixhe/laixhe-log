//! Bevy 0.19 入门示例：演示排他系统（Exclusive System）。
//!
//! 普通系统只能通过系统参数（Res / Query / Commands 等）访问 World，
//! 而排他系统直接拿到 `&mut World`，可以任意读写，不受 ECS 并行调度约束，
//! 适合做「必须独占访问」的全局操作。
//!
//! 学习重点：
//! - 排他系统参数就是 `&mut World`
//! - world.resource::<T>() / resource_mut::<T>() 读写资源
//! - world.entities().len() 统计实体数量

use bevy::{prelude::*, text::FontSourceTemplate};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// 排他系统用来节流打印的时间记录
#[derive(Resource)]
struct LastReport(f32);

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .insert_resource(LastReport(0.0))
        .add_systems(Startup, setup)
        .add_systems(Update, (spin, exclusive_system))
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // 一个会旋转的圆（由普通系统 spin 驱动）
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(60.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.2, 0.8, 0.4)))),
        Transform::default(),
    ));

    commands.spawn_scene(bsn! {
        Text2d::new("排他系统：&mut World 直接访问（观察终端日志）")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(24.0),
        }
        Transform::from_xyz(0.0, -260.0, 0.0)
    });
}

// 普通系统：并行运行，用 Query 旋转所有圆
fn spin(time: Res<Time>, mut query: Query<&mut Transform, With<Mesh2d>>) {
    for mut transform in &mut query {
        transform.rotate_z(time.delta_secs());
    }
}

// 排他系统：独占 &mut World，每秒统计一次实体数量
fn exclusive_system(world: &mut World) {
    // 读时间资源
    let now = world.resource::<Time>().elapsed_secs();
    let last = world.resource::<LastReport>().0;

    if now - last > 1.0 {
        // 写资源
        world.resource_mut::<LastReport>().0 = now;

        // 直接统计实体数量
        let count = world.entities().len();
        info!("[排他系统] 当前共 {count} 个实体");
    }
}
