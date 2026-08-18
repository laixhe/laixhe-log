//! Bevy 0.19 入门示例：演示资产热重载。
//!
//! Bevy 的 AssetServer 默认会「watch」资产文件：运行期间修改图片/音频等文件，
//! Bevy 会自动重新加载（热重载），并通过 AssetEvent 消息通知系统。
//!
//! 学习重点：
//! - AssetServer::load 加载资产，返回 Handle
//! - AssetEvent<A> 是 Message，用 MessageReader 读取加载/修改/移除事件
//! - 修改 assets 下的文件后，Bevy 自动热重载

use bevy::{prelude::*, text::FontSourceTemplate};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        .add_systems(Update, watch_asset)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // 加载图片：Bevy 默认会 watch 文件，修改后自动热重载
    let handle = asset_server.load("images/bevy_bird_dark.png");
    commands.spawn((
        Sprite {
            image: handle,
            ..default()
        },
        Transform::default(),
    ));

    commands.spawn_scene(bsn! {
        Text2d::new("资产热重载：修改 assets/images/bevy_bird_dark.png 后会自动重载")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(20.0),
        }
        Transform::from_xyz(0.0, -260.0, 0.0)
    });
}

// 监听图片资产的加载 / 修改 / 移除事件
fn watch_asset(mut events: MessageReader<AssetEvent<Image>>) {
    for event in events.read() {
        match event {
            AssetEvent::LoadedWithDependencies { .. } => info!("[资产] 图片已加载"),
            AssetEvent::Modified { .. } => info!("[资产] 图片文件已修改，自动热重载"),
            AssetEvent::Removed { .. } => info!("[资产] 图片已移除"),
            _ => {}
        }
    }
}
