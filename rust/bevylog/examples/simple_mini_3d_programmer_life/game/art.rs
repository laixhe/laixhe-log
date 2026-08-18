//! 低模美术道具库：用基础几何体拼出各类小道具，供五个区域复用。
//! 所有道具都挂 GameRoot + SceneRoot，切换场景时一并清理。

use bevy::prelude::*;

use super::components::*;
use super::street_lights::StreetLamp;

// 常用低模配色
pub const WOOD: Color = Color::srgb(0.62, 0.44, 0.28);
pub const WOOD_DARK: Color = Color::srgb(0.45, 0.31, 0.20);
pub const BRICK: Color = Color::srgb(0.80, 0.55, 0.42);
pub const WALL: Color = Color::srgb(0.93, 0.86, 0.74);
pub const ROOF_RED: Color = Color::srgb(0.72, 0.25, 0.20);
pub const ROOF_BLUE: Color = Color::srgb(0.32, 0.45, 0.62);
pub const FOLIAGE: Color = Color::srgb(0.32, 0.60, 0.34);
pub const FOLIAGE_LIGHT: Color = Color::srgb(0.45, 0.72, 0.40);
pub const METAL: Color = Color::srgb(0.55, 0.60, 0.66);
pub const LAMP_YELLOW: Color = Color::srgb(1.0, 0.85, 0.45);
pub const SCREEN: Color = Color::srgb(0.22, 0.30, 0.45);
pub const GLASS: Color = Color::srgb(0.55, 0.78, 0.88);

struct Mat(Handle<StandardMaterial>);

impl Mat {
    fn new(materials: &mut Assets<StandardMaterial>, color: Color, rough: f32) -> Self {
        Self(materials.add(StandardMaterial {
            base_color: color,
            perceptual_roughness: rough,
            ..default()
        }))
    }
}

// 简单盒体 / 圆柱 / 球
fn box_mesh(meshes: &mut Assets<Mesh>, x: f32, y: f32, z: f32) -> Handle<Mesh> {
    meshes.add(Cuboid::new(x, y, z))
}

fn cyl_mesh(meshes: &mut Assets<Mesh>, r: f32, h: f32) -> Handle<Mesh> {
    meshes.add(Cylinder::new(r, h))
}

// 旋转后的 AABB 半宽（给可旋转家具挂 Solid 碰撞盒用）
fn rot_half(w: f32, d: f32, rot_y: f32) -> Vec2 {
    let (s, c) = rot_y.sin_cos();
    Vec2::new(
        (w / 2.0) * c.abs() + (d / 2.0) * s.abs(),
        (w / 2.0) * s.abs() + (d / 2.0) * c.abs(),
    )
}

// 给可碰撞实体用的便捷构造函数（half = XZ 半宽，bottom = 底部高度）
fn solid(half: Vec2, bottom: f32) -> Solid {
    Solid { half, bottom }
}

// ==================== 墙 / 屋顶 ====================
pub fn spawn_wall(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    size: Vec3,
    pos: Vec3,
    color: Color,
) {
    commands.spawn((
        GameRoot,
        SceneRoot,
        Visibility::default(),
        Mesh3d(box_mesh(meshes, size.x, size.y, size.z)),
        MeshMaterial3d(Mat::new(materials, color, 0.9).0),
        solid(Vec2::new(size.x / 2.0, size.z / 2.0), pos.y - size.y / 2.0),
        Transform::from_translation(pos),
    ));
}

/// 纯色装饰方块（非实心，不带碰撞）：热点垫下的家具 / 小道具 / 探索点装饰都用它拼装。
/// 与 `solid` 的实心家具不同，玩家可以自由穿过（如床/书桌/摊位这类"互动点位上的模型"）。
pub fn spawn_block(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    pos: Vec3,
    size: Vec3,
    color: Color,
) {
    commands.spawn((
        GameRoot,
        SceneRoot,
        Mesh3d(box_mesh(meshes, size.x, size.y, size.z)),
        MeshMaterial3d(Mat::new(materials, color, 0.9).0),
        Transform::from_translation(pos),
    ));
}

/// 单坡屋顶：一块旋转的薄板
#[allow(dead_code, clippy::too_many_arguments)]
pub fn spawn_roof(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    w: f32,
    d: f32,
    pos: Vec3,
    rot_y: f32,
    color: Color,
) {
    commands.spawn((
        GameRoot,
        SceneRoot,
        Visibility::default(),
        Mesh3d(box_mesh(meshes, w, 0.12, d)),
        MeshMaterial3d(Mat::new(materials, color, 0.8).0),
        Transform::from_translation(pos).with_rotation(Quat::from_rotation_y(rot_y)),
    ));
}

// ==================== 植物 ====================
/// 低模树：树干 + 两层球冠
pub fn spawn_tree(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    pos: Vec3,
    scale: f32,
) {
    let trunk = Mat::new(materials, Color::srgb(0.45, 0.30, 0.18), 0.9);
    let crown = Mat::new(materials, FOLIAGE, 0.9);
    let crown_light = Mat::new(materials, FOLIAGE_LIGHT, 0.9);
    let trunk_mesh = cyl_mesh(meshes, 0.22 * scale, 1.0 * scale);
    let crown_mesh = meshes.add(Sphere::new(0.95 * scale));
    let crown_small = meshes.add(Sphere::new(0.7 * scale));
    commands
        .spawn((
            GameRoot,
            SceneRoot,
            Visibility::default(),
            solid(Vec2::splat(0.35 * scale), 0.0),
            Transform::from_translation(pos).with_scale(Vec3::ONE * scale),
        ))
        .with_children(|p| {
            p.spawn((
                Mesh3d(trunk_mesh.clone()),
                MeshMaterial3d(trunk.0.clone()),
                Transform::from_xyz(0.0, 0.5, 0.0),
            ));
            p.spawn((
                Mesh3d(crown_mesh),
                MeshMaterial3d(crown.0),
                Transform::from_xyz(0.0, 1.8, 0.0),
            ));
            p.spawn((
                Mesh3d(crown_small),
                MeshMaterial3d(crown_light.0),
                Transform::from_xyz(0.25, 2.4, 0.25),
            ));
        });
}

/// 盆栽
pub fn spawn_plant(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    pos: Vec3,
) {
    let pot = Mat::new(materials, BRICK, 0.8);
    let leaf = Mat::new(materials, FOLIAGE, 0.9);
    commands
        .spawn((
            GameRoot,
            SceneRoot,
            Visibility::default(),
            solid(Vec2::splat(0.32), 0.0),
            Transform::from_translation(pos),
        ))
        .with_children(|p| {
            p.spawn((
                Mesh3d(cyl_mesh(meshes, 0.28, 0.35)),
                MeshMaterial3d(pot.0.clone()),
                Transform::from_xyz(0.0, 0.18, 0.0),
            ));
            p.spawn((
                Mesh3d(meshes.add(Sphere::new(0.34))),
                MeshMaterial3d(leaf.0),
                Transform::from_xyz(0.0, 0.65, 0.0),
            ));
        });
}

// ==================== 家具 ====================
/// 地毯
pub fn spawn_rug(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    pos: Vec3,
    size: Vec2,
    color: Color,
) {
    commands.spawn((
        GameRoot,
        SceneRoot,
        Visibility::default(),
        Mesh3d(box_mesh(meshes, size.x, 0.03, size.y)),
        MeshMaterial3d(Mat::new(materials, color, 0.95).0),
        Transform::from_translation(pos + Vec3::Y * 0.015),
    ));
}

/// 桌子：桌面 + 4 腿
pub fn spawn_table(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    pos: Vec3,
    size: Vec2,
    height: f32,
) {
    let top = Mat::new(materials, WOOD, 0.8);
    let leg = Mat::new(materials, WOOD_DARK, 0.85);
    commands
        .spawn((
            GameRoot,
            SceneRoot,
            Visibility::default(),
            solid(Vec2::new(size.x / 2.0, size.y / 2.0), 0.0),
            Transform::from_translation(pos),
        ))
        .with_children(|p| {
            p.spawn((
                Mesh3d(box_mesh(meshes, size.x, 0.1, size.y)),
                MeshMaterial3d(top.0.clone()),
                Transform::from_xyz(0.0, height - 0.05, 0.0),
            ));
            for (dx, dz) in [
                (-size.x / 2.0 + 0.15, -size.y / 2.0 + 0.15),
                (size.x / 2.0 - 0.15, -size.y / 2.0 + 0.15),
                (-size.x / 2.0 + 0.15, size.y / 2.0 - 0.15),
                (size.x / 2.0 - 0.15, size.y / 2.0 - 0.15),
            ] {
                p.spawn((
                    Mesh3d(box_mesh(meshes, 0.09, height, 0.09)),
                    MeshMaterial3d(leg.0.clone()),
                    Transform::from_xyz(dx, height / 2.0, dz),
                ));
            }
        });
}

/// 椅子
pub fn spawn_chair(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    pos: Vec3,
    rot_y: f32,
) {
    let wood = Mat::new(materials, WOOD, 0.8);
    commands
        .spawn((
            GameRoot,
            SceneRoot,
            Visibility::default(),
            solid(Vec2::splat(0.25), 0.0),
            Transform::from_translation(pos).with_rotation(Quat::from_rotation_y(rot_y)),
        ))
        .with_children(|p| {
            p.spawn((
                Mesh3d(box_mesh(meshes, 0.5, 0.07, 0.5)),
                MeshMaterial3d(wood.0.clone()),
                Transform::from_xyz(0.0, 0.45, 0.0),
            ));
            p.spawn((
                Mesh3d(box_mesh(meshes, 0.5, 0.5, 0.07)),
                MeshMaterial3d(wood.0.clone()),
                Transform::from_xyz(0.0, 0.65, -0.22),
            ));
            for (dx, dz) in [(-0.2, -0.2), (0.2, -0.2), (-0.2, 0.2), (0.2, 0.2)] {
                p.spawn((
                    Mesh3d(box_mesh(meshes, 0.06, 0.45, 0.06)),
                    MeshMaterial3d(wood.0.clone()),
                    Transform::from_xyz(dx, 0.22, dz),
                ));
            }
        });
}

/// 沙发：座 + 背 + 扶手
pub fn spawn_sofa(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    pos: Vec3,
    rot_y: f32,
) {
    let fab = Mat::new(materials, Color::srgb(0.55, 0.42, 0.32), 0.95);
    commands
        .spawn((
            GameRoot,
            SceneRoot,
            Visibility::default(),
            solid(rot_half(1.8, 0.7, rot_y), 0.0),
            Transform::from_translation(pos).with_rotation(Quat::from_rotation_y(rot_y)),
        ))
        .with_children(|p| {
            p.spawn((
                Mesh3d(box_mesh(meshes, 1.8, 0.35, 0.7)),
                MeshMaterial3d(fab.0.clone()),
                Transform::from_xyz(0.0, 0.25, 0.0),
            ));
            p.spawn((
                Mesh3d(box_mesh(meshes, 1.8, 0.6, 0.2)),
                MeshMaterial3d(fab.0.clone()),
                Transform::from_xyz(0.0, 0.65, -0.28),
            ));
            for dx in [-0.82, 0.82] {
                p.spawn((
                    Mesh3d(box_mesh(meshes, 0.18, 0.55, 0.7)),
                    MeshMaterial3d(fab.0.clone()),
                    Transform::from_xyz(dx, 0.4, 0.0),
                ));
            }
        });
}

/// 书架
pub fn spawn_bookshelf(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    pos: Vec3,
    rot_y: f32,
) {
    let wood = Mat::new(materials, WOOD_DARK, 0.85);
    let book = Mat::new(materials, Color::srgb(0.75, 0.35, 0.3), 0.9);
    commands
        .spawn((
            GameRoot,
            SceneRoot,
            Visibility::default(),
            solid(rot_half(1.6, 0.3, rot_y), 0.0),
            Transform::from_translation(pos).with_rotation(Quat::from_rotation_y(rot_y)),
        ))
        .with_children(|p| {
            p.spawn((
                Mesh3d(box_mesh(meshes, 1.6, 1.6, 0.3)),
                MeshMaterial3d(wood.0.clone()),
                Transform::from_xyz(0.0, 0.8, 0.0),
            ));
            // 几本竖放的书
            for (i, h) in [(0, 0.6), (1, 0.75), (2, 0.5), (3, 0.65)] {
                p.spawn((
                    Mesh3d(box_mesh(meshes, 0.12, h, 0.2)),
                    MeshMaterial3d(book.0.clone()),
                    Transform::from_xyz(-0.5 + i as f32 * 0.32, 0.3 + h / 2.0, 0.0),
                ));
            }
        });
}

/// 文件柜
pub fn spawn_cabinet(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    pos: Vec3,
    rot_y: f32,
) {
    let metal = Mat::new(materials, METAL, 0.6);
    let handle = Mat::new(materials, Color::srgb(0.3, 0.32, 0.38), 0.5);
    commands
        .spawn((
            GameRoot,
            SceneRoot,
            Visibility::default(),
            solid(rot_half(0.9, 0.5, rot_y), 0.0),
            Transform::from_translation(pos).with_rotation(Quat::from_rotation_y(rot_y)),
        ))
        .with_children(|p| {
            p.spawn((
                Mesh3d(box_mesh(meshes, 0.9, 1.4, 0.5)),
                MeshMaterial3d(metal.0.clone()),
                Transform::from_xyz(0.0, 0.7, 0.0),
            ));
            p.spawn((
                Mesh3d(box_mesh(meshes, 0.06, 0.5, 0.02)),
                MeshMaterial3d(handle.0),
                Transform::from_xyz(0.0, 0.95, 0.26),
            ));
        });
}

/// 显示器 + 底座（工位用）
pub fn spawn_monitor(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    pos: Vec3,
    rot_y: f32,
) {
    let screen = Mat::new(materials, SCREEN, 0.5);
    let stand = Mat::new(materials, Color::srgb(0.25, 0.25, 0.28), 0.6);
    commands
        .spawn((
            GameRoot,
            SceneRoot,
            Visibility::default(),
            Transform::from_translation(pos).with_rotation(Quat::from_rotation_y(rot_y)),
        ))
        .with_children(|p| {
            p.spawn((
                Mesh3d(box_mesh(meshes, 0.95, 0.55, 0.05)),
                MeshMaterial3d(screen.0.clone()),
                Transform::from_xyz(0.0, 0.42, 0.0),
            ));
            p.spawn((
                Mesh3d(box_mesh(meshes, 0.08, 0.3, 0.08)),
                MeshMaterial3d(stand.0.clone()),
                Transform::from_xyz(0.0, 0.18, -0.02),
            ));
            p.spawn((
                Mesh3d(box_mesh(meshes, 0.28, 0.05, 0.2)),
                MeshMaterial3d(stand.0),
                Transform::from_xyz(0.0, 0.03, -0.02),
            ));
        });
}

// ==================== 建筑 ====================
/// 楼房：体块 + 窗户条 + 屋顶
pub fn spawn_building(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    pos: Vec3,
    size: Vec3,
    body: Color,
    roof: Color,
) {
    let body_m = Mat::new(materials, body, 0.9);
    let roof_m = Mat::new(materials, roof, 0.85);
    let glass_m = Mat::new(materials, GLASS, 0.3);
    let frame_m = Mat::new(materials, WOOD_DARK, 0.85);
    commands
        .spawn((
            GameRoot,
            SceneRoot,
            Visibility::default(),
            solid(Vec2::new(size.x / 2.0, size.z / 2.0), pos.y - size.y / 2.0),
            Transform::from_translation(pos),
        ))
        .with_children(|p| {
            p.spawn((
                Mesh3d(box_mesh(meshes, size.x, size.y, size.z)),
                MeshMaterial3d(body_m.0.clone()),
                Transform::from_xyz(0.0, size.y / 2.0, 0.0),
            ));
            // 正面窗户条
            let win_w = size.x * 0.7;
            let n = (win_w / 0.7).floor().max(2.0) as i32;
            for i in 0..n {
                let wx = -win_w / 2.0 + win_w * (i as f32 + 0.5) / n as f32;
                p.spawn((
                    Mesh3d(box_mesh(meshes, 0.5, 0.45, 0.04)),
                    MeshMaterial3d(glass_m.0.clone()),
                    Transform::from_xyz(wx, size.y * 0.65, size.z / 2.0 + 0.02),
                ));
            }
            // 屋顶
            p.spawn((
                Mesh3d(box_mesh(meshes, size.x * 1.1, 0.2, size.z * 1.1)),
                MeshMaterial3d(roof_m.0.clone()),
                Transform::from_xyz(0.0, size.y + 0.1, 0.0),
            ));
            // 门
            p.spawn((
                Mesh3d(box_mesh(meshes, 0.7, 0.9, 0.05)),
                MeshMaterial3d(frame_m.0),
                Transform::from_xyz(0.0, 0.45, size.z / 2.0 + 0.02),
            ));
        });
}

/// 门框：两柱 + 横梁 + 门板
#[allow(dead_code)]
pub fn spawn_door_frame(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    pos: Vec3,
    rot_y: f32,
) {
    let wood = Mat::new(materials, WOOD_DARK, 0.85);
    let door = Mat::new(materials, BRICK, 0.9);
    commands
        .spawn((
            GameRoot,
            SceneRoot,
            Visibility::default(),
            Transform::from_translation(pos).with_rotation(Quat::from_rotation_y(rot_y)),
        ))
        .with_children(|p| {
            p.spawn((
                Mesh3d(box_mesh(meshes, 0.16, 1.0, 0.14)),
                MeshMaterial3d(wood.0.clone()),
                Transform::from_xyz(-0.85, 0.5, 0.0),
            ));
            p.spawn((
                Mesh3d(box_mesh(meshes, 0.16, 1.0, 0.14)),
                MeshMaterial3d(wood.0.clone()),
                Transform::from_xyz(0.85, 0.5, 0.0),
            ));
            p.spawn((
                Mesh3d(box_mesh(meshes, 1.86, 0.16, 0.14)),
                MeshMaterial3d(wood.0.clone()),
                Transform::from_xyz(0.0, 0.95, 0.0),
            ));
            p.spawn((
                Mesh3d(box_mesh(meshes, 1.5, 1.55, 0.06)),
                MeshMaterial3d(door.0),
                Transform::from_xyz(0.0, 0.78, -0.02),
            ));
        });
}

/// 路灯：杆 + 灯头 + 光球（光球带 StreetLamp 标记，晚上自动点亮）
pub fn spawn_lamp(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    pos: Vec3,
) {
    let pole = Mat::new(materials, METAL, 0.6);
    let bulb = Mat::new(materials, LAMP_YELLOW, 0.4);
    commands
        .spawn((
            GameRoot,
            SceneRoot,
            Visibility::default(),
            solid(Vec2::splat(0.16), 0.0),
            Transform::from_translation(pos),
        ))
        .with_children(|p| {
            p.spawn((
                Mesh3d(cyl_mesh(meshes, 0.09, 3.2)),
                MeshMaterial3d(pole.0.clone()),
                Transform::from_xyz(0.0, 1.6, 0.0),
            ));
            p.spawn((
                Mesh3d(box_mesh(meshes, 0.06, 0.5, 0.5)),
                MeshMaterial3d(pole.0.clone()),
                Transform::from_xyz(0.0, 3.0, 0.0),
            ));
            let bulb_mat = bulb.0.clone();
            p.spawn((
                StreetLamp {
                    mat: bulb_mat.clone(),
                },
                Mesh3d(meshes.add(Sphere::new(0.22))),
                MeshMaterial3d(bulb_mat),
                Transform::from_xyz(0.0, 2.9, 0.0),
            ));
        });
}

/// 长椅：座 + 背 + 腿
pub fn spawn_bench(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    pos: Vec3,
    rot_y: f32,
    is_solid: bool,
) {
    let wood = Mat::new(materials, WOOD, 0.8);
    let mut e = commands.spawn((
        GameRoot,
        SceneRoot,
        Visibility::default(),
        Transform::from_translation(pos).with_rotation(Quat::from_rotation_y(rot_y)),
    ));
    // 互动点位上的长椅（公园长椅热点）非实心，玩家可走到长椅前触发；
    // 校园/街道的纯装饰长椅带碰撞，挡路不穿行。
    if is_solid {
        e.insert(solid(rot_half(1.4, 0.5, rot_y), 0.0));
    }
    e.with_children(|p| {
            p.spawn((
                Mesh3d(box_mesh(meshes, 1.4, 0.1, 0.5)),
                MeshMaterial3d(wood.0.clone()),
                Transform::from_xyz(0.0, 0.45, 0.0),
            ));
            p.spawn((
                Mesh3d(box_mesh(meshes, 1.4, 0.45, 0.08)),
                MeshMaterial3d(wood.0.clone()),
                Transform::from_xyz(0.0, 0.68, -0.22),
            ));
            for (dx, dz) in [(-0.6, -0.18), (0.6, -0.18), (-0.6, 0.18), (0.6, 0.18)] {
                p.spawn((
                    Mesh3d(box_mesh(meshes, 0.08, 0.45, 0.08)),
                    MeshMaterial3d(wood.0.clone()),
                    Transform::from_xyz(dx, 0.22, dz),
                ));
            }
        });
}

/// 围栏段：两根立柱 + 两根横杆
pub fn spawn_fence(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    pos: Vec3,
    len: f32,
    color: Color,
) {
    let m = Mat::new(materials, color, 0.85);
    commands
        .spawn((
            GameRoot,
            SceneRoot,
            Visibility::default(),
            solid(Vec2::new(len / 2.0, 0.16), 0.0),
            Transform::from_translation(pos),
        ))
        .with_children(|p| {
            for dx in [-len / 2.0, len / 2.0] {
                p.spawn((
                    Mesh3d(box_mesh(meshes, 0.12, 0.9, 0.12)),
                    MeshMaterial3d(m.0.clone()),
                    Transform::from_xyz(dx, 0.45, 0.0),
                ));
            }
            for hy in [0.45, 0.75] {
                p.spawn((
                    Mesh3d(box_mesh(meshes, len, 0.08, 0.08)),
                    MeshMaterial3d(m.0.clone()),
                    Transform::from_xyz(0.0, hy, 0.0),
                ));
            }
        });
}

/// 吊灯：线 + 灯罩 + 光球
pub fn spawn_hanging_lamp(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    pos: Vec3,
) {
    let wire = Mat::new(materials, Color::srgb(0.2, 0.2, 0.22), 0.7);
    let shade = Mat::new(materials, Color::srgb(0.35, 0.35, 0.38), 0.7);
    let bulb = Mat::new(materials, LAMP_YELLOW, 0.4);
    commands
        .spawn((
            GameRoot,
            SceneRoot,
            Visibility::default(),
            Transform::from_translation(pos),
        ))
        .with_children(|p| {
            p.spawn((
                Mesh3d(cyl_mesh(meshes, 0.02, 2.6)),
                MeshMaterial3d(wire.0.clone()),
                Transform::from_xyz(0.0, 1.3, 0.0),
            ));
            p.spawn((
                Mesh3d(cyl_mesh(meshes, 0.3, 0.25)),
                MeshMaterial3d(shade.0),
                Transform::from_xyz(0.0, 0.1, 0.0),
            ));
            p.spawn((
                Mesh3d(meshes.add(Sphere::new(0.16))),
                MeshMaterial3d(bulb.0),
                Transform::from_xyz(0.0, -0.05, 0.0),
            ));
        });
}

/// 白板
pub fn spawn_whiteboard(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    pos: Vec3,
    rot_y: f32,
) {
    let board = Mat::new(materials, Color::srgb(0.95, 0.95, 0.93), 0.5);
    let frame = Mat::new(materials, METAL, 0.7);
    commands
        .spawn((
            GameRoot,
            SceneRoot,
            Visibility::default(),
            Transform::from_translation(pos).with_rotation(Quat::from_rotation_y(rot_y)),
        ))
        .with_children(|p| {
            p.spawn((
                Mesh3d(box_mesh(meshes, 1.8, 1.1, 0.05)),
                MeshMaterial3d(frame.0.clone()),
                Transform::from_xyz(0.0, 1.1, 0.0),
            ));
            p.spawn((
                Mesh3d(box_mesh(meshes, 1.7, 1.0, 0.03)),
                MeshMaterial3d(board.0),
                Transform::from_xyz(0.0, 1.1, 0.03),
            ));
        });
}

/// 太阳（自发光球）与云朵
pub fn spawn_sky(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let sun = meshes.add(Sphere::new(1.2));
    let sun_mat = materials.add(StandardMaterial {
        base_color: LAMP_YELLOW,
        emissive: LinearRgba::new(1.0, 0.9, 0.5, 1.0),
        perceptual_roughness: 0.3,
        ..default()
    });
    commands.spawn((
        GameRoot,
        SceneRoot,
        Visibility::default(),
        Mesh3d(sun),
        MeshMaterial3d(sun_mat),
        Transform::from_xyz(-16.0, 17.0, -17.0),
    ));

    let cloud = Mat::new(materials, Color::srgb(0.99, 0.99, 1.0), 1.0);
    let cloud_mesh = meshes.add(Sphere::new(1.3));
    for (x, z) in [(-12.0, -13.0), (11.0, -14.0), (13.0, 12.0)] {
        commands
            .spawn((
                GameRoot,
                SceneRoot,
                Visibility::default(),
                Transform::from_xyz(x, 15.5, z),
            ))
            .with_children(|p| {
                p.spawn((
                    Mesh3d(cloud_mesh.clone()),
                    MeshMaterial3d(cloud.0.clone()),
                    Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(1.8, 0.6, 1.0)),
                ));
                p.spawn((
                    Mesh3d(cloud_mesh.clone()),
                    MeshMaterial3d(cloud.0.clone()),
                    Transform::from_xyz(1.4, 0.2, 0.3).with_scale(Vec3::new(1.2, 0.5, 0.8)),
                ));
            });
    }
}
