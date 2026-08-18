//! Bevy 0.19 入门示例：演示资源（Asset）基础：加载、句柄、Assets<T> 访问。
//!
//! 学习重点：
//! - AssetServer::load 异步加载资源，返回 Handle（轻量句柄）
//! - Handle 可克隆，克隆共享同一份底层资源，不复制数据
//! - Assets<T> 是资源容器，用 handle 从里面取出实际数据
//!
//! 观察：两个精灵显示同一张图（共享句柄），日志打印图片尺寸。

use bevy::prelude::*;

// 用 Resource 保存 handle，方便后续系统访问
#[derive(Resource)]
struct Logo(Handle<Image>);

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        .add_systems(Update, report)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // 加载图片资源，返回 Handle
    let handle: Handle<Image> = asset_server.load("images/bevy_logo.png");

    // 克隆 handle：两个精灵共享同一份图片数据（不会复制底层数据）
    commands.spawn((
        Sprite {
            image: handle.clone(),
            ..default()
        },
        Transform::from_xyz(-150.0, 0.0, 0.0),
    ));
    commands.spawn((
        Sprite {
            image: handle.clone(),
            ..default()
        },
        Transform::from_xyz(150.0, 0.0, 0.0),
    ));

    // 把原始 handle 存进 Resource
    commands.insert_resource(Logo(handle));
}

fn report(time: Res<Time>, logo: Res<Logo>, images: Res<Assets<Image>>, mut last_log: Local<f32>) {
    if time.elapsed_secs() - *last_log < 1.0 {
        return;
    }
    *last_log = time.elapsed_secs();

    // 通过 handle 从 Assets<Image> 容器里取出实际数据
    if let Some(image) = images.get(&logo.0) {
        info!(
            "[资源] 图片已加载，尺寸 {}x{}",
            image.width(),
            image.height()
        );
    } else {
        info!("[资源] 图片加载中...");
    }
}
