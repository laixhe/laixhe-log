//! Bevy 0.19 入门示例：演示 bevy_egui 在 3D 环境中的用法。
//!
//! egui 是「屏幕空间覆盖层」：无论世界是 2D 还是 3D，它都直接叠加在画面最上层。
//! 这是 3D 游戏里最常见的「游戏 UI」形态——3D 场景负责世界，egui 负责面板。
//!
//! 学习重点：
//! - UI 系统写法与 2D 完全一样：EguiPlugin + EguiPrimaryContextPass + EguiContexts，
//!   只需把相机从 Camera2d 换成 Camera3d
//! - egui 控件直接驱动 3D 世界：滑块控制相机环绕 / 物体自转，勾选框开关灯光
//! - 相机环绕：用极坐标（距离 + 角度）算出相机位置，looking_at 看向场景中心
//! - 灯光动态控制：DirectionalLight（太阳平行光）+ PointLight（点光源）
//!
//! 操作：拖动滑块环绕相机、调整距离；勾选「物体自转」并调速度；开关点光源。

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::text::Font;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};

// 中文字体路径（项目统一使用 Yozai 圆体）
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// 面板状态：egui 是即时模式，控件的值由我们自己保存
#[derive(Resource)]
struct PanelState {
    auto_spin: bool,       // 勾选框：物体是否自转
    spin_speed: f32,       // 滑块：自转速度
    orbit_distance: f32,   // 滑块：相机环绕半径
    orbit_angle: f32,      // 滑块：相机环绕角度（弧度）
    light_on: bool,        // 勾选框：点光源开关
}

impl Default for PanelState {
    fn default() -> Self {
        Self {
            auto_spin: true,
            spin_speed: 1.0,
            orbit_distance: 8.0,
            orbit_angle: 0.0,
            light_on: true,
        }
    }
}

// 标记组件：会自转的物体
#[derive(Component)]
struct Spinner;

// 标记组件：可开关的点光源
#[derive(Component)]
struct ToggleLight;

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
        .add_plugins(EguiPlugin::default())
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .init_resource::<PanelState>()
        .add_systems(Startup, (setup, setup_egui_font))
        .add_systems(EguiPrimaryContextPass, (apply_egui_font, egui_ui_system))
        // 普通 Bevy 系统：把面板状态应用到 3D 世界
        .add_systems(Update, (update_camera, spin_objects, update_light))
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 3D 相机：从 (8, 2, 0) 看向原点，Y 轴朝上
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(8.0, 2.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 方向光（太阳）：平行光，illuminance 单位 lux
    commands.spawn((
        DirectionalLight {
            illuminance: 5000.0,
            ..default()
        },
        Transform::from_xyz(4.0, 10.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 点光源：暖色，可从面板开关（改成 0 即关闭）
    commands.spawn((
        ToggleLight,
        PointLight {
            color: Color::srgb(1.0, 0.8, 0.5),
            intensity: 2000.0,
            range: 6.0,
            radius: 0.2,
            ..default()
        },
        Transform::from_xyz(-2.0, 2.5, 2.0),
    ));

    // 地面平面：half_size 6 → 实际 12x12
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(6.0)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.3, 0.35),
            perceptual_roughness: 0.9,
            ..default()
        })),
    ));

    // 红色立方体（会自转）
    commands.spawn((
        Spinner,
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.2),
            ..default()
        })),
        Transform::from_xyz(-2.0, 0.5, 0.0),
    ));

    // 蓝色球体（会自转，带一点金属感）
    commands.spawn((
        Spinner,
        Mesh3d(meshes.add(Sphere::new(0.6))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.4, 0.9),
            metallic: 0.3,
            perceptual_roughness: 0.4,
            ..default()
        })),
        Transform::from_xyz(2.0, 0.6, 0.0),
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
    if *applied {
        return Ok(());
    }
    let Some(font_handle) = font_handle else {
        return Ok(());
    };
    let Some(font) = fonts.get(&font_handle.0) else {
        return Ok(());
    };

    let mut font_defs = egui::FontDefinitions::default();
    font_defs.font_data.insert(
        "yozai".to_owned(),
        egui::FontData::from_owned(font.data.as_ref().to_vec()).into(),
    );
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

// egui 主 UI 系统：面板里的控件（和 2D 示例写法完全一样）
fn egui_ui_system(mut state: ResMut<PanelState>, mut contexts: EguiContexts) -> Result {
    let ctx = contexts.ctx_mut()?;

    egui::Window::new("3D 控制面板")
        .default_width(300.0)
        .show(ctx, |ui| {
            ui.heading("控制 3D 场景");
            ui.separator();

            // 物体自转
            ui.checkbox(&mut state.auto_spin, "物体自转");
            ui.add(egui::Slider::new(&mut state.spin_speed, 0.0..=5.0).text("自转速度"));

            ui.separator();

            // 相机环绕（极坐标：距离 + 角度）
            ui.add(egui::Slider::new(&mut state.orbit_distance, 3.0..=15.0).text("相机距离"));
            ui.add(egui::Slider::new(
                &mut state.orbit_angle,
                0.0..=std::f32::consts::TAU,
            ).text("环绕角度"));

            ui.separator();

            // 灯光开关
            ui.checkbox(&mut state.light_on, "点光源");

            // 实时显示相机位置（和 update_camera 里相同的公式）
            ui.label(format!(
                "相机位置 ({:.1}, {:.1}, {:.1})",
                state.orbit_distance * state.orbit_angle.cos(),
                2.0,
                state.orbit_distance * state.orbit_angle.sin(),
            ));
        });
    Ok(())
}

// 相机环绕：把极坐标（距离 + 角度）换算成位置，看向场景中心
fn update_camera(state: Res<PanelState>, mut q: Single<&mut Transform, With<Camera3d>>) {
    q.translation = Vec3::new(
        state.orbit_distance * state.orbit_angle.cos(),
        2.0,
        state.orbit_distance * state.orbit_angle.sin(),
    );
    q.look_at(Vec3::ZERO, Vec3::Y);
}

// 物体自转：勾选时按设定速度绕 Y 轴旋转
fn spin_objects(
    state: Res<PanelState>,
    time: Res<Time>,
    mut q: Query<&mut Transform, With<Spinner>>,
) {
    if state.auto_spin {
        for mut transform in &mut q {
            transform.rotate_y(time.delta_secs() * state.spin_speed);
        }
    }
}

// 点光源开关：把 intensity 改成 0（关）或原值（开）
fn update_light(state: Res<PanelState>, mut q: Query<&mut PointLight, With<ToggleLight>>) {
    for mut light in &mut q {
        light.intensity = if state.light_on { 2000.0 } else { 0.0 };
    }
}
