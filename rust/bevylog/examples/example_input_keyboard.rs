//! Bevy 0.19 入门示例：演示键盘输入进阶（按键状态、组合键、修饰键）。
//! 方向键 / WASD 按住移动方块，空格刚按下生成圆形，R 清除全部，Ctrl+R 组合键重置位置。
//!
//! 学习重点：
//! - `ButtonInput<KeyCode>` 的三种按键状态：`pressed`（按住中）、`just_pressed`（本帧刚按下）、`just_released`（本帧刚松开）
//! - `pressed` 用于「持续」动作（如按住方向键连续移动）；`just_pressed` 用于「一次性」动作（如生成、切换），避免按住时每帧重复触发
//! - 组合键：`ctrl + R` 用 `ctrl.pressed(...) && key.just_pressed(...)` 组合判断
//! - 修饰键 `KeyCode::ControlLeft / ControlRight / ShiftLeft / AltLeft` 也是普通按键，用 `ButtonInput` 统一检测
//! - 每帧读取 `get_just_pressed()` 可列出本帧所有刚按下的键（适合做按键记录 / 快捷键提示）
//!
//! 操作方式：
//! - 方向键 / WASD：按住移动方块
//! - 空格：生成一个圆形
//! - R：清除所有圆形
//! - Ctrl + R：方块回到原点

use bevy::input::keyboard::KeyCode;
use bevy::{prelude::*, text::FontSourceTemplate};

// 中文字体路径
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .init_resource::<LastKeys>()
        .add_systems(Startup, setup)
        // 用 .chain() 保证顺序：先处理移动/生成，再更新提示文本（保证显示的计数准确）
        .add_systems(
            Update,
            (move_player, handle_shortcuts, update_info_text).chain(),
        )
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // 玩家方块：绿色正方形，可被方向键 / WASD 移动。
    // 加上 Player 标记，后续系统用 Single<&mut Transform, With<Player>> 精确找到它。
    commands.spawn((
        Player,
        Sprite::from_color(Color::srgb(0.3, 1.0, 0.5), Vec2::splat(40.0)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // 底部提示文本
    commands.spawn_scene(bsn! {
        Text2d::new("")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(20.0),
        }
        Transform::from_xyz(0.0, -260.0, 0.0)
    });
}

// 玩家方块（用 With<Sprite> 过滤出带精灵组件的实体）
#[derive(Component)]
struct Player;

// 移动方块：按住方向键 / WASD 持续移动。
fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Single<&mut Transform, With<Player>>,
) {
    // 移动方向（x 左右，y 上下），初始为 0
    let mut direction = Vec2::ZERO;

    // 方向键和 WASD 是两组等价按键，都叠加到 direction 上。
    // 用 pressed（按住中）实现「按住就持续移动」，每帧根据 delta 累加。
    if keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if keys.pressed(KeyCode::ArrowUp) || keys.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keys.pressed(KeyCode::ArrowDown) || keys.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }

    // 有输入才移动；normalize 让斜向移动速度和对角一致（不按 sqrt(2) 加速）
    if direction != Vec2::ZERO {
        let speed = 200.0;
        player.translation += direction.normalize().extend(0.0) * speed * time.delta_secs();
    }
}

// 处理「一次性」按键和组合键。
fn handle_shortcuts(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut circles: Query<Entity, With<CircleMarker>>,
    mut player: Single<&mut Transform, With<Player>>,
    // 记录本帧刚按下的键，用于日志展示（说明 just_pressed 可以拿到「是哪些键刚被按下」）
    mut last_shortcut: ResMut<LastKeys>,
) {
    // 空格：just_pressed（本帧刚按下）生成一个圆形。
    // 用 just_pressed 而非 pressed，避免按住空格时每帧都生成。
    if keys.just_pressed(KeyCode::Space) {
        commands.spawn((
            CircleMarker,
            Sprite::from_color(Color::srgb(1.0, 0.8, 0.2), Vec2::splat(20.0)),
            Transform::from_xyz(0.0, 100.0, 0.0),
        ));
        info!("[键盘] 空格刚按下 → 生成圆形");
    }

    // R：清除所有圆形
    if keys.just_pressed(KeyCode::KeyR) {
        let count = circles.iter().count();
        for entity in &mut circles {
            commands.entity(entity).despawn();
        }
        info!("[键盘] R 刚按下 → 清除 {count} 个圆形");
    }

    // 组合键 Ctrl + R：方块回到原点。
    // 注意：组合键判断 = 修饰键用 pressed（按住中）+ 普通键用 just_pressed（刚按下）。
    let ctrl = keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight);
    if ctrl && keys.just_pressed(KeyCode::KeyR) {
        player.translation = Vec3::ZERO;
        info!("[键盘] Ctrl+R 组合键 → 方块回到原点");
    }

    // 演示 just_released：松开 Shift 时打印日志（一次性触发，不会每帧刷屏）
    if keys.just_released(KeyCode::ShiftLeft) || keys.just_released(KeyCode::ShiftRight) {
        info!("[键盘] Shift 刚松开");
    }

    // 演示 get_just_pressed：本帧所有刚按下的键（用于记录快捷键 / 按键历史）。
    // 只在有按键时才更新本地状态，避免每帧空转。
    let pressed: Vec<String> = keys.get_just_pressed().map(|k| format!("{k:?}")).collect();
    if !pressed.is_empty() {
        last_shortcut.0 = pressed.join(", ");
    }
}

// 圆形标记（用 With<CircleMarker> 过滤圆形实体，避免把玩家方块也算进去）
#[derive(Component)]
struct CircleMarker;

// 记录本帧最后按下的键（用资源在多个系统间共享数据；Local 只能在一个系统内保存状态）
#[derive(Resource, Default)]
struct LastKeys(String);

// 更新底部提示文本：显示圆形数量、方块位置、最后按下的键。
fn update_info_text(
    circles: Query<&CircleMarker>,
    player: Single<&Transform, With<Player>>,
    mut text: Single<&mut Text2d>,
    last_shortcut: Res<LastKeys>,
    mut last_text: Local<String>,
) {
    let count = circles.iter().count();
    let pos = player.translation;
    let new_text = format!(
        "方向键/WASD 移动 | 空格 生成 | R 清除 | Ctrl+R 回原点 | 圆形：{count} | 方块：({:.0}, {:.0}) | 本帧按键：{}",
        pos.x, pos.y, last_shortcut.0
    );

    // 节流：内容变化才更新文本，避免 CJK 重排刷屏
    if *last_text != new_text {
        *last_text = new_text.clone();
        text.0 = new_text;
    }
}
