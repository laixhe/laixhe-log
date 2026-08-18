use bevy::prelude::*;

use crate::pages::router::GameState;

// 加载画面标记组件（退出 Loading 时用于清理）
#[derive(Component, Clone, Default)]
pub struct Loading;

// 加载画面计时器：短暂停留后自动进入主菜单
#[derive(Resource, Deref, DerefMut)]
pub struct LoadingTimer(Timer);

pub fn setup_loading(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        (
            Text("loading...")
            TextFont {
                font_size: FontSize::Px(60.0),
            }
            TextColor(Color::srgb(0.2, 0.8, 0.2))
            Loading
        )
    });
    // 短暂停留后进入菜单（顺带给中文字体一点懒加载时间）
    commands.insert_resource(LoadingTimer(Timer::from_seconds(0.8, TimerMode::Once)));
}

pub fn cleanup_loading(mut commands: Commands, query: Query<Entity, With<Loading>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

/// 加载画面倒计时：计时结束后进入主菜单
pub fn countdown(
    time: Res<Time>,
    mut timer: ResMut<LoadingTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if timer.tick(time.delta()).is_finished() {
        next_state.set(GameState::Menu);
    }
}
