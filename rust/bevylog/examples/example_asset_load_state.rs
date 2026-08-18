//! Bevy 0.19 入门示例：演示资源加载状态（LoadState 异步检测）。
//!
//! 学习重点：
//! - AssetServer::load 是异步的，返回 Handle 时资产可能还没加载完
//! - 用 asset_server.load_state / is_loaded 判断是否加载完成
//! - 加载完成后再渲染，避免拿到空数据
//!
//! 观察：日志打印加载状态，加载完成后才显示图片。

use bevy::asset::LoadState;
use bevy::prelude::*;

#[derive(Resource)]
struct Logo(Handle<Image>);

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_when_loaded)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut logo: ResMut<Logo>) {
    commands.spawn(Camera2d);
    // 发起加载，但此时资产可能还没就绪
    logo.0 = asset_server.load("images/bevy_logo.png");
}

fn spawn_when_loaded(
    logo: Res<Logo>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut commands: Commands,
    mut spawned: Local<bool>,
    mut last_log: Local<f32>,
) {
    if *spawned {
        return;
    }

    let state = asset_server.load_state(&logo.0);

    // 每秒打印一次状态，避免刷屏
    if time.elapsed_secs() - *last_log >= 1.0 {
        *last_log = time.elapsed_secs();
        info!("[资源] 加载状态: {state:?}");
    }

    // 加载完成后才渲染
    if let LoadState::Loaded = state {
        info!("[资源] 图片加载完成，渲染精灵");
        commands.spawn((
            Sprite {
                image: logo.0.clone(),
                ..default()
            },
            Transform::default(),
        ));
        *spawned = true;
    }
}
