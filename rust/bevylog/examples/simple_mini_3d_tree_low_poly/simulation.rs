//! 模拟时钟：空格键暂停 / 恢复。游戏逻辑的时间增量统一从 `SimulationClock::scaled_delta`
//! 取得（暂停时返回 0），这样暂停时所有殖民者行为、建造进度都会停住。

use bevy::prelude::*;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SimulationClock>()
            .add_systems(Update, control_time);
    }
}

#[derive(Resource, Debug, Clone)]
pub struct SimulationClock {
    pub paused: bool,
    pub speed: f32,
}

impl Default for SimulationClock {
    fn default() -> Self {
        Self {
            paused: false,
            speed: 1.0,
        }
    }
}

impl SimulationClock {
    pub fn scaled_delta(&self, time: &Time) -> f32 {
        if self.paused {
            0.0
        } else {
            time.delta_secs() * self.speed
        }
    }

    pub fn label(&self) -> String {
        if self.paused {
            "已暂停".to_string()
        } else {
            format!("{:.0}x", self.speed)
        }
    }
}

pub fn control_time(keyboard: Res<ButtonInput<KeyCode>>, mut clock: ResMut<SimulationClock>) {
    if keyboard.just_pressed(KeyCode::Space) {
        clock.paused = !clock.paused;
    }
}
