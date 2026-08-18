//! Bevy 0.19 入门示例：演示随机数生成（rand crate）。
//! 按空格随机生成一批颜色、位置、大小都随机的圆形，按 C 清空。
//!
//! 学习重点：
//! - `rand::rng()` 获取线程随机数生成器（每次调用开销很小，可直接在系统里用）
//! - `rng.random::<f32>()` 生成 [0, 1) 的浮点数（配合 Color::srgb 生成随机颜色）
//! - `rng.random_range(a..b)` 生成 [a, b) 区间的数（随机位置、随机大小）
//! - `rng.random_bool(p)` 以概率 p 返回 true/false（按概率生成特殊效果）
//! - 这些方法来自 `rand::RngExt` trait，需要 `use rand::RngExt;`
//!
//! 操作方式：
//! - 空格：随机生成 20 个圆形（30% 概率生成一个「金块」大圆形）
//! - C：清空所有圆形

use bevy::{prelude::*, text::FontSourceTemplate};
use rand::RngExt;

// 中文字体路径
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// 圆形标记（用于清空时统计/删除）
#[derive(Component)]
struct RandomDot;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        // .chain() 保证顺序：先处理生成/清空，再更新文本
        .add_systems(Update, (spawn_dots, clear_dots, update_text).chain())
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn_scene(bsn! {
        Text2d::new("")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(22.0),
        }
        Transform::from_xyz(0.0, -260.0, 0.0)
    });
}

// 空格：随机生成 20 个圆形。
fn spawn_dots(keys: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    if !keys.just_pressed(KeyCode::Space) {
        return;
    }

    // 获取线程随机数生成器
    let mut rng = rand::rng();

    for _ in 0..20 {
        // 随机位置：x 在 [-400, 400)，y 在 [-280, 280)
        let x = rng.random_range(-400.0..400.0);
        let y = rng.random_range(-280.0..280.0);
        // 随机大小：直径在 [8, 40) 之间
        let size = rng.random_range(8.0..40.0);
        // 随机颜色：rgb 各取 [0,1) 的随机数
        let color = Color::srgb(rng.random(), rng.random(), rng.random());

        commands.spawn((
            RandomDot,
            Sprite::from_color(color, Vec2::splat(size)),
            Transform::from_xyz(x, y, 0.0),
        ));
    }

    // random_bool(0.3)：30% 概率生成一个「金块」（固定颜色的大圆）
    if rng.random_bool(0.3) {
        commands.spawn((
            RandomDot,
            Sprite::from_color(Color::srgb(1.0, 0.85, 0.2), Vec2::splat(60.0)),
            Transform::from_xyz(rng.random_range(-300.0..300.0), 200.0, 0.0),
        ));
        info!("[随机] 生成金块！");
    }

    info!("[随机] 已生成 20 个随机圆形");
}

// C：清空所有圆形。
fn clear_dots(
    keys: Res<ButtonInput<KeyCode>>,
    dots: Query<Entity, With<RandomDot>>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::KeyC) {
        let count = dots.iter().count();
        for entity in &dots {
            commands.entity(entity).despawn();
        }
        info!("[随机] 已清空 {count} 个圆形");
    }
}

// 更新提示文本：显示当前圆形数量。
fn update_text(dots: Query<&RandomDot>, mut text: Single<&mut Text2d>, mut last: Local<String>) {
    let new_text = format!(
        "空格：随机生成  |  C：清空  |  圆形：{}",
        dots.iter().count()
    );

    // 节流：内容变化才更新文本，避免 CJK 重排刷屏
    if *last != new_text {
        *last = new_text.clone();
        text.0 = new_text;
    }
}
