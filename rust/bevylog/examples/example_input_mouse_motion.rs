//! Bevy 0.19 入门示例：演示鼠标移动（相对位移）与拖拽检测。
//! 按住鼠标左键并移动，蓝色光标会跟随鼠标「相对位移」移动；松开后停止，模拟拖拽。
//!
//! 学习重点：
//! - `AccumulatedMouseMotion` 资源：本帧鼠标相对移动量（delta: Vec2），每帧自动重置
//! - 相对位移 vs 绝对位置：`cursor_position()` 是「鼠标在窗口里的绝对坐标」，
//!   `AccumulatedMouseMotion.delta` 是「这一帧移动了多少」——FPS 视角、拖拽常用相对位移
//! - `ButtonInput<MouseButton>::pressed` 检测「按住中」→ 实现拖拽
//! - 窗口坐标 +y 朝下、世界坐标 +y 朝上 → 用相对位移时 y 需取反
//! - 用资源 `DragState` 在多个系统间共享拖拽状态（Local 只能在一个系统内保存状态）
//!
//! 操作方式：
//! - 按住鼠标左键并拖动：蓝色光标跟随移动
//! - 松开左键：光标停止（拖拽结束）

use bevy::input::mouse::{AccumulatedMouseMotion, MouseButton};
use bevy::window::PrimaryWindow;
use bevy::{prelude::*, text::FontSourceTemplate};

// 中文字体路径
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        // .chain() 保证顺序：先移动光标，再更新提示文本
        .add_systems(Update, (move_cursor, update_info_text).chain())
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // 蓝色光标：按住左键拖动时跟随鼠标相对位移移动
    commands.spawn((
        Cursor,
        Sprite::from_color(Color::srgb(0.3, 0.7, 1.0), Vec2::splat(24.0)),
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

// 光标标记（用来精确定位「那个蓝色方块」）
#[derive(Component)]
struct Cursor;

// 拖拽状态（跨系统共享：move_cursor 写入，update_info_text 读取）
#[derive(Resource, Default)]
struct DragState {
    active: bool,
    last_delta: Vec2,
}

// 根据鼠标相对位移移动光标（仅按住左键时，模拟拖拽）。
fn move_cursor(
    // 本帧鼠标相对位移：鼠标这一帧移动了多少（窗口像素坐标，+y 朝下）
    motion: Res<AccumulatedMouseMotion>,
    // 鼠标按键：检测左键是否按住中
    mouse: Res<ButtonInput<MouseButton>>,
    mut cursor: Single<&mut Transform, With<Cursor>>,
    mut drag: ResMut<DragState>,
) {
    let delta = motion.delta;

    // 拖拽 = 左键按住中（pressed）；just_pressed 只在本帧刚按下瞬间为 true，不适合「按住期间持续」
    drag.active = mouse.pressed(MouseButton::Left);

    if drag.active {
        // 相对位移坐标转换：窗口坐标 +y 朝下，2D 世界坐标 +y 朝上，所以 y 取反。
        // x 方向两者一致，直接累加。
        cursor.translation += Vec3::new(delta.x, -delta.y, 0.0);
    }

    drag.last_delta = delta;
}

// 更新底部提示文本：显示拖拽状态、本帧相对位移、光标位置。
fn update_info_text(
    window: Single<&Window, With<PrimaryWindow>>,
    cursor: Single<&Transform, With<Cursor>>,
    drag: Res<DragState>,
    mut text: Single<&mut Text2d>,
    mut last_text: Local<String>,
) {
    // 绝对鼠标位置（窗口坐标），用于对比「相对位移」
    let abs = window
        .cursor_position()
        .map(|p| format!("({:.0}, {:.0})", p.x, p.y))
        .unwrap_or_else(|| "窗口外".to_string());

    let state = if drag.active { "拖拽中" } else { "空闲" };
    let pos = cursor.translation;
    let new_text = format!(
        "按住左键拖动光标 | 状态：{state} | 相对位移：({:.1}, {:.1}) | 绝对位置：{abs} | 光标：({:.0}, {:.0})",
        drag.last_delta.x, drag.last_delta.y, pos.x, pos.y
    );

    // 节流：内容变化才更新文本，避免 CJK 重排刷屏
    if *last_text != new_text {
        *last_text = new_text.clone();
        text.0 = new_text;
    }
}
