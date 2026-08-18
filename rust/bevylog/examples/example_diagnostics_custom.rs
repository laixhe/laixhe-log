//! Bevy 0.19 入门示例：演示自定义诊断指标。
//!
//! 学习重点：
//! - register_diagnostic 注册自定义诊断指标（带单位后缀）
//! - Diagnostics 系统参数添加测量值（add_measurement）
//! - DiagnosticsStore 读取自定义指标
//!
//! 观察：每秒打印一次自定义的「敌人数量」指标。

use bevy::diagnostic::{
    Diagnostic, DiagnosticPath, Diagnostics, DiagnosticsStore, RegisterDiagnostic,
};
use bevy::prelude::*;

// 自定义诊断路径
const ENEMY_COUNT: DiagnosticPath = DiagnosticPath::const_new("enemy_count");

#[derive(Component)]
struct Enemy;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .register_diagnostic(Diagnostic::new(ENEMY_COUNT).with_suffix("个"))
        .add_systems(Startup, setup)
        .add_systems(Update, (measure_enemy_count, report_enemy_count))
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // 生成 50 个敌人
    for _ in 0..50 {
        commands.spawn(Enemy);
    }
    info!("[诊断] 生成了 50 个敌人");
}

// 每帧测量敌人数量，写入自定义诊断指标
fn measure_enemy_count(mut diagnostics: Diagnostics, enemies: Query<(), With<Enemy>>) {
    let count = enemies.iter().count();
    diagnostics.add_measurement(&ENEMY_COUNT, || count as f64);
}

// 每秒读取并打印自定义指标
fn report_enemy_count(
    time: Res<Time>,
    diagnostics: Res<DiagnosticsStore>,
    mut last_log: Local<f32>,
) {
    if time.elapsed_secs() - *last_log < 1.0 {
        return;
    }
    *last_log = time.elapsed_secs();

    if let Some(d) = diagnostics.get(&ENEMY_COUNT) {
        info!("[诊断] 敌人数量: {:.0}", d.value().unwrap_or(0.0));
    }
}
