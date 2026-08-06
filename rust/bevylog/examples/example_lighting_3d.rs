//! Bevy 0.19 入门示例：演示光照与阴影（3D 场景）。
//! 场景包含地面、立方体、球体，一个轨道运动的点光源投射动态阴影，
//! 一个方向光源提供基础照明，按空格切换阴影开关。
//!
//! 学习重点：
//! - 3D 相机：Camera3d + looking_at 设置相机位置和朝向（Y 轴朝上，-Z 朝前）
//! - 方向光 DirectionalLight：平行光线（如太阳），illuminance 单位是 lux（勒克斯）
//! - 点光源 PointLight：从一点向四周辐射，intensity 单位是 lumens（流明），range 控制照射范围
//! - shadow_maps_enabled：开启后 3D 网格自动投射和接收阴影
//! - StandardMaterial：PBR 材质，base_color / metallic / perceptual_roughness 控制外观
//! - 3D 网格：Mesh3d + MeshMaterial3d + Cuboid / Sphere / Plane3d 等三维图元
//! - 双相机叠加：Camera3d 渲染 3D 场景，Camera2d 渲染文本覆盖层（order + ClearColorConfig::None）
//! - 轨道动画：sin / cos 让点光源绕场景旋转，展示动态阴影变化

use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        // 深蓝夜空色背景，突出灯光、金属高光和阴影对比度
        .insert_resource(ClearColor(Color::srgb(0.1, 0.12, 0.18)))
        .add_systems(Startup, setup)
        // 两个系统：orbit_light 让点光源绕场景旋转，toggle_shadows 处理空格按键。
        // 不需要 .chain()：它们访问的组件不冲突。
        .add_systems(Update, (orbit_light, toggle_shadows))
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 3D 相机：从 (6, 5, 6) 看向原点，Y 轴朝上。
    // Camera3d::default() 会自动添加 Camera 和 Projection 组件（通过 #[require]）。
    // 相机坐标系：+X 右、+Y 上、+Z 朝后（屏幕外），-Z 朝前（屏幕内）。
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(6.0, 5.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 2D 覆盖层相机：渲染在 3D 相机之上（order 越大越晚渲染）。
    // clear_color 设为 None：不清除画面，保留 3D 相机的渲染结果，只叠加 2D 文本。
    // Camera2d 是单元结构体（无字段），通过 #[require] 自动添加 Camera 组件，
    // 这里显式添加 Camera 来覆盖默认的 order 和 clear_color。
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
    ));

    // 地面：10x10 平面（half_size = 5.0 表示半宽半高，实际尺寸 10x10）。
    // Plane3d::new(法线, 半尺寸) ：法线 Vec3::Y 表示水平地面，朝上。
    // 深灰色、高粗糙度（哑光表面），用于接收阴影。
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(5.0)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.3, 0.35),
            perceptual_roughness: 0.9,
            ..default()
        })),
    ));

    // 红色立方体：非金属、高粗糙度（哑光表面，散射为主）
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.2),
            metallic: 0.0,
            perceptual_roughness: 0.8,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    // 绿色立方体：金属、低粗糙度（镜面反射，高光锐利）
    // metallic: 1.0 = 纯金属表面，反射环境光；perceptual_roughness: 0.2 = 光滑表面
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.8, 0.2),
            metallic: 1.0,
            perceptual_roughness: 0.2,
            ..default()
        })),
        Transform::from_xyz(-2.0, 0.5, 1.0),
    ));

    // 蓝色球体：非金属、中等粗糙度
    // Sphere::new(0.5) 的 0.5 是半径
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.4, 0.8),
            metallic: 0.0,
            perceptual_roughness: 0.5,
            ..default()
        })),
        Transform::from_xyz(2.0, 0.5, -1.0),
    ));

    // 点光源：暖色，绕场景旋转，投射阴影。
    // intensity 单位是 lumens（流明）：3000 流明 ≈ 亮 LED 灯。
    // range 控制照射范围（世界单位），超出范围的光照衰减为零。
    // shadow_maps_enabled: true 开启阴影投射。
    commands.spawn((
        PointLight {
            color: Color::srgb(1.0, 0.9, 0.7),
            intensity: 3000.0,
            range: 15.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 4.0, 3.0),
    ));

    // 方向光：冷色（如月光），从右上前方照射，投射阴影。
    // illuminance 单位是 lux（勒克斯）：5000 lux ≈ 阴天室外。
    // 方向光本质是平行光（类似太阳光），transform.translation 本身不会影响照明效果
    // —— 但 from_xyz(3, 5, 3) 在这里仅作为 looking_at() 的参考起点，
    //   与目标点 (0,0,0) 共同决定旋转（朝向），让 -Z 轴指向目标方向。
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(0.6, 0.7, 1.0),
            illuminance: 5000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(3.0, 5.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 底部提示文本（由 2D 覆盖层相机渲染）
    commands.spawn((
        Text2d::new("光照与阴影：空格切换阴影 | 点光源轨道运动"),
        TextFont {
            font_size: FontSize::Px(30.0),
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, -280.0, 0.0),
    ));
}

// 点光源轨道动画：绕场景中心旋转，展示动态阴影变化。
fn orbit_light(
    time: Res<Time>,
    // Single 查询：期望恰好一个 PointLight 实体，多个/0 个时 panic（快速暴露配置错误）
    mut transform: Single<&mut Transform, With<PointLight>>,
) {
    let t = time.elapsed_secs();
    // 圆形轨道：半径 3（绕 Y 轴在 XZ 平面旋转），高度 4（Y 轴高度固定）
    let radius = 3.0;
    let height = 4.0;
    // cos/sin 计算圆周上的位置：XZ 平面圆周，Y 固定
    transform.translation = Vec3::new(t.cos() * radius, height, t.sin() * radius);
}

// 阴影开关：按空格同时切换点光源和方向光的阴影，对比有无阴影的视觉效果。
// 必须同时切两个光源：场景中 PointLight 和 DirectionalLight 都投射阴影，
// 只切一个的话另一个的阴影还在，视觉效果不明显（会以为「没反应」）。
fn toggle_shadows(
    keyboard: Res<ButtonInput<KeyCode>>,
    // Single 查询：期望恰好一个 PointLight 实体
    mut point_light: Single<&mut PointLight>,
    // Single 查询：期望恰好一个 DirectionalLight 实体
    mut directional_light: Single<&mut DirectionalLight>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        // 同时切换两个光源的 shadow_maps_enabled
        point_light.shadow_maps_enabled = !point_light.shadow_maps_enabled;
        directional_light.shadow_maps_enabled = !directional_light.shadow_maps_enabled;
        info!(
            "[光照] 阴影: {}",
            if point_light.shadow_maps_enabled { "开" } else { "关" }
        );
    }
}
