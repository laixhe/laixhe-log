//! Bevy 0.19 入门示例：演示 virtual_joystick —— 虚拟摇杆（移动端 / 触摸屏 / 鼠标模拟）。
//!
//! 虚拟摇杆把鼠标 / 触摸输入映射成一个「轴值」Vec2（范围 -1~1），
//! 适合手游触屏或桌面鼠标模拟，省去手写触摸跟踪和死区逻辑。
//!
//! 核心概念：
//! - VirtualJoystickPlugin<S>：注册摇杆插件，S 是摇杆 ID 的类型（如 String，多摇杆区分用）
//! - 摇杆结构：根节点（VirtualJoystickNode） + 三个子节点
//!   （交互区 InteractionArea / 底盘 UIBackground / 摇杆头 UIKnob）
//! - Behavior 行为：JoystickFixed 固定位置 / JoystickFloating 点哪出现哪 /
//!   JoystickDynamic 可拖动底盘 / JoystickDeadZone 死区 / 单轴 / 隐形
//! - 读取输入：MessageReader<VirtualJoystickMessage<S>>，msg.axis() 返回轴值
//!
//! 操作：鼠标在左下角摇杆上按住并拖动（或触摸屏），蓝色方块随之移动；
//! 终端输出按下 / 松开日志，左下角文本实时显示轴值。

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::text::{FontSource, FontSourceTemplate};
use virtual_joystick::*;

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";
const SPEED: f32 = 300.0; // 移动速度（像素/秒）

// 玩家标记
#[derive(Component)]
struct Player;

// 轴值文本标记
#[derive(Component)]
struct AxisText;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            // 静默 icu_provider 缺 CJK 字典数据的警告（保留默认的 wgpu/naga 过滤）
            filter: "wgpu=error,naga=warn,icu_provider=off".to_string(),
            ..default()
        }))
        // 注册摇杆插件：S = String，用字符串区分多个摇杆
        .add_plugins(VirtualJoystickPlugin::<String>::default())
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, (setup, spawn_joystick))
        .add_systems(Update, (move_player, show_axis))
        .run()
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // 玩家：一个蓝色圆形
    commands.spawn((
        Player,
        Mesh2d(meshes.add(Circle::new(30.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.2, 0.6, 1.0)))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // 轴值文本：显示在摇杆上方，由 show_axis 更新
    commands.spawn((
        AxisText,
        Text2d::new("摇杆轴值：(0.00, 0.00)"),
        TextColor(Color::srgb(0.7, 0.8, 1.0)),
        TextFont {
            font: FontSource::Handle(asset_server.load(FONT_PATH)),
            font_size: FontSize::Px(18.0),
            ..default()
        },
        Transform::from_xyz(-230.0, -210.0, 0.0),
    ));

    // 底部提示文本
    commands.spawn_scene(bsn! {
        Text2d::new("按住左下角摇杆拖动（鼠标 / 触摸）")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(20.0),
        }
        Transform::from_xyz(0.0, -280.0, 0.0)
    });
}

// 手动搭建摇杆（不用图片，纯色圆形 UI 节点）：
// 根节点 = VirtualJoystickBundle + Node 样式；子节点 = 交互区 / 底盘 / 摇杆头
fn spawn_joystick(mut commands: Commands) {
    commands
        .spawn((
            VirtualJoystickBundle::new(
                VirtualJoystickNode::<String>::default()
                    .with_id("move".to_string())
                    .with_behavior(JoystickFixed),
            )
            .set_style(Node {
                width: px(160),
                height: px(160),
                position_type: PositionType::Absolute,
                left: px(50),
                bottom: px(50),
                ..default()
            }),
        ))
        .with_children(|parent| {
            // 交互区：占满整个摇杆区域，负责接收鼠标 / 触摸输入
            parent.spawn((
                VirtualJoystickInteractionArea,
                Node {
                    width: percent(100),
                    height: percent(100),
                    ..default()
                },
            ));
            // 底盘：半透明白色圆形（update_ui 系统会按需移动它）
            parent.spawn((
                VirtualJoystickUIBackground,
                Node {
                    position_type: PositionType::Absolute,
                    width: px(160),
                    height: px(160),
                    border_radius: BorderRadius::MAX,
                    ..default()
                },
                BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.12)),
            ));
            // 摇杆头：蓝色圆形，位置由 update_ui 按轴值每帧更新
            parent.spawn((
                VirtualJoystickUIKnob,
                Node {
                    position_type: PositionType::Absolute,
                    width: px(64),
                    height: px(64),
                    border_radius: BorderRadius::MAX,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.3, 0.7, 1.0, 0.9)),
                ZIndex(1),
            ));
        });
}

// 读取摇杆消息驱动玩家移动
fn move_player(
    mut reader: MessageReader<VirtualJoystickMessage<String>>,
    mut player: Single<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    for msg in reader.read() {
        // axis() 返回 Vec2，范围 -1~1；y 方向库已按直觉反转（上推为正）
        let axis = *msg.axis();
        player.translation += axis.extend(0.0) * SPEED * time.delta_secs();

        // 事件类型：Press 按下 / Drag 拖动中 / Up 松开
        match msg.get_type() {
            VirtualJoystickMessageType::Press => {
                info!("[摇杆] 按下 @ {:?}", msg.value());
            }
            VirtualJoystickMessageType::Up => {
                info!("[摇杆] 松开 @ {:?}", msg.value());
            }
            VirtualJoystickMessageType::Drag => {}
        }
    }
}

// 实时显示当前轴值
fn show_axis(
    mut reader: MessageReader<VirtualJoystickMessage<String>>,
    mut text: Single<&mut Text2d, With<AxisText>>,
) {
    for msg in reader.read() {
        let axis = msg.axis();
        text.0 = format!("摇杆轴值：({:.2}, {:.2})", axis.x, axis.y);
    }
}
