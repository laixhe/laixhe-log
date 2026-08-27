//! Bevy 0.19 入门示例：演示 bevy_embedded_assets —— 把资源打包进 exe。
//!
//! 传统方式：资源放在 assets/ 文件夹，运行时从磁盘读取（exe 必须和 assets 一起分发）。
//! 嵌入方式：编译时把整个 assets/ 文件夹的字节嵌入二进制，运行时直接从内存读取
//! （exe 单独一个文件就能跑，无需外部资源文件夹）。
//!
//! 学习重点：
//! - 插件：EmbeddedAssetPlugin 必须在 DefaultPlugins 之前添加
//! - PluginMode::ReplaceDefault：用「嵌入式资源源」替换默认的文件源，
//!   之后 asset_server.load(...) 全部从二进制里读
//! - 代价：改了资源必须重新编译；运行时无法加载 assets 文件夹里新增的文件
//! - 其他模式：AutoLoad（默认，用 embedded:// 前缀访问）、ReplaceAndFallback（嵌入优先、缺省回退文件）
//!
//! 操作：窗口显示 bevy logo + 播放音效 + 中文文本——所有资源都来自 exe 内部。

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::text::{Font, FontSource};
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};

fn main() -> AppExit {
    App::new()
        // 注意：EmbeddedAssetPlugin 必须在 DefaultPlugins（内含 AssetPlugin）之前添加
        .add_plugins((
            EmbeddedAssetPlugin {
                mode: PluginMode::ReplaceDefault,
            },
            DefaultPlugins.set(LogPlugin {
                // 静默 icu_provider 缺 CJK 字典数据的警告（保留默认的 wgpu/naga 过滤）
                filter: "wgpu=error,naga=warn,icu_provider=off".to_string(),
                ..default()
            }),
        ))
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // 这些 load 路径和普通用法完全一样，但数据来自 exe 内部的嵌入字节
    let logo: Handle<Image> = asset_server.load("images/bevy_logo.png");
    let blip: Handle<AudioSource> = asset_server.load("audio/blip.wav");
    let font: Handle<Font> = asset_server.load("fonts/Yozai-Regular.ttf");

    // 显示嵌入的 logo 图片
    commands.spawn((
        Sprite::from_image(logo),
        Transform::from_xyz(0.0, 80.0, 0.0),
    ));
    // 播放嵌入的音效
    commands.spawn(AudioPlayer::new(blip));
    // 用嵌入的字体显示文本
    commands.spawn((
        Text2d::new("资源已嵌入 exe（EmbeddedAssetPlugin）"),
        TextColor(Color::WHITE),
        TextFont {
            font: FontSource::Handle(font),
            font_size: FontSize::Px(26.0),
            ..default()
        },
        Transform::from_xyz(0.0, -200.0, 0.0),
    ));

    info!("[嵌入资源] 图片 / 音效 / 字体均从二进制内嵌数据加载");
}
