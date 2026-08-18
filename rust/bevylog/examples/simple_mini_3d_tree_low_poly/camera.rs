//! 轨道相机：按住鼠标右键拖拽旋转视角，中键拖拽平移，滚轮缩放。
//! 相机始终围绕焦点（地形高度自动跟随）旋转，距离限制在 [min_radius, max_radius]。

use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};

use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    prelude::*,
};

use crate::terrain::{TerrainSeed, terrain_height};
use crate::types::MAP_BUILD_HALF_EXTENT;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, control_camera);
    }
}

#[derive(Component)]
pub struct OrbitCamera {
    focus: Vec3,
    radius: f32,
    yaw: f32,
    pitch: f32,
    rotate_sensitivity: f32,
    pan_sensitivity: f32,
    zoom_sensitivity: f32,
    min_radius: f32,
    max_radius: f32,
    min_pitch: f32,
    max_pitch: f32,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            focus: Vec3::new(0.0, 0.5, 0.0),
            radius: 16.0,
            yaw: -FRAC_PI_4,
            pitch: FRAC_PI_4,
            rotate_sensitivity: 0.006,
            pan_sensitivity: 0.0025,
            zoom_sensitivity: 0.14,
            min_radius: 4.0,
            max_radius: 240.0,
            min_pitch: 0.18,
            max_pitch: FRAC_PI_2 - 0.08,
        }
    }
}

impl OrbitCamera {
    fn with_focus_y(mut self, y: f32) -> Self {
        self.focus.y = y;
        self
    }
}

pub fn spawn_camera(commands: &mut Commands, seed: u64) {
    let center_height = terrain_height(seed, 0.0, 0.0);
    let orbit_camera = OrbitCamera::default().with_focus_y(center_height + 0.5);
    let mut transform = Transform::default();
    sync_camera_transform(&orbit_camera, &mut transform);

    commands.spawn((Camera3d::default(), transform, orbit_camera));
}

pub fn control_camera(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
    terrain_seed: Res<TerrainSeed>,
    mut camera_query: Query<(&mut OrbitCamera, &mut Transform)>,
) {
    let Ok((mut camera, mut transform)) = camera_query.single_mut() else {
        return;
    };

    let drag_delta = mouse_motion.delta;
    if mouse_buttons.pressed(MouseButton::Right) && drag_delta != Vec2::ZERO {
        camera.yaw -= drag_delta.x * camera.rotate_sensitivity;
        camera.pitch = (camera.pitch + drag_delta.y * camera.rotate_sensitivity)
            .clamp(camera.min_pitch, camera.max_pitch);
    }

    if mouse_buttons.pressed(MouseButton::Middle) && drag_delta != Vec2::ZERO {
        let right = transform.rotation * Vec3::X;
        let mut forward = transform.rotation * -Vec3::Z;
        forward.y = 0.0;
        forward = forward.normalize_or_zero();

        let pan_scale = camera.radius * camera.pan_sensitivity;
        camera.focus += (-right * drag_delta.x + forward * drag_delta.y) * pan_scale;
        camera.focus.x = camera
            .focus
            .x
            .clamp(-MAP_BUILD_HALF_EXTENT, MAP_BUILD_HALF_EXTENT);
        camera.focus.z = camera
            .focus
            .z
            .clamp(-MAP_BUILD_HALF_EXTENT, MAP_BUILD_HALF_EXTENT);
    }

    let scroll_delta = mouse_scroll.delta.y;
    if scroll_delta.abs() > f32::EPSILON {
        let zoom_factor = (-scroll_delta * camera.zoom_sensitivity).exp();
        camera.radius = (camera.radius * zoom_factor).clamp(camera.min_radius, camera.max_radius);
    }

    camera.focus.y = terrain_height(terrain_seed.0, camera.focus.x, camera.focus.z) + 0.5;
    sync_camera_transform(&camera, &mut transform);
}

fn sync_camera_transform(camera: &OrbitCamera, transform: &mut Transform) {
    let horizontal_radius = camera.radius * camera.pitch.cos();
    let offset = Vec3::new(
        horizontal_radius * camera.yaw.sin(),
        camera.radius * camera.pitch.sin(),
        horizontal_radius * camera.yaw.cos(),
    );

    transform.translation = camera.focus + offset;
    transform.look_at(camera.focus, Vec3::Y);
}
