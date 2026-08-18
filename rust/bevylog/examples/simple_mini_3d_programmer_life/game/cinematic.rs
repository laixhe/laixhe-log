//! 章节剧情演出：章节切换/关键节点时播放「红匾大字报」过场——
//! 全屏遮罩 + 中央红匾 + 大标题，持续数秒后自动消失；
//! 播放期间锁操作（见 movement::handle_click）。

use bevy::prelude::*;
use bevy::text::FontSource;

use super::components::*;
use super::resources::*;

#[derive(Component)]
pub struct CinematicRoot;

// ==================== 过场 UI ====================
pub fn cinematic_ui(
    time: Res<Time>,
    mut cinematic: ResMut<Cinematic>,
    mut commands: Commands,
    assets: Res<AssetServer>,
    roots: Query<Entity, With<CinematicRoot>>,
    // Local 记住「过场 UI 是否已生成」：过场开始后只生成一次，结束后清理一次，
    // 避免每帧重复 spawn / despawn（对比 modal.rs 用 Local 缓存弹窗版本号是同一思路）。
    mut spawned: Local<bool>,
) {
    if cinematic.active {
        cinematic.t += time.delta_secs();
        if cinematic.t >= cinematic.duration {
            cinematic.active = false;
        } else if !*spawned {
            *spawned = true;
            debug!("[演出] 过场开始：{} / {}", cinematic.title, cinematic.sub);
            spawn_overlay(&mut commands, &assets, &cinematic);
        }
    } else if *spawned {
        *spawned = false;
        let old: Vec<Entity> = roots.iter().collect();
        // 过场 overlay 与子实体都带 CinematicRoot，try_despawn 容忍随父级连带删除的重复删除
        for e in old {
            commands.entity(e).try_despawn();
        }
    }
}

fn spawn_overlay(commands: &mut Commands, assets: &Res<AssetServer>, cinematic: &Cinematic) {
    let font = |size: f32| TextFont {
        font: FontSource::Handle(assets.load(FONT_PATH)),
        font_size: FontSize::Px(size),
        ..default()
    };
    let red = Color::srgb(0.72, 0.15, 0.12);
    let gold = Color::srgb(0.93, 0.78, 0.45);
    commands
        .spawn((
            // 同时挂 GameRoot：若过场未播完就触发结局/退出 Playing，随清理一并移除，
            // 避免残留在结局页挡住按钮（FocusPolicy::Block）。
            GameRoot,
            CinematicRoot,
            Node {
                width: percent(100),
                height: percent(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.10, 0.08, 0.05, 0.45)),
            bevy::ui::FocusPolicy::Block,
        ))
        .with_children(|root| {
            root.spawn((
                CinematicRoot,
                Node {
                    width: percent(64),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: px(10),
                    padding: UiRect::axes(px(36), px(24)),
                    border: UiRect::all(px(4)),
                    border_radius: BorderRadius::all(px(10)),
                    ..default()
                },
                BorderColor::all(gold),
                BackgroundColor(red),
            ))
            .with_children(|plaque| {
                plaque.spawn((
                    CinematicRoot,
                    Text::new(cinematic.title.clone()),
                    TextColor(Color::srgb(1.0, 0.93, 0.72)),
                    font(40.0),
                ));
                if !cinematic.sub.is_empty() {
                    plaque.spawn((
                        CinematicRoot,
                        Text::new(cinematic.sub.clone()),
                        TextColor(Color::srgb(1.0, 0.97, 0.88)),
                        font(22.0),
                    ));
                }
            });
        });
}
