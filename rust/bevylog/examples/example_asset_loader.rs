//! Bevy 0.19 入门示例：演示 bevy_asset_loader —— 集中式资源加载。
//!
//! 传统方式：每个系统各自 asset_server.load(...)，还要自己判断加载状态。
//! bevy_asset_loader：用 #[derive(AssetCollection)] 声明一个「资源集合」，
//! 在 Loading 状态统一加载，全部完成后自动切换到下一状态，
//! 并把集合插入为 Resource —— 此时可以放心使用集合里的所有句柄。
//!
//! 学习重点：
//! - #[derive(AssetCollection, Resource)] + #[asset(path = "...")] 声明资源集合
//! - add_loading_state(LoadingState::new(加载状态).continue_to_state(下一状态).load_collection::<T>())
//! - init_state 声明状态（Loading 默认 + Ready）
//! - OnEnter(Ready) 里使用集合（此时资源必然已加载完毕）
//!
//! 操作：启动后短暂显示「加载中...」，加载完成后显示 bevy logo、播放音效、显示中文文本。

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::text::{Font, FontSource};
use bevy_asset_loader::prelude::*;

// 中文字体路径（加载画面直接用它，与集合并行加载；bevy 按路径缓存，不会重复加载）
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// 应用状态：Loading（默认，加载资源）→ Ready（加载完成，进入内容）
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Loading,
    Ready,
}

// 资源集合：一个结构体集中声明所有要加载的资源。
// 每个字段是一个资源句柄，#[asset(path)] 指定文件路径。
#[derive(AssetCollection, Resource)]
struct GameAssets {
    #[asset(path = "images/bevy_logo.png")]
    logo: Handle<Image>,
    #[asset(path = "audio/blip.wav")]
    blip: Handle<AudioSource>,
    #[asset(path = "fonts/Yozai-Regular.ttf")]
    font: Handle<Font>,
}

// 标记组件：加载中的提示文本（进入 Ready 时销毁）
#[derive(Component)]
struct LoadingUI;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            // 静默 icu_provider 缺 CJK 字典数据的警告（保留默认的 wgpu/naga 过滤）
            filter: "wgpu=error,naga=warn,icu_provider=off".to_string(),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        // 声明状态（Loading 为默认初始状态）
        .init_state::<GameState>()
        // 配置加载状态：
        // - LoadingState::new(Loading)：进入 Loading 状态时开始加载
        // - continue_to_state(Ready)：全部加载完成后自动切换到 Ready
        // - load_collection::<GameAssets>()：加载该资源集合
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Ready)
                .load_collection::<GameAssets>(),
        )
        .add_systems(OnEnter(GameState::Loading), setup_loading)
        .add_systems(OnExit(GameState::Loading), teardown_loading)
        // OnEnter(Ready)：此时 GameAssets 已插入为资源，可以放心使用
        .add_systems(OnEnter(GameState::Ready), setup_ready)
        .run()
}

// 加载中：只生成相机和提示文本
fn setup_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        LoadingUI,
        Text2d::new("加载中..."),
        TextColor(Color::WHITE),
        TextFont {
            // 加载画面自己通过 AssetServer 加载字体（与集合同一路径，共享缓存）
            font: FontSource::Handle(asset_server.load(FONT_PATH)),
            font_size: FontSize::Px(32.0),
            ..default()
        },
    ));
}

// 离开加载状态：销毁加载 UI
fn teardown_loading(mut commands: Commands, loading_ui: Query<Entity, With<LoadingUI>>) {
    for entity in &loading_ui {
        commands.entity(entity).despawn();
    }
}

// 进入 Ready：GameAssets 一定加载完毕，直接使用集合里的句柄
fn setup_ready(mut commands: Commands, assets: Res<GameAssets>) {
    // 显示 logo 图片
    commands.spawn((
        Sprite::from_image(assets.logo.clone()),
        Transform::from_xyz(0.0, 80.0, 0.0),
    ));
    // 播放音效（AudioPlayer 挂上就开始播放，一次）
    commands.spawn(AudioPlayer::new(assets.blip.clone()));
    // 用集合里加载的中文字体显示文本
    commands.spawn((
        Text2d::new("资源加载完成！"),
        TextColor(Color::WHITE),
        TextFont {
            font: FontSource::Handle(assets.font.clone()),
            font_size: FontSize::Px(28.0),
            ..default()
        },
        Transform::from_xyz(0.0, -200.0, 0.0),
    ));
    info!("[资源加载] 全部资源加载完成，进入 Ready 状态");
}
