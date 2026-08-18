use bevy::prelude::*;
use bevy::text::FontSource;
use rand::RngExt;

use crate::pages::router::GameState;

// 中文字体路径（与其它示例共用同一资产）
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";
// 星星图集（与 example_2d_texture_atlas 共用：4 帧 64x64 弹跳小球）
const STAR_SHEET: &str = "images/ball_spritesheet.png";
// 收集音效（与 example_audio 共用）
const BLIP_SOUND: &str = "audio/blip.wav";

// ==================== 游戏常量配置 ====================
const PLAYER_SPEED: f32 = 260.0; // 玩家移动速度（像素/秒）
const PLAYER_RADIUS: f32 = 20.0; // 玩家碰撞半径
const STAR_RADIUS: f32 = 32.0; // 星星碰撞半径（约等于图集半帧宽）
const METEOR_RADIUS: f32 = 14.0; // 陨石碰撞半径
const SPAWN_RANGE: f32 = 450.0; // 星星/陨石在玩家周围随机出现的范围
const FRAME_COUNT: usize = 4; // 图集帧数

// ==================== 组件定义 ====================
// 游戏根标记：所有 Playing 页的实体都挂上，OnExit 时一键清理
#[derive(Component, Clone, Default)]
pub struct GameRoot;

// 玩家
#[derive(Component)]
pub struct Player;

// 得分文本
#[derive(Component, Clone, Default)]
pub struct ScoreText;

// 可收集的星星（图集动画精灵）
#[derive(Component)]
pub struct Star;

// 落下的陨石
#[derive(Component)]
pub struct Meteor {
    pub speed: f32,
}

// 视差背景层（factor 越小越远，随相机移动越慢）
#[derive(Component)]
pub struct ParallaxLayer {
    pub factor: f32,
}

// 粒子（拖尾 / 收集爆炸共用）：带速度 + 生命周期
#[derive(Component)]
pub struct Particle {
    pub velocity: Vec2,
    pub lifetime: f32,
    pub max_lifetime: f32,
}

// 图集动画计时器（用于星星帧切换）
#[derive(Component)]
pub struct AnimationTimer(pub Timer);

// ==================== 资源定义 ====================
#[derive(Resource)]
pub struct Score(pub u32);

#[derive(Resource)]
pub struct StarSpawnTimer(pub Timer);

#[derive(Resource)]
pub struct MeteorSpawnTimer(pub Timer);

// 星星图集资产（setup_game 中创建一次，生成星星时复用句柄）
#[derive(Resource)]
pub struct StarAssets {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

// ==================== 进入游戏页 ====================
pub fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut score: ResMut<Score>,
) {
    score.0 = 0;
    info!("[游戏] 开始：星空收集");

    // 准备星星图集资产：一张图切成 4 个 64x64 的格子
    let texture = asset_server.load(STAR_SHEET);
    let layout = texture_atlases.add(TextureAtlasLayout::from_grid(
        UVec2::splat(64),
        FRAME_COUNT as u32,
        1,
        None,
        None,
    ));
    commands.insert_resource(StarAssets { texture, layout });

    // 玩家：黄色圆形（Mesh2d 过程式创建）
    commands.spawn((
        GameRoot,
        Player,
        Mesh2d(meshes.add(Circle::new(PLAYER_RADIUS))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.95, 0.85, 0.3)))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // 视差背景：两层星空小点，factor 不同 → 随相机移动速度不同（近快远慢）
    for (factor, color) in [
        (0.15, Color::srgb(0.18, 0.26, 0.42)),
        (0.3, Color::srgb(0.26, 0.38, 0.55)),
    ] {
        commands
            .spawn((
                GameRoot,
                ParallaxLayer { factor },
                Transform::from_xyz(0.0, 0.0, -2.0),
            ))
            .with_children(|parent| {
                let mut rng = rand::rng();
                for _ in 0..80 {
                    parent.spawn((
                        Sprite::from_color(color, Vec2::splat(4.0)),
                        Transform::from_xyz(
                            rng.random_range(-1400.0..1400.0),
                            rng.random_range(-900.0..900.0),
                            0.0,
                        ),
                    ));
                }
            });
    }

    // 得分 UI（屏幕左上角）
    commands
        .spawn((
            GameRoot,
            Node {
                width: percent(100),
                padding: UiRect::all(px(12)),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                ScoreText,
                Text::new("得分: 0"),
                TextColor(Color::WHITE),
                TextFont {
                    font: FontSource::Handle(asset_server.load(FONT_PATH)),
                    font_size: FontSize::Px(26.0),
                    ..default()
                },
            ));
        });
}

// ==================== 退出游戏页 ====================
pub fn cleanup_game(mut commands: Commands, query: Query<Entity, With<GameRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

// ==================== 玩家移动 ====================
pub fn player_movement_system(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Single<&mut Transform, With<Player>>,
) {
    let mut dir = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        dir.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
        dir.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        dir.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        dir.x += 1.0;
    }
    // normalize_or_zero：斜向移动不加速，按键为 0 时不产生 NaN（只计算一次，x/y 复用）
    let dir = dir.normalize_or_zero();
    player.translation.x += dir.x * PLAYER_SPEED * time.delta_secs();
    player.translation.y += dir.y * PLAYER_SPEED * time.delta_secs();
}

// ==================== 粒子系统（玩家拖尾 + 收集爆炸） ====================
// 每帧在玩家位置生成 2 个粒子，形成尾迹
pub fn player_trail_system(mut commands: Commands, player: Single<&Transform, With<Player>>) {
    let mut rng = rand::rng();
    for _ in 0..2 {
        let velocity = Vec2::new(rng.random_range(-60.0..60.0), rng.random_range(-60.0..60.0));
        commands.spawn((
            GameRoot,
            Particle {
                velocity,
                lifetime: 0.6,
                max_lifetime: 0.6,
            },
            Sprite::from_color(Color::srgb(0.95, 0.85, 0.3), Vec2::splat(8.0)),
            Transform::from_translation(player.translation),
        ));
    }
}

// 更新粒子：漂移 + 缩小 + 淡出 + 销毁
pub fn update_particles(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut Particle, &mut Transform, &mut Sprite)>,
) {
    for (entity, mut particle, mut tf, mut sprite) in &mut q {
        particle.lifetime -= time.delta_secs();
        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }
        tf.translation += particle.velocity.extend(0.0) * time.delta_secs();
        let t = particle.lifetime / particle.max_lifetime;
        tf.scale = Vec3::splat(t);
        sprite.color.set_alpha(t);
    }
}

// ==================== 星星生成 / 图集动画 ====================
// 定时在玩家周围随机位置生成一颗星星
pub fn star_spawn_system(
    time: Res<Time>,
    mut timer: ResMut<StarSpawnTimer>,
    mut commands: Commands,
    stars: Res<StarAssets>,
    player: Single<&Transform, With<Player>>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        let mut rng = rand::rng();
        let offset = Vec2::new(
            rng.random_range(-SPAWN_RANGE..SPAWN_RANGE),
            rng.random_range(-SPAWN_RANGE..SPAWN_RANGE),
        );
        let pos = player.translation.truncate() + offset;
        commands.spawn((
            GameRoot,
            Star,
            Sprite {
                image: stars.texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: stars.layout.clone(),
                    index: 0,
                }),
                ..default()
            },
            AnimationTimer(Timer::from_seconds(0.12, TimerMode::Repeating)),
            Transform::from_xyz(pos.x, pos.y, 0.0),
        ));
    }
}

// 星星图集动画：定时递增帧索引循环
pub fn star_animate_system(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut Sprite), With<Star>>,
) {
    for (mut timer, mut sprite) in &mut query {
        if timer.0.tick(time.delta()).just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = (atlas.index + 1) % FRAME_COUNT;
            }
        }
    }
}

// ==================== 陨石生成 / 下落 ====================
// 定时从玩家上方随机位置生成一颗陨石
pub fn meteor_spawn_system(
    time: Res<Time>,
    mut timer: ResMut<MeteorSpawnTimer>,
    mut commands: Commands,
    player: Single<&Transform, With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        let mut rng = rand::rng();
        let x = player.translation.x + rng.random_range(-SPAWN_RANGE..SPAWN_RANGE);
        let speed = 120.0 + rng.random_range(0.0..120.0);
        commands.spawn((
            GameRoot,
            Meteor { speed },
            Mesh2d(meshes.add(Circle::new(METEOR_RADIUS))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.9, 0.3, 0.3)))),
            Transform::from_xyz(x, player.translation.y + 420.0, 0.0),
        ));
    }
}

// 陨石匀速下落，落到相机下方后销毁
pub fn meteor_move_system(
    time: Res<Time>,
    mut commands: Commands,
    camera: Single<&Transform, (With<Camera2d>, Without<Player>)>,
    // Without 过滤：避免与相机 / 玩家对 Transform 的访问冲突（Bevy B0001）
    mut q: Query<(Entity, &mut Transform, &Meteor), (Without<Camera2d>, Without<Player>)>,
) {
    for (entity, mut tf, meteor) in &mut q {
        tf.translation.y -= meteor.speed * time.delta_secs();
        if tf.translation.y < camera.translation.y - 520.0 {
            commands.entity(entity).despawn();
        }
    }
}

// ==================== 碰撞检测 ====================
// 玩家与星星：圆形距离碰撞 → 收集 + 得分 + 爆炸粒子 + 音效
pub fn collect_stars_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player: Single<&Transform, With<Player>>,
    mut stars: Query<(Entity, &Transform), With<Star>>,
    mut score: ResMut<Score>,
) {
    let player_pos = player.translation.truncate();
    for (entity, tf) in &mut stars {
        let star_pos = tf.translation.truncate();
        if player_pos.distance(star_pos) < PLAYER_RADIUS + STAR_RADIUS {
            // 星星消失
            commands.entity(entity).despawn();
            score.0 += 1;

            // 收集爆炸：一圈金色粒子向四周飞散
            let mut rng = rand::rng();
            for _ in 0..10 {
                let velocity = Vec2::new(
                    rng.random_range(-200.0..200.0),
                    rng.random_range(-200.0..200.0),
                );
                commands.spawn((
                    GameRoot,
                    Particle {
                        velocity,
                        lifetime: 0.5,
                        max_lifetime: 0.5,
                    },
                    Sprite::from_color(Color::srgb(1.0, 0.9, 0.4), Vec2::splat(6.0)),
                    Transform::from_xyz(star_pos.x, star_pos.y, 0.0),
                ));
            }

            // 播放一次性音效（播完自动销毁实体）
            commands.spawn((
                AudioPlayer::new(asset_server.load(BLIP_SOUND)),
                PlaybackSettings::DESPAWN,
            ));
            info!("[收集] 得分 +1，当前 {}", score.0);
        }
    }
}

// 玩家与陨石：碰到即游戏结束
pub fn meteor_hit_system(
    player: Single<&Transform, With<Player>>,
    meteors: Query<&Transform, With<Meteor>>,
    score: Res<Score>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let player_pos = player.translation.truncate();
    for tf in &meteors {
        if player_pos.distance(tf.translation.truncate()) < PLAYER_RADIUS + METEOR_RADIUS {
            info!("[碰撞] 被陨石击中！最终得分 {}", score.0);
            next_state.set(GameState::GameOver);
            return;
        }
    }
}

// ==================== 相机平滑跟随 ====================
// 指数平滑（lerp + 帧率无关 factor），相机平滑追向玩家
pub fn camera_follow_system(
    time: Res<Time>,
    player: Single<&Transform, With<Player>>,
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let target = player.translation.truncate();
    let current = camera.translation.truncate();
    let factor = 1.0 - (-5.0 * time.delta_secs()).exp();
    let new = current.lerp(target, factor);
    camera.translation.x = new.x;
    camera.translation.y = new.y;
}

// ==================== 视差滚动 ====================
// 背景层位置 = -相机位置 × factor：越远（factor 越小）动得越慢
pub fn parallax_system(
    camera: Single<&Transform, (With<Camera2d>, Without<ParallaxLayer>)>,
    mut layers: Query<(&ParallaxLayer, &mut Transform)>,
) {
    for (layer, mut tf) in &mut layers {
        tf.translation.x = -camera.translation.x * layer.factor;
        tf.translation.y = -camera.translation.y * layer.factor;
    }
}

// ==================== UI 更新 / 退出 ====================
// 分数变化时才更新文本（resource_changed 避免每帧重写触发布局重排）
pub fn update_score_display(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    if let Ok(mut text) = query.single_mut() {
        text.0 = format!("得分: {}", score.0);
    }
}

// ESC 返回主菜单
pub fn esc_to_menu(keys: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Menu);
    }
}
