//! Bevy 0.19 入门示例：演示 Bevy 原生 UI 在 3D 环境中的用法。
//! 3D 场景 + 屏幕空间 UI（HUD 信息面板 + 按钮），按钮交互实时控制 3D 世界。
//!
//! 学习重点：
//! - UI 是「屏幕空间覆盖层」：无论 Camera2d 还是 Camera3d，Node 都渲染在画面最上层，
//!   Camera3d 同时渲染 3D 世界和 UI，不需要单独的 UI 相机
//! - 原生 UI 是「保留模式」：节点树常驻世界，用 Interaction 响应事件
//!   （对比 example_egui_3d 的即时模式 egui，两者都能做 3D 游戏 UI）
//! - 常见 HUD 模式：Update 系统每帧更新文本（FPS / 相机位置）、按钮读写状态资源
//! - 3D 控制：旋转物体 / 开关方向光 / 重置相机
//!
//! 操作：点击「自转：开/关」控制物体旋转；「灯光：开/关」切换方向光；「重置相机」恢复视角。

use bevy::{log::LogPlugin, prelude::*, text::FontSourceTemplate};

// 中文字体路径
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// 全局游戏状态：UI 按钮和 3D 世界系统共享
#[derive(Resource)]
struct GameState {
    spin: bool,     // 物体是否自转
    light_on: bool, // 方向光是否开启
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            spin: true,
            light_on: true,
        }
    }
}

// 标记组件（bsn! 宏要求组件实现 Clone + Default，宏内部用模板反射构造实体）
#[derive(Component, Clone, Default)]
struct SpinObject; // 会自转的 3D 物体
#[derive(Component, Clone, Default)]
struct HudCamera; // 3D 相机（HUD 查询它的位置）
#[derive(Component, Clone, Default)]
struct HudInfo; // 顶部信息文本
#[derive(Component, Clone, Default)]
struct SpinButton; // 自转切换按钮
#[derive(Component, Clone, Default)]
struct LightButton; // 灯光切换按钮
#[derive(Component, Clone, Default)]
struct ResetButton; // 重置相机按钮

// 按钮三种状态的颜色常量
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

// 相机初始位置（重置按钮会恢复到这里）
const CAMERA_START: Vec3 = Vec3::new(0.0, 3.0, 8.0);

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            // 静默 icu_provider 缺 CJK 字典数据的警告（保留默认的 wgpu/naga 过滤）
            filter: "wgpu=error,naga=warn,icu_provider=off".to_string(),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.1, 0.12, 0.18)))
        .init_resource::<GameState>()
        .add_systems(Startup, setup)
        // 三个系统互不依赖，每帧执行：
        // - update_objects：把状态应用到 3D 世界（旋转 / 灯光）
        // - update_hud：刷新 HUD 文本（FPS / 相机位置）
        // - button_system：处理按钮交互
        .add_systems(Update, (update_objects, update_hud, button_system))
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ---- 3D 场景 ----
    // 3D 相机：从 (0, 3, 8) 看向原点。Camera3d 同时负责渲染 3D 世界和 UI
    commands.spawn((
        HudCamera,
        Camera3d::default(),
        Transform::from_xyz(0.0, 3.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 方向光（太阳）：illuminance 单位 lux
    commands.spawn((
        DirectionalLight {
            illuminance: 5000.0,
            ..default()
        },
        Transform::from_xyz(3.0, 8.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 地面平面
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
        SpinObject,
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.2),
            ..default()
        })),
        Transform::from_xyz(-2.0, 0.5, 0.0),
    ));

    // 蓝色球体（会自转）
    commands.spawn((
        SpinObject,
        Mesh3d(meshes.add(Sphere::new(0.6))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.4, 0.9),
            metallic: 0.3,
            perceptual_roughness: 0.4,
            ..default()
        })),
        Transform::from_xyz(2.0, 0.6, 0.0),
    ));

    // ---- 屏幕空间 UI（HUD）----
    commands.spawn_scene(bsn! {
        // 根容器：占满全屏，纵向排列，上下留边
        Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            padding: UiRect::all(px(16.0)),
        }
        Children [
            // 顶部面板：信息 + 三个按钮
            (
                Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: px(10.0),
                    padding: UiRect::all(px(14.0)),
                    border_radius: BorderRadius::all(px(10.0)),
                }
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.55))
                Children [
                    (
                        // HUD 信息文本：每帧由 update_hud 更新
                        Text::new("")
                        TextColor(Color::WHITE)
                        TextFont {
                            font: FontSourceTemplate::Handle(FONT_PATH),
                            font_size: FontSize::Px(20.0),
                        }
                        HudInfo
                    ),
                    (
                        // 自转切换按钮
                        Button
                        Node {
                            width: px(140),
                            height: px(42),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border_radius: BorderRadius::all(px(8.0)),
                        }
                        BackgroundColor(NORMAL_BUTTON)
                        SpinButton
                        Children [
                            (
                                Text::new("自转：开")
                                TextColor(Color::WHITE)
                                TextFont {
                                    font: FontSourceTemplate::Handle(FONT_PATH),
                                    font_size: FontSize::Px(18.0),
                                }
                            )
                        ]
                    ),
                    (
                        Button
                        Node {
                            width: px(140),
                            height: px(42),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border_radius: BorderRadius::all(px(8.0)),
                        }
                        BackgroundColor(NORMAL_BUTTON)
                        LightButton
                        Children [
                            (
                                Text::new("灯光：开")
                                TextColor(Color::WHITE)
                                TextFont {
                                    font: FontSourceTemplate::Handle(FONT_PATH),
                                    font_size: FontSize::Px(18.0),
                                }
                            )
                        ]
                    ),
                    (
                        Button
                        Node {
                            width: px(140),
                            height: px(42),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border_radius: BorderRadius::all(px(8.0)),
                        }
                        BackgroundColor(NORMAL_BUTTON)
                        ResetButton
                        Children [
                            (
                                Text::new("重置相机")
                                TextColor(Color::WHITE)
                                TextFont {
                                    font: FontSourceTemplate::Handle(FONT_PATH),
                                    font_size: FontSize::Px(18.0),
                                }
                            )
                        ]
                    ),
                ]
            ),
            // 底部提示文本
            (
                Node {
                    align_self: AlignSelf::Center,
                }
                Children [
                    (
                        Text::new("Bevy 原生 UI：屏幕空间覆盖层，可叠加在 3D 画面上")
                        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.85))
                        TextFont {
                            font: FontSourceTemplate::Handle(FONT_PATH),
                            font_size: FontSize::Px(18.0),
                        }
                    )
                ]
            ),
        ]
    });
}

// 把状态应用到 3D 世界：自转物体 + 切换灯光
fn update_objects(
    state: Res<GameState>,
    time: Res<Time>,
    mut spin_q: Query<&mut Transform, With<SpinObject>>,
    mut light_q: Query<&mut DirectionalLight>,
) {
    if state.spin {
        for mut transform in &mut spin_q {
            transform.rotate_y(time.delta_secs());
        }
    }
    for mut light in &mut light_q {
        light.illuminance = if state.light_on { 5000.0 } else { 0.0 };
    }
}

// 刷新 HUD 信息文本：FPS + 相机位置
fn update_hud(
    time: Res<Time>,
    camera: Single<&Transform, With<HudCamera>>,
    mut info: Single<&mut Text, With<HudInfo>>,
    mut last_text: Local<String>,
) {
    let new_text = format!(
        "FPS：{:.0}\n相机位置：({:.1}, {:.1}, {:.1})",
        time.delta_secs().recip(),
        camera.translation.x,
        camera.translation.y,
        camera.translation.z,
    );
    // 内容不变时不更新，避免每帧重排 CJK 文本刷 ICU4X 警告
    if *last_text != new_text {
        *last_text = new_text.clone();
        info.0 = new_text;
    }
}

// 按钮交互：颜色反馈 + 三个按钮各自的逻辑
fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &Children,
            Option<&SpinButton>,
            Option<&LightButton>,
            Option<&ResetButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut state: ResMut<GameState>,
    mut camera: Single<&mut Transform, With<HudCamera>>,
) {
    for (interaction, mut bg_color, children, spin_btn, light_btn, reset_btn) in
        &mut interaction_query
    {
        // 颜色随悬停 / 按下变化
        *bg_color = match *interaction {
            Interaction::Pressed => PRESSED_BUTTON.into(),
            Interaction::Hovered => HOVERED_BUTTON.into(),
            Interaction::None => NORMAL_BUTTON.into(),
        };

        // 只在「按下」瞬间执行逻辑（避免按住时重复触发）
        if *interaction != Interaction::Pressed {
            continue;
        }
        if let Ok(mut text) = text_query.get_mut(children[0]) {
            if spin_btn.is_some() {
                state.spin = !state.spin;
                text.0 = if state.spin { "自转：开" } else { "自转：关" }.to_string();
                info!("[UI3D] 物体自转 = {}", state.spin);
            } else if light_btn.is_some() {
                state.light_on = !state.light_on;
                text.0 = if state.light_on { "灯光：开" } else { "灯光：关" }.to_string();
                info!("[UI3D] 方向光 = {}", state.light_on);
            } else if reset_btn.is_some() {
                camera.translation = CAMERA_START;
                camera.look_at(Vec3::ZERO, Vec3::Y);
                info!("[UI3D] 相机已重置");
            }
        }
    }
}
