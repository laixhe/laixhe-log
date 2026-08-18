//! Bevy 0.19 入门示例：演示多音源混合（多个 AudioSink 同时播放）。
//!
//! 学习重点：
//! - 每个 AudioPlayer 实体在开始播放后会自动插入一个 AudioSink
//! - 多个音源可以同时播放，各自有独立的 AudioSink
//! - 用 Query<&AudioSink> 统计当前同时播放的音源数量
//!
//! 操作：空格连续触发多个音效，背景音乐始终循环，日志显示当前音源数量。

use bevy::prelude::*;

const BG_SOUND: &str = "audio/bg.wav";
const BLIP_SOUND: &str = "audio/blip.wav";

#[derive(Component)]
struct BackgroundMusic;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (play_sfx, count_sinks))
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // 背景音乐循环播放
    commands.spawn((
        BackgroundMusic,
        AudioPlayer::new(asset_server.load(BG_SOUND)),
        PlaybackSettings::LOOP,
    ));
    info!("[音频] 按 空格 连续触发音效");
}

// 空格触发一个一次性音效（可与背景音乐、其他音效同时播放）
fn play_sfx(
    keys: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::Space) {
        commands.spawn((
            AudioPlayer::new(asset_server.load(BLIP_SOUND)),
            PlaybackSettings::DESPAWN,
        ));
    }
}

// 每秒统计当前正在播放的音源数量
fn count_sinks(time: Res<Time>, sinks: Query<&AudioSink>, mut last_log: Local<f32>) {
    if time.elapsed_secs() - *last_log < 1.0 {
        return;
    }
    *last_log = time.elapsed_secs();

    let count = sinks.iter().count();
    info!("[音频] 当前同时播放的音源数量: {count}");
}
