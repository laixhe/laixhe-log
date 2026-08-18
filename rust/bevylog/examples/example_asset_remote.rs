//! Bevy 0.19 入门示例：远程资产异步加载（AssetServer + https URL）。
//! 用 AssetServer 从网络 URL 异步加载图片，加载完成后显示为精灵。
//!
//! 学习重点：
//! - 需在 Cargo.toml 里给 bevy 加 `https` 特性（启用远程 https 资产读取）
//! - `AssetServer::load` 对远程 URL 和本地文件用法完全一致，都返回 `Handle<T>`
//! - 加载是异步的，用 `load_state` / `LoadState` 跟踪进度：
//!   - `Loading`：正在下载
//!   - `Loaded`：下载完成，可以渲染
//!   - `Failed`：下载失败（断网 / 404 / 超时），应优雅处理而不是崩溃
//! - 远程图片和本地图片一样，加载完成后 `Sprite::from_image` 直接显示
//!
//! 观察：屏幕显示加载状态，加载完成后显示远程图片；断网时显示「加载失败」。
//!
//! 提示：把 REMOTE_IMAGE_URL 换成任意图片 URL 即可测试。

use bevy::asset::LoadState;
use bevy::{prelude::*, text::FontSourceTemplate};

// 中文字体路径
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// 远程图片 URL（可替换成任意直链图片地址）
const REMOTE_IMAGE_URL: &str =
    "https://raw.githubusercontent.com/bevyengine/bevy/main/assets/branding/bevy_logo_light.png";

// 保存远程图片的句柄
#[derive(Resource)]
struct RemoteImage(Handle<Image>);

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        // .chain() 保证顺序：先检测加载，再更新状态文本
        .add_systems(Update, (check_loaded, update_text).chain())
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // 发起远程加载：和本地加载写法一样，只是路径是 https URL
    let handle: Handle<Image> = asset_server.load(REMOTE_IMAGE_URL);
    commands.insert_resource(RemoteImage(handle));

    // 状态文本
    commands.spawn_scene(bsn! {
        Text2d::new("")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(24.0),
        }
        Transform::from_xyz(0.0, -200.0, 0.0)
    });
}

// 检测加载状态：Loaded 后显示图片，Failed 后打印错误（都只处理一次）。
fn check_loaded(
    remote: Res<RemoteImage>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut finished: Local<bool>,
) {
    if *finished {
        return;
    }

    match asset_server.load_state(&remote.0) {
        LoadState::Loaded => {
            commands.spawn((Sprite::from_image(remote.0.clone()), Transform::default()));
            info!("[远程资产] 加载完成，已显示远程图片");
            *finished = true;
        }
        LoadState::Failed(e) => {
            error!("[远程资产] 加载失败：{e}");
            *finished = true;
        }
        _ => {}
    }
}

// 更新状态文本。
fn update_text(
    remote: Res<RemoteImage>,
    asset_server: Res<AssetServer>,
    mut text: Single<&mut Text2d>,
    mut last: Local<String>,
) {
    let status = match asset_server.load_state(&remote.0) {
        LoadState::NotLoaded => "未开始加载",
        LoadState::Loading => "加载中...",
        LoadState::Loaded => "已加载 ✓",
        LoadState::Failed(_) => "加载失败 ✗",
    };

    let new_text = format!("远程资产加载（AssetServer）\n状态：{status}");

    // 节流：内容变化才更新文本，避免 CJK 重排刷屏
    if *last != new_text {
        *last = new_text.clone();
        text.0 = new_text;
    }
}
