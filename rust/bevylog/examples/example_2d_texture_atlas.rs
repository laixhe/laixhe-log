//! Bevy 0.19 入门示例：演示图集动画（TextureAtlas / TextureAtlasLayout）。
//! 用一张包含 4 帧的精灵图集（雪碧图）循环播放「弹跳小球」动画。
//!
//! 学习重点：
//! - TextureAtlasLayout：描述图集如何切分（用 from_grid 按网格切分成多个格子）
//! - TextureAtlas：挂在 Sprite 上，通过 index 指定当前显示第几帧
//! - 动画 = 每隔一段时间递增 index 并循环
//! - 精灵图集是一张图包含多帧，比多张图更省内存、切换更快

use bevy::prelude::*;

// 图集资源路径（assets/ 目录内的相对路径）
const SPRITESHEET: &str = "images/ball_spritesheet.png";

// 帧参数：每帧 64x64，共 4 帧，排成 1 行
const FRAME_SIZE: u32 = 64;
const FRAME_COUNT: usize = 4;

// 动画计时组件：控制切换帧的间隔
#[derive(Component)]
struct AnimationTimer(Timer);

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.08)))
        .add_systems(Startup, setup)
        .add_systems(Update, animate_sprite)
        .run()
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // 图集布局资源库：add() 把布局存入并返回 Handle
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);

    // 加载图集图片
    let texture = asset_server.load(SPRITESHEET);
    // 把图集切成 4 个 64x64 的格子（1 行 4 列，索引从左到右）
    let layout =
        TextureAtlasLayout::from_grid(UVec2::splat(FRAME_SIZE), FRAME_COUNT as u32, 1, None, None);
    let layout_handle = texture_atlases.add(layout);

    // 生成精灵：Sprite（图片 + texture_atlas 字段指定图集）+ 动画计时器
    commands.spawn((
        Sprite {
            image: texture,
            // TextureAtlas 不是独立组件，而是 Sprite 的一个字段（Option<TextureAtlas>）
            texture_atlas: Some(TextureAtlas {
                layout: layout_handle,
                index: 0,
            }),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

// 循环播放动画：每隔一段时间递增帧索引（取模循环）
fn animate_sprite(time: Res<Time>, mut query: Query<(&mut AnimationTimer, &mut Sprite)>) {
    for (mut timer, mut sprite) in &mut query {
        if timer.0.tick(time.delta()).just_finished() {
            // 从 Sprite 的 texture_atlas 字段取出可变引用，改 index 切换帧
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = (atlas.index + 1) % FRAME_COUNT;
            }
        }
    }
}
