//! Bevy 0.19 入门示例：演示音频播放控制（play / pause / stop / mute）。
//!
//! 学习重点：
//! - AudioSink 提供完整的播放控制方法
//! - play：恢复播放；pause：暂停；stop：停止；toggle_mute：静音切换
//! - 这些方法来自 AudioSinkPlayback trait
//!
//! 操作：空格播放/恢复，P 暂停，S 停止，M 静音切换。

use bevy::prelude::*;

const SOUND: &str = "audio/bg.wav";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, control_playback)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // 循环播放背景音乐（AudioSink 会在开始播放后自动插入）
    commands.spawn((
        AudioPlayer::new(asset_server.load(SOUND)),
        PlaybackSettings::LOOP,
    ));
    info!("[音频] 空格 播放/恢复 | P 暂停 | S 停止 | M 静音");
}

fn control_playback(
    keys: Res<ButtonInput<KeyCode>>,
    // AudioSink 是异步插入的，用 Option 处理尚未加载完成的情况
    mut sink: Option<Single<&mut AudioSink>>,
) {
    let Some(sink) = sink.as_mut() else {
        return;
    };

    if keys.just_pressed(KeyCode::Space) {
        sink.play();
        info!("[音频] 播放/恢复");
    }
    if keys.just_pressed(KeyCode::KeyP) {
        sink.pause();
        info!("[音频] 暂停");
    }
    if keys.just_pressed(KeyCode::KeyS) {
        sink.stop();
        info!("[音频] 停止");
    }
    if keys.just_pressed(KeyCode::KeyM) {
        sink.toggle_mute();
        info!("[音频] 静音 {}", if sink.is_muted() { "开" } else { "关" });
    }
}
