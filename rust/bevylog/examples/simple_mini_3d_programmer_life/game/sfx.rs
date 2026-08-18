//! 程序化音效：启动时用代码生成几段 WAV（正弦波 + 衰减），零音频资产依赖。
//! 播放用 bevy_audio 的 AudioPlayer（需要 bevy/wav feature）。

use bevy::audio::{AudioPlayer, AudioSource, PlaybackSettings};
use bevy::prelude::*;

const RATE: u32 = 22050;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Sfx {
    Click, // 点击/确认
    Step,  // 走路
    Alert, // 事件弹窗
    Horn,  // 汽车鸣笛（行人横穿马路时）
}

#[derive(Resource)]
pub struct SoundBank {
    pub click: Handle<AudioSource>,
    pub step: Handle<AudioSource>,
    pub alert: Handle<AudioSource>,
    pub horn: Handle<AudioSource>,
}

// 生成一段衰减正弦音
fn tone(freq: f32, dur: f32, vol: f32) -> Vec<f32> {
    let n = (dur * RATE as f32) as usize;
    (0..n)
        .map(|i| {
            let t = i as f32 / RATE as f32;
            (t * freq * 2.0 * std::f32::consts::PI).sin() * vol * (1.0 - t / dur)
        })
        .collect()
}

fn make_wav(samples: &[f32]) -> Vec<u8> {
    let data_len = (samples.len() * 2) as u32;
    let mut wav = Vec::with_capacity(44 + samples.len() * 2);
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&(36 + data_len).to_le_bytes());
    wav.extend_from_slice(b"WAVE");
    wav.extend_from_slice(b"fmt ");
    wav.extend_from_slice(&16u32.to_le_bytes());
    wav.extend_from_slice(&1u16.to_le_bytes()); // PCM
    wav.extend_from_slice(&1u16.to_le_bytes()); // mono
    wav.extend_from_slice(&RATE.to_le_bytes());
    wav.extend_from_slice(&(RATE * 2).to_le_bytes()); // byte rate
    wav.extend_from_slice(&2u16.to_le_bytes()); // block align
    wav.extend_from_slice(&16u16.to_le_bytes()); // bits per sample
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data_len.to_le_bytes());
    for s in samples {
        let v = (s.clamp(-1.0, 1.0) * 32000.0) as i16;
        wav.extend_from_slice(&v.to_le_bytes());
    }
    wav
}

pub fn setup_sfx(mut commands: Commands, mut assets: ResMut<Assets<AudioSource>>) {
    let mut add = |samples: Vec<f32>| {
        assets.add(AudioSource {
            bytes: make_wav(&samples).into(),
        })
    };
    let mut alert = tone(660.0, 0.08, 0.35);
    alert.extend(tone(660.0, 0.08, 0.35));
    let mut step = tone(220.0, 0.07, 0.45);
    step.extend(tone(160.0, 0.06, 0.3));
    // 汽车鸣笛：两声短促的喇叭（模拟"哔哔"）
    let mut horn = tone(392.0, 0.15, 0.6);
    horn.extend(tone(392.0, 0.15, 0.6));
    commands.insert_resource(SoundBank {
        click: add(tone(640.0, 0.06, 0.5)),
        step: add(step),
        alert: add(alert),
        horn: add(horn),
    });
}

pub fn play(commands: &mut Commands, bank: &SoundBank, kind: Sfx) {
    let handle = match kind {
        Sfx::Click => &bank.click,
        Sfx::Step => &bank.step,
        Sfx::Alert => &bank.alert,
        Sfx::Horn => &bank.horn,
    };
    commands.spawn((AudioPlayer(handle.clone()), PlaybackSettings::DESPAWN));
}
