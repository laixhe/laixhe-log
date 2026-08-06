//! Bevy 0.19 入门示例：演示输入系统（鼠标按键、鼠标位置、滚轮）。
//! 鼠标左键点击在鼠标位置生成彩色圆形，右键清除所有圆形，滚轮缩放所有圆形。
//!
//! 学习重点：
//! - ButtonInput<MouseButton> 鼠标按键检测（和键盘 ButtonInput<KeyCode> 用法一致）
//! - Window::cursor_position() 获取鼠标位置（窗口逻辑坐标，原点在左上角、+y 朝下）
//! - 窗口坐标 ↔ 世界坐标的转换（窗口原点左上角、+y 朝下；世界原点屏幕中心、+y 朝上 → y 方向需翻转）
//! - AccumulatedMouseScroll 鼠标滚轮累积量（每帧自动重置）
//! - Single 查询获取主窗口（With<PrimaryWindow> 过滤）
//! - .chain() 排序多个系统（保证文本更新在输入处理之后，显示的圆形数量准确）

use bevy::input::mouse::AccumulatedMouseScroll;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        // 背景色设为黑色，让彩色圆形更醒目
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        // 三个系统用 .chain() 保证顺序：先处理输入（生成/清除/缩放），再更新文本。
        // 这样 update_info_text 显示的圆形数量是准确的（不会少算当帧生成的）。
        .add_systems(
            Update,
            (handle_mouse_input, handle_mouse_scroll, update_info_text).chain(),
        )
        .run()
}

fn setup(mut commands: Commands) {
    // 生成 2D 相机（必须有一个 Camera2d 才能看到画面）
    commands.spawn(Camera2d);
    // 生成信息文本（底部）：初始为空字符串，由 update_info_text 每帧更新内容
    commands.spawn((
        Text2d::new(""),
        TextFont {
            font_size: FontSize::Px(20.0),
            ..default()
        },
        TextColor(Color::WHITE),
        // 位置：屏幕中下方（2D 坐标系原点在屏幕中心，+y 朝上，所以 y = -280 是下方）
        Transform::from_xyz(0.0, -280.0, 0.0),
    ));
}

// 处理鼠标点击：左键生成圆形，右键清除所有圆形。
fn handle_mouse_input(
    // 鼠标按键：ButtonInput<MouseButton> 和键盘 ButtonInput<KeyCode> 用法完全一致，
    // 只是泛型参数从 KeyCode 换成了 MouseButton（Left / Right / Middle / Other(u16)）
    mouse: Res<ButtonInput<MouseButton>>,
    // 获取主窗口：With<PrimaryWindow> 过滤出主窗口（多窗口时区分哪个是主窗口）。
    // Single 表示「期望恰好一个」匹配实体；这里只读访问用 &Window（不需要 &mut）。
    window: Single<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // 查询所有圆形实体（用于右键清除）；With<Mesh2d> 过滤出带网格组件的实体
    circles: Query<Entity, With<Mesh2d>>,
) {
    // 左键点击：在鼠标位置生成一个彩色圆形
    // just_pressed 检测「本帧刚按下」（区别于 pressed 的「按住中」），避免按住时连续生成
    if mouse.just_pressed(MouseButton::Left) {
        // cursor_position 返回窗口逻辑坐标（原点在窗口左上角，+y 朝下）。
        // 返回 None 表示鼠标在窗口外。
        if let Some(cursor) = window.cursor_position() {
            // 窗口坐标 → 世界坐标转换（注意两个坐标系的 y 轴方向相反！）：
            // - 窗口坐标：原点左上角，+y 朝下
            // - 2D 世界坐标：原点屏幕中心（Camera2d 默认在原点），+y 朝上
            // 所以 x 方向直接减半宽，y 方向要用半高减 cursor.y（翻转方向）。
            let half = Vec2::new(window.width() / 2.0, window.height() / 2.0);
            let world_pos = Vec2::new(cursor.x - half.x, half.y - cursor.y);
            // 详细日志：打印坐标转换的每一步，方便调试转换逻辑。
            // 多行格式让窗口坐标、世界坐标、转换公式的对应关系一目了然。
            info!(
                "[输入] 左键点击\n  \
                 窗口尺寸   = {:.0} x {:.0}\n  \
                 窗口坐标   = ({:.0}, {:.0})  ← 原点左上角, +y 朝下\n  \
                 世界坐标   = ({:.0}, {:.0})  ← 原点屏幕中心, +y 朝上\n  \
                 转换公式   = (cursor.x - width/2, height/2 - cursor.y)\n  \
                            = ({:.0} - {:.0}, {:.0} - {:.0})",
                window.width(), window.height(),
                cursor.x, cursor.y,
                world_pos.x, world_pos.y,
                cursor.x, half.x, half.y, cursor.y,
            );
            // 基于位置生成颜色：x 越靠右越红，越靠左越绿（演示动态着色，不需要随机数）
            let t: f32 = (world_pos.x + 400.0) / 800.0;
            let color = Color::srgb(t.clamp(0.0, 1.0), (1.0 - t).clamp(0.0, 1.0), 0.5);
            // 生成圆形：网格 + 材质 + 变换 组成一个实体
            commands.spawn((
                // Circle::new(30.0) 的 30.0 是半径（世界单位 = 像素）
                Mesh2d(meshes.add(Circle::new(30.0))),
                MeshMaterial2d(materials.add(ColorMaterial::from(color))),
                Transform::from_xyz(world_pos.x, world_pos.y, 0.0),
            ));
        }
    }

    // 右键点击：清除所有圆形
    if mouse.just_pressed(MouseButton::Right) {
        // 遍历所有圆形实体并 despawn（销毁实体及其所有组件）
        for entity in &circles {
            commands.entity(entity).despawn();
        }
        info!("[输入] 清除所有圆形");
    }
}

// 处理鼠标滚轮：缩放所有圆形。
fn handle_mouse_scroll(
    // 鼠标滚轮累积量：AccumulatedMouseScroll 是资源，记录本帧滚轮的累积滚动量。
    // - delta.y > 0 表示向上滚（通常放大）
    // - delta.y < 0 表示向下滚（通常缩小）
    // 每帧自动重置（由 Bevy 内部的 accumulate_mouse_scroll_system 系统处理）。
    scroll: Res<AccumulatedMouseScroll>,
    // 查询所有圆形的 Transform（用于缩放）
    mut circles: Query<&mut Transform, With<Mesh2d>>,
) {
    // 只在滚轮有滚动时处理（delta.y != 0）
    if scroll.delta.y != 0.0 {
        // 滚轮向上放大，向下缩小：缩放因子 = 1 + 滚动量 × 灵敏度
        let scale_delta = 1.0 + scroll.delta.y * 0.1;
        // 日志：打印滚轮原始量和计算出的缩放因子，方便调试滚轮灵敏度
        info!(
            "[输入] 滚轮 | delta.y = {:.2} | 缩放因子 = {:.2}x | 圆形数 = {}",
            scroll.delta.y,
            scale_delta,
            circles.iter().count()
        );
        for mut transform in &mut circles {
            // scale 是 Transform 的缩放字段（Vec3），.x / .y 是水平和垂直缩放。
            // 乘以 scale_delta 实现等比缩放（x 和 y 同时缩放）
            transform.scale.x *= scale_delta;
            transform.scale.y *= scale_delta;
            // 限制缩放范围（0.2~5.0 倍），避免太大或太小看不见
            transform.scale.x = transform.scale.x.clamp(0.2, 5.0);
            transform.scale.y = transform.scale.y.clamp(0.2, 5.0);
            // 日志：打印每个圆形缩放后的实际 scale（圆形数多时会刷屏，调试时可注释掉）
            debug!(
                "[输入]   圆形缩放后 scale = ({:.2}, {:.2})",
                transform.scale.x, transform.scale.y
            );
        }
    }
}

// 更新底部信息文本：显示圆形数量和鼠标坐标。
fn update_info_text(
    window: Single<&Window, With<PrimaryWindow>>,
    // 查询所有圆形（用于计数）；只需要计数，不需要访问数据，所以用 &Mesh2d
    circles: Query<&Mesh2d>,
    // 查询信息文本：Single<&mut Text2d> 期望恰好一个匹配实体
    mut text: Single<&mut Text2d>,
    // 系统本地状态：记录上一帧鼠标是否在窗口内，用来检测「进入 / 离开窗口」的瞬间
    mut last_in_window: Local<bool>,
) {
    // 计数圆形数量
    let count = circles.iter().count();
    // 获取鼠标坐标，在窗口外时显示「窗口外」
    let cursor_pos = window.cursor_position();
    // 检测鼠标进入 / 离开窗口的瞬间（只在状态变化时打印，避免每帧刷屏）
    let in_window = cursor_pos.is_some();
    if in_window != *last_in_window {
        if in_window {
            info!("[输入] 鼠标进入窗口");
        } else {
            info!("[输入] 鼠标离开窗口");
        }
        *last_in_window = in_window;
    }
    let cursor = cursor_pos
        .map(|p| format!("({:.0}, {:.0})", p.x, p.y))
        .unwrap_or_else(|| "窗口外".to_string());
    // Text2d(pub String) 是元组结构体，字段 pub，用 .0 直接访问内部 String 赋值
    // 标注「窗口」：明确显示的是窗口坐标（原点左上角、+y 朝下），与圆形位置用的世界坐标（原点中心、+y 朝上）区分开
    text.0 = format!(
        "左键生成 | 右键清除 | 滚轮缩放 | 圆形：{} | 鼠标(窗口)：{}",
        count, cursor
    );
}
