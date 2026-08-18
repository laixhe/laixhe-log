//! 低多边形殖民模拟（Bevy 0.19 综合示例）
//!
//! 玩法一句话：在程序化生成的地形上，指挥殖民者采集资源、建造房屋与工坊，
//! 安排工作，让聚落自动运转起来。
//!
//! 运行方式见本目录 README.md（`cargo run --example simple_mini_3d_tree_low_poly`）。
//!
//! # 模块总览
//!
//! | 模块 | 职责 |
//! | --- | --- |
//! | `types` | 游戏数据定义：资源/建筑/施工种类、建筑属性表、网格坐标换算 |
//! | `math` | 数学工具：射线求交地形（步进 + 二分）、xz 平面距离/边界 |
//! | `terrain` | 程序化地形：分形值噪声生成高度与植被分类、资源点分布 |
//! | `simulation` | 模拟时钟：暂停 / 速度倍率（空格键） |
//! | `camera` | 轨道相机：右键旋转 / 中键平移 / 滚轮缩放 |
//! | `world` | 世界初始化：材质与网格资源、地形网格、资源节点、中央仓库、初始殖民者 |
//! | `resources` | 资源与库存：Inventory 容量规则、公共库存汇总 |
//! | `navigation` | 寻路系统：分块网格 + 双向 A* + 异步路径规划 + 路径缓存 |
//! | `colonist` | 殖民者 AI：需求（饥饿）、住房/岗位分配、寻路、采集、搬运、建造 |
//! | `building` | 建造系统：输入 → 预览 → 放置 → 蓝图生命周期（含农场） |
//! | `farm` | 农场：角点多边形校验、覆盖层网格、作物排布 |
//! | `selection` | 点选系统：鼠标拾取地面 + 优先命中，Gizmos 高亮 |
//! | `ui` | HUD：资源栏、建造按钮、选中面板、岗位调节、FPS |
//! | `debug_console` | 调试台：加资源/加人、快速建造、线框开关（`` ` `` 打开） |
//!
//! 每个系统的大致数据流：
//! `terrain` 生成地图 → `world` 搭场景 → `colonist` + `navigation` 驱动角色行动 →
//! `building` 处理玩家的建造操作 → `resources` 统一管理物资 → `ui` 反馈状态。

mod building;
mod camera;
mod colonist;
mod debug_console;
mod farm;
mod math;
mod navigation;
mod resources;
mod selection;
mod simulation;
mod terrain;
mod types;
mod ui;
mod world;

use bevy::pbr::wireframe::WireframePlugin;
use bevy::prelude::*;
use bevy::render::RenderDebugFlags;

fn main() {
    // 命令行参数：--hide-ui 隐藏 HUD，--wireframe 以线框模式启动（便于观察网格结构）。
    let hide_ui = std::env::args().any(|arg| arg == "--hide-ui");
    let wireframe = std::env::args().any(|arg| arg == "--wireframe");

    let mut app = App::new();

    app.insert_resource(ClearColor(Color::srgb(0.76, 0.8, 0.86)));

    if hide_ui {
        app.insert_resource(ui::UiVisibility { visible: false });
    }
    if wireframe {
        app.insert_resource(debug_console::DebugConsoleState {
            wireframe_mode: true,
            ..default()
        });
    }

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        // 窗口标题：默认会用示例名，这里改成中文。
        primary_window: Some(Window {
            title: "低多边形殖民模拟".to_string(),
            ..default()
        }),
        ..default()
    }))
    // 线框渲染插件：0.19 里线框由 RenderDebugFlags 控制，
    // 用空标志初始化后，运行时通过调试台 / --wireframe 再打开。
    .add_plugins(WireframePlugin {
        debug_flags: RenderDebugFlags::empty(),
    })
    // FPS 统计，用于 UI 右上角显示。
    .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
    // 各业务插件。顺序无严格要求（插件内部自己保证系统链式顺序）：
    // - simulation: 游戏时钟（空格暂停）
    // - camera:     轨道相机控制
    // - terrain:    注册地形种子与生成配置（种子即世界随机数根）
    // - world:      Startup 阶段搭建整个场景
    // - building:   建造输入 → 预览 → 放置 → 蓝图生命周期
    // - selection:  鼠标点选与高亮
    // - colonist:   殖民者 AI + 寻路（内部注册 NavGrid / PathPlanner）
    // - ui:         HUD 界面
    // - debug_console: 调试台（`` ` `` 开关）
    .add_plugins((
        simulation::SimulationPlugin,
        camera::CameraPlugin,
        terrain::TerrainPlugin,
        world::WorldPlugin,
        building::BuildingPlugin,
        selection::SelectionPlugin,
        colonist::ColonistPlugin,
        ui::UiPlugin,
        debug_console::DebugConsolePlugin,
    ))
    .run();
}
