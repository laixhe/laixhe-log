//! Bevy 0.19 入门示例：演示音频系统（AudioPlayer / PlaybackSettings / AudioSink）。
//! 循环播放背景音乐，按空格播放一次短音效，按 P 暂停 / 继续背景音乐。
//!
//! 学习重点：
//! - AudioPlayer：音频播放组件，内部持有音频资源的 Handle，实体挂上它就开始播放
//! - PlaybackSettings：播放配置（循环 / 单次、音量、速度、播完是否自动销毁等）
//! - PlaybackSettings::LOOP / ONCE / DESPAWN 等预置常量
//! - AudioSink：Bevy 在开始播放时自动插入的组件，用于播放中实时控制（暂停 / 继续 / 音量 / 速度）
//! - 音效 vs 背景音乐：音效用 DESPAWN（播完自动销毁实体），背景音乐用 LOOP（持续循环）
//! - asset_server.load 是异步的，AudioSink 要等音频加载完成、开始播放后才会出现（因此用 Option<Single> 查询）

use bevy::{prelude::*, text::FontSourceTemplate};

// 中文字体路径
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// 音频资源路径（相对于 assets/ 目录，AssetServer 使用）
const BG_SOUND: &str = "audio/bg.wav";
const BLIP_SOUND: &str = "audio/blip.wav";

// 背景音乐标记组件：用于区分「背景音乐实体」和「音效实体」
#[derive(Component)]
struct BackgroundMusic;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        // 两个系统：按空格播音效、按 P 暂停 / 继续背景音乐
        .add_systems(Update, (play_sound_effect, control_background))
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // 背景音乐：循环播放。
    // AudioPlayer::new 接收 Handle<AudioSource>，asset_server.load 异步加载音频文件。
    // PlaybackSettings::LOOP 让这段音频循环播放；AudioPlayer 通过 #[require] 会自动带上
    // 默认的 PlaybackSettings::ONCE，这里显式写出 LOOP 覆盖它。
    commands.spawn((
        BackgroundMusic,
        AudioPlayer::new(asset_server.load(BG_SOUND)),
        PlaybackSettings::LOOP,
    ));

    // 提示文本（bsn! 声明式构建）
    commands.spawn_scene(bsn! {
        Text2d::new("音频：空格播音效 | P 暂停/继续背景音乐")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(30.0),
        }
        Transform::from_xyz(0.0, -300.0, 0.0)
    });
}

// 播放一次性音效：按空格播放一个短音，播完自动销毁。
fn play_sound_effect(
    keyboard: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        // PlaybackSettings::DESPAWN：播放一次，播完后自动 despawn 这个实体。
        // 每次按空格都生成一个新实体播放，播完自动清理，无需手动管理生命周期。
        commands.spawn((
            AudioPlayer::new(asset_server.load(BLIP_SOUND)),
            PlaybackSettings::DESPAWN,
        ));
        info!("[音频] 播放音效");
    }
}

// 控制背景音乐：按 P 切换暂停 / 继续。
fn control_background(
    keyboard: Res<ButtonInput<KeyCode>>,
    // AudioSink 由 Bevy 在音频开始播放时自动插入；加载是异步的，刚启动的几帧可能还没有。
    // 因此用 Option<Single<...>>：没有时就跳过本帧，有了再控制。
    background: Option<Single<&AudioSink, With<BackgroundMusic>>>,
) {
    let Some(background) = background else {
        return;
    };
    if keyboard.just_pressed(KeyCode::KeyP) {
        // toggle_playback 来自 AudioSinkPlayback trait：在「播放中」和「已暂停」之间切换。
        background.toggle_playback();
        info!(
            "[音频] 背景音乐 {}",
            if background.is_paused() {
                "暂停"
            } else {
                "继续"
            }
        );
    }
}
