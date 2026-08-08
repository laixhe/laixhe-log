//! Bevy 0.19 入门示例：演示 2D 光照与阴影（使用 bevy_firefly crate）。
//! 黑暗场景中，一个轨道运动的点光源照亮障碍物并投射动态软阴影，
//! 按空格切换阴影开关，对比有无阴影的视觉效果。
//!
//! 学习重点：
//! - bevy_firefly 是第三方 2D 光照库（Bevy 0.19 core 没有原生 2D 光照支持）
//! - FireflyPlugin 插件：必须 add 到 App 才能启用 2D 光照渲染
//! - FireflyConfig 相机配置：ambient_brightness 环境光亮度、soft_shadows 软阴影开关
//! - PointLight2d 2D 点光源：color / intensity / radius / cast_shadows
//! - Occluder2d 光照遮挡体：遮挡光线并投射阴影（需配合 Mesh2d 才能被看见）
//! - 软阴影：FireflyConfig.soft_shadows 让阴影边缘平滑过渡
//! - 轨道动画：sin / cos 让点光源绕场景旋转，展示动态阴影变化

use bevy::{prelude::*, text::FontSourceTemplate};
use bevy_firefly::prelude::*;

// 中文字体路径
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        // FireflyPlugin 必须和 DefaultPlugins 一起注册，才会启用 2D 光照渲染管线
        .add_plugins((DefaultPlugins, FireflyPlugin))
        // 背景色设为接近全黑：环境光极暗时，未被光照到的区域接近全黑，
        // 让点光源照亮的区域更突出，阴影对比度更强
        .insert_resource(ClearColor(Color::srgb(0.02, 0.02, 0.03)))
        .add_systems(Startup, setup)
        // 两个系统：orbit_light 让点光源绕场景旋转，toggle_shadows 处理空格按键。
        // 不需要 .chain()：它们访问的组件不冲突（一个改 Transform，一个改 PointLight2d）。
        .add_systems(Update, (orbit_light, toggle_shadows))
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 2D 相机 + FireflyConfig：bevy_firefly 要求相机上挂 FireflyConfig 才会渲染光照。
    // 注意：FireflyConfig 不能加到多个相机上，否则会 panic（库内部限制单相机渲染）。
    // ambient_brightness 设为 0.05（极暗）：未照亮区域接近全黑，突出点光源效果。
    //   - 0.0 = 完全无环境光（未照亮处纯黑）
    //   - 1.0 = 全亮环境光（光照效果被环境光淹没，看不出点光源）
    // soft_shadows: true 开启软阴影，让阴影边缘平滑过渡（默认就是 true，这里显式写出便于学习）。
    commands.spawn((
        Camera2d,
        FireflyConfig {
            ambient_brightness: 0.05,
            soft_shadows: true,
            ..default()
        },
    ));

    // 障碍物 1：中心灰色圆形（同时挂 Mesh2d 可见 + Occluder2d 遮挡）。
    // 重要：Occluder2d 只是「光照遮挡数据」，本身不渲染任何图形！
    // 必须配合 Mesh2d + MeshMaterial2d 才能让玩家「看见」这个障碍物。
    // mesh 和 occluder 的形状/尺寸必须对齐：mesh 是玩家看到的图形，occluder 是投射阴影的遮挡体。
    // 如果两者不一致，会出现「看到的障碍物」和「投射的阴影」不匹配（比如阴影比障碍物大或小）。
    // 这里 Circle::new(50.0) 和 Occluder2d::circle(50.0) 都是半径 50，完全对齐。
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.7, 0.7, 0.7)))),
        Occluder2d::circle(50.0),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // 障碍物 2：左侧蓝灰色矩形（全不透明遮挡体，投射纯黑阴影）。
    // Occluder2d::polygon 接收顶点列表，返回 Option（顶点数 < 2 时返回 None）。
    // 顶点相对于实体 translation；这里用方形 4 个顶点演示多边形遮挡体。
    // polygon() 会自动检测顶点是顺时针还是逆时针，无需关心顺序。
    // opacity 默认 1.0（完全挡光，投射纯黑阴影）。
    // 顶点 ±40 和 Rectangle::new(80.0, 80.0) 对齐（80/2=40）。
    let square = Occluder2d::polygon([
        Vec2::new(-40.0, 40.0),  // 左上
        Vec2::new(40.0, 40.0),   // 右上
        Vec2::new(40.0, -40.0),  // 右下
        Vec2::new(-40.0, -40.0), // 左下
    ])
    .expect("方形遮挡体顶点数 >= 2，不会返回 None");
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(80.0, 80.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.6, 0.6, 0.8)))),
        square,
        Transform::from_xyz(-200.0, 50.0, 0.0),
    ));

    // 障碍物 3：右侧粉灰色矩形（半透明遮挡体，投射灰色「彩色」阴影）。
    // with_opacity(0.7) 链式构造：opacity 0.7 = 70% 挡光，投射半透明阴影（不是纯黑）。
    // opacity 0.0 = 完全不挡光（无阴影），1.0 = 完全挡光（纯黑阴影）。
    let half_square = Occluder2d::polygon([
        Vec2::new(-40.0, 40.0),
        Vec2::new(40.0, 40.0),
        Vec2::new(40.0, -40.0),
        Vec2::new(-40.0, -40.0),
    ])
    .expect("方形遮挡体顶点数 >= 2，不会返回 None")
    .with_opacity(0.7);
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(80.0, 80.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.8, 0.6, 0.6)))),
        half_square,
        Transform::from_xyz(200.0, 50.0, 0.0),
    ));

    // 点光源：暖色，绕场景旋转，投射阴影。
    // color：暖白色（偏黄），alpha 被忽略。
    // intensity：光强 2.0（默认 1.0，这里调亮让光照效果更明显）。
    // radius：光照范围 300 像素（默认 100），覆盖大部分场景。
    // cast_shadows：true 开启阴影投射（默认就是 true，这里显式写出便于 toggle_shadows 系统切换）。
    // spawn 位置 (180, 0) = t=0 时的轨道位置（cos(0)*180=180, sin(0)*180=0），
    // 避免光源初始在原点（中心障碍物内部），和 example_animation 行星预计算初始位置的风格一致。
    commands.spawn((
        PointLight2d {
            color: Color::srgb(1.0, 0.85, 0.6),
            intensity: 2.0,
            radius: 300.0,
            cast_shadows: true,
            ..default()
        },
        Transform::from_xyz(180.0, 0.0, 0.0),
    ));

    // 底部提示文本（spawn_scene + bsn! 宏声明式构建实体）
    commands.spawn_scene(bsn! {
        Text2d::new("2D 光照与阴影：空格切换阴影 | 点光源轨道运动")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(30.0),
        }
        Transform::from_xyz(0.0, -280.0, 0.0)
    });
}

// 点光源轨道动画：绕场景中心旋转，展示动态阴影变化。
fn orbit_light(
    time: Res<Time>,
    // Single 查询：期望恰好一个 PointLight2d 实体，多个/0 个时 panic（快速暴露配置错误）
    mut transform: Single<&mut Transform, With<PointLight2d>>,
) {
    let t = time.elapsed_secs();
    // 圆形轨道：半径 180（绕原点旋转），光源在 XY 平面运动
    let radius = 180.0;
    // cos/sin 计算圆周上的位置：X = cos×r, Y = sin×r，z 固定 0（2D 光照只在 XY 平面计算）
    transform.translation = Vec3::new(t.cos() * radius, t.sin() * radius, 0.0);
}

// 阴影开关：按空格切换点光源的 cast_shadows，对比有无阴影的视觉效果。
fn toggle_shadows(
    keyboard: Res<ButtonInput<KeyCode>>,
    // Single 查询：期望恰好一个 PointLight2d 实体
    mut light: Single<&mut PointLight2d>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        light.cast_shadows = !light.cast_shadows;
        info!(
            "[光照] 点光源阴影: {}",
            if light.cast_shadows { "开" } else { "关" }
        );
    }
}
