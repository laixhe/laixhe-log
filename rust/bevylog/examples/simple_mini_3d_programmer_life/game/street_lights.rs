//! 街道路灯：马路沿线的路灯在晚上自动点亮（暖黄自发光），白天熄灭。
//! 路灯的发光球挂载 [`StreetLamp`] 组件（存发光材质 Handle），由 [`street_lights_tick`]
//! 按时段（晚上 = `Phase::Evening`）切换 emissive。

use bevy::prelude::*;

use super::resources::{GameClock, Phase};

/// 路灯发光球：晚上点亮（emissive 由 street_lights_tick 按时段切换）
#[derive(Component)]
pub struct StreetLamp {
    pub mat: Handle<StandardMaterial>,
}

/// 街道路灯点亮系统：晚上（`Phase::Evening`）开启暖黄灯光，白天熄灭
pub fn street_lights_tick(
    clock: Res<GameClock>,
    lamps: Query<&StreetLamp>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let on = clock.phase == Phase::Evening;
    for lamp in &lamps {
        if let Some(mut m) = materials.get_mut(&lamp.mat) {
            m.emissive = if on {
                LinearRgba::new(1.0, 0.8, 0.5, 1.0)
            } else {
                LinearRgba::BLACK
            };
        }
    }
}
