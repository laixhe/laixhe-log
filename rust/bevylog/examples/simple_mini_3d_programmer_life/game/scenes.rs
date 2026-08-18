//! 城市地图的搭建：一张 120×120 的连续大城市，十字主路贯穿全城，
//! 环路围出市中心 CBD，支路连接五个区域（家/校园/食堂/办公室/公园），
//! 校园周边还散落着探索点（夜市/观景台/涂鸦墙）。沿街有一排排小楼街区，
//! 玩家可自由行走，或在交通站点乘地铁/公交/共享单车。
//! 场景只在进入游戏时构建一次，hud::scene_manager 不再按地点重建。

use bevy::prelude::*;
use bevy::text::FontSource;
use bevy::ui::FocusPolicy;
use bevy::window::PrimaryWindow;

use super::art;
use super::components::*;
use super::npc::NPCS;
use super::resources::*;

// ==================== 区域布局数据 ====================
// 各区域热点（相对区域中心）；区域中心见 resources.rs 的 *_CENTER。
const HOME_HOTSPOTS: &[(HotspotKind, f32, f32)] = &[
    (HotspotKind::Bed, -6.0, 4.5),
    (HotspotKind::Desk, 0.0, 5.5),
    (HotspotKind::Books, 4.0, 6.0),
    (HotspotKind::Kitchen, -7.0, -4.5),
    (HotspotKind::Computer, 6.0, -3.5),
    (HotspotKind::Phone, 8.5, 0.5),
    (HotspotKind::Tv, -2.6, 4.0),
    (HotspotKind::Bathroom, 9.2, 5.0),
    (HotspotKind::Fridge, -9.0, -3.0),
];

const CAMPUS_HOTSPOTS: &[(HotspotKind, f32, f32)] = &[
    (HotspotKind::Track, -6.0, -8.0),
    (HotspotKind::TechGroup, 5.0, -8.0),
    (HotspotKind::Library, -11.0, 3.4),
    (HotspotKind::Lab, 9.5, 3.4),
    (HotspotKind::CampusShop, 10.0, -8.0),
    (HotspotKind::DormBed, 8.0, -5.5),
    (HotspotKind::DormGame, 8.0, -3.5),
    (HotspotKind::DormSnack, 8.0, -7.5),
];

const CAFETERIA_HOTSPOTS: &[(HotspotKind, f32, f32)] = &[
    (HotspotKind::Canteen1, -8.0, 0.0),
    (HotspotKind::Canteen2, -3.0, 0.0),
    (HotspotKind::Microwave, 3.0, 0.0),
    (HotspotKind::InstantNoodle, 8.0, 3.0),
    (HotspotKind::DrinkMachine, -8.0, 4.5),
    (HotspotKind::MilkTea, -10.5, -5.5),
    (HotspotKind::FruitStand, 10.5, -6.0),
];

const OFFICE_HOTSPOTS: &[(HotspotKind, f32, f32)] = &[
    (HotspotKind::Workstation, 0.0, 0.0),
    (HotspotKind::Lounge, -8.0, 4.0),
    (HotspotKind::Slacking, 6.0, 6.0),
    (HotspotKind::Takeout, 8.0, -6.0),
    (HotspotKind::Coffee, -11.0, 4.5),
    (HotspotKind::Meeting, 9.5, 2.2),
    (HotspotKind::Printer, 11.5, -2.0),
];

const PARK_HOTSPOTS: &[(HotspotKind, f32, f32)] = &[
    (HotspotKind::ParkBench, -5.0, 0.0),
    (HotspotKind::ParkBench, 5.0, 3.0),
    (HotspotKind::ParkFountain, 0.0, -5.0),
];

// 全城交通站点（世界坐标）：地铁站 = 各区域门口（与 resources::station_pos 一致），
// 公交站 = 环路四角，共享单车 = 市中心 + 各区域门口。
const CITY_STOPS: &[(HotspotKind, f32, f32)] = &[
    (HotspotKind::SubwayStop, -42.0, 15.0), // 家站
    (HotspotKind::SubwayStop, 15.0, -42.0), // 校园站
    (HotspotKind::SubwayStop, 42.0, -15.0), // 食堂站
    (HotspotKind::SubwayStop, -15.0, 42.0), // 公司站
    (HotspotKind::BusStop, 24.0, 24.0),
    (HotspotKind::BusStop, 24.0, -24.0),
    (HotspotKind::BusStop, -24.0, 24.0),
    (HotspotKind::BusStop, -24.0, -24.0),
    (HotspotKind::BikeSpot, -5.0, 0.0),    // 市中心单车点
    (HotspotKind::BikeSpot, -42.0, 11.0),  // 家门口
    (HotspotKind::BikeSpot, 11.0, -42.0),  // 校园门口
    (HotspotKind::BikeSpot, 42.0, -11.0),  // 食堂门口
    (HotspotKind::BikeSpot, -11.0, 42.0),  // 公司门口
    (HotspotKind::SubwayStop, 42.0, 15.0), // 公园站
    (HotspotKind::BikeSpot, 42.0, 20.0),   // 公园门口
];

// 校园周边探索点（世界坐标）：校园西门夜市 / 校园东侧观景台 / 校园北墙外涂鸦墙
const CAMPUS_SURROUNDS: &[(HotspotKind, f32, f32)] = &[
    (HotspotKind::NightMarket, 8.0, -47.0),
    (HotspotKind::Lookout, 50.0, -42.0),
    (HotspotKind::Graffiti, 28.0, -30.0),
];

// 各区域出生点（世界坐标）
pub fn spawn_pos(loc: Location) -> Vec3 {
    match loc {
        // 家出生点取床边（(-42,33)），离北墙（z=35.4）约 2.4，
        // 避免落在碰撞膨胀格内导致 A* 起点被墙围死
        Location::Home => HOME_CENTER + Vec3::new(0.0, 0.0, 5.0),
        Location::Campus => CAMPUS_CENTER + Vec3::new(0.0, 0.0, -9.0),
        Location::Cafeteria => CAFETERIA_CENTER + Vec3::new(0.0, 0.0, 8.0),
        Location::Office => OFFICE_CENTER + Vec3::new(0.0, 0.0, 8.0),
        // 公园出生点取南门口（靠近地铁站 (42,15)）
        Location::Park => PARK_CENTER + Vec3::new(0.0, 0.0, -8.0),
        Location::Road => Vec3::ZERO,
    }
}

// 区域名字（HUD 提示用）
pub fn location_name(loc: Location) -> &'static str {
    match loc {
        Location::Home => "家",
        Location::Campus => "校园",
        Location::Cafeteria => "食堂",
        Location::Office => "办公室",
        Location::Park => "公园",
        Location::Road => "市中心",
    }
}

// ==================== 城市构建（一次性） ====================
pub fn build_city(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    assets: &Res<AssetServer>,
    paper: &PaperTexture,
) {
    // 城市大地（120×120 浅灰绿）
    commands.spawn((
        GameRoot,
        SceneRoot,
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(WORLD_HALF * 2.0)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.55, 0.62, 0.48),
            base_color_texture: Some(paper.0.clone()),
            perceptual_roughness: 0.95,
            ..default()
        })),
    ));

    // ===== 道路网：主路 + 环路 + 支路 =====
    let road_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.82, 0.78, 0.68),
        perceptual_roughness: 0.9,
        ..default()
    });
    let road_dark = materials.add(StandardMaterial {
        base_color: Color::srgb(0.72, 0.67, 0.58),
        perceptual_roughness: 0.9,
        ..default()
    });
    // 铺一段路：中心 (cx, cz)、尺寸 (sx, sz)
    let mut road = |cx: f32, cz: f32, sx: f32, sz: f32, mat: &Handle<StandardMaterial>| {
        commands.spawn((
            GameRoot,
            SceneRoot,
            Mesh3d(meshes.add(Cuboid::new(sx, 0.03, sz))),
            MeshMaterial3d(mat.clone()),
            Transform::from_xyz(cx, 0.015, cz),
        ));
    };

    // 十字主路（宽 10，贯穿全城）
    road(0.0, 0.0, WORLD_HALF * 2.0, 10.0, &road_mat);
    road(0.0, 0.0, 10.0, WORLD_HALF * 2.0, &road_mat);

    // 环路（围住市中心 CBD 的矩形次干道，宽 7，边为线段）
    for sz in [24.0, -24.0] {
        road(0.0, sz, 48.0, 7.0, &road_dark);
    }
    for sx in [24.0, -24.0] {
        road(sx, 0.0, 7.0, 48.0, &road_dark);
    }

    // 支路（区域 ↔ 主路的连接路，宽 5）：家/校园/食堂/公司/公园
    road(-42.0, 12.0, 5.0, 14.0, &road_dark); // 家：南接主路，北达家门口（不再铺进室内）
    road(10.0, -42.0, 10.0, 5.0, &road_dark); // 校园：北接主路
    road(42.0, -10.0, 5.0, 10.0, &road_dark); // 食堂：北接主路
    road(-10.0, 42.0, 10.0, 5.0, &road_dark); // 公司：南接主路
    road(42.0, 8.0, 5.0, 14.0, &road_dark); // 公园：南接主路，北达公园南门

    // 中心广场（十字路口中央小环岛 4×4）：保持空旷且不遮挡四向斑马线（±3.2），
    // 盆栽移到主路旁人行道，避免挡路
    commands.spawn((
        GameRoot,
        SceneRoot,
        Mesh3d(meshes.add(Cuboid::new(4.0, 0.05, 4.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.66, 0.60, 0.50),
            perceptual_roughness: 0.9,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.04, 0.0),
    ));
    art::spawn_plant(commands, meshes, materials, Vec3::new(6.5, 0.0, 6.5));

    // 沿街街区建筑（主路两侧 + 环路四角，营造城市街区感）
    spawn_street_buildings(commands, meshes, materials);

    // 方向光（开阴影，增强立体感）
    commands.spawn((
        GameRoot,
        SceneRoot,
        DayLight,
        DirectionalLight {
            illuminance: 9000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(6.0, 20.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 远景：太阳 + 云朵
    art::spawn_sky(commands, meshes, materials);

    // 五个区域（内容以区域中心为原点）
    build_home(commands, meshes, materials, assets);
    build_campus(commands, meshes, materials, assets);
    build_cafeteria(commands, meshes, materials, assets);
    build_office(commands, meshes, materials, assets);
    // 第五区域：公园（城东北公共休闲区）
    build_park(commands, meshes, materials, assets);

    // 沿主路与环路的路灯
    for x in [-36.0, -28.0, 28.0, 36.0] {
        art::spawn_lamp(commands, meshes, materials, Vec3::new(x, 0.0, 6.5));
    }
    for z in [-36.0, -28.0, 28.0, 36.0] {
        art::spawn_lamp(commands, meshes, materials, Vec3::new(6.5, 0.0, z));
    }
    for corner in [(-24.0, 20.0), (24.0, -20.0), (20.0, 24.0), (-20.0, -24.0)] {
        art::spawn_lamp(
            commands,
            meshes,
            materials,
            Vec3::new(corner.0, 0.0, corner.1),
        );
    }

    // 全城交通站点
    for (kind, x, z) in CITY_STOPS {
        spawn_hotspot(
            commands,
            *kind,
            Vec3::new(*x, 0.0, *z),
            meshes,
            materials,
            assets,
        );
    }

    // 校园周边探索点（夜市 / 观景台 / 涂鸦墙）
    spawn_campus_surrounds(commands, meshes, materials, assets);

    // ===== 城市交通设施：红绿灯 / 车辆 / 斑马线 / 过马路行人 =====
    super::traffic::spawn_lights(commands, meshes, materials);
    super::traffic::spawn_vehicles(commands, meshes, materials);
    super::traffic::spawn_crosswalk(commands, meshes, materials);
    super::traffic::spawn_crossing_ped(
        commands,
        meshes,
        materials,
        0,
        Vec2::new(-7.0, 3.2),
        Vec2::new(7.0, 3.2),
        0.0,
        0.7,
        Color::srgb(0.72, 0.55, 0.42),
    );
    // 第二个行人在斑马线中间出发，错开横穿相位，更容易与车辆相遇
    super::traffic::spawn_crossing_ped(
        commands,
        meshes,
        materials,
        0,
        Vec2::new(7.0, -3.2),
        Vec2::new(-7.0, -3.2),
        7.0,
        0.8,
        Color::srgb(0.45, 0.62, 0.78),
    );
    // 东西向斑马线行人（横穿 z=0 的东西向主路，看东西向信号灯）
    super::traffic::spawn_crossing_ped(
        commands,
        meshes,
        materials,
        0,
        Vec2::new(3.2, 7.0),
        Vec2::new(3.2, -7.0),
        3.0,
        0.9,
        Color::srgb(0.80, 0.45, 0.55),
    );
    super::traffic::spawn_crossing_ped(
        commands,
        meshes,
        materials,
        0,
        Vec2::new(-3.2, -7.0),
        Vec2::new(-3.2, 7.0),
        10.0,
        0.8,
        Color::srgb(0.60, 0.75, 0.45),
    );

    // 街道行人：主干道两侧人行道散步（WanderNpc 会 A* 绕行，不穿建筑）。
    // 路线尽量限定在环路（z=±24）以内；校园支路西侧那条例外——
    // 它沿校园支路（z=-28 → -44）散步，属于校区内部人行道，不在环路上。
    for (x, z0, z1, col) in [
        (-6.5, 18.0, 0.0, Color::srgb(0.70, 0.50, 0.45)),
        (-6.5, 0.0, -18.0, Color::srgb(0.50, 0.65, 0.55)),
        (6.5, 18.0, 0.0, Color::srgb(0.60, 0.55, 0.72)),
        (6.5, 0.0, -18.0, Color::srgb(0.85, 0.70, 0.50)),
        (4.0, -28.0, -44.0, Color::srgb(0.55, 0.72, 0.60)), // 校园支路西侧
        (-38.0, 0.0, 12.0, Color::srgb(0.75, 0.58, 0.48)),  // 家门支路东侧
    ] {
        spawn_decor_npc(
            commands,
            meshes,
            materials,
            Vec3::new(x, 0.0, z0),
            col,
            Some((Vec3::new(x, 0.0, z0), Vec3::new(x, 0.0, z1), 0.8)),
        );
    }
}

// ==================== 校园周边探索点 ====================
// 三个轻量探索点散落在校园周边（世界坐标，不归属任何区域）：
// 西门夜市（晚上才出摊）/ 东侧观景台（免费登高）/ 北墙外涂鸦墙（随机彩蛋）。
fn spawn_campus_surrounds(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    assets: &Res<AssetServer>,
) {
    // 世界坐标装饰方块（非实心，逻辑统一在 art::spawn_block）
    let mut furn = |x: f32, z: f32, y: f32, sx: f32, sy: f32, sz: f32, c: Color| {
        art::spawn_block(
            commands,
            meshes,
            materials,
            Vec3::new(x, y, z),
            Vec3::new(sx, sy, sz),
            c,
        );
    };

    // 西门夜市：摊车（台面 + 四腿 + 立柱 + 遮阳棚 + 台上食物 + 暖色灯箱）
    furn(8.0, -47.0, 0.55, 1.6, 0.1, 0.7, art::WOOD);
    for (dx, dz) in [(-0.7, -0.28), (0.7, -0.28), (-0.7, 0.28), (0.7, 0.28)] {
        furn(8.0 + dx, -47.0 + dz, 0.27, 0.08, 0.55, 0.08, art::WOOD_DARK);
    }
    furn(7.4, -47.0, 1.05, 0.08, 1.5, 0.08, art::WOOD_DARK); // 左立柱
    furn(8.6, -47.0, 1.05, 0.08, 1.5, 0.08, art::WOOD_DARK); // 右立柱
    furn(
        8.0,
        -47.0,
        1.85,
        1.9,
        0.08,
        1.1,
        Color::srgb(0.85, 0.55, 0.35),
    ); // 遮阳棚
    furn(
        7.6,
        -47.0,
        0.68,
        0.3,
        0.16,
        0.22,
        Color::srgb(0.85, 0.55, 0.30),
    ); // 烤串堆
    furn(
        8.4,
        -47.0,
        0.68,
        0.3,
        0.16,
        0.22,
        Color::srgb(0.70, 0.80, 0.45),
    );
    furn(
        8.0,
        -47.0,
        1.35,
        0.26,
        0.3,
        0.26,
        Color::srgb(0.95, 0.85, 0.55),
    ); // 暖色灯箱

    // 东侧观景台：平台 + 南侧台阶 + 四角围栏立柱 + 南侧横杆
    furn(
        50.0,
        -42.0,
        0.3,
        4.0,
        0.55,
        3.0,
        Color::srgb(0.68, 0.62, 0.54),
    );
    furn(
        50.0,
        -44.2,
        0.15,
        1.4,
        0.3,
        1.2,
        Color::srgb(0.72, 0.66, 0.58),
    );
    for (dx, dz) in [(-1.8, -1.3), (1.8, -1.3), (-1.8, 1.3), (1.8, 1.3)] {
        furn(50.0 + dx, -42.0 + dz, 0.75, 0.12, 0.9, 0.12, art::WOOD_DARK);
    }
    furn(50.0, -43.3, 0.75, 3.6, 0.08, 0.08, art::WOOD_DARK); // 南侧横杆

    // 北墙外涂鸦墙：矮墙 + 彩色涂鸦块
    furn(
        28.0,
        -30.0,
        0.7,
        4.0,
        1.4,
        0.25,
        Color::srgb(0.80, 0.78, 0.74),
    );
    furn(
        27.6,
        -30.0,
        0.75,
        0.4,
        0.4,
        0.06,
        Color::srgb(0.85, 0.45, 0.40),
    );
    furn(
        28.3,
        -30.0,
        0.55,
        0.4,
        0.4,
        0.06,
        Color::srgb(0.40, 0.60, 0.85),
    );
    furn(
        28.9,
        -30.0,
        0.9,
        0.4,
        0.4,
        0.06,
        Color::srgb(0.90, 0.80, 0.35),
    );

    // 探索点热点（垫 + 图标 + 标签）
    for (kind, x, z) in CAMPUS_SURROUNDS {
        spawn_hotspot(
            commands,
            *kind,
            Vec3::new(*x, 0.0, *z),
            meshes,
            materials,
            assets,
        );
    }
}

// 沿街小楼：一排排低模建筑让街道有"街区"感
fn spawn_street_buildings(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    // 一栋小楼：底层 + 楼体 + 平屋顶
    let mut block = |x: f32, z: f32, color: Color| {
        let base = materials.add(StandardMaterial {
            base_color: color,
            perceptual_roughness: 0.9,
            ..default()
        });
        let roof = materials.add(StandardMaterial {
            base_color: Color::srgb(0.55, 0.50, 0.45),
            perceptual_roughness: 0.9,
            ..default()
        });
        commands
            .spawn((
                GameRoot,
                SceneRoot,
                Visibility::default(),
                Solid {
                    half: Vec2::new(2.0, 2.0),
                    bottom: 0.0,
                },
                Transform::from_xyz(x, 0.0, z),
            ))
            .with_children(|b| {
                b.spawn((
                    Mesh3d(meshes.add(Cuboid::new(4.0, 3.0, 4.0))),
                    MeshMaterial3d(base.clone()),
                    Transform::from_xyz(0.0, 1.5, 0.0),
                ));
                b.spawn((
                    Mesh3d(meshes.add(Cuboid::new(4.4, 0.2, 4.4))),
                    MeshMaterial3d(roof.clone()),
                    Transform::from_xyz(0.0, 3.2, 0.0),
                ));
            });
    };
    let colors = [
        Color::srgb(0.75, 0.55, 0.45),
        Color::srgb(0.62, 0.68, 0.75),
        Color::srgb(0.70, 0.62, 0.50),
        Color::srgb(0.68, 0.60, 0.68),
        Color::srgb(0.80, 0.68, 0.55),
    ];
    // 垂直主路（x=0）两侧 x=±9 各四排；水平主路（z=0）两侧 z=±9 各四排。
    // 环内两排（11/18）、环外两排（31/34），避开环路(±24)与四条支路。
    let rows = [11.0, 18.0, 31.0, 34.0];
    for (i, r) in rows.iter().enumerate() {
        // 垂直主路两侧
        block(9.0, *r, colors[i % colors.len()]);
        block(-9.0, *r, colors[(i + 1) % colors.len()]);
        block(9.0, -*r, colors[(i + 2) % colors.len()]);
        block(-9.0, -*r, colors[(i + 3) % colors.len()]);
        // 水平主路两侧（x 偏移错开，避免与垂直排重叠）
        let x = [15.0, 18.0, 30.0, 36.0][i];
        block(x, 9.0, colors[(i + 4) % colors.len()]);
        block(x, -9.0, colors[(i + 5) % colors.len()]);
        block(-x, 9.0, colors[(i + 6) % colors.len()]);
        block(-x, -9.0, colors[(i + 7) % colors.len()]);
    }
    // 环路四角内侧各一栋
    block(18.0, 18.0, colors[1]);
    block(18.0, -18.0, colors[2]);
    block(-18.0, 18.0, colors[3]);
    block(-18.0, -18.0, colors[4]);
}

// ==================== 家（城西） ====================
fn build_home(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    assets: &Res<AssetServer>,
) {
    let o = HOME_CENTER;
    // 室内地板
    commands.spawn((
        GameRoot,
        SceneRoot,
        Mesh3d(meshes.add(Cuboid::new(20.0, 0.08, 14.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.89, 0.77, 0.60),
            perceptual_roughness: 0.9,
            ..default()
        })),
        Transform::from_xyz(o.x, 0.02, o.z),
    ));

    // 围墙（U 形，南面留 5 宽门洞，正对家门支路）
    let wall_y = 0.55;
    art::spawn_wall(
        commands,
        meshes,
        materials,
        Vec3::new(20.0, wall_y, 0.25),
        o + Vec3::new(0.0, wall_y / 2.0, 7.4),
        art::WALL,
    ); // 北墙
    art::spawn_wall(
        commands,
        meshes,
        materials,
        Vec3::new(0.25, wall_y, 15.0),
        o + Vec3::new(-10.0, wall_y / 2.0, 0.0),
        art::WALL,
    ); // 西墙
    art::spawn_wall(
        commands,
        meshes,
        materials,
        Vec3::new(0.25, wall_y, 15.0),
        o + Vec3::new(10.0, wall_y / 2.0, 0.0),
        art::WALL,
    ); // 东墙
    art::spawn_wall(
        commands,
        meshes,
        materials,
        Vec3::new(7.5, wall_y, 0.25),
        o + Vec3::new(-6.25, wall_y / 2.0, -7.4),
        art::WALL,
    ); // 南墙左段
    art::spawn_wall(
        commands,
        meshes,
        materials,
        Vec3::new(7.5, wall_y, 0.25),
        o + Vec3::new(6.25, wall_y / 2.0, -7.4),
        art::WALL,
    ); // 南墙右段

    // 屋顶框架（四角立柱 + 顶部边框，不做实顶，保持俯视通透）
    for (x, z) in [(-9.6, 7.0), (9.6, 7.0), (-9.6, -7.0), (9.6, -7.0)] {
        art::spawn_wall(
            commands,
            meshes,
            materials,
            Vec3::new(0.16, 1.2, 0.16),
            o + Vec3::new(x, 0.6, z),
            art::WOOD_DARK,
        );
    }
    art::spawn_wall(
        commands,
        meshes,
        materials,
        Vec3::new(19.6, 0.12, 0.16),
        o + Vec3::new(0.0, 1.24, 7.0),
        art::WOOD_DARK,
    );
    art::spawn_wall(
        commands,
        meshes,
        materials,
        Vec3::new(19.6, 0.12, 0.16),
        o + Vec3::new(0.0, 1.24, -7.0),
        art::WOOD_DARK,
    );
    art::spawn_wall(
        commands,
        meshes,
        materials,
        Vec3::new(0.16, 0.12, 14.3),
        o + Vec3::new(-9.6, 1.24, 0.0),
        art::WOOD_DARK,
    );
    art::spawn_wall(
        commands,
        meshes,
        materials,
        Vec3::new(0.16, 0.12, 14.3),
        o + Vec3::new(9.6, 1.24, 0.0),
        art::WOOD_DARK,
    );

    // 家具
    art::spawn_rug(
        commands,
        meshes,
        materials,
        o + Vec3::new(-6.0, 0.0, 3.3),
        Vec2::new(2.6, 1.8),
        Color::srgb(0.72, 0.52, 0.40),
    );
    art::spawn_sofa(
        commands,
        meshes,
        materials,
        o + Vec3::new(-2.6, 0.0, 2.6),
        0.0,
    );
    art::spawn_table(
        commands,
        meshes,
        materials,
        o + Vec3::new(2.2, 0.0, 3.0),
        Vec2::new(1.1, 0.7),
        0.55,
    );
    art::spawn_bookshelf(
        commands,
        meshes,
        materials,
        o + Vec3::new(4.0, 0.0, 4.2),
        0.0,
    );
    art::spawn_plant(commands, meshes, materials, o + Vec3::new(-9.2, 0.0, 6.2));
    art::spawn_plant(commands, meshes, materials, o + Vec3::new(9.2, 0.0, 6.2));
    art::spawn_hanging_lamp(commands, meshes, materials, o + Vec3::new(0.0, 0.0, 1.0));
    art::spawn_hanging_lamp(commands, meshes, materials, o + Vec3::new(-3.0, 0.0, -4.5));

    // 厨房台面
    art::spawn_wall(
        commands,
        meshes,
        materials,
        Vec3::new(2.2, 0.1, 0.7),
        o + Vec3::new(-7.0, 0.75, -3.0),
        art::BRICK,
    );
    art::spawn_wall(
        commands,
        meshes,
        materials,
        Vec3::new(0.4, 0.55, 0.55),
        o + Vec3::new(-8.0, 0.55, -3.0),
        Color::srgb(0.45, 0.55, 0.65),
    );

    // 互动家具模型（与热点垫同位、不带碰撞，纯视觉；实心家具是上面那些沙发/桌子/书架）
    // 相对区域中心的装饰方块（非实心，逻辑统一在 art::spawn_block）
    let mut furn = |x: f32, z: f32, y: f32, sx: f32, sy: f32, sz: f32, c: Color| {
        art::spawn_block(
            commands,
            meshes,
            materials,
            o + Vec3::new(x, y, z),
            Vec3::new(sx, sy, sz),
            c,
        );
    };
    // 床（床垫 + 床头板 + 枕头，床头靠北墙）
    furn(
        -6.0,
        4.5,
        0.18,
        1.7,
        0.35,
        1.0,
        Color::srgb(0.58, 0.66, 0.76),
    );
    furn(-6.0, 5.05, 0.35, 1.8, 0.7, 0.15, art::WOOD_DARK);
    furn(
        -6.3,
        4.9,
        0.34,
        0.5,
        0.14,
        0.3,
        Color::srgb(0.97, 0.95, 0.90),
    );
    // 书桌（桌面 + 四腿）
    furn(0.0, 5.5, 0.55, 1.4, 0.08, 0.7, art::WOOD);
    for (dx, dz) in [(-0.62, -0.28), (0.62, -0.28), (-0.62, 0.28), (0.62, 0.28)] {
        furn(dx, 5.5 + dz, 0.27, 0.08, 0.55, 0.08, art::WOOD_DARK);
    }
    // 电脑桌（桌面 + 四腿 + 显示器）
    furn(6.0, -3.5, 0.55, 1.4, 0.08, 0.7, art::WOOD);
    for (dx, dz) in [(-0.62, -0.28), (0.62, -0.28), (-0.62, 0.28), (0.62, 0.28)] {
        furn(dx, -3.5 + dz, 0.27, 0.08, 0.55, 0.08, art::WOOD_DARK);
    }
    furn(
        6.0,
        -3.3,
        0.85,
        0.95,
        0.5,
        0.06,
        Color::srgb(0.15, 0.18, 0.25),
    ); // 屏幕
    furn(6.0, -3.3, 0.6, 0.3, 0.12, 0.08, Color::srgb(0.3, 0.3, 0.34)); // 底座
    // 电视柜 + 屏幕（沙发对面，正对客厅）
    furn(-2.6, 4.0, 0.25, 1.5, 0.5, 0.4, art::WOOD_DARK);
    furn(
        -2.6,
        4.0,
        0.75,
        1.1,
        0.65,
        0.06,
        Color::srgb(0.15, 0.18, 0.25),
    );
    // 浴缸（外框 + 水面，东北角）
    furn(
        9.2,
        5.0,
        0.28,
        1.6,
        0.55,
        0.9,
        Color::srgb(0.93, 0.95, 0.96),
    );
    furn(
        9.2,
        5.0,
        0.32,
        1.35,
        0.4,
        0.65,
        Color::srgb(0.45, 0.70, 0.85),
    );
    // 冰箱（箱体 + 拉手，厨房台面旁贴西墙）
    furn(
        -9.0,
        -3.0,
        0.75,
        0.8,
        1.5,
        0.7,
        Color::srgb(0.90, 0.92, 0.94),
    );
    furn(
        -9.0,
        -3.35,
        0.95,
        0.06,
        0.35,
        0.06,
        Color::srgb(0.65, 0.68, 0.72),
    );

    for (kind, x, z) in HOME_HOTSPOTS {
        spawn_hotspot(
            commands,
            *kind,
            o + Vec3::new(*x, 0.0, *z),
            meshes,
            materials,
            assets,
        );
    }

    // 邻居赖哥（家门口外楼道）：玩家独居，赖哥住隔壁，出门能碰到；
    spawn_npc_entity(
        commands,
        meshes,
        materials,
        assets,
        o + Vec3::new(-1.0, 0.0, -8.5),
        1,
        None,
    );
}

// ==================== 校园（城南） ====================
fn build_campus(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    assets: &Res<AssetServer>,
) {
    let o = CAMPUS_CENTER;
    // 草地
    commands.spawn((
        GameRoot,
        SceneRoot,
        Mesh3d(meshes.add(Cuboid::new(26.0, 0.06, 26.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.68, 0.79, 0.58),
            perceptual_roughness: 0.95,
            ..default()
        })),
        Transform::from_xyz(o.x, 0.01, o.z),
    ));

    // 主路（纵向条带）
    commands.spawn((
        GameRoot,
        SceneRoot,
        Mesh3d(meshes.add(Cuboid::new(1.4, 0.03, 20.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.86, 0.78, 0.62),
            perceptual_roughness: 0.9,
            ..default()
        })),
        Transform::from_xyz(o.x, 0.015, o.z),
    ));

    // 教学楼（远景西侧）
    art::spawn_building(
        commands,
        meshes,
        materials,
        o + Vec3::new(-11.0, 0.0, -2.0),
        Vec3::new(6.5, 4.2, 5.5),
        Color::srgb(0.90, 0.84, 0.70),
        art::ROOF_RED,
    );
    // 宿舍楼（东北）
    art::spawn_building(
        commands,
        meshes,
        materials,
        o + Vec3::new(11.5, 0.0, -4.5),
        Vec3::new(4.8, 3.6, 4.8),
        Color::srgb(0.85, 0.75, 0.62),
        art::ROOF_BLUE,
    );

    // 互动新建筑：图书馆（西北）/ 实验室（东北）/ 小卖部（东南角小卖亭）
    art::spawn_building(
        commands,
        meshes,
        materials,
        o + Vec3::new(-11.0, 0.0, 6.5),
        Vec3::new(4.6, 3.6, 3.6),
        Color::srgb(0.86, 0.80, 0.92),
        art::ROOF_BLUE,
    );
    art::spawn_building(
        commands,
        meshes,
        materials,
        o + Vec3::new(9.5, 0.0, 6.5),
        Vec3::new(4.2, 3.4, 3.4),
        Color::srgb(0.82, 0.88, 0.90),
        art::ROOF_RED,
    );
    art::spawn_building(
        commands,
        meshes,
        materials,
        o + Vec3::new(10.0, 0.0, -10.5),
        Vec3::new(2.6, 2.0, 2.6),
        Color::srgb(0.93, 0.84, 0.68),
        art::ROOF_RED,
    );
    // 小卖部门口零食筐（装饰，不阻挡行走）
    for (dx, dz) in [(-0.95, -0.35), (0.95, 0.35)] {
        commands.spawn((
            GameRoot,
            SceneRoot,
            Mesh3d(meshes.add(Cuboid::new(0.5, 0.4, 0.5))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.76, 0.56, 0.36),
                perceptual_roughness: 0.9,
                ..default()
            })),
            Transform::from_xyz(o.x + 10.0 + dx, 0.2, o.z - 10.5 + dz),
        ));
    }

    // 操场：跑道（外浅内深两圈矩形）+ 内场
    let track_m = materials.add(StandardMaterial {
        base_color: Color::srgb(0.80, 0.62, 0.45),
        perceptual_roughness: 0.95,
        ..default()
    });
    commands.spawn((
        GameRoot,
        SceneRoot,
        Mesh3d(meshes.add(Cuboid::new(7.6, 0.05, 3.4))),
        MeshMaterial3d(track_m.clone()),
        Transform::from_xyz(o.x - 6.0, 0.03, o.z - 8.0),
    ));
    commands.spawn((
        GameRoot,
        SceneRoot,
        Mesh3d(meshes.add(Cuboid::new(6.6, 0.06, 2.4))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.60, 0.70, 0.50),
            perceptual_roughness: 0.95,
            ..default()
        })),
        Transform::from_xyz(o.x - 6.0, 0.04, o.z - 8.0),
    ));

    // 路灯 / 长椅 / 树 / 围栏 / 花坛
    art::spawn_lamp(commands, meshes, materials, o + Vec3::new(-3.0, 0.0, 0.5));
    art::spawn_lamp(commands, meshes, materials, o + Vec3::new(3.0, 0.0, 1.5));
    art::spawn_bench(
        commands,
        meshes,
        materials,
        o + Vec3::new(-8.0, 0.0, -3.5),
        0.6,
        true,
    );
    art::spawn_bench(
        commands,
        meshes,
        materials,
        o + Vec3::new(6.0, 0.0, -4.5),
        -0.6,
        true,
    );
    for (x, z, s) in [
        (-4.0, -3.0, 1.0),
        (3.0, -4.0, 1.2),
        (-5.0, 3.0, 0.9),
        (6.0, 4.0, 1.1),
        (-2.5, -6.0, 0.8),
        (2.0, 6.5, 0.9),
    ] {
        art::spawn_tree(commands, meshes, materials, o + Vec3::new(x, 0.0, z), s);
    }
    // 东侧围栏
    for z in [-10.0, -6.0, -2.0, 2.0, 6.0] {
        art::spawn_fence(
            commands,
            meshes,
            materials,
            o + Vec3::new(13.2, 0.0, z),
            3.6,
            art::WOOD,
        );
    }
    // 花坛（两段矮墙围的小方块）
    art::spawn_wall(
        commands,
        meshes,
        materials,
        Vec3::new(1.4, 0.3, 0.12),
        o + Vec3::new(-0.9, 0.15, 3.2),
        art::BRICK,
    );
    art::spawn_wall(
        commands,
        meshes,
        materials,
        Vec3::new(0.12, 0.3, 1.4),
        o + Vec3::new(-0.2, 0.15, 3.2),
        art::BRICK,
    );
    art::spawn_plant(commands, meshes, materials, o + Vec3::new(-0.55, 0.0, 3.2));

    // 宿舍生活区（宿舍楼前西侧空地：床 / 室友电脑 / 零食柜，均不带碰撞）
    // 相对区域中心的装饰方块（非实心，逻辑统一在 art::spawn_block）
    let mut furn = |x: f32, z: f32, y: f32, sx: f32, sy: f32, sz: f32, c: Color| {
        art::spawn_block(
            commands,
            meshes,
            materials,
            o + Vec3::new(x, y, z),
            Vec3::new(sx, sy, sz),
            c,
        );
    };
    // 宿舍床（床垫 + 床头板 + 枕头）
    furn(
        8.0,
        -5.5,
        0.18,
        1.6,
        0.35,
        0.9,
        Color::srgb(0.58, 0.66, 0.76),
    );
    furn(8.0, -5.5, 0.34, 1.7, 0.65, 0.14, art::WOOD_DARK);
    furn(
        8.0,
        -4.8,
        0.32,
        0.5,
        0.13,
        0.28,
        Color::srgb(0.97, 0.95, 0.90),
    ); // 枕头
    // 室友电脑（桌面 + 四腿 + 屏幕）
    furn(8.0, -3.5, 0.55, 1.3, 0.08, 0.65, art::WOOD);
    for (dx, dz) in [(-0.57, -0.26), (0.57, -0.26), (-0.57, 0.26), (0.57, 0.26)] {
        furn(8.0 + dx, -3.5 + dz, 0.27, 0.08, 0.55, 0.08, art::WOOD_DARK);
    }
    furn(
        8.0,
        -3.3,
        0.85,
        0.9,
        0.5,
        0.06,
        Color::srgb(0.15, 0.18, 0.25),
    ); // 屏幕
    furn(
        8.0,
        -3.3,
        0.6,
        0.28,
        0.12,
        0.08,
        Color::srgb(0.3, 0.3, 0.34),
    ); // 底座
    // 零食柜（柜体 + 柜顶零食盒）
    furn(8.0, -7.5, 0.6, 0.7, 1.2, 0.5, Color::srgb(0.90, 0.92, 0.94));
    furn(
        8.0,
        -7.5,
        1.28,
        0.28,
        0.14,
        0.18,
        Color::srgb(0.85, 0.55, 0.30),
    );
    furn(
        8.0,
        -7.5,
        1.26,
        0.22,
        0.12,
        0.16,
        Color::srgb(0.70, 0.80, 0.45),
    );

    for (kind, x, z) in CAMPUS_HOTSPOTS {
        spawn_hotspot(
            commands,
            *kind,
            o + Vec3::new(*x, 0.0, *z),
            meshes,
            materials,
            assets,
        );
    }

    // 导师陈教授（教学楼前）可对话；两位同学在主路散步
    spawn_npc_entity(
        commands,
        meshes,
        materials,
        assets,
        o + Vec3::new(-8.0, 0.0, 2.0),
        2,
        None,
    );
    spawn_decor_npc(
        commands,
        meshes,
        materials,
        o + Vec3::new(-2.0, 0.0, 5.0),
        Color::srgb(0.90, 0.62, 0.45),
        Some((
            o + Vec3::new(-3.0, 0.0, 6.0),
            o + Vec3::new(3.0, 0.0, 6.5),
            0.8,
        )),
    );
    spawn_decor_npc(
        commands,
        meshes,
        materials,
        o + Vec3::new(2.0, 0.0, -1.5),
        Color::srgb(0.60, 0.75, 0.55),
        Some((
            o + Vec3::new(-1.5, 0.0, -2.0),
            o + Vec3::new(4.5, 0.0, -1.0),
            0.7,
        )),
    );
}

// ==================== 食堂（城东） ====================
fn build_cafeteria(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    assets: &Res<AssetServer>,
) {
    let o = CAFETERIA_CENTER;
    // 地板
    commands.spawn((
        GameRoot,
        SceneRoot,
        Mesh3d(meshes.add(Cuboid::new(26.0, 0.06, 26.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.90, 0.82, 0.70),
            perceptual_roughness: 0.95,
            ..default()
        })),
        Transform::from_xyz(o.x, 0.01, o.z),
    ));

    // 北侧柜台（带取餐窗口亮条）
    art::spawn_wall(
        commands,
        meshes,
        materials,
        Vec3::new(9.0, 1.2, 0.8),
        o + Vec3::new(0.0, 0.6, 5.5),
        art::BRICK,
    );
    art::spawn_wall(
        commands,
        meshes,
        materials,
        Vec3::new(7.0, 0.5, 0.06),
        o + Vec3::new(0.0, 1.1, 5.95),
        glass_color(),
    );
    art::spawn_wall(
        commands,
        meshes,
        materials,
        Vec3::new(0.5, 1.2, 0.8),
        o + Vec3::new(-9.5, 0.6, 5.5),
        art::WALL,
    );
    art::spawn_wall(
        commands,
        meshes,
        materials,
        Vec3::new(0.5, 1.2, 0.8),
        o + Vec3::new(9.5, 0.6, 5.5),
        art::WALL,
    );

    // 餐桌 + 餐椅
    for x in [-5.5, 0.0, 5.5] {
        art::spawn_table(
            commands,
            meshes,
            materials,
            o + Vec3::new(x, 0.0, -4.0),
            Vec2::new(2.2, 1.2),
            0.7,
        );
        art::spawn_chair(
            commands,
            meshes,
            materials,
            o + Vec3::new(x - 1.2, 0.0, -4.0),
            1.57,
        );
        art::spawn_chair(
            commands,
            meshes,
            materials,
            o + Vec3::new(x + 1.2, 0.0, -4.0),
            -1.57,
        );
        art::spawn_chair(
            commands,
            meshes,
            materials,
            o + Vec3::new(x, 0.0, -3.0),
            std::f32::consts::PI,
        );
        art::spawn_chair(
            commands,
            meshes,
            materials,
            o + Vec3::new(x, 0.0, -5.0),
            0.0,
        );
    }

    // 吊灯 + 绿植
    art::spawn_hanging_lamp(commands, meshes, materials, o + Vec3::new(-5.5, 0.0, -4.0));
    art::spawn_hanging_lamp(commands, meshes, materials, o + Vec3::new(5.5, 0.0, -4.0));
    art::spawn_plant(commands, meshes, materials, o + Vec3::new(-11.5, 0.0, -2.0));
    art::spawn_plant(commands, meshes, materials, o + Vec3::new(11.5, 0.0, -2.0));

    // 柜台台面上的菜品（托盘 + 菜色块）
    let plate = materials.add(StandardMaterial {
        base_color: Color::srgb(0.94, 0.92, 0.86),
        perceptual_roughness: 0.9,
        ..default()
    });
    let dish_colors = [
        Color::srgb(0.85, 0.55, 0.30), // 红烧肉
        Color::srgb(0.45, 0.68, 0.35), // 青菜
        Color::srgb(0.90, 0.80, 0.40), // 番茄炒蛋
    ];
    for (i, col) in dish_colors.iter().enumerate() {
        let x = -3.5 + i as f32 * 2.2;
        commands.spawn((
            GameRoot,
            SceneRoot,
            Mesh3d(meshes.add(Cuboid::new(0.85, 0.05, 0.55))),
            MeshMaterial3d(plate.clone()),
            Transform::from_xyz(o.x + x, 1.22, o.z + 5.1),
        ));
        commands.spawn((
            GameRoot,
            SceneRoot,
            Mesh3d(meshes.add(Cuboid::new(0.6, 0.14, 0.36))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: *col,
                perceptual_roughness: 0.9,
                ..default()
            })),
            Transform::from_xyz(o.x + x, 1.32, o.z + 5.1),
        ));
    }

    // 取餐排队 NPC（两个方块人，站柜台前）
    spawn_queue_npc(
        commands,
        meshes,
        materials,
        o + Vec3::new(-5.5, 0.0, 3.8),
        Color::srgb(0.72, 0.45, 0.40),
        1.0,
    );
    spawn_queue_npc(
        commands,
        meshes,
        materials,
        o + Vec3::new(-3.6, 0.0, 3.8),
        Color::srgb(0.40, 0.62, 0.48),
        2.4,
    );

    // 南侧装饰墙（留门）
    art::spawn_wall(
        commands,
        meshes,
        materials,
        Vec3::new(24.0, 0.5, 0.3),
        o + Vec3::new(0.0, 0.25, -9.5),
        Color::srgb(0.88, 0.80, 0.66),
    );

    // 互动家具模型（与热点垫同位、不带碰撞，纯视觉）
    // 相对区域中心的装饰方块（非实心，逻辑统一在 art::spawn_block）
    let mut furn = |x: f32, z: f32, y: f32, sx: f32, sy: f32, sz: f32, c: Color| {
        art::spawn_block(
            commands,
            meshes,
            materials,
            o + Vec3::new(x, y, z),
            Vec3::new(sx, sy, sz),
            c,
        );
    };
    // 饮料机（柜台西侧通道：机身 + 屏幕 + 出水口）
    furn(
        -8.0,
        4.5,
        0.85,
        0.7,
        1.6,
        0.5,
        Color::srgb(0.28, 0.36, 0.48),
    );
    furn(
        -8.0,
        4.5,
        1.2,
        0.42,
        0.42,
        0.06,
        Color::srgb(0.55, 0.78, 0.90),
    ); // 屏幕
    furn(
        -8.0,
        4.5,
        0.45,
        0.2,
        0.2,
        0.2,
        Color::srgb(0.75, 0.78, 0.82),
    ); // 出水口
    // 奶茶吧台（西南角：台面 + 腿 + 奶茶杯 + 吸管）
    furn(-10.5, -5.5, 0.7, 1.2, 0.08, 0.6, art::WOOD);
    for (dx, dz) in [(-0.5, -0.24), (0.5, -0.24), (-0.5, 0.24), (0.5, 0.24)] {
        furn(-10.5 + dx, -5.5 + dz, 0.35, 0.08, 0.7, 0.08, art::WOOD_DARK);
    }
    furn(
        -10.5,
        -5.5,
        0.82,
        0.2,
        0.24,
        0.2,
        Color::srgb(0.85, 0.72, 0.60),
    ); // 奶茶杯
    furn(
        -10.5,
        -5.5,
        1.0,
        0.06,
        0.4,
        0.06,
        Color::srgb(0.85, 0.45, 0.4),
    ); // 吸管
    // 水果摊（东南角：摊桌 + 腿 + 几颗水果）
    furn(10.5, -6.0, 0.55, 1.4, 0.08, 0.7, art::WOOD);
    for (dx, dz) in [(-0.6, -0.28), (0.6, -0.28), (-0.6, 0.28), (0.6, 0.28)] {
        furn(10.5 + dx, -6.0 + dz, 0.27, 0.08, 0.55, 0.08, art::WOOD_DARK);
    }
    for (i, c) in [
        Color::srgb(0.85, 0.35, 0.30), // 苹果
        Color::srgb(0.90, 0.75, 0.30), // 橙子
        Color::srgb(0.55, 0.78, 0.40), // 青提
    ]
    .iter()
    .enumerate()
    {
        let fx = 10.5 - 0.3 + i as f32 * 0.3;
        furn(fx, -6.0, 0.72, 0.22, 0.22, 0.22, *c);
    }
    furn(
        10.9,
        -5.7,
        0.6,
        0.3,
        0.16,
        0.22,
        Color::srgb(0.62, 0.44, 0.28),
    ); // 果篮

    for (kind, x, z) in CAFETERIA_HOTSPOTS {
        spawn_hotspot(
            commands,
            *kind,
            o + Vec3::new(*x, 0.0, *z),
            meshes,
            materials,
            assets,
        );
    }

    // 一个同学端着餐盘找座位（巡逻）
    spawn_decor_npc(
        commands,
        meshes,
        materials,
        o + Vec3::new(-2.0, 0.0, 0.5),
        Color::srgb(0.55, 0.60, 0.85),
        Some((
            o + Vec3::new(-2.0, 0.0, 0.5),
            o + Vec3::new(3.0, 0.0, 0.5),
            0.9,
        )),
    );
}

// 排队 NPC：躯干 + 头 + 发，站定后轻微上下浮动
fn spawn_queue_npc(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    pos: Vec3,
    shirt: Color,
    phase: f32,
) {
    let cloth = materials.add(StandardMaterial {
        base_color: shirt,
        perceptual_roughness: 0.75,
        ..default()
    });
    let skin = materials.add(StandardMaterial {
        base_color: Color::srgb(0.95, 0.78, 0.62),
        perceptual_roughness: 0.7,
        ..default()
    });
    let hair = materials.add(StandardMaterial {
        base_color: Color::srgb(0.20, 0.16, 0.14),
        perceptual_roughness: 0.7,
        ..default()
    });
    let pants = materials.add(StandardMaterial {
        base_color: Color::srgb(0.28, 0.30, 0.36),
        perceptual_roughness: 0.8,
        ..default()
    });
    commands
        .spawn((
            GameRoot,
            SceneRoot,
            Visibility::default(),
            QueueNpc { phase },
            Transform::from_translation(pos),
        ))
        .with_children(|n| {
            n.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.46, 0.6, 0.3))),
                MeshMaterial3d(cloth.clone()),
                Transform::from_xyz(0.0, 0.75, 0.0),
            ));
            n.spawn((
                Mesh3d(meshes.add(Sphere::new(0.2))),
                MeshMaterial3d(skin),
                Transform::from_xyz(0.0, 1.25, 0.0),
            ));
            n.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.26, 0.1, 0.26))),
                MeshMaterial3d(hair),
                Transform::from_xyz(0.0, 1.38, 0.0),
            ));
            n.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.2, 0.5, 0.2))),
                MeshMaterial3d(pants),
                Transform::from_xyz(0.0, 0.28, 0.0),
            ));
        });
}

// 排队 NPC 轻微浮动（站久了会晃）
pub fn npc_bob(time: Res<Time>, mut npcs: Query<(&mut Transform, &QueueNpc)>) {
    let t = time.elapsed_secs();
    for (mut tf, npc) in &mut npcs {
        tf.translation.y = (t * 1.8 + npc.phase).sin() * 0.03;
    }
}

// ==================== 办公室（城北） ====================
fn build_office(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    assets: &Res<AssetServer>,
) {
    let o = OFFICE_CENTER;
    // 地板
    commands.spawn((
        GameRoot,
        SceneRoot,
        Mesh3d(meshes.add(Cuboid::new(26.0, 0.06, 26.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.72, 0.80, 0.87),
            perceptual_roughness: 0.95,
            ..default()
        })),
        Transform::from_xyz(o.x, 0.01, o.z),
    ));

    // 隔断
    art::spawn_wall(
        commands,
        meshes,
        materials,
        Vec3::new(3.4, 1.3, 0.12),
        o + Vec3::new(-4.2, 0.65, -3.5),
        Color::srgb(0.62, 0.68, 0.76),
    );
    art::spawn_wall(
        commands,
        meshes,
        materials,
        Vec3::new(3.4, 1.3, 0.12),
        o + Vec3::new(4.2, 0.65, -3.5),
        Color::srgb(0.62, 0.68, 0.76),
    );
    art::spawn_wall(
        commands,
        meshes,
        materials,
        Vec3::new(3.4, 1.3, 0.12),
        o + Vec3::new(-4.2, 0.65, 3.5),
        Color::srgb(0.62, 0.68, 0.76),
    );
    art::spawn_wall(
        commands,
        meshes,
        materials,
        Vec3::new(3.4, 1.3, 0.12),
        o + Vec3::new(4.2, 0.65, 3.5),
        Color::srgb(0.62, 0.68, 0.76),
    );

    // 办公桌（装饰，工位热点自带图标）
    art::spawn_table(
        commands,
        meshes,
        materials,
        o + Vec3::new(3.5, 0.0, 2.0),
        Vec2::new(1.6, 0.8),
        0.75,
    );
    art::spawn_monitor(
        commands,
        meshes,
        materials,
        o + Vec3::new(3.5, 0.0, 2.4),
        0.0,
    );
    art::spawn_cabinet(
        commands,
        meshes,
        materials,
        o + Vec3::new(5.2, 0.0, 0.0),
        -0.5,
    );

    // 茶水间区
    art::spawn_table(
        commands,
        meshes,
        materials,
        o + Vec3::new(-9.5, 0.0, 5.5),
        Vec2::new(1.0, 0.6),
        0.7,
    );
    art::spawn_plant(commands, meshes, materials, o + Vec3::new(-6.5, 0.0, 6.5));

    // 摸鱼区沙发
    art::spawn_sofa(
        commands,
        meshes,
        materials,
        o + Vec3::new(6.0, 0.0, 8.0),
        std::f32::consts::PI,
    );
    art::spawn_table(
        commands,
        meshes,
        materials,
        o + Vec3::new(6.0, 0.0, 7.0),
        Vec2::new(0.8, 0.5),
        0.5,
    );

    // 吊灯 / 白板 / 绿植
    art::spawn_hanging_lamp(commands, meshes, materials, o + Vec3::new(-4.0, 0.0, 0.0));
    art::spawn_hanging_lamp(commands, meshes, materials, o + Vec3::new(4.0, 0.0, 2.0));
    art::spawn_whiteboard(
        commands,
        meshes,
        materials,
        o + Vec3::new(-6.0, 0.0, 0.0),
        1.57,
    );
    art::spawn_plant(commands, meshes, materials, o + Vec3::new(11.0, 0.0, -8.0));

    // 南侧装饰墙
    art::spawn_wall(
        commands,
        meshes,
        materials,
        Vec3::new(24.0, 0.6, 0.3),
        o + Vec3::new(0.0, 0.3, -9.5),
        Color::srgb(0.55, 0.62, 0.72),
    );

    // 互动家具模型（与热点垫同位、不带碰撞，纯视觉）
    // 相对区域中心的装饰方块（非实心，逻辑统一在 art::spawn_block）
    let mut furn = |x: f32, z: f32, y: f32, sx: f32, sy: f32, sz: f32, c: Color| {
        art::spawn_block(
            commands,
            meshes,
            materials,
            o + Vec3::new(x, y, z),
            Vec3::new(sx, sy, sz),
            c,
        );
    };
    // 中央工位（工位热点下：桌面 + 四腿 + 显示器）
    furn(0.0, 0.0, 0.55, 1.4, 0.08, 0.7, art::WOOD);
    for (dx, dz) in [(-0.62, -0.28), (0.62, -0.28), (-0.62, 0.28), (0.62, 0.28)] {
        furn(dx, dz, 0.27, 0.08, 0.55, 0.08, art::WOOD_DARK);
    }
    furn(
        0.0,
        0.2,
        0.85,
        0.95,
        0.5,
        0.06,
        Color::srgb(0.15, 0.18, 0.25),
    ); // 屏幕
    furn(0.0, 0.2, 0.6, 0.3, 0.12, 0.08, Color::srgb(0.3, 0.3, 0.34)); // 底座
    // 咖啡机（茶水间西侧：机身 + 顶部壶 + 杯座）
    furn(
        -11.0,
        4.5,
        0.5,
        0.6,
        0.95,
        0.45,
        Color::srgb(0.32, 0.32, 0.36),
    );
    furn(
        -11.0,
        4.5,
        1.06,
        0.34,
        0.16,
        0.28,
        Color::srgb(0.75, 0.78, 0.82),
    );
    furn(
        -10.7,
        4.25,
        0.2,
        0.14,
        0.18,
        0.14,
        Color::srgb(0.92, 0.88, 0.82),
    ); // 杯子
    // 会议室（东侧开放会议角：会议桌 + 两侧椅子 + 北侧装饰墙，均为非实心装饰）
    furn(9.5, 1.0, 0.75, 1.6, 0.08, 0.9, art::WOOD);
    for (dx, dz) in [(-0.7, -0.38), (0.7, -0.38), (-0.7, 0.38), (0.7, 0.38)] {
        furn(9.5 + dx, 1.0 + dz, 0.37, 0.09, 0.75, 0.09, art::WOOD_DARK);
    }
    furn(8.0, 1.0, 0.45, 0.5, 0.07, 0.5, art::WOOD); // 西侧椅座
    furn(8.0, 1.0, 0.65, 0.5, 0.5, 0.07, art::WOOD); // 西侧椅背
    furn(11.0, 1.0, 0.45, 0.5, 0.07, 0.5, art::WOOD); // 东侧椅座
    furn(11.0, 1.0, 0.65, 0.5, 0.5, 0.07, art::WOOD); // 东侧椅背
    furn(9.5, 3.2, 0.6, 2.6, 1.2, 0.12, Color::srgb(0.62, 0.68, 0.76)); // 北侧隔墙
    // 打印机（东侧靠墙：机身 + 出纸口）
    furn(
        11.5,
        -2.0,
        0.3,
        0.7,
        0.55,
        0.4,
        Color::srgb(0.93, 0.94, 0.95),
    );
    furn(
        11.5,
        -2.0,
        0.52,
        0.45,
        0.1,
        0.3,
        Color::srgb(0.45, 0.48, 0.52),
    );
    // 外卖区小餐桌（外卖热点下）
    furn(8.0, -6.0, 0.5, 1.0, 0.08, 0.6, art::WOOD);
    for (dx, dz) in [(-0.42, -0.24), (0.42, -0.24), (-0.42, 0.24), (0.42, 0.24)] {
        furn(dx, -6.0 + dz, 0.25, 0.06, 0.5, 0.06, art::WOOD_DARK);
    }

    for (kind, x, z) in OFFICE_HOTSPOTS {
        spawn_hotspot(
            commands,
            *kind,
            o + Vec3::new(*x, 0.0, *z),
            meshes,
            materials,
            assets,
        );
    }

    // 四位同事：Mentor 张哥 / 产品小赵 / Leader 王总 / HR 李姐
    spawn_npc_entity(
        commands,
        meshes,
        materials,
        assets,
        o + Vec3::new(2.6, 0.0, 0.8),
        4,
        None,
    );
    spawn_npc_entity(
        commands,
        meshes,
        materials,
        assets,
        o + Vec3::new(-2.6, 0.0, 0.8),
        6,
        None,
    );
    spawn_npc_entity(
        commands,
        meshes,
        materials,
        assets,
        o + Vec3::new(2.6, 0.0, 5.2),
        5,
        None,
    );
    spawn_npc_entity(
        commands,
        meshes,
        materials,
        assets,
        o + Vec3::new(-2.6, 0.0, 5.2),
        7,
        None,
    );
}

// ==================== 公园（城东北） ====================
fn build_park(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    assets: &Res<AssetServer>,
) {
    let o = PARK_CENTER;
    // 草地
    commands.spawn((
        GameRoot,
        SceneRoot,
        Mesh3d(meshes.add(Cuboid::new(26.0, 0.06, 26.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.52, 0.72, 0.42),
            perceptual_roughness: 0.95,
            ..default()
        })),
        Transform::from_xyz(o.x, 0.01, o.z),
    ));

    // 十字小径（散步道）
    let path = materials.add(StandardMaterial {
        base_color: Color::srgb(0.80, 0.72, 0.58),
        perceptual_roughness: 0.95,
        ..default()
    });
    commands.spawn((
        GameRoot,
        SceneRoot,
        Mesh3d(meshes.add(Cuboid::new(2.0, 0.03, 20.0))),
        MeshMaterial3d(path.clone()),
        Transform::from_xyz(o.x, 0.015, o.z),
    ));
    commands.spawn((
        GameRoot,
        SceneRoot,
        Mesh3d(meshes.add(Cuboid::new(20.0, 0.03, 2.0))),
        MeshMaterial3d(path.clone()),
        Transform::from_xyz(o.x, 0.015, o.z),
    ));

    // 中央喷泉：水池 + 水柱（南侧）
    let pool = materials.add(StandardMaterial {
        base_color: Color::srgb(0.55, 0.62, 0.68),
        perceptual_roughness: 0.5,
        ..default()
    });
    let water = materials.add(StandardMaterial {
        base_color: Color::srgb(0.45, 0.70, 0.85),
        perceptual_roughness: 0.2,
        ..default()
    });
    commands.spawn((
        GameRoot,
        SceneRoot,
        Mesh3d(meshes.add(Cylinder::new(2.0, 0.18))),
        MeshMaterial3d(pool),
        Transform::from_xyz(o.x, 0.09, o.z - 5.0),
    ));
    commands.spawn((
        GameRoot,
        SceneRoot,
        Mesh3d(meshes.add(Cylinder::new(0.25, 1.6))),
        MeshMaterial3d(water),
        Transform::from_xyz(o.x, 0.8, o.z - 5.0),
    ));

    // 长椅 + 树（长椅是互动热点，非实心，玩家可走到长椅前歇脚）
    for (x, z) in [(-5.0, 0.0), (5.0, 3.0)] {
        art::spawn_bench(commands, meshes, materials, o + Vec3::new(x, 0.0, z), 0.0, false);
    }
    for (x, z, s) in [
        (-8.0, 6.0, 1.3),
        (8.0, -8.0, 1.1),
        (-9.0, -3.0, 1.0),
        (9.0, 6.0, 1.2),
        (-3.0, 9.0, 0.9),
        (6.0, -6.0, 1.1),
    ] {
        art::spawn_tree(commands, meshes, materials, o + Vec3::new(x, 0.0, z), s);
    }

    // 热点：两处长椅 + 喷泉
    for (kind, x, z) in PARK_HOTSPOTS {
        spawn_hotspot(
            commands,
            *kind,
            o + Vec3::new(*x, 0.0, *z),
            meshes,
            materials,
            assets,
        );
    }

    // 日志确认：喷泉与长椅的世界坐标（对照 PARK_HOTSPOTS 相对坐标 (0,-5)/(-5,0)/(5,3)）
    info!(
        "[公园] 中心 ({:.0},{:.0})：喷泉 ({:.0},{:.0})，长椅 ({:.0},{:.0}) / ({:.0},{:.0})",
        o.x,
        o.z,
        o.x + 0.0,
        o.z - 5.0,
        o.x - 5.0,
        o.z + 0.0,
        o.x + 5.0,
        o.z + 3.0,
    );

    // 公园里散步的游客
    spawn_decor_npc(
        commands,
        meshes,
        materials,
        o + Vec3::new(-3.0, 0.0, 2.0),
        Color::srgb(0.75, 0.68, 0.55),
        Some((
            o + Vec3::new(-6.0, 0.0, 4.0),
            o + Vec3::new(6.0, 0.0, 4.0),
            0.8,
        )),
    );
    spawn_decor_npc(
        commands,
        meshes,
        materials,
        o + Vec3::new(3.0, 0.0, -2.0),
        Color::srgb(0.62, 0.75, 0.65),
        Some((
            o + Vec3::new(-5.0, 0.0, -3.0),
            o + Vec3::new(5.0, 0.0, -3.0),
            0.7,
        )),
    );
}

// ==================== NPC 实体 ====================
// 可对话 NPC：方块人（衣服色 = 对话树角色色）+ 头顶名牌，走近点击可聊天。
fn spawn_npc_entity(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    _assets: &Res<AssetServer>,
    pos: Vec3,
    idx: usize,
    wander: Option<(Vec3, Vec3, f32)>,
) {
    let npc = &super::npc::NPCS[idx];
    let cloth = materials.add(StandardMaterial {
        base_color: npc.color,
        perceptual_roughness: 0.75,
        ..default()
    });
    let skin = materials.add(StandardMaterial {
        base_color: Color::srgb(0.95, 0.78, 0.62),
        perceptual_roughness: 0.7,
        ..default()
    });
    let hair = materials.add(StandardMaterial {
        base_color: Color::srgb(0.20, 0.16, 0.14),
        perceptual_roughness: 0.7,
        ..default()
    });
    let pants = materials.add(StandardMaterial {
        base_color: Color::srgb(0.28, 0.30, 0.36),
        perceptual_roughness: 0.8,
        ..default()
    });
    let mut root = commands.spawn((
        GameRoot,
        SceneRoot,
        NpcMarker { idx },
        // 必须有 Visibility：其下的 Text2d 名牌依赖它计算可见性（否则 Bevy 报 B0004）
        Visibility::default(),
        Transform::from_translation(pos),
    ));
    root.with_children(|n| {
        n.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.46, 0.6, 0.3))),
            MeshMaterial3d(cloth.clone()),
            Transform::from_xyz(0.0, 0.75, 0.0),
        ));
        n.spawn((
            Mesh3d(meshes.add(Sphere::new(0.2))),
            MeshMaterial3d(skin),
            Transform::from_xyz(0.0, 1.25, 0.0),
        ));
        n.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.26, 0.1, 0.26))),
            MeshMaterial3d(hair),
            Transform::from_xyz(0.0, 1.38, 0.0),
        ));
        n.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.2, 0.5, 0.2))),
            MeshMaterial3d(pants),
            Transform::from_xyz(0.0, 0.28, 0.0),
        ));
        // 头顶名牌：Bevy 0.19 的 Text2d 加任何非单位 scale 都不渲染，改由 UI 投影实现
        // （见 spawn_world_labels / update_world_labels），此处不再生成 Text2d。
    });
    if let Some((from, to, speed)) = wander {
        root.insert(WanderNpc {
            from,
            to,
            speed,
            t: 0.0,
            path: Vec::new(),
        });
    }
}

// ==================== UI 世界标签（名牌 / 热点标签） ====================
// 方案总览：Bevy 0.19 的 Text2d 无法用 Transform.scale 缩小（任何非单位缩放都会导致
// 文字不渲染），因此名牌 / 标签改为 UI 文字（屏幕像素）。
// 数据流：每帧 update_world_labels 把 NPC / 热点实体（WorldLabel.target）的世界坐标
// 投影到屏幕（Camera::world_to_viewport），写入对应 UI 节点的 left/top。
// 名牌以固定屏幕字号显示，不随距离缩放；UI 相机层级更高，永不被 3D 场景遮挡。
// 可见性：仅玩家靠近时淡入，远离或出屏时淡出，避免名牌瞬间出现/消失的生硬感。
//
// 以下常量集中控制名牌的显示行为，要调「显示距离 / 字号 / 位置 / 淡入速度」只改这里：
const NPC_LABEL_RADIUS: f32 = 6.0;     // 玩家离 NPC 多近才显示名牌（世界单位）
const HOTSPOT_LABEL_RADIUS: f32 = 5.0; // 玩家离热点多近才显示标签（世界单位）
const LABEL_HYSTERESIS: f32 = 1.25;    // 熄灭延迟系数：走远到半径的 1.25 倍才淡出，防边界闪烁
const LABEL_FADE_SPEED: f32 = 8.0;     // 淡入淡出速度（每秒 alpha 变化量），约 0.15 秒完成过渡
const LABEL_TOP_BIAS: f32 = 1.1;       // 文字相对锚点再上移的字高倍数，让文字贴着目标头顶
const LABEL_SMOOTHING: f32 = 0.35;     // 名牌屏幕位置的帧间插值系数（0 = 关闭；越大跟随越平滑，
                                       // 但快速移动时文字滞后感越明显，0.35 是平滑与跟手之间的折中）
const NPC_LABEL_OFFSET: f32 = 1.35;    // NPC 名牌锚点相对脚下的世界高度（头顶附近）
const HOTSPOT_LABEL_OFFSET: f32 = 1.3; // 热点标签锚点相对地面的世界高度（垫子上方）
const NPC_LABEL_SIZE: f32 = 13.0;      // NPC 名牌字号（px）
const HOTSPOT_LABEL_SIZE: f32 = 12.0;  // 热点标签字号（px）

/// 进入 Playing 后为每个 NPC 和热点创建对应的 UI 名牌（初始隐藏，投影到屏幕时显示）
pub fn spawn_world_labels(
    mut commands: Commands,
    assets: Res<AssetServer>,
    npcs: Query<(Entity, &NpcMarker)>,
    hotspots: Query<(Entity, &Hotspot)>,
) {
    let font = FontSource::Handle(assets.load(FONT_PATH));
    let labels = |commands: &mut ChildSpawnerCommands,
                      target: Entity,
                      text: String,
                      offset: f32,
                      size: f32| {
        // 估算文本宽度用于水平居中：中文字符宽约等于字号，`字符数 × 字号` 足够接近。
        // 不用真实宽度（TextLayoutInfo）是因为名牌在 UI 布局完成前就要定位，
        // 中英文混排时会有几像素偏差，对名牌来说可接受。
        let est_width = text.chars().count() as f32 * size;
        commands.spawn((
            GameRoot,
            WorldLabel {
                target,
                offset,
                est_width,
                font_size: size,
                last_left: f32::NAN,
                last_top: f32::NAN,
                cur_left: f32::NAN,
                cur_top: f32::NAN,
                showing: false,
                alpha: 0.0,
            },
            Node {
                position_type: PositionType::Absolute,
                left: px(-500.0),
                top: px(-500.0),
                ..default()
            },
            Text::new(text),
            TextColor(Color::srgb(0.28, 0.22, 0.16)),
            TextFont {
                font: font.clone(),
                font_size: FontSize::Px(size),
                ..default()
            },
            Visibility::Hidden,
        ));
    };
    commands
        .spawn((
            GameRoot,
            Node {
                position_type: PositionType::Absolute,
                width: percent(100),
                height: percent(100),
                ..default()
            },
            FocusPolicy::Pass,
        ))
        .with_children(|root| {
            for (e, npc) in &npcs {
                let d = &NPCS[npc.idx];
                labels(
                    root,
                    e,
                    format!("{} · {}", d.name, d.tag),
                    NPC_LABEL_OFFSET,
                    NPC_LABEL_SIZE,
                );
            }
            for (e, hp) in &hotspots {
                labels(root, e, hp.kind.label().to_string(), HOTSPOT_LABEL_OFFSET, HOTSPOT_LABEL_SIZE);
            }
        });
}

/// 每帧把 NPC / 热点世界坐标投影到屏幕，更新对应 UI 名牌的位置
pub fn update_world_labels(
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &Transform), With<Camera3d>>,
    player: Single<&Transform, With<PlayerRoot>>,
    time: Res<Time>,
    npcs: Query<(Entity, &Transform), With<NpcMarker>>,
    hotspots: Query<(Entity, &Transform), With<Hotspot>>,
    mut labels: Query<(&mut WorldLabel, &mut Node, &mut TextColor, &mut Visibility)>,
) {
    let (cam, cam_tf) = *camera;
    // 相机每帧由 camera_follow 更新 Transform，但 GlobalTransform 要到 PostUpdate 才传播。
    // 相机是顶层实体（无父级），Transform 即 GlobalTransform，直接重建本帧视图矩阵，
    // 避免名牌相对移动中的画面滞后一帧。
    let cam_gt = GlobalTransform::from(*cam_tf);
    let win_w = window.width();
    let win_h = window.height();
    let pl = player.translation;
    // 淡入淡出速度（/秒）：约 0.15 秒完成显隐过渡，避免名牌瞬间出现/消失的生硬感。
    // 位置只在像素取整后实际变化时才写入 Node：否则每帧改 left/top 都会让
    // bevy_ui 的 text_system 判定 ComputedNode 变化而重布局全部中文文本，
    // 既拖慢帧率又刷出大量 ICU4X 分词警告。
    let fade_step = LABEL_FADE_SPEED * time.delta_secs();
    // NPC 与热点共用同一段「距离判断 + 投影 + 淡显隐」逻辑，所以用闭包复用；
    // Bevy 的系统参数（Query 等）只能在系统函数里声明，不能传进普通函数，闭包可借用 labels。
    let project = |target: Entity,
                   base: Vec3,
                   radius: f32,
                   labels: &mut Query<(&mut WorldLabel, &mut Node, &mut TextColor, &mut Visibility)>| {
        for (mut l, mut node, mut color, mut vis) in
            labels.iter_mut().filter(|(l, _, _, _)| l.target == target)
        {
            let dist = Vec2::new(base.x - pl.x, base.z - pl.z).length();
            // 迟滞：进半径亮起，出 LABEL_HYSTERESIS 倍半径才熄灭，防止在边界来回走动时名牌闪烁
            if l.showing {
                if dist > radius * LABEL_HYSTERESIS {
                    l.showing = false;
                }
            } else if dist < radius {
                l.showing = true;
            }
            // 亮起时投影定位；目标投影出屏则视为不可见（跟随淡出，不残留原位）
            let mut on_screen = false;
            if l.showing {
                let world = base + Vec3::Y * l.offset;
                if let Ok(p) = cam.world_to_viewport(&cam_gt, world) {
                    if p.x >= 0.0 && p.x <= win_w && p.y >= 0.0 && p.y <= win_h {
                        on_screen = true;
                        // 目标屏幕坐标（未取整的浮点），作为帧间插值的收敛目标
                        let target_x = p.x - l.est_width / 2.0;
                        let target_y = p.y - l.font_size * LABEL_TOP_BIAS;
                        // 帧间插值：cur 每帧向目标逼近（LABEL_SMOOTHING 越大跟随越平滑，滞后感也越明显）；
                        // 首帧直接落到目标，避免开局从远处飘入。
                        if l.cur_left.is_nan() {
                            l.cur_left = target_x;
                            l.cur_top = target_y;
                        } else {
                            l.cur_left += (target_x - l.cur_left) * LABEL_SMOOTHING;
                            l.cur_top += (target_y - l.cur_top) * LABEL_SMOOTHING;
                        }
                        // 取整到整数像素后写入：亚像素位置会让文字渲染到像素网格之间，抖动/发虚；
                        // 取整值不变时跳过写入，避免触发文本重布局（见函数上方注释）。
                        let left = l.cur_left.round();
                        let top = l.cur_top.round();
                        if left != l.last_left || top != l.last_top {
                            node.left = px(left);
                            node.top = px(top);
                            l.last_left = left;
                            l.last_top = top;
                        }
                    }
                }
            }
            // 淡入/淡出：alpha 变化时才写颜色，避免无谓的 UI 变化检测
            let next = (l.alpha + if l.showing && on_screen { fade_step } else { -fade_step })
                .clamp(0.0, 1.0);
            if next != l.alpha {
                l.alpha = next;
                color.0.set_alpha(l.alpha);
            }
            // 完全透明后真正隐藏节点（省去提取/渲染开销），同时重置插值位置：
            // 下次靠近时名牌直接落位，而不是从旧屏幕位置「飞」到新位置。
            let want = if l.alpha > 0.0 {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
            if *vis != want {
                *vis = want;
                if want == Visibility::Hidden {
                    l.cur_left = f32::NAN;
                    l.cur_top = f32::NAN;
                }
            }
        }
    };
    for (e, tf) in &npcs {
        project(e, tf.translation, NPC_LABEL_RADIUS, &mut labels);
    }
    for (e, tf) in &hotspots {
        project(e, tf.translation, HOTSPOT_LABEL_RADIUS, &mut labels);
    }
}

// 装饰 NPC（无对话）：可巡逻，纯活跃气氛
fn spawn_decor_npc(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    pos: Vec3,
    shirt: Color,
    wander: Option<(Vec3, Vec3, f32)>,
) {
    let cloth = materials.add(StandardMaterial {
        base_color: shirt,
        perceptual_roughness: 0.75,
        ..default()
    });
    let skin = materials.add(StandardMaterial {
        base_color: Color::srgb(0.95, 0.78, 0.62),
        perceptual_roughness: 0.7,
        ..default()
    });
    let hair = materials.add(StandardMaterial {
        base_color: Color::srgb(0.20, 0.16, 0.14),
        perceptual_roughness: 0.7,
        ..default()
    });
    let pants = materials.add(StandardMaterial {
        base_color: Color::srgb(0.28, 0.30, 0.36),
        perceptual_roughness: 0.8,
        ..default()
    });
    let mut root = commands.spawn((
        GameRoot,
        SceneRoot,
        Visibility::default(),
        Transform::from_translation(pos),
    ));
    root.with_children(|n| {
        n.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.46, 0.6, 0.3))),
            MeshMaterial3d(cloth.clone()),
            Transform::from_xyz(0.0, 0.75, 0.0),
        ));
        n.spawn((
            Mesh3d(meshes.add(Sphere::new(0.2))),
            MeshMaterial3d(skin),
            Transform::from_xyz(0.0, 1.25, 0.0),
        ));
        n.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.26, 0.1, 0.26))),
            MeshMaterial3d(hair),
            Transform::from_xyz(0.0, 1.38, 0.0),
        ));
        n.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.2, 0.5, 0.2))),
            MeshMaterial3d(pants),
            Transform::from_xyz(0.0, 0.28, 0.0),
        ));
    });
    if let Some((from, to, speed)) = wander {
        root.insert(WanderNpc {
            from,
            to,
            speed,
            t: 0.0,
            path: Vec::new(),
        });
    }
}

// ==================== NPC 巡逻 ====================
// 在 from/to 之间往返走动：先用 A* 算出绕行路径，再沿路径走（含走路浮动与朝向翻转）。
// 巡逻同样做碰撞推出：NPC 不会穿墙/穿建筑。

// 路径总长
fn path_length(path: &[Vec2]) -> f32 {
    path.windows(2).map(|w| w[0].distance(w[1])).sum()
}

// 路径上按比例取点（0..1）
fn point_on_path(path: &[Vec2], frac: f32) -> Vec2 {
    let total = path_length(path);
    if total <= 0.0 {
        return path[0];
    }
    let target = total * frac.clamp(0.0, 1.0);
    let mut acc = 0.0;
    for w in path.windows(2) {
        let seg = w[0].distance(w[1]);
        if acc + seg >= target {
            let t = if seg > 0.0 { (target - acc) / seg } else { 0.0 };
            return w[0].lerp(w[1], t);
        }
        acc += seg;
    }
    *path.last().unwrap()
}

pub fn wander_npcs(
    time: Res<Time>,
    map: Res<super::collision::CollisionMap>,
    mut npcs: Query<(&mut Transform, &mut WanderNpc)>,
) {
    let dt = time.delta_secs();
    for (mut tf, mut w) in &mut npcs {
        // 首次巡逻：用 A* 计算绕行路径（直线畅通时就是两点直线）
        if w.path.is_empty() {
            w.path = super::collision::find_path(&map, w.from.xz(), w.to.xz())
                .unwrap_or_else(|| vec![w.from.xz(), w.to.xz()]);
            w.t = 0.0;
        }
        let total = path_length(&w.path);
        let phase = if total > 0.0 {
            (w.t / total).fract()
        } else {
            0.0
        };
        // 前半程去、后半程回，s 恒为 0..1 递增的路径位置
        let s = if phase < 0.5 {
            phase * 2.0
        } else {
            (phase - 0.5) * 2.0
        };
        let p = point_on_path(&w.path, s);
        let p_next = point_on_path(&w.path, (s + 0.01).min(1.0));
        let dir = p_next - p;
        let swing = (w.t * 9.0).sin() * 0.06;
        tf.translation = Vec3::new(p.x, swing, p.y);
        super::collision::resolve(&mut tf.translation, &map.boxes);
        tf.rotation = Quat::from_rotation_y(dir.x.atan2(dir.y));
        w.t += dt * w.speed;
    }
}

// ==================== 昼夜光照 ====================
// 按时段调整主方向光与环境光：上午明亮、工作偏白、午饭暖、晚上暖橙偏暗。
pub fn update_daylight(
    clock: Res<GameClock>,
    mut light: Query<&mut DirectionalLight, With<DayLight>>,
    mut ambient: ResMut<GlobalAmbientLight>,
) {
    let (color, illuminance, amb_bright, amb_color) = match clock.phase {
        Phase::Morning => (
            Color::srgb(1.0, 0.97, 0.90),
            9000.0,
            380.0,
            Color::srgb(1.0, 0.98, 0.95),
        ),
        Phase::Work => (
            Color::srgb(1.0, 0.96, 0.88),
            8800.0,
            360.0,
            Color::srgb(1.0, 0.97, 0.92),
        ),
        Phase::Lunch => (
            Color::srgb(1.0, 0.94, 0.84),
            8500.0,
            340.0,
            Color::srgb(1.0, 0.95, 0.88),
        ),
        Phase::Evening => (
            Color::srgb(1.0, 0.78, 0.58),
            6200.0,
            230.0,
            Color::srgb(1.0, 0.82, 0.68),
        ),
    };
    for mut l in &mut light {
        l.color = color;
        l.illuminance = illuminance;
    }
    ambient.brightness = amb_bright;
    ambient.color = amb_color;
}

fn glass_color() -> Color {
    Color::srgb(0.55, 0.78, 0.88)
}

// ==================== 热点 ====================
// 圆垫 + 差异化图标（文字标签由 UI 投影实现）
fn spawn_hotspot(
    commands: &mut Commands,
    kind: HotspotKind,
    pos: Vec3,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    _assets: &Res<AssetServer>,
) {
    let mat = meshes.add(Cylinder::new(1.15, 0.08));
    let mat_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.93, 0.90, 0.82),
        perceptual_roughness: 0.95,
        ..default()
    });
    commands
        .spawn((
            GameRoot,
            SceneRoot,
            Hotspot { kind },
            // 必须有 Visibility：其下的 Text2d 标签依赖它计算可见性（否则 Bevy 报 B0004）
            Visibility::default(),
            Transform::from_xyz(pos.x, 0.0, pos.z),
        ))
        .with_children(|p| {
            p.spawn((
                Mesh3d(mat.clone()),
                MeshMaterial3d(mat_mat.clone()),
                Transform::from_xyz(0.0, 0.04, 0.0),
            ));
            // 差异化图标
            hotspot_icon(p, kind, meshes, materials);
            // 文字标签：由 UI 投影实现（spawn_world_labels / update_world_labels），
            // 此处不再生成 Text2d（Bevy 0.19 Text2d 无法缩放）。
        });
}

// 每种热点一种低模造型（用基础几何体拼装）
#[allow(clippy::too_many_lines)]
fn hotspot_icon(
    p: &mut ChildSpawnerCommands,
    kind: HotspotKind,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    // 放一个带颜色的盒体
    let icon_box = |p: &mut ChildSpawnerCommands,
                    m: &mut Assets<Mesh>,
                    mt: &mut Assets<StandardMaterial>,
                    color: Color,
                    sx: f32,
                    sy: f32,
                    sz: f32,
                    x: f32,
                    y: f32,
                    z: f32| {
        let mesh = m.add(Cuboid::new(sx, sy, sz));
        let mat = mt.add(StandardMaterial {
            base_color: color,
            perceptual_roughness: 0.85,
            ..default()
        });
        p.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(mat),
            Transform::from_translation(Vec3::new(x, y, z)),
        ));
    };
    // 放一个带颜色的圆柱
    let icon_cyl = |p: &mut ChildSpawnerCommands,
                    m: &mut Assets<Mesh>,
                    mt: &mut Assets<StandardMaterial>,
                    color: Color,
                    r: f32,
                    h: f32,
                    x: f32,
                    y: f32,
                    z: f32| {
        let mesh = m.add(Cylinder::new(r, h));
        let mat = mt.add(StandardMaterial {
            base_color: color,
            perceptual_roughness: 0.85,
            ..default()
        });
        p.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(mat),
            Transform::from_translation(Vec3::new(x, y, z)),
        ));
    };
    // 放一个带颜色的球
    let icon_ball = |p: &mut ChildSpawnerCommands,
                     m: &mut Assets<Mesh>,
                     mt: &mut Assets<StandardMaterial>,
                     color: Color,
                     r: f32,
                     x: f32,
                     y: f32,
                     z: f32| {
        let mesh = m.add(Sphere::new(r));
        let mat = mt.add(StandardMaterial {
            base_color: color,
            perceptual_roughness: 0.85,
            ..default()
        });
        p.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(mat),
            Transform::from_translation(Vec3::new(x, y, z)),
        ));
    };

    let (mx, mt) = (meshes.as_mut(), materials.as_mut());
    match kind {
        HotspotKind::Bed => {
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.45, 0.60, 0.95),
                1.5,
                0.28,
                0.95,
                0.0,
                0.25,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.45, 0.60, 0.95),
                0.25,
                0.4,
                0.95,
                -0.7,
                0.4,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.98, 0.97, 0.92),
                0.45,
                0.12,
                0.35,
                0.5,
                0.42,
                0.0,
            );
        }
        HotspotKind::Desk => {
            icon_box(p, mx, mt, wood_color(), 1.1, 0.08, 0.6, 0.0, 0.35, 0.0);
            icon_box(p, mx, mt, wood_color(), 0.08, 0.35, 0.08, -0.45, 0.17, 0.0);
            icon_box(p, mx, mt, wood_color(), 0.08, 0.35, 0.08, 0.45, 0.17, 0.0);
        }
        HotspotKind::Books => {
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.70, 0.35, 0.32),
                0.14,
                0.5,
                0.22,
                -0.16,
                0.3,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.35, 0.55, 0.75),
                0.14,
                0.62,
                0.22,
                0.0,
                0.35,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.45, 0.68, 0.42),
                0.14,
                0.42,
                0.22,
                0.16,
                0.26,
                0.0,
            );
        }
        HotspotKind::Kitchen => {
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.80, 0.62, 0.45),
                1.2,
                0.1,
                0.7,
                0.0,
                0.4,
                0.0,
            );
            icon_cyl(
                p,
                mx,
                mt,
                Color::srgb(0.65, 0.68, 0.72),
                0.24,
                0.18,
                0.0,
                0.55,
                0.0,
            );
        }
        HotspotKind::Computer => {
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.22, 0.30, 0.45),
                0.9,
                0.55,
                0.06,
                0.0,
                0.45,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.3, 0.3, 0.34),
                0.1,
                0.25,
                0.1,
                0.0,
                0.22,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.3, 0.3, 0.34),
                0.35,
                0.05,
                0.2,
                0.0,
                0.1,
                0.0,
            );
        }
        HotspotKind::Phone => {
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.40, 0.78, 0.45),
                0.6,
                0.1,
                0.3,
                0.0,
                0.22,
                0.0,
            );
        }
        // 电视：屏幕 + 底座
        HotspotKind::Tv => {
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.15, 0.18, 0.25),
                0.95,
                0.55,
                0.07,
                0.0,
                0.5,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.3, 0.3, 0.34),
                0.4,
                0.05,
                0.22,
                0.0,
                0.22,
                0.0,
            );
        }
        // 浴室：浴缸 + 水面
        HotspotKind::Bathroom => {
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.93, 0.95, 0.96),
                1.1,
                0.1,
                0.62,
                0.0,
                0.18,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.45, 0.70, 0.85),
                0.92,
                0.08,
                0.46,
                0.0,
                0.22,
                0.0,
            );
        }
        // 冰箱：箱体 + 拉手
        HotspotKind::Fridge => {
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.90, 0.92, 0.94),
                0.6,
                0.95,
                0.42,
                0.0,
                0.52,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.65, 0.68, 0.72),
                0.05,
                0.32,
                0.05,
                0.0,
                0.52,
                0.22,
            );
        }
        HotspotKind::Track => {
            icon_cyl(
                p,
                mx,
                mt,
                Color::srgb(0.82, 0.62, 0.42),
                1.25,
                0.08,
                0.0,
                0.16,
                0.0,
            );
            icon_cyl(
                p,
                mx,
                mt,
                Color::srgb(0.62, 0.72, 0.52),
                1.0,
                0.1,
                0.0,
                0.2,
                0.0,
            );
        }
        HotspotKind::TechGroup => {
            icon_ball(p, mx, mt, Color::srgb(0.35, 0.65, 0.85), 0.4, 0.0, 0.5, 0.0);
            icon_ball(
                p,
                mx,
                mt,
                Color::srgb(0.55, 0.78, 0.92),
                0.22,
                0.42,
                0.38,
                0.0,
            );
        }
        // 图书馆：书架层板 + 一排竖版书本
        HotspotKind::Library => {
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.62, 0.44, 0.28),
                1.25,
                0.08,
                0.34,
                0.0,
                0.3,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.70, 0.35, 0.32),
                0.14,
                0.44,
                0.22,
                -0.36,
                0.42,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.35, 0.55, 0.75),
                0.14,
                0.52,
                0.22,
                -0.12,
                0.46,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.45, 0.68, 0.42),
                0.14,
                0.4,
                0.22,
                0.12,
                0.4,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.82, 0.64, 0.35),
                0.14,
                0.48,
                0.22,
                0.36,
                0.44,
                0.0,
            );
        }
        // 实验室：烧瓶 + 试管
        HotspotKind::Lab => {
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.86, 0.94, 0.90),
                0.66,
                0.08,
                0.66,
                0.0,
                0.16,
                0.0,
            );
            icon_cyl(
                p,
                mx,
                mt,
                Color::srgb(0.55, 0.78, 0.90),
                0.16,
                0.42,
                0.0,
                0.32,
                0.0,
            );
            icon_cyl(
                p,
                mx,
                mt,
                Color::srgb(0.70, 0.82, 0.90),
                0.1,
                0.32,
                -0.32,
                0.34,
                0.0,
            );
            icon_cyl(
                p,
                mx,
                mt,
                Color::srgb(0.80, 0.55, 0.45),
                0.12,
                0.1,
                0.16,
                0.3,
                0.0,
            );
        }
        // 小卖部：货架 + 零食
        HotspotKind::CampusShop => {
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.62, 0.44, 0.28),
                0.9,
                0.5,
                0.14,
                0.0,
                0.42,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.85, 0.55, 0.30),
                0.18,
                0.16,
                0.16,
                -0.24,
                0.34,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.70, 0.80, 0.45),
                0.18,
                0.16,
                0.16,
                0.02,
                0.44,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.85, 0.40, 0.35),
                0.18,
                0.16,
                0.16,
                0.26,
                0.32,
                0.0,
            );
        }
        // 宿舍床：床垫 + 床头 + 枕头
        HotspotKind::DormBed => {
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.45, 0.60, 0.95),
                1.5,
                0.28,
                0.95,
                0.0,
                0.25,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.45, 0.60, 0.95),
                0.25,
                0.4,
                0.95,
                -0.7,
                0.4,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.98, 0.97, 0.92),
                0.45,
                0.12,
                0.35,
                0.5,
                0.42,
                0.0,
            );
        }
        // 室友电脑：屏幕 + 手柄
        HotspotKind::DormGame => {
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.22, 0.30, 0.45),
                0.85,
                0.5,
                0.06,
                0.0,
                0.5,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.3, 0.3, 0.34),
                0.32,
                0.06,
                0.2,
                0.0,
                0.2,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.80, 0.45, 0.40),
                0.4,
                0.12,
                0.18,
                0.0,
                0.32,
                0.0,
            );
        }
        // 零食柜：柜体 + 零食
        HotspotKind::DormSnack => {
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.90, 0.92, 0.94),
                0.6,
                0.85,
                0.4,
                0.0,
                0.46,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.85, 0.55, 0.30),
                0.24,
                0.14,
                0.18,
                -0.14,
                0.85,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.70, 0.80, 0.45),
                0.2,
                0.12,
                0.16,
                0.16,
                0.85,
                0.0,
            );
        }
        HotspotKind::Canteen1 | HotspotKind::Canteen2 => {
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.92, 0.90, 0.84),
                0.7,
                0.06,
                0.5,
                0.0,
                0.2,
                0.0,
            );
            let food = if kind == HotspotKind::Canteen1 {
                Color::srgb(0.85, 0.62, 0.30)
            } else {
                Color::srgb(0.80, 0.40, 0.35)
            };
            icon_box(p, mx, mt, food, 0.45, 0.16, 0.3, 0.0, 0.32, 0.0);
        }
        HotspotKind::Microwave => {
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.55, 0.62, 0.70),
                0.8,
                0.4,
                0.5,
                0.0,
                0.35,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.2, 0.25, 0.32),
                0.35,
                0.28,
                0.02,
                0.0,
                0.38,
                0.26,
            );
        }
        HotspotKind::InstantNoodle => {
            icon_cyl(
                p,
                mx,
                mt,
                Color::srgb(0.85, 0.55, 0.30),
                0.28,
                0.5,
                0.0,
                0.32,
                0.0,
            );
            icon_cyl(
                p,
                mx,
                mt,
                Color::srgb(0.9, 0.7, 0.4),
                0.3,
                0.06,
                0.0,
                0.58,
                0.0,
            );
        }
        // 饮料机：机身 + 杯
        HotspotKind::DrinkMachine => {
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.28, 0.36, 0.48),
                0.55,
                0.85,
                0.4,
                0.0,
                0.48,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.55, 0.78, 0.90),
                0.34,
                0.32,
                0.06,
                0.0,
                0.7,
                0.0,
            );
            icon_cyl(
                p,
                mx,
                mt,
                Color::srgb(0.92, 0.88, 0.82),
                0.14,
                0.24,
                0.0,
                0.3,
                0.0,
            );
        }
        // 奶茶：杯 + 吸管
        HotspotKind::MilkTea => {
            icon_cyl(
                p,
                mx,
                mt,
                Color::srgb(0.85, 0.72, 0.60),
                0.2,
                0.42,
                0.0,
                0.25,
                0.0,
            );
            icon_cyl(
                p,
                mx,
                mt,
                Color::srgb(0.92, 0.84, 0.74),
                0.22,
                0.06,
                0.0,
                0.5,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.85, 0.45, 0.40),
                0.05,
                0.38,
                0.05,
                0.1,
                0.55,
                0.0,
            );
        }
        // 水果摊：摊桌 + 三颗水果
        HotspotKind::FruitStand => {
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.62, 0.44, 0.28),
                0.9,
                0.08,
                0.5,
                0.0,
                0.3,
                0.0,
            );
            icon_ball(
                p,
                mx,
                mt,
                Color::srgb(0.85, 0.35, 0.30),
                0.13,
                -0.22,
                0.42,
                0.0,
            );
            icon_ball(
                p,
                mx,
                mt,
                Color::srgb(0.90, 0.75, 0.30),
                0.13,
                0.0,
                0.44,
                0.0,
            );
            icon_ball(
                p,
                mx,
                mt,
                Color::srgb(0.55, 0.78, 0.40),
                0.13,
                0.22,
                0.4,
                0.0,
            );
        }
        HotspotKind::Workstation => {
            icon_box(p, mx, mt, wood_color(), 1.4, 0.08, 0.8, 0.0, 0.35, 0.0);
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.22, 0.30, 0.45),
                0.85,
                0.5,
                0.05,
                0.0,
                0.75,
                -0.15,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.3, 0.3, 0.34),
                0.1,
                0.25,
                0.1,
                0.0,
                0.5,
                -0.15,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.3, 0.3, 0.34),
                0.5,
                0.03,
                0.22,
                0.0,
                0.4,
                -0.1,
            );
        }
        HotspotKind::Lounge => {
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.72, 0.78, 0.84),
                0.55,
                0.9,
                0.45,
                0.0,
                0.55,
                0.0,
            );
            icon_cyl(
                p,
                mx,
                mt,
                Color::srgb(0.55, 0.78, 0.90),
                0.1,
                0.6,
                0.0,
                1.2,
                0.0,
            );
        }
        HotspotKind::Slacking => {
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.75, 0.45, 0.40),
                1.3,
                0.3,
                0.55,
                0.0,
                0.3,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.75, 0.45, 0.40),
                1.3,
                0.5,
                0.15,
                0.0,
                0.65,
                -0.22,
            );
        }
        HotspotKind::Takeout => {
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.85, 0.50, 0.75),
                0.6,
                0.3,
                0.5,
                0.0,
                0.3,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.95, 0.75, 0.85),
                0.66,
                0.08,
                0.56,
                0.0,
                0.5,
                0.0,
            );
        }
        // 咖啡机：机身 + 杯
        HotspotKind::Coffee => {
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.32, 0.32, 0.36),
                0.55,
                0.85,
                0.4,
                0.0,
                0.48,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.75, 0.78, 0.82),
                0.3,
                0.14,
                0.26,
                0.0,
                0.98,
                0.0,
            );
            icon_cyl(
                p,
                mx,
                mt,
                Color::srgb(0.92, 0.88, 0.82),
                0.14,
                0.24,
                0.0,
                0.3,
                0.0,
            );
        }
        // 会议室：会议桌 + 两把椅子
        HotspotKind::Meeting => {
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.62, 0.44, 0.28),
                1.05,
                0.08,
                0.62,
                0.0,
                0.3,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.62, 0.44, 0.28),
                0.4,
                0.3,
                0.3,
                -0.75,
                0.4,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.62, 0.44, 0.28),
                0.4,
                0.3,
                0.3,
                0.75,
                0.4,
                0.0,
            );
        }
        // 打印机：机身 + 出纸口
        HotspotKind::Printer => {
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.93, 0.94, 0.95),
                0.62,
                0.5,
                0.38,
                0.0,
                0.4,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.45, 0.48, 0.52),
                0.42,
                0.08,
                0.3,
                0.0,
                0.58,
                0.0,
            );
        }
        // 地铁站：站牌 + 进站口
        HotspotKind::SubwayStop => {
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.25, 0.40, 0.65),
                0.5,
                0.08,
                0.35,
                0.0,
                0.18,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.35, 0.62, 0.95),
                0.12,
                0.7,
                0.3,
                -0.55,
                0.42,
                0.0,
            );
        }
        // 公交站：候车亭 + 顶棚
        HotspotKind::BusStop => {
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.55, 0.42, 0.25),
                0.12,
                0.9,
                0.12,
                -0.5,
                0.45,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.55, 0.42, 0.25),
                0.12,
                0.9,
                0.12,
                0.5,
                0.45,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.60, 0.55, 0.48),
                1.2,
                0.08,
                0.5,
                0.0,
                0.92,
                0.0,
            );
        }
        // 共享单车：车轮 + 车架
        HotspotKind::BikeSpot => {
            icon_cyl(
                p,
                mx,
                mt,
                Color::srgb(0.15, 0.15, 0.18),
                0.35,
                0.06,
                -0.4,
                0.4,
                0.0,
            );
            icon_cyl(
                p,
                mx,
                mt,
                Color::srgb(0.15, 0.15, 0.18),
                0.35,
                0.06,
                0.4,
                0.4,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.85, 0.55, 0.25),
                0.9,
                0.06,
                0.08,
                0.0,
                0.6,
                0.0,
            );
        }
        // 夜市：遮阳棚 + 摊台 + 烤串
        HotspotKind::NightMarket => {
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.85, 0.55, 0.35),
                1.1,
                0.06,
                0.6,
                0.0,
                1.1,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.62, 0.44, 0.28),
                0.9,
                0.08,
                0.45,
                0.0,
                0.42,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.85, 0.55, 0.30),
                0.14,
                0.22,
                0.14,
                -0.2,
                0.6,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.70, 0.80, 0.45),
                0.14,
                0.2,
                0.14,
                0.2,
                0.58,
                0.0,
            );
        }
        // 观景台：平台 + 围栏立柱
        HotspotKind::Lookout => {
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.68, 0.62, 0.54),
                1.2,
                0.14,
                0.9,
                0.0,
                0.3,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.62, 0.44, 0.28),
                0.1,
                0.35,
                0.1,
                -0.45,
                0.45,
                -0.3,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.62, 0.44, 0.28),
                0.1,
                0.35,
                0.1,
                0.45,
                0.45,
                -0.3,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.62, 0.44, 0.28),
                0.1,
                0.35,
                0.1,
                -0.45,
                0.45,
                0.3,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.62, 0.44, 0.28),
                0.1,
                0.35,
                0.1,
                0.45,
                0.45,
                0.3,
            );
        }
        // 涂鸦墙：墙面 + 彩色涂鸦
        HotspotKind::Graffiti => {
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.80, 0.78, 0.74),
                1.1,
                0.6,
                0.1,
                0.0,
                0.4,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.85, 0.45, 0.40),
                0.28,
                0.28,
                0.05,
                -0.3,
                0.5,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.40, 0.60, 0.85),
                0.28,
                0.28,
                0.05,
                0.05,
                0.4,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.90, 0.80, 0.35),
                0.28,
                0.28,
                0.05,
                0.32,
                0.55,
                0.0,
            );
        }
        // 公园长椅：座板 + 靠背 + 腿
        HotspotKind::ParkBench => {
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.62, 0.44, 0.28),
                1.2,
                0.08,
                0.5,
                0.0,
                0.3,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.62, 0.44, 0.28),
                0.08,
                0.3,
                0.08,
                -0.45,
                0.15,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.62, 0.44, 0.28),
                0.08,
                0.3,
                0.08,
                0.45,
                0.15,
                0.0,
            );
            icon_box(
                p,
                mx,
                mt,
                Color::srgb(0.62, 0.44, 0.28),
                1.2,
                0.32,
                0.08,
                0.0,
                0.55,
                -0.24,
            );
        }
        // 公园喷泉：水池 + 水柱
        HotspotKind::ParkFountain => {
            icon_cyl(
                p,
                mx,
                mt,
                Color::srgb(0.55, 0.62, 0.68),
                0.8,
                0.1,
                0.0,
                0.15,
                0.0,
            );
            icon_cyl(
                p,
                mx,
                mt,
                Color::srgb(0.45, 0.70, 0.85),
                0.12,
                0.9,
                0.0,
                0.55,
                0.0,
            );
        }
    }
}

// art.rs 的颜色常量复用到图标（避免额外导入依赖循环，这里直接定义）
fn wood_color() -> Color {
    Color::srgb(0.62, 0.44, 0.28)
}
