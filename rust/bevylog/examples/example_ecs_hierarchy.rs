//! Bevy 0.19 入门示例：演示父子层级（ChildOf / Children）与局部 / 全局 Transform。
//! 中心一个父圆旋转，两个子圆挂在它下面，子圆跟随父圆一起转。
//!
//! 学习重点：
//! - ChildOf：挂在子实体上、指向父实体（Bevy 0.19 用关系组件，旧版叫 Parent）
//! - with_children：在父实体上声明子实体，自动建立 ChildOf / Children 关系
//! - Transform 是「局部坐标」（相对父实体），GlobalTransform 是「全局坐标」（相对世界原点）
//! - 父实体旋转时，子实体的局部 Transform 不变，但 GlobalTransform 跟着变

use bevy::prelude::*;

// 父实体标记：用于系统里旋转它
#[derive(Component)]
struct Spinner;

// 子实体标记：用于系统里读取它的局部 / 全局变换
#[derive(Component)]
struct Orbiter;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .add_systems(Update, spin_and_log)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // 父实体：中心黄色圆，带 Spinner 标记
    commands
        .spawn((
            Spinner,
            Mesh2d(meshes.add(Circle::new(40.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.9, 0.8, 0.2)))),
            Transform::default(),
        ))
        // with_children：下面声明的实体都是它的子实体
        .with_children(|parent| {
            // 子实体 1：蓝色小圆，局部偏移 (120, 0)，会跟着父实体一起转
            parent.spawn((
                Orbiter,
                Mesh2d(meshes.add(Circle::new(20.0))),
                MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.2, 0.6, 1.0)))),
                Transform::from_xyz(120.0, 0.0, 0.0),
            ));
            // 子实体 2：红色小圆，局部偏移 (0, 90)
            parent.spawn((
                Mesh2d(meshes.add(Circle::new(15.0))),
                MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.9, 0.3, 0.3)))),
                Transform::from_xyz(0.0, 90.0, 0.0),
            ));
        });
}

fn spin_and_log(
    time: Res<Time>,
    // 旋转父实体（局部旋转，子实体会跟着一起转）。
    // 注意：这里用 Without<Orbiter> 排除子实体——因为下面的 orbiter 查询也访问 Transform，
    // 如果不加，Bevy 无法静态证明两个查询访问的是不同实体，运行时会报 B0001 冲突。
    mut spinner: Single<&mut Transform, (With<Spinner>, Without<Orbiter>)>,
    // 读取子实体的局部 Transform 和全局 GlobalTransform
    orbiter: Single<(&Transform, &GlobalTransform), With<Orbiter>>,
    // 系统本地状态：记录上次打印日志的时间，避免每帧刷屏
    mut last_log: Local<f32>,
) {
    spinner.rotation = Quat::from_rotation_z(time.elapsed_secs());

    // 每 1 秒打印一次，观察局部坐标与全局坐标的区别
    if time.elapsed_secs() - *last_log > 1.0 {
        *last_log = time.elapsed_secs();
        let (local, global) = orbiter.into_inner();
        // 局部坐标永远是 (120, 0)，全局坐标随父实体旋转而改变
        info!(
            "子实体 局部 = ({:.0}, {:.0}) | 全局 = ({:.0}, {:.0})",
            local.translation.x,
            local.translation.y,
            global.translation().x,
            global.translation().y,
        );
    }
}
