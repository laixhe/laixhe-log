use bevy::prelude::*;

fn main() {
    App::new()
        // 添加默认功能（窗口、渲染器、输入）
        .add_plugins(DefaultPlugins)
        // 仅在启动时执行一次
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 生成 2D 相机
    commands.spawn(Camera2d);

    // 生成圆形
    commands.spawn((
        // 网格
        Mesh2d(meshes.add(Circle::new(50.0))),
        // 材质
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(1.0, 1.0, 0.0)))),
        // 变换（位置）
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // 生成文本
    commands.spawn((
        // 文本
        Text2d::new("Hello, world!"),
        // 变换（位置）
        Transform::from_xyz(0.0, -100.0, 0.0),
    ));
}
