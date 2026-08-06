//! Bevy 0.19 入门示例：生成 2D 相机、一个黄色圆形和一段文本，演示最基本的应用结构与场景初始化。

use bevy::prelude::*;

fn main() -> AppExit {
    // App 是 Bevy 应用的核心：它管理「世界」（所有实体/组件）和「调度器」（系统执行顺序）
    App::new()
        // ClearColor 是一个全局资源，控制窗口背景色。
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        // 注册默认插件组：包含窗口、渲染器、输入、资产加载等基础功能
        .add_plugins(DefaultPlugins)
        // 把 setup 系统加入 Startup 调度：只在应用启动时执行一次（用于初始化场景）
        .add_systems(Startup, setup)
        // 启动主循环：开始渲染窗口、每帧执行 Update 系统直到窗口关闭
        // 返回 AppExit 表示应用退出时的状态
        .run()
}

fn setup(
    // 允许向游戏世界添加内容（生成实体）
    mut commands: Commands,
    // 网格资源库：add() 把网格存入并返回一个 Handle（轻量引用）
    mut meshes: ResMut<Assets<Mesh>>,
    // 材质资源库：add() 把材质存入并返回一个 Handle
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 生成 2D 相机：没有相机实体就看不到任何东西（相机决定把世界渲染到屏幕的哪里）
    commands.spawn(Camera2d);

    // 生成圆形：spawn 接收一个「组件元组」（也叫 Bundle），元组里的所有组件会被加到同一个实体上
    commands.spawn((
        // Mesh2d 是 2D 网格组件，内部持有网格的 Handle
        // Circle::new(50.0) 的 50.0 是半径（像素）
        Mesh2d(meshes.add(Circle::new(50.0))),
        // MeshMaterial2d 是 2D 材质组件，内部持有材质的 Handle
        // srgb 的三个参数是 R/G/B，范围 0.0~1.0（不是 0~255）；1,1,0 = 黄色
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(1.0, 1.0, 0.0)))),
        // 变换（位置 / 旋转 / 缩放），default() = 原点、无旋转、缩放为 1
        Transform::default(),
    ));

    // 生成文本
    commands.spawn((
        // Text2d 是在世界中渲染文本的组件（区别于固定在屏幕上的 UI 文本）
        Text2d::new("Hello, world!"),
        // TextFont 是文本样式组件（字号、字体、行高等）。
        // 这里只显式指定 font_size；其余字段用 ..default() 填充，
        // 其中 font 字段默认用 Bevy 内置字体。
        // 要用自定义字体：在 setup 参数加 asset_server: Res<AssetServer>，
        // 然后在这里显式写 font: asset_server.load("fonts/xxx.ttf") 覆盖默认字体。
        TextFont {
            font_size: FontSize::Px(30.0),
            ..default()
        },
        // 变换（位置）：2D 坐标系原点在屏幕中心，+x 向右、+y 向上；
        // y = -100 表示在中心点下方 100 像素
        Transform::from_xyz(0.0, -100.0, 0.0),
    ));
}
