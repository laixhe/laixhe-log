//! Bevy 0.19 入门示例：演示音频音量 / 速度的实时控制。
//! 循环播放背景音乐，用键盘实时调节音量和播放速度。
//!
//! 学习重点：
//! - AudioSink：音频播放的「控制句柄」，音频加载完成后自动插入到实体上
//! - set_volume(Volume::Linear(倍数)) 调音量（0.0 ~ 1.0）
//! - set_speed(倍数) 调播放速度（0.5 慢速 ~ 2.0 快速，改变音调）
//! - 音量用 Volume 枚举表示（Linear 线性 / Decibels 分贝），不是普通 f32

use bevy::audio::Volume;
use bevy::prelude::*;

// 音频资源路径
const SOUND: &str = "audio/bg.wav";

// 音频状态资源：记录当前音量和速度（Local 无法区分未初始化，用资源更清晰）
#[derive(Resource)]
struct AudioState {
    volume: f32,
    speed: f32,
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(AudioState {
            volume: 1.0,
            speed: 1.0,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, control_audio)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 循环播放背景音乐（AudioSink 会在音频加载完成后自动插入）
    commands.spawn((
        AudioPlayer::new(asset_server.load(SOUND)),
        PlaybackSettings::LOOP,
    ));
}

// 键盘控制：上下方向键调音量，左右方向键调速度
fn control_audio(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut sink: Single<&mut AudioSink>,
    mut state: ResMut<AudioState>,
) {
    let mut changed = false;

    // 音量：0.0 ~ 1.0，每次 0.1
    if keyboard.just_pressed(KeyCode::ArrowUp) {
        state.volume = (state.volume + 0.1).min(1.0);
        changed = true;
    }
    if keyboard.just_pressed(KeyCode::ArrowDown) {
        state.volume = (state.volume - 0.1).max(0.0);
        changed = true;
    }

    // 速度：0.5 ~ 2.0，每次 0.1
    if keyboard.just_pressed(KeyCode::ArrowRight) {
        state.speed = (state.speed + 0.1).min(2.0);
        changed = true;
    }
    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        state.speed = (state.speed - 0.1).max(0.5);
        changed = true;
    }

    // 只在变化时应用并打印，避免每帧刷屏
    if changed {
        sink.set_volume(Volume::Linear(state.volume));
        sink.set_speed(state.speed);
        info!(
            "[音频] 音量 = {:.1}，速度 = {:.1}",
            state.volume, state.speed
        );
    }
}
