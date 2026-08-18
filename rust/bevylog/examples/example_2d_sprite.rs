//! Bevy 0.19 入门示例：用 AssetServer 加载图片生成 Sprite，演示资源加载与精灵显示。
//! 如果 assets/images/bevy_bird_dark.png 不存在，则降级为 100×100 像素的纯色方块。

use bevy::prelude::*;

// 两个常量指向同一个文件，但基点不同：
// - ASSET_REL 给 AssetServer::load 用，它的起始路径是项目根目录下的 assets/ 文件夹，
//   所以只传 assets/ 内部的相对路径 images/bevy_bird_dark.png。
// - ASSET_FS_PATH 给 std::path::Path::exists 用，它从项目根目录（Cargo.toml 所在目录）开始，
//   所以要带上 assets/ 前缀。
const ASSET_REL: &str = "images/bevy_bird_dark.png";
const ASSET_FS_PATH: &str = "assets/images/bevy_bird_dark.png";

fn main() -> AppExit {
    App::new()
        // 注册默认插件组：窗口、渲染器、输入、资产加载等
        .add_plugins(DefaultPlugins)
        // 启动时执行一次 setup 初始化场景
        .add_systems(Startup, setup)
        .run()
}

// 关于 Sprite 组件：bevy 提供的 2D 精灵组件，实体挂上它就会在 2D 相机里显示。
// 主要字段：image（图片句柄）、color（着色/变暗）、custom_size（指定大小）、rect（裁剪区域）。
// 各字段的详细说明见下方 Sprite {} 处的注释。
//
// 关键技巧：image 用空 Handle（Handle::default()）+ 设置 custom_size，
// 可以渲染一个由 color 决定颜色的纯色方块——不需要任何图片资产。
//
// 新手提示：asset_server.load 是异步加载——即使文件不存在，启动时也不会立刻报错，
// 只会显示空白窗口。因此本示例先用 std::path::Path::exists 判断文件是否存在，
// 存在则加载图片，不存在则降级为一个 100×100 像素的纯色方块（用空 Handle + custom_size）。
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 生成 2D 相机（必须有一个 Camera2d 才能看到画面）
    commands.spawn(Camera2d);

    // 判断资产文件是否真实存在于文件系统。
    // 注意：这里用 std::path::Path 同步检查，能立刻得到结果；
    // 而 asset_server.load 是异步的，即使文件不存在也会返回 Handle，
    // 直到渲染时才发现资产加载失败，不利于在启动时做降级判断。
    let has_image = std::path::Path::new(ASSET_FS_PATH).exists();

    // 根据 has_image 分流构建 Sprite 的三个关键字段：
    // - image：图片句柄。存在则用 AssetServer 加载；不存在则用空 Handle（Handle::default()）。
    // - custom_size：Sprite 大小。存在则用 None（按图片原始大小）；不存在则固定 100×100 像素。
    // - color：两个分支都用红色，会乘到每像素上（图片时整体偏红，纯色方块时即方块颜色）。
    let (image, custom_size, color) = if has_image {
        info!("找到资产 {}，加载图片精灵", ASSET_FS_PATH);
        (asset_server.load(ASSET_REL), None, Srgba::RED.into())
    } else {
        info!("未找到资产 {}，降级为 100×100 纯色方块精灵", ASSET_FS_PATH);
        (
            Handle::default(),
            Some(Vec2::new(100.0, 100.0)),
            Srgba::RED.into(),
        )
    };

    // 生成精灵：图片组件 + 变换组件 组成一个实体（spawn 的元组即 Bundle）。
    // 这里保持 commands.spawn((...)) 元组写法而不是 bsn!：因为 image / custom_size 依赖
    // 上面的 has_image 条件判断（图片缺失时回退纯色方块），属于命令式分支逻辑，
    // 无法用 bsn! 的声明式写法简洁表达。
    commands.spawn((
        Sprite {
            // image：图片句柄。空 Handle 配合 custom_size 可渲染纯色方块
            image,
            // color：会乘到图片每个像素上；纯色方块时即方块颜色
            color,
            // custom_size：指定 Sprite 大小；None 时为图片默认大小
            custom_size,
            // 其余字段（rect 等）用默认值
            ..default()
        },
        // 位置：屏幕中心上方 50 像素（2D 中 +y 朝上）
        Transform::from_xyz(0.0, 50.0, 0.0),
    ));
}
