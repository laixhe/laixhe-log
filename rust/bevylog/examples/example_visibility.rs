//! Bevy 0.19 入门示例：演示实体可见性（Visibility / InheritedVisibility / ViewVisibility）。
//! 一个黄色父方块带一个蓝色子方块，父方块隐藏时子方块跟着隐藏（可见性继承）。
//!
//! 学习重点：
//! - `Visibility` 是「用户手动设置」的可见性，有三个值：
//!   - `Inherited`（默认）：跟随父实体（根实体继承 = 可见）
//!   - `Hidden`：无条件隐藏（且会传播给子实体）
//!   - `Visible`：无条件显示（不受父实体隐藏影响）
//! - `InheritedVisibility` 是「系统计算后」的结果，用来判断实体实际是否可见（只读）
//! - 父实体隐藏 → 子实体（Inherited）的 InheritedVisibility 变为 false，即使子实体自己没改
//! - `ViewVisibility` 更进一步（视锥剔除结果），新手先了解前两个即可
//!
//! 操作方式：
//! - T：隐藏 / 显示父方块（观察子方块跟着一起消失 / 出现）
//! - 1：隐藏 / 显示红色独立方块（演示 Visible ↔ Hidden 直接切换）

use bevy::{prelude::*, text::FontSourceTemplate};

// 中文字体路径
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// 标记：父方块
#[derive(Component)]
struct Parent;

// 标记：子方块
#[derive(Component)]
struct Child;

// 标记：红色独立方块
#[derive(Component)]
struct Red;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        // .chain() 保证顺序：先处理输入，再更新文本（读到的 InheritedVisibility 是本帧最新值）
        .add_systems(Update, (handle_input, update_text).chain())
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // 父方块（黄色）+ 子方块（蓝色，挂在父方块下）。
    // 两者都没显式设 Visibility，默认是 Inherited。
    commands
        .spawn((
            Parent,
            Sprite::from_color(Color::srgb(0.9, 0.8, 0.2), Vec2::splat(50.0)),
            Transform::from_xyz(-180.0, 0.0, 0.0),
        ))
        .with_children(|p| {
            p.spawn((
                Child,
                Sprite::from_color(Color::srgb(0.2, 0.6, 1.0), Vec2::splat(22.0)),
                Transform::from_xyz(120.0, 0.0, 0.0),
            ));
        });

    // 红色独立方块：显式设为 Visible，用于演示 Visible ↔ Hidden 直接切换
    commands.spawn((
        Red,
        Visibility::Visible,
        Sprite::from_color(Color::srgb(0.9, 0.3, 0.3), Vec2::splat(40.0)),
        Transform::from_xyz(60.0, 80.0, 0.0),
    ));

    commands.spawn_scene(bsn! {
        Text2d::new("")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(20.0),
        }
        Transform::from_xyz(0.0, -240.0, 0.0)
    });
}

// 处理输入：T 切换父方块（继承），1 切换红色方块（直接）。
fn handle_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut parent: Single<&mut Visibility, With<Parent>>,
    mut red: Single<&mut Visibility, With<Red>>,
) {
    // 父方块：在 Inherited 和 Hidden 之间切换。
    // Inherited 时父方块（根实体）可见，Hidden 时隐藏并传播给子方块。
    if keys.just_pressed(KeyCode::KeyT) {
        parent.toggle_inherited_hidden();
        info!("[可见性] 父方块切换为 {:?}", *parent);
    }

    // 红色方块：在 Visible 和 Hidden 之间切换（它初始是 Visible，所以用这个 toggle）
    if keys.just_pressed(KeyCode::Digit1) {
        red.toggle_visible_hidden();
        info!("[可见性] 红色方块切换为 {:?}", *red);
    }
}

// 更新提示文本：显示父/子/红方块的设置值和实际可见性。
fn update_text(
    parent: Single<&Visibility, With<Parent>>,
    parent_inherited: Single<&InheritedVisibility, With<Parent>>,
    child_inherited: Single<&InheritedVisibility, With<Child>>,
    red: Single<&Visibility, With<Red>>,
    mut text: Single<&mut Text2d>,
    mut last: Local<String>,
) {
    let new_text = format!(
        "T：切换父方块  |  1：切换红方块\n\
         父设置={:?} 实际={} | 子实际={} | 红设置={:?}",
        *parent,
        parent_inherited.get(),
        child_inherited.get(),
        *red
    );

    // 节流：内容变化才更新文本，避免 CJK 重排刷屏
    if *last != new_text {
        *last = new_text.clone();
        text.0 = new_text;
    }
}
