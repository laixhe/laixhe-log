//! Bevy 0.19 组合学习示例：UI 综合应用「待办清单」。
//! Todo → Settings 双页应用：添加 / 删除 / 选中待办事项，设置页调节音效 / 音量 / 主题色，
//! 支持 serde + ron 存档 / 读档（待办列表与设置一起保存）。
//!
//! 学习重点（组合了 UI 阶段的多个核心概念）：
//! - 布局：Flexbox（Column/Row + justify/align + row_gap/column_gap）+ 盒模型（padding/border）
//! - 控件：TextInput 文本输入、Button 按钮、Checkbox 复选框、Slider 滑块、
//!   RadioGroup 单选组、ListBox 列表框、ScrollArea 滚动区域
//! - 控件事件：ValueChange<T> 事件 + observer 外部状态管理（视觉由 app 自己维护）
//! - 状态机：AppPage（Todo / Settings）页面切换，OnEnter / OnExit 整页清理
//! - 数据与渲染分离：AppData（Resource）是唯一数据源，列表 UI 由脏标记驱动重建
//! - 序列化：serde derive + ron::to_string / from_str 存档 / 读档
//!
//! 操作：输入内容回车 / 点击「添加」新增 | 点击列表选中，可「删除选中」
//!       | 「保存」/「读档」或 S / L 快捷键 | 「设置」页调音效 / 音量 / 主题色。

use bevy::prelude::*;
use bevy::ui_widgets::checkbox_self_update;

mod pages;
mod state;
use pages::router::AppPage;

// === 主函数：应用初始化 ===
fn main() -> AppExit {
    App::new()
        // 配置窗口标题
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "待办清单 Todo App".to_string(),
                ..default()
            }),
            ..default()
        }))
        // 页面状态机（默认进入 Todo 页）
        .init_state::<AppPage>()
        // 全局资源：应用数据 / 存档槽 / 列表脏标记 / 选中索引
        .init_resource::<state::AppData>()
        .init_resource::<state::SaveSlot>()
        .init_resource::<state::ListDirty>()
        .init_resource::<state::SelectedIndex>()
        // 启动时执行一次：生成 UI 相机
        .add_systems(Startup, setup_camera)
        // 控件自带 observer：Checkbox 自动维护 Checked 组件
        .add_observer(checkbox_self_update)
        // 业务 observer：监听控件值变化，更新 AppData + 视觉
        .add_observer(pages::todo::on_list_change)
        .add_observer(pages::settings::on_checkbox_change)
        .add_observer(pages::settings::on_slider_change)
        .add_observer(pages::settings::on_radio_change)
        // === Todo 页（清单） ===
        .add_systems(
            OnEnter(AppPage::Todo),
            (pages::todo::setup_todo, log_state_enter),
        )
        .add_systems(OnExit(AppPage::Todo), pages::todo::cleanup_todo)
        .add_systems(
            Update,
            (
                pages::todo::rebuild_list_system,
                pages::todo::on_add,
                pages::todo::on_delete,
                pages::todo::on_save,
                pages::todo::on_load,
                pages::todo::on_settings,
            )
                .chain()
                .run_if(in_state(AppPage::Todo)),
        )
        // === Settings 页（设置） ===
        .add_systems(
            OnEnter(AppPage::Settings),
            (pages::settings::setup_settings, log_state_enter),
        )
        .add_systems(OnExit(AppPage::Settings), pages::settings::cleanup_settings)
        .add_systems(
            Update,
            pages::settings::on_back.run_if(in_state(AppPage::Settings)),
        )
        .run()
}

// === 相机 ===
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

// === 状态切换日志 ===
fn log_state_enter(state: Res<State<AppPage>>) {
    info!("[状态] 进入 {:?}", state.get());
}
