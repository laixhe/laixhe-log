//! Bevy 0.19 入门示例：演示 bevy_egui 在 2D 环境中的用法 —— 即时模式 GUI 库。
//!
//! egui 与 Bevy 原生 UI 的区别：
//! - Bevy 原生 UI 是「保留模式」：UI 节点树常驻世界，用组件描述结构，系统响应事件。
//! - egui 是「即时模式」：每一帧从头重画整个界面，没有常驻的节点树。
//!   因此控件的值（滑块、勾选框、文本框……）必须由我们自己保存（本例存在 PanelState 资源里）。
//!
//! 学习重点：
//! - 插件：.add_plugins(EguiPlugin::default())
//! - UI 系统注册在 EguiPrimaryContextPass 调度（每帧执行，egui 上下文已就绪）
//! - EguiContexts::ctx_mut() 拿到 egui::Context，之后全是纯 egui 代码
//! - 常用控件：Window / Button / Checkbox / Slider / TextEdit
//! - 中文字体：egui 内置字体没有 CJK 字形，需要把 ttf 注入 FontDefinitions
//! - 与 Bevy 世界交互：egui 控件直接读写 Bevy 的组件（精灵缩放）与资源（状态）
//!
//! 操作：拖动 egui 窗口；点击按钮、勾选「旋转」、拖动「缩放」滑块、输入文字，
//! 观察屏幕中央精灵的变化和终端日志。
//!
//! 3D 环境版本见 example_egui_3d.rs。

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::text::{Font, FontSource};
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};

// 中文字体路径（项目统一使用 Yozai 圆体）
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// 面板状态：egui 是即时模式，控件当前的「值」必须由我们自己保存
#[derive(Resource)]
struct PanelState {
    click_count: u32, // 按钮点击次数
    rotate: bool,     // 勾选框：精灵是否自转
    scale: f32,       // 滑块：精灵缩放倍率
    message: String,  // 文本框：世界文本显示的内容
}

impl Default for PanelState {
    fn default() -> Self {
        Self {
            click_count: 0,
            rotate: false,
            scale: 1.0,
            message: "你好，egui！".to_string(),
        }
    }
}

// 标记组件：受 egui 面板控制的精灵 / 世界文本（分开标记，查询时不互相干扰）
#[derive(Component)]
struct DemoSprite;
#[derive(Component)]
struct DemoText;

// 待注入 egui 的中文字体句柄（bevy 异步加载完成后注入）
#[derive(Resource)]
struct EguiFontHandle(Handle<Font>);

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            // 静默 icu_provider 缺 CJK 字典数据的警告（保留默认的 wgpu/naga 过滤）
            filter: "wgpu=error,naga=warn,icu_provider=off".to_string(),
            ..default()
        }))
        // 注册 egui 插件：负责输入转发、上下文创建、UI 渲染
        .add_plugins(EguiPlugin::default())
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .init_resource::<PanelState>()
        .add_systems(Startup, (setup, setup_egui_font))
        // egui 的 UI 系统注册在 EguiPrimaryContextPass：每帧执行，且主上下文已就绪
        .add_systems(EguiPrimaryContextPass, (apply_egui_font, egui_ui_system))
        // 精灵自转：普通 Bevy 系统，每帧读取面板状态
        .add_systems(Update, rotate_sprite)
        .run()
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // 屏幕中央的精灵：受 egui 面板控制（缩放 / 旋转）
    commands.spawn((
        DemoSprite,
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.2, 0.6, 1.0)))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // 面板下方的世界文本：实时显示文本框输入的内容
    commands.spawn((
        DemoText,
        Text2d::new(""),
        TextColor(Color::WHITE),
        TextFont {
            font: FontSource::Handle(asset_server.load(FONT_PATH)),
            font_size: FontSize::Px(24.0),
            ..default()
        },
        Transform::from_xyz(0.0, -200.0, 0.0),
    ));
}

// 用 bevy 的 AssetServer 异步加载字体，保存句柄供注入 egui
fn setup_egui_font(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(EguiFontHandle(asset_server.load(FONT_PATH)));
}

// 等字体加载完成后，把它注入 egui 的 FontDefinitions（放在比例字体族最前面）
fn apply_egui_font(
    font_handle: Option<Res<EguiFontHandle>>,
    fonts: Res<Assets<Font>>,
    mut contexts: EguiContexts,
    mut applied: Local<bool>,
) -> Result {
    // 已注入过就直接返回
    if *applied {
        return Ok(());
    }
    // 字体资源还没插入/加载完，等下一帧再试
    let Some(font_handle) = font_handle else {
        return Ok(());
    };
    let Some(font) = fonts.get(&font_handle.0) else {
        return Ok(());
    };

    // FontDefinitions 描述 egui 使用的所有字体；默认只有英文字形
    let mut font_defs = egui::FontDefinitions::default();
    font_defs.font_data.insert(
        "yozai".to_owned(),
        egui::FontData::from_owned(font.data.as_ref().to_vec()).into(),
    );
    // 把中文字体插到 Proportional 族最前面，中文会优先用它渲染
    font_defs
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "yozai".to_owned());
    font_defs
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("yozai".to_owned());
    contexts.ctx_mut()?.set_fonts(font_defs);
    *applied = true;
    Ok(())
}

// egui 主 UI 系统：面板里所有控件都在这里声明（注意返回类型是 bevy::prelude::Result）
fn egui_ui_system(
    mut state: ResMut<PanelState>,
    mut contexts: EguiContexts,
    mut q_sprite: Query<&mut Transform, With<DemoSprite>>,
    mut q_text: Query<&mut Text2d, With<DemoText>>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    egui::Window::new("egui 控制面板")
        .default_width(300.0)
        .show(ctx, |ui| {
            ui.heading("常用控件");
            ui.separator();

            // 按钮：点击计数 +1
            if ui.button("点击我").clicked() {
                state.click_count += 1;
                info!("[egui] 按钮被点击，累计 {} 次", state.click_count);
            }
            ui.label(format!("点击次数：{}", state.click_count));

            ui.separator();

            // 勾选框：控制精灵是否自转
            ui.checkbox(&mut state.rotate, "让精灵旋转");

            // 滑块：控制精灵缩放倍率
            ui.add(egui::Slider::new(&mut state.scale, 0.2..=3.0).text("精灵缩放"));

            ui.separator();

            // 单行文本框：实时反映到世界文本
            ui.label("世界文本内容：");
            ui.text_edit_singleline(&mut state.message);
        });

    // 把面板状态应用到 Bevy 世界（egui 与 Bevy 的交互点）
    if let Ok(mut transform) = q_sprite.single_mut() {
        transform.scale = Vec3::splat(state.scale);
    }
    if let Ok(mut text) = q_text.single_mut() {
        text.0 = state.message.clone();
    }
    Ok(())
}

// 精灵自转：勾选「让精灵旋转」时每帧绕 z 轴旋转
fn rotate_sprite(
    state: Res<PanelState>,
    mut q: Single<&mut Transform, With<DemoSprite>>,
    time: Res<Time>,
) {
    if state.rotate {
        q.rotate_z(time.delta_secs() * 2.0);
    }
}
