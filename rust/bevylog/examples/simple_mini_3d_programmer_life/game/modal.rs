//! 弹窗系统：公司列表 / 笔试答题 / 对话树 / 随机事件 / 通勤
//! 五种弹窗共用一套骨架。modal_ui 按 Modal.kind 变化时重建内容，
//! handle_modal_buttons 统一处理各弹窗的按钮点击。

use bevy::prelude::*;
use bevy::text::FontSource;

use crate::router::GameState;

use super::components::*;
use super::job::{self, COMPANIES, JobPipeline, TIER_NAMES};
use super::npc::{DlgEffect, NPCS, QUIZ};
use super::progression;
use super::resources::*;

// ==================== 弹窗总控 ====================
// 监听 Modal（kind + version）变化：变化时清掉旧弹窗 UI，按类型重建。
#[allow(clippy::too_many_arguments)]
pub fn modal_ui(
    mut modal: ResMut<Modal>,
    dialog: Res<DialogueState>,
    quiz: Res<QuizState>,
    pipeline: Res<JobPipeline>,
    mut event: ResMut<EventState>,
    bank: Res<super::sfx::SoundBank>,
    mut commands: Commands,
    assets: Res<AssetServer>,
    roots: Query<
        Entity,
        Or<(
            With<DlgRoot>,
            With<QuizRoot>,
            With<CompanyRoot>,
            With<CommuteRoot>,
            With<EventRoot>,
        )>,
    >,
    // Local 是"系统私有的持久变量"（函数退出不销毁，下次运行还在）。
    // 这里用它记住上次渲染的 (弹窗类型, 版本号)：两者都没变就直接返回，
    // 保证弹窗只在打开/切换时重建一次，不会每帧重复创建。
    mut local: Local<(Option<ModalKind>, u32)>,
) {
    if local.0 == modal.kind && local.1 == modal.version {
        return;
    }
    local.0 = modal.kind;
    local.1 = modal.version;

    let old: Vec<Entity> = roots.iter().collect();
    // try_despawn：弹窗的父/子实体可能带同一 Root 标记，despawn 父时会连带删掉子，
    // 此时再删子就会报 "Entity despawned"。try_despawn 对已随父级删除的实体静默跳过。
    for e in old {
        commands.entity(e).try_despawn();
    }

    debug!(
        "[弹窗] 重建：kind={:?} version={}",
        modal.kind, modal.version
    );
    match modal.kind {
        Some(ModalKind::Dialogue) => spawn_dialogue(&mut commands, &assets, &dialog),
        Some(ModalKind::Quiz) => spawn_quiz(&mut commands, &assets, &quiz),
        Some(ModalKind::Commute) => spawn_transit_panel(&mut commands, &assets),
        Some(ModalKind::Company) => spawn_company(&mut commands, &assets, &pipeline),
        Some(ModalKind::Event) => {
            super::sfx::play(&mut commands, &bank, super::sfx::Sfx::Alert);
            spawn_event(&mut commands, &assets, &event);
        }
        None => {
            // 晚间事件被弹窗顶替（EventState.pending）时，弹窗关闭后再触发
            if event.pending {
                event.pending = false;
                super::progression::roll_evening_event(&mut modal, &mut event);
            }
        }
    }
}

// ==================== 通用样式 ====================
fn font(assets: &Res<AssetServer>, size: f32) -> TextFont {
    TextFont {
        font: FontSource::Handle(assets.load(FONT_PATH)),
        font_size: FontSize::Px(size),
        ..default()
    }
}

const GOLD: Color = Color::srgb(0.83, 0.62, 0.22);
const PANEL_BG: Color = Color::srgb(0.97, 0.93, 0.84);
const INK: Color = Color::srgb(0.25, 0.2, 0.15);

// ==================== 对话弹窗 ====================
fn spawn_dialogue(commands: &mut Commands, assets: &Res<AssetServer>, state: &DialogueState) {
    // 越界防护：对话树是纯数据，下标由状态机维护，异常时直接不渲染（避免 panic）
    let Some(npc) = NPCS.get(state.npc) else {
        warn!("[弹窗] 对话 NPC 下标越界：{}", state.npc);
        return;
    };
    let Some(node) = npc.nodes.get(state.node) else {
        warn!("[弹窗] 对话节点下标越界：{}", state.node);
        return;
    };

    commands
        .spawn((
            GameRoot,
            DlgRoot,
            Node {
                width: percent(100),
                height: percent(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.15, 0.12, 0.08, 0.5)),
            bevy::ui::FocusPolicy::Block,
        ))
        .with_children(|overlay| {
            overlay
                .spawn((
                    GameRoot,
                    DlgRoot,
                    Node {
                        width: percent(88),
                        max_height: percent(62),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(10),
                        padding: UiRect::all(px(16)),
                        border: UiRect::all(px(3)),
                        border_radius: BorderRadius::all(px(12)),
                        ..default()
                    },
                    BorderColor::all(GOLD),
                    BackgroundColor(PANEL_BG),
                ))
                .with_children(|panel| {
                    // 姓名牌 + 标签
                    panel
                        .spawn((
                            GameRoot,
                            Node {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                column_gap: px(10),
                                ..default()
                            },
                        ))
                        .with_children(|head| {
                            head.spawn((
                                GameRoot,
                                Node {
                                    width: px(14),
                                    height: px(14),
                                    border_radius: BorderRadius::MAX,
                                    ..default()
                                },
                                BackgroundColor(npc.color),
                            ));
                            head.spawn((
                                GameRoot,
                                Text::new(format!("{} · {}", npc.name, npc.tag)),
                                TextColor(Color::srgb(0.72, 0.15, 0.12)),
                                font(assets, 22.0),
                            ));
                        });
                    // 对话内容
                    panel.spawn((
                        GameRoot,
                        Text::new(node.text),
                        TextColor(INK),
                        font(assets, 19.0),
                    ));
                    // 选项按钮
                    for (i, opt) in node.options.iter().enumerate() {
                        panel
                            .spawn((
                                GameRoot,
                                OptionButton(i),
                                Button,
                                Node {
                                    width: percent(100),
                                    padding: UiRect::new(px(14), px(14), px(8), px(8)),
                                    border_radius: BorderRadius::all(px(8)),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.93, 0.86, 0.74)),
                                BorderColor::all(Color::srgb(0.75, 0.62, 0.45)),
                            ))
                            .with_children(|btn| {
                                btn.spawn((
                                    GameRoot,
                                    Text::new(opt.label),
                                    TextColor(Color::srgb(0.3, 0.22, 0.12)),
                                    font(assets, 18.0),
                                ));
                            });
                    }
                });
        });
}

// ==================== 笔试弹窗 ====================
fn spawn_quiz(commands: &mut Commands, assets: &Res<AssetServer>, state: &QuizState) {
    // 越界防护：与 spawn_event 一致，题库下标异常时跳过渲染
    let Some(q) = QUIZ.get(state.q) else {
        warn!("[弹窗] 笔试题库下标越界：{}", state.q);
        return;
    };

    commands
        .spawn((
            GameRoot,
            QuizRoot,
            Node {
                width: percent(100),
                height: percent(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.15, 0.12, 0.08, 0.5)),
            bevy::ui::FocusPolicy::Block,
        ))
        .with_children(|overlay| {
            overlay
                .spawn((
                    GameRoot,
                    QuizRoot,
                    Node {
                        width: percent(82),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(12),
                        padding: UiRect::all(px(18)),
                        border: UiRect::all(px(3)),
                        border_radius: BorderRadius::all(px(12)),
                        ..default()
                    },
                    BorderColor::all(GOLD),
                    BackgroundColor(PANEL_BG),
                ))
                .with_children(|panel| {
                    panel.spawn((
                        GameRoot,
                        Text::new("📝 笔试答题（答对 +25% 通过率，答错 -15%）"),
                        TextColor(Color::srgb(0.72, 0.15, 0.12)),
                        font(assets, 22.0),
                    ));
                    panel.spawn((GameRoot, Text::new(q.q), TextColor(INK), font(assets, 20.0)));
                    for (i, opt) in q.opts.iter().enumerate() {
                        panel
                            .spawn((
                                GameRoot,
                                QuizOption(i),
                                Button,
                                Node {
                                    width: percent(100),
                                    padding: UiRect::new(px(14), px(14), px(9), px(9)),
                                    border_radius: BorderRadius::all(px(8)),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.90, 0.84, 0.70)),
                                BorderColor::all(Color::srgb(0.72, 0.58, 0.40)),
                            ))
                            .with_children(|btn| {
                                btn.spawn((
                                    GameRoot,
                                    Text::new(*opt),
                                    TextColor(Color::srgb(0.28, 0.2, 0.12)),
                                    font(assets, 18.0),
                                ));
                            });
                    }
                    // 放弃笔试：视为未通过（被拒），给玩家一条不答题的退出路径
                    panel
                        .spawn((
                            GameRoot,
                            QuizGiveUp,
                            Button,
                            Node {
                                width: percent(100),
                                justify_content: JustifyContent::Center,
                                padding: UiRect::new(px(14), px(14), px(6), px(6)),
                                border_radius: BorderRadius::all(px(8)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.88, 0.82, 0.68)),
                            BorderColor::all(Color::srgb(0.75, 0.62, 0.45)),
                        ))
                        .with_children(|btn| {
                            btn.spawn((
                                GameRoot,
                                Text::new("放弃笔试（视为未通过）"),
                                TextColor(Color::srgb(0.4, 0.3, 0.18)),
                                font(assets, 17.0),
                            ));
                        });
                });
        });
}

// ==================== 交通面板 ====================
// 在交通站点交互后打开：选目的地区域，随后自动乘车（地铁/公交）前往。
// 说明：本弹窗用「命令式」commands.spawn + .with_children 编写。
// Bevy 0.19 也提供声明式场景宏 bsn!，但它要求组件实现 FromTemplate，
// 而本示例的文本都带「运行时字体句柄」（TextFont + FontSource::Handle），
// 其模板类型不接受运行时值，无法用 bsn! 表达，因此这里沿用命令式写法。
fn spawn_transit_panel(commands: &mut Commands, assets: &Res<AssetServer>) {
    // 乘车目的地：新增 Location 变体时必须在此添加乘车入口（此处是软编码，无编译期检查）
    let targets = [
        (Location::Home, "🏠 家"),
        (Location::Campus, "🏫 校园"),
        (Location::Cafeteria, "🍚 食堂"),
        (Location::Office, "🏢 公司"),
        (Location::Park, "🌳 公园"),
    ];
    commands
        .spawn((
            GameRoot,
            CommuteRoot,
            Node {
                width: percent(100),
                height: percent(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.15, 0.12, 0.08, 0.5)),
            bevy::ui::FocusPolicy::Block,
        ))
        .with_children(|overlay| {
            overlay
                .spawn((
                    GameRoot,
                    CommuteRoot,
                    Node {
                        width: percent(72),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(12),
                        padding: UiRect::all(px(18)),
                        border: UiRect::all(px(3)),
                        border_radius: BorderRadius::all(px(12)),
                        ..default()
                    },
                    BorderColor::all(GOLD),
                    BackgroundColor(PANEL_BG),
                ))
                .with_children(|panel| {
                    panel.spawn((
                        GameRoot,
                        Text::new("🚉 坐车去哪儿？"),
                        TextColor(Color::srgb(0.72, 0.15, 0.12)),
                        font(assets, 24.0),
                    ));
                    for (loc, label) in targets {
                        panel
                            .spawn((
                                GameRoot,
                                CommuteButton(loc),
                                Button,
                                Node {
                                    width: percent(100),
                                    padding: UiRect::new(px(14), px(14), px(10), px(10)),
                                    border_radius: BorderRadius::all(px(8)),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.88, 0.90, 0.85)),
                                BorderColor::all(Color::srgb(0.6, 0.62, 0.5)),
                            ))
                            .with_children(|btn| {
                                btn.spawn((
                                    GameRoot,
                                    Text::new(label),
                                    TextColor(Color::srgb(0.25, 0.25, 0.2)),
                                    font(assets, 20.0),
                                ));
                            });
                    }
                });
        });
}

// ==================== 随机事件弹窗 ====================
fn spawn_event(commands: &mut Commands, assets: &Res<AssetServer>, state: &EventState) {
    let Some(ev) = progression::EVENTS.get(state.idx) else {
        return;
    };
    commands
        .spawn((
            GameRoot,
            EventRoot,
            Node {
                width: percent(100),
                height: percent(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.15, 0.12, 0.08, 0.55)),
            bevy::ui::FocusPolicy::Block,
        ))
        .with_children(|overlay| {
            overlay
                .spawn((
                    GameRoot,
                    EventRoot,
                    Node {
                        width: percent(66),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(10),
                        padding: UiRect::all(px(18)),
                        border: UiRect::all(px(3)),
                        border_radius: BorderRadius::all(px(12)),
                        ..default()
                    },
                    BorderColor::all(GOLD),
                    BackgroundColor(PANEL_BG),
                ))
                .with_children(|panel| {
                    panel.spawn((
                        GameRoot,
                        Text::new(format!("🌙 {}", ev.title)),
                        TextColor(Color::srgb(0.72, 0.15, 0.12)),
                        font(assets, 24.0),
                    ));
                    panel.spawn((
                        GameRoot,
                        Text::new(ev.desc),
                        TextColor(INK),
                        font(assets, 18.0),
                    ));
                    for (i, choice) in ev.choices.iter().enumerate() {
                        panel
                            .spawn((
                                GameRoot,
                                EventOption(i),
                                Button,
                                Node {
                                    width: percent(100),
                                    padding: UiRect::new(px(12), px(12), px(8), px(8)),
                                    border_radius: BorderRadius::all(px(8)),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.92, 0.86, 0.72)),
                                BorderColor::all(GOLD),
                            ))
                            .with_children(|btn| {
                                btn.spawn((
                                    GameRoot,
                                    Text::new(choice.label),
                                    TextColor(INK),
                                    font(assets, 18.0),
                                ));
                            });
                    }
                });
        });
}

// ==================== 投简历面板 ====================
fn spawn_company(commands: &mut Commands, assets: &Res<AssetServer>, pipeline: &JobPipeline) {
    commands
        .spawn((
            GameRoot,
            CompanyRoot,
            Node {
                width: percent(100),
                height: percent(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.15, 0.12, 0.08, 0.5)),
            bevy::ui::FocusPolicy::Block,
        ))
        .with_children(|overlay| {
            overlay
                .spawn((
                    GameRoot,
                    CompanyRoot,
                    Node {
                        width: percent(88),
                        height: percent(88),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(6),
                        padding: UiRect::all(px(16)),
                        border: UiRect::all(px(3)),
                        border_radius: BorderRadius::all(px(12)),
                        ..default()
                    },
                    BorderColor::all(GOLD),
                    BackgroundColor(PANEL_BG),
                ))
                .with_children(|panel| {
                    panel.spawn((
                        GameRoot,
                        Text::new("💻 投简历 · 找工作（大厂钱多难进，小厂稳；笔试答对有加成）"),
                        TextColor(Color::srgb(0.72, 0.15, 0.12)),
                        font(assets, 22.0),
                    ));
                    for (i, comp) in COMPANIES.iter().enumerate() {
                        let applied = pipeline.applied_to(i);
                        panel
                            .spawn((
                                GameRoot,
                                CompanyRow(i),
                                Node {
                                    width: percent(100),
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::SpaceBetween,
                                    padding: UiRect::new(px(8), px(8), px(5), px(5)),
                                    border: UiRect::bottom(px(1)),
                                    ..default()
                                },
                                BorderColor::all(Color::srgb(0.85, 0.78, 0.64)),
                            ))
                            .with_children(|row| {
                                row.spawn((
                                    GameRoot,
                                    Text::new(format!(
                                        "{} · {} · 日薪 ¥{:.0}",
                                        comp.name, TIER_NAMES[comp.tier as usize], comp.salary
                                    )),
                                    TextColor(Color::srgb(0.3, 0.24, 0.16)),
                                    font(assets, 17.0),
                                ));
                                if let Some(app_idx) = applied {
                                    row.spawn((
                                        GameRoot,
                                        Text::new(pipeline.apps[app_idx].status_text()),
                                        TextColor(Color::srgb(0.55, 0.45, 0.32)),
                                        font(assets, 16.0),
                                    ));
                                } else {
                                    row.spawn((
                                        GameRoot,
                                        ApplyButton(i),
                                        Button,
                                        Node {
                                            padding: UiRect::new(px(14), px(14), px(5), px(5)),
                                            border_radius: BorderRadius::MAX,
                                            ..default()
                                        },
                                        BackgroundColor(Color::srgb(0.72, 0.15, 0.12)),
                                    ))
                                    .with_children(|btn| {
                                        btn.spawn((
                                            GameRoot,
                                            Text::new("投递"),
                                            TextColor(Color::WHITE),
                                            font(assets, 16.0),
                                        ));
                                    });
                                }
                            });
                    }
                    // 关闭按钮
                    panel
                        .spawn((
                            GameRoot,
                            ClosePanelButton,
                            Button,
                            Node {
                                width: percent(100),
                                justify_content: JustifyContent::Center,
                                padding: UiRect::new(px(14), px(14), px(8), px(8)),
                                border_radius: BorderRadius::all(px(8)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.88, 0.82, 0.68)),
                            BorderColor::all(GOLD),
                        ))
                        .with_children(|btn| {
                            btn.spawn((
                                GameRoot,
                                Text::new("关闭面板"),
                                TextColor(Color::srgb(0.4, 0.3, 0.18)),
                                font(assets, 18.0),
                            ));
                        });
                });
        });
}

// ==================== 弹窗按钮处理 ====================
#[allow(clippy::too_many_arguments)]
pub fn handle_modal_buttons(
    mut options: Query<(&Interaction, &OptionButton), Changed<Interaction>>,
    mut commute_btns: Query<(&Interaction, &CommuteButton), Changed<Interaction>>,
    mut apply_btns: Query<(&Interaction, &ApplyButton), Changed<Interaction>>,
    mut close_btns: Query<(&Interaction, &ClosePanelButton), Changed<Interaction>>,
    mut dialog: ResMut<DialogueState>,
    mut modal: ResMut<Modal>,
    mut pipeline: ResMut<JobPipeline>,
    mut stats: ResMut<PlayerStats>,
    mut flags: ResMut<GameFlags>,
    mut toast: ResMut<ToastLog>,
    clock: Res<GameClock>,
    mut settle: ResMut<DialogSettle>,
    mut transit: ResMut<TransitState>,
) {
    // 对话选项（每天限一次效果：先按天重置结算记录）
    if settle.day != clock.day {
        settle.day = clock.day;
        settle.npcs.clear();
    }
    for (inter, opt) in &mut options {
        if *inter == Interaction::Pressed {
            dialog_navigate(&mut dialog, &mut modal, &mut stats, &mut toast, &mut settle, opt.0);
            break;
        }
    }
    // 交通面板：选目的地区域 → 乘车前往
    for (inter, target) in &mut commute_btns {
        if *inter == Interaction::Pressed {
            apply_commute(target.0, &mut transit, &mut toast);
            modal.close();
            break;
        }
    }
    // 投递
    for (inter, btn) in &mut apply_btns {
        if *inter == Interaction::Pressed {
            job::apply_to(btn.0, &mut pipeline, &mut flags, &mut toast, &mut stats);
            modal.refresh();
            break;
        }
    }
    // 关闭
    for (inter, _) in &mut close_btns {
        if *inter == Interaction::Pressed {
            modal.close();
            break;
        }
    }
}

// 随机事件选项（独立系统，避免主按钮系统参数超限）
pub fn handle_event_buttons(
    mut event_opts: Query<(&Interaction, &EventOption), Changed<Interaction>>,
    mut modal: ResMut<Modal>,
    event_state: Res<EventState>,
    mut stats: ResMut<PlayerStats>,
    mut toast: ResMut<ToastLog>,
    mut over: ResMut<OverInfo>,
    mut ending: ResMut<Ending>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (inter, opt) in event_opts.iter_mut() {
        if *inter == Interaction::Pressed {
            progression::apply_event_choice(event_state.idx, opt.0, &mut stats, &mut toast);
            modal.close();
            // 事件选项可能扣心态/扣钱到触发结局（如「抽歪了心态崩了」），立即判定
            progression::check_over(&stats, &mut over, &mut ending, &mut next_state);
            break;
        }
    }
}

// 笔试答题选项（独立系统：结算后立即判定失败结局，避免与主按钮系统参数超限）
pub fn handle_quiz_buttons(
    mut quiz_opts: Query<(&Interaction, &QuizOption), Changed<Interaction>>,
    mut give_ups: Query<(&Interaction, &QuizGiveUp), Changed<Interaction>>,
    quiz_state: Res<QuizState>,
    mut pipeline: ResMut<JobPipeline>,
    mut stats: ResMut<PlayerStats>,
    mut flags: ResMut<GameFlags>,
    mut toast: ResMut<ToastLog>,
    clock: Res<GameClock>,
    mut modal: ResMut<Modal>,
    mut over: ResMut<OverInfo>,
    mut ending: ResMut<Ending>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // 放弃笔试：视为未通过（被拒），提供不答题的退出路径
    for (inter, _) in &mut give_ups {
        if *inter == Interaction::Pressed {
            let chapter = clock.chapter();
            toast.push("你放弃了这次笔试，被标记为未通过。");
            job::resolve_written(
                quiz_state.app,
                false,
                &mut pipeline,
                &mut stats,
                &mut flags,
                &mut toast,
                chapter,
            );
            modal.close();
            progression::check_over(&stats, &mut over, &mut ending, &mut next_state);
            return;
        }
    }
    for (inter, opt) in &mut quiz_opts {
        if *inter == Interaction::Pressed {
            let Some(q) = QUIZ.get(quiz_state.q) else {
                warn!("[弹窗] 笔试答题下标越界：{}", quiz_state.q);
                continue;
            };
            let chapter = clock.chapter();
            job::resolve_written(
                quiz_state.app,
                opt.0 == q.correct,
                &mut pipeline,
                &mut stats,
                &mut flags,
                &mut toast,
                chapter,
            );
            modal.close();
            // 笔试答错被拒可能把心态扣到 0（拒绝 -8），立即判定结局
            progression::check_over(&stats, &mut over, &mut ending, &mut next_state);
            break;
        }
    }
}

// 对话导航：选项 → 下一节点 / 跳转其它 NPC（>= 1000）/ 结束（None）。
// 选择选项、进入新节点时会结算各自携带的效果（DlgEffect，见 npc.rs）。
// 防刷：每个 NPC 的对话效果每天只结算一次（DialogSettle 记录当天已结算的 NPC）。
fn dialog_navigate(
    dialog: &mut DialogueState,
    modal: &mut Modal,
    stats: &mut PlayerStats,
    toast: &mut ToastLog,
    settle: &mut DialogSettle,
    opt_idx: usize,
) {
    // 越界防护：与渲染侧 spawn_dialogue 一致，异常数据优雅跳过（避免 panic）
    let npc_idx = dialog.npc;
    let Some(npc) = NPCS.get(npc_idx) else {
        warn!("[弹窗] 对话 NPC 下标越界：{}", npc_idx);
        return;
    };
    let Some(node) = npc.nodes.get(dialog.node) else {
        warn!("[弹窗] 对话节点下标越界：{}", dialog.node);
        return;
    };
    if opt_idx >= node.options.len() {
        return;
    }
    // 当天已与该 NPC 结算过效果 → 本次只跳转、不再给效果
    let fresh = !settle.npcs.contains(&npc_idx);
    let opt = &node.options[opt_idx];
    let mut settled_any = false;
    if fresh {
        settled_any |= apply_dlg_effect(opt.effect, stats, toast);
    }
    match opt.next {
        None => modal.close(),
        Some(n) if n >= 1000 => {
            dialog.npc = n - 1000;
            dialog.node = 0;
            modal.refresh();
        }
        Some(n) => {
            dialog.node = n;
            if fresh {
                // 进入目标节点时结算该节点效果（如妈妈寄腊肠回心态）
                if let Some(tgt) = npc.nodes.get(n) {
                    settled_any |= apply_dlg_effect(tgt.effect, stats, toast);
                }
            }
            modal.refresh();
        }
    }
    if settled_any && !settle.npcs.contains(&npc_idx) {
        settle.npcs.push(npc_idx);
    }
}

/// 结算一次对话效果（选项或节点携带），并把变化通过 toast 反馈给玩家。
/// 返回是否实际结算（用于「每天限一次」的记录）。
fn apply_dlg_effect(effect: Option<DlgEffect>, stats: &mut PlayerStats, toast: &mut ToastLog) -> bool {
    let Some(e) = effect else {
        return false;
    };
    let mut msg = String::new();
    if e.mentality != 0.0 {
        change(&mut stats.mentality, e.mentality);
        let sign = if e.mentality > 0.0 { '+' } else { '-' };
        let icon = if e.mentality > 0.0 { "😊" } else { "😔" };
        msg.push_str(&format!("{icon} 心态{sign}{:.0}", e.mentality.abs()));
    }
    if let Some((idx, d)) = e.skill {
        let before = stats.skills[idx];
        stats.skills[idx] = (before + d).clamp(0.0, 100.0);
        let gain = stats.skills[idx] - before;
        if !msg.is_empty() {
            msg.push(' ');
        }
        let sign = if gain >= 0.0 { '+' } else { '-' };
        msg.push_str(&format!("{} {sign}{gain:.0}", SKILL_NAMES[idx]));
    }
    if !msg.is_empty() {
        toast.push(msg);
    }
    true
}

// 交通面板选择目的地后出发：调用 start_transit 自动乘车前往该区域站点
fn apply_commute(target: Location, transit: &mut TransitState, toast: &mut ToastLog) {
    let mode = transit.mode; // 站点交互时已设定（Subway / Bus）
    super::transit::start_transit(transit, mode, target);
    toast.push(format!(
        "🚉 上车出发，前往{}！",
        super::scenes::location_name(target)
    ));
}

// ==================== 单元测试 ====================
// 验证对话效果结算（DlgEffect）正确应用到属性并给出 toast 反馈。
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dlg_effect_applies_mentality_and_skill() {
        let mut stats = PlayerStats::default();
        let mut toast = ToastLog::default();

        // 心态效果：80 → 85，弹 toast
        apply_dlg_effect(Some(DlgEffect::ment(5.0)), &mut stats, &mut toast);
        assert_eq!(stats.mentality, 85.0);
        assert_eq!(stats.skills[3], 18.0, "心态效果不应影响技能");
        assert_eq!(toast.items.len(), 1);
        assert!(toast.items[0].0.contains("心态+5"), "toast 应标明心态 +5");

        // 技能效果：社交 18 → 20
        apply_dlg_effect(Some(DlgEffect::skill(3, 2.0)), &mut stats, &mut toast);
        assert_eq!(stats.skills[3], 20.0);
        assert_eq!(stats.mentality, 85.0, "技能效果不应影响心态");
        assert_eq!(toast.items.len(), 2);
        assert!(toast.items[1].0.contains("社交"), "toast 应标明技能名");
    }

    #[test]
    fn dlg_effect_none_is_noop() {
        let mut stats = PlayerStats::default();
        let mut toast = ToastLog::default();

        apply_dlg_effect(None, &mut stats, &mut toast);

        assert_eq!(stats.mentality, 80.0);
        assert_eq!(stats.skills, [18.0; 5]);
        assert_eq!(stats.money, 2200.0);
        assert!(toast.items.is_empty(), "无效果不应弹 toast");
    }

    #[test]
    fn dlg_effect_skill_clamps_at_100() {
        let mut stats = PlayerStats {
            skills: [99.0; 5],
            ..default()
        };
        let mut toast = ToastLog::default();

        apply_dlg_effect(Some(DlgEffect::skill(2, 5.0)), &mut stats, &mut toast);

        assert_eq!(stats.skills[2], 100.0, "技能不得超过 100");
    }

    #[test]
    fn dialog_effect_settles_once_per_day() {
        let mut dialog = DialogueState { npc: 2, node: 0 }; // 陈教授：选项 0 带心态 +5
        let mut modal = Modal::default();
        let mut stats = PlayerStats::default();
        let mut toast = ToastLog::default();
        let mut settle = DialogSettle::default();

        // 第 1 次点「要要要！」：结算效果并记录该 NPC
        dialog_navigate(&mut dialog, &mut modal, &mut stats, &mut toast, &mut settle, 0);
        assert_eq!(stats.mentality, 85.0, "首次对话应结算心态 +5");
        assert_eq!(settle.npcs, vec![2], "当天已结算的 NPC 应被记录");
        assert_eq!(toast.items.len(), 1);

        // 玩家关掉重开对话，第 2 次点同一选项：只跳转、不再结算
        dialog = DialogueState { npc: 2, node: 0 };
        dialog_navigate(&mut dialog, &mut modal, &mut stats, &mut toast, &mut settle, 0);
        assert_eq!(stats.mentality, 85.0, "同一天重复对话不应再次结算");
        assert_eq!(settle.npcs, vec![2], "记录不应重复");
        assert_eq!(toast.items.len(), 1, "不应再弹效果 toast");
    }

    #[test]
    fn dialog_settle_resets_on_new_day() {
        let mut dialog = DialogueState { npc: 2, node: 0 };
        let mut modal = Modal::default();
        let mut stats = PlayerStats::default();
        let mut toast = ToastLog::default();
        let mut settle = DialogSettle { day: 1, npcs: vec![2] };

        // 模拟 handle_modal_buttons 的跨天重置逻辑
        let new_day = 2;
        if settle.day != new_day {
            settle.day = new_day;
            settle.npcs.clear();
        }

        dialog_navigate(&mut dialog, &mut modal, &mut stats, &mut toast, &mut settle, 0);
        assert_eq!(stats.mentality, 85.0, "新的一天应重新允许结算");
        assert_eq!(settle.npcs, vec![2]);
    }
}
