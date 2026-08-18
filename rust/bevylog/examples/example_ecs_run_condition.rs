//! Bevy 0.19 入门示例：演示运行条件（run_if）。
//!
//! run_if 给系统加一个「是否执行」的判断：条件为 true 才运行，false 直接跳过。
//! 相比在系统内部写 if 判断，run_if 让意图更清晰，也能让调度器提前知道系统不会运行。
//!
//! 学习重点：
//! - run_if(自定义函数)：用返回 bool 的系统函数作为条件
//! - run_if(resource_changed::<T>())：只在资源被修改的那帧运行
//! - run_if(in_state(...))：只在某个状态下运行（见 example_ecs_state）
//! - .or_else() / .and()：组合多个条件

use bevy::{prelude::*, text::FontSourceTemplate};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

#[derive(Component)]
struct Player;

// 游戏开关：控制「是否允许玩家移动」
#[derive(Resource)]
struct GameEnabled(bool);

// 分数：演示 resource_changed 只在分数变化时刷新
#[derive(Resource, Default)]
struct Score(u32);

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .insert_resource(GameEnabled(true))
        .init_resource::<Score>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                // handle_input 先运行（写 GameEnabled / Score），
                // 再用 .chain() 保证后面的条件能读到本帧的最新值。
                handle_input,
                // 自定义条件：只有 GameEnabled 为 true 时才移动
                move_player.run_if(game_enabled),
                // 内置条件：只在 Score 被修改的那帧刷新
                refresh_score.run_if(resource_changed::<Score>),
            )
                .chain(),
        )
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Player,
        Mesh2d(meshes.add(Circle::new(30.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.2, 0.8, 0.4)))),
        Transform::from_xyz(-200.0, 0.0, 0.0),
    ));

    commands.spawn_scene(bsn! {
        Text2d::new("空格：开关移动  |  S：加分（触发 resource_changed）")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(24.0),
        }
        Transform::from_xyz(0.0, -260.0, 0.0)
    });
}

// 自定义运行条件：返回 bool 的系统函数。返回 false 时 move_player 整帧跳过。
fn game_enabled(enabled: Res<GameEnabled>) -> bool {
    enabled.0
}

// 移动玩家：只在 game_enabled 条件为 true 时运行
fn move_player(time: Res<Time>, mut player: Single<&mut Transform, With<Player>>) {
    player.translation.y = (time.elapsed_secs() * 2.0).sin() * 120.0;
}

// 刷新分数：只在 Score 资源被修改的那帧运行（平时完全跳过）
fn refresh_score(score: Res<Score>) {
    info!("[条件] 分数变化：{}", score.0);
}

// 输入：空格切换游戏开关，S 加分
fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut enabled: ResMut<GameEnabled>,
    mut score: ResMut<Score>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        enabled.0 = !enabled.0;
        info!("[条件] 游戏{}", if enabled.0 { "开启" } else { "关闭" });
    }
    if keyboard.just_pressed(KeyCode::KeyS) {
        score.0 += 1;
    }
}
