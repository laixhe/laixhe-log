//! Bevy 0.19 入门示例：演示全局音量（GlobalVolume）。
//!
//! 学习重点：
//! - GlobalVolume 是控制所有音频的全局音量资源
//! - 修改 GlobalVolume 只影响「之后开始播放」的音频，不影响已在播放的
//! - Volume::Linear 表示线性音量（1.0 为原始音量）
//!
//! 操作：上下方向键调全局音量，空格播放一个新音效（应用新音量）。

use bevy::audio::Volume;
use bevy::prelude::*;

const SOUND: &str = "audio/blip.wav";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GlobalVolume::new(Volume::Linear(1.0)))
        .add_systems(Startup, setup)
        .add_systems(Update, (adjust_volume, play_sfx))
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    info!("[音频] 按 空格 播放音效，上下方向键调全局音量");
}

// 上下方向键调整全局音量
fn adjust_volume(keys: Res<ButtonInput<KeyCode>>, mut volume: ResMut<GlobalVolume>) {
    let mut changed = false;
    if keys.just_pressed(KeyCode::ArrowUp) {
        let v = (volume.volume.to_linear() + 0.1).min(1.0);
        volume.volume = Volume::Linear(v);
        changed = true;
    }
    if keys.just_pressed(KeyCode::ArrowDown) {
        let v = (volume.volume.to_linear() - 0.1).max(0.0);
        volume.volume = Volume::Linear(v);
        changed = true;
    }
    if changed {
        info!("[音频] 全局音量 = {:.1}", volume.volume.to_linear());
    }
}

// 空格播放音效：新播放的音频会使用当前全局音量
fn play_sfx(
    keys: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::Space) {
        commands.spawn((
            AudioPlayer::new(asset_server.load(SOUND)),
            PlaybackSettings::DESPAWN,
        ));
        info!("[音频] 播放音效（应用当前全局音量）");
    }
}
