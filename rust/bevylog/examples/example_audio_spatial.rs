//! Bevy 0.19 入门示例：演示 3D 空间音效（SpatialAudio）。
//! 一个发声实体在 3D 场景中左右移动，声音随位置在左右声道间平移（立体声定位）。
//!
//! 学习重点：
//! - SpatialListener：挂在「听者」（通常是相机）上，代表耳朵的位置
//! - PlaybackSettings::with_spatial(true)：把音频设为空间音频
//! - 空间音频下，声源实体的 Transform 决定声音来源方向，Bevy 自动做左右声道平移
//! - 普通音频（非空间）不受位置影响，两声道一样

use bevy::prelude::*;

// 音频资源路径
const SOUND: &str = "audio/bg.wav";

// 发声实体标记（用于移动它）
#[derive(Component)]
struct MovingSound;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.08)))
        .add_systems(Startup, setup)
        .add_systems(Update, move_sound)
        .run()
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 3D 相机 + SpatialListener：空间音频的「耳朵」，声音相对它定位
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
        SpatialListener::default(),
    ));

    // 方向光：照亮场景，让小球可见
    commands.spawn((
        DirectionalLight {
            illuminance: 3000.0,
            ..default()
        },
        Transform::from_xyz(0.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 发声实体：小球 + 空间音频（LOOP 循环 + with_spatial(true)）
    // 小球的 Transform 决定声音来源，移动它声音就会左右平移
    commands.spawn((
        MovingSound,
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.8, 0.4),
            ..default()
        })),
        Transform::from_xyz(-6.0, 0.0, 0.0),
        AudioPlayer::new(asset_server.load(SOUND)),
        PlaybackSettings::LOOP.with_spatial(true),
    ));
}

// 让发声小球左右来回移动，声音随位置平移
fn move_sound(time: Res<Time>, mut query: Query<&mut Transform, With<MovingSound>>) {
    for mut transform in &mut query {
        // 用 sin 在 x 轴 -6 ~ 6 之间往返
        transform.translation.x = time.elapsed_secs().sin() * 6.0;
    }
}
