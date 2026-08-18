//! 章节与每日循环：advance_day 推进天数（触发上班结算 / 交房租 /
//! 求职管线推进），roll_evening_event 触发随机事件之一（当前 7 个），
//! 以及 26 周结束后的结局判定。

use bevy::prelude::*;

use crate::router::GameState;

use super::resources::*;
use rand::RngExt;

// ==================== 数值常量（带设计意图，调参看这里） ====================
pub const INTERN_OK_LINE: f32 = 55.0; // 转正答辩通过线（算法 + 项目均值）
const EVENING_EVENT_CHANCE: f32 = 0.7; // 进入晚上触发随机事件的概率
const SAVINGS_MID: f32 = 5000.0; // 结局分档：存款 > 5000 视为「有点积蓄」
const SAVINGS_HIGH: f32 = 8000.0; // 结局分档：存款 > 8000 视为「能辞职创业」
const SKILL_AVG_GOOD: f32 = 70.0; // 结局分档：技能均值 >= 70 视为「吃透 AI 工作流」

// ==================== 随机事件数据（选项化） ====================
// 晚间随机事件：标题 + 描述 + 2-3 个选项，每个选项不同数值影响。
pub struct EvChoice {
    pub label: &'static str,
    pub ment: f32,
    pub money: f32,
    pub energy: f32,
    pub satiety: f32,
    pub health: f32,
    pub skill: Option<(usize, f32)>,
}

pub struct EvPopup {
    pub title: &'static str,
    pub desc: &'static str,
    pub choices: &'static [EvChoice],
}

pub const EVENTS: &[EvPopup] = &[
    EvPopup {
        title: "深夜焦虑",
        desc: "刷到「35 岁被裁」的热搜，你心头一紧，翻来覆去睡不着。",
        choices: &[
            EvChoice {
                label: "刷两道题压压惊",
                ment: -2.0,
                money: 0.0,
                energy: -8.0,
                satiety: 0.0,
                health: 0.0,
                skill: Some((0, 3.0)),
            },
            EvChoice {
                label: "看点开心的视频",
                ment: 6.0,
                money: 0.0,
                energy: 0.0,
                satiety: 0.0,
                health: 0.0,
                skill: None,
            },
            EvChoice {
                label: "直接睡觉，明天再说",
                ment: 2.0,
                money: 0.0,
                energy: 4.0,
                satiety: 0.0,
                health: 0.0,
                skill: None,
            },
        ],
    },
    EvPopup {
        title: "深夜食堂",
        desc: "肚子咕咕叫，要不要来点夜宵？",
        choices: &[
            EvChoice {
                label: "泡面加蛋（省）",
                ment: 0.0,
                money: -5.0,
                energy: 0.0,
                satiety: 25.0,
                health: -2.0,
                skill: None,
            },
            EvChoice {
                label: "点份外卖犒劳自己",
                ment: 5.0,
                money: -20.0,
                energy: 0.0,
                satiety: 40.0,
                health: 0.0,
                skill: None,
            },
            EvChoice {
                label: "忍一忍，喝水",
                ment: -3.0,
                money: 0.0,
                energy: 0.0,
                satiety: 0.0,
                health: 2.0,
                skill: None,
            },
        ],
    },
    EvPopup {
        title: "猎头来电",
        desc: "深夜接到猎头电话，画了个大饼：『大厂机会，薪资翻倍，考虑一下？』",
        choices: &[
            EvChoice {
                label: "聊十分钟，套点信息",
                ment: 3.0,
                money: 0.0,
                energy: -3.0,
                satiety: 0.0,
                health: 0.0,
                skill: Some((3, 1.5)),
            },
            EvChoice {
                label: "礼貌挂断，早点睡",
                ment: 1.0,
                money: 0.0,
                energy: 2.0,
                satiety: 0.0,
                health: 0.0,
                skill: None,
            },
            EvChoice {
                label: "激动得睡不着",
                ment: -4.0,
                money: 0.0,
                energy: -5.0,
                satiety: 0.0,
                health: 0.0,
                skill: None,
            },
        ],
    },
    EvPopup {
        title: "房东通知",
        desc: "房东发来消息：下个月涨租 200。这日子没法过了。",
        choices: &[
            EvChoice {
                label: "讨价还价，软磨硬泡",
                ment: 3.0,
                money: -50.0,
                energy: -4.0,
                satiety: 0.0,
                health: 0.0,
                skill: Some((3, 2.0)),
            },
            EvChoice {
                label: "认了，多接点活",
                ment: -5.0,
                money: 0.0,
                energy: -6.0,
                satiety: 0.0,
                health: 0.0,
                skill: Some((2, 2.5)),
            },
            EvChoice {
                label: "考虑搬去远一点的地方",
                ment: 2.0,
                money: 0.0,
                energy: 0.0,
                satiety: 0.0,
                health: 0.0,
                skill: None,
            },
        ],
    },
    EvPopup {
        title: "AI 工具分享",
        desc: "邻居赖哥拉你一起研究一个新出的 AI 编程工具，说能大幅提效。",
        choices: &[
            EvChoice {
                label: "认真研究两小时",
                ment: 2.0,
                money: 0.0,
                energy: -8.0,
                satiety: 0.0,
                health: 0.0,
                skill: Some((0, 3.5)),
            },
            EvChoice {
                label: "先收藏，改天看",
                ment: 1.0,
                money: 0.0,
                energy: 0.0,
                satiety: 0.0,
                health: 0.0,
                skill: None,
            },
            EvChoice {
                label: "拉着赖哥聊到深夜",
                ment: 6.0,
                money: 0.0,
                energy: -10.0,
                satiety: 0.0,
                health: 0.0,
                skill: Some((3, 2.0)),
            },
        ],
    },
    EvPopup {
        title: "游戏抽卡",
        desc: "新版本抽卡活动开了，你盯着那个 10 连的按钮犹豫不决。",
        choices: &[
            EvChoice {
                label: "氪 30 抽一波",
                ment: 8.0,
                money: -30.0,
                energy: 0.0,
                satiety: 0.0,
                health: 0.0,
                skill: None,
            },
            EvChoice {
                label: "白嫖党，忍住",
                ment: 2.0,
                money: 0.0,
                energy: 0.0,
                satiety: 0.0,
                health: 0.0,
                skill: None,
            },
            EvChoice {
                label: "抽歪了，心态崩了",
                ment: -10.0,
                money: -10.0,
                energy: 0.0,
                satiety: 0.0,
                health: 0.0,
                skill: None,
            },
        ],
    },
    EvPopup {
        title: "技术直播",
        desc: "晚上有一场大牛的技术直播，讲 AI 全流程开发。",
        choices: &[
            EvChoice {
                label: "全程看完并做笔记",
                ment: 3.0,
                money: 0.0,
                energy: -10.0,
                satiety: 0.0,
                health: 0.0,
                skill: Some((2, 4.0)),
            },
            EvChoice {
                label: "看半小时就困了",
                ment: 0.0,
                money: 0.0,
                energy: -3.0,
                satiety: 0.0,
                health: 0.0,
                skill: Some((2, 1.0)),
            },
            EvChoice {
                label: "睡大觉，明天看重播",
                ment: 1.0,
                money: 0.0,
                energy: 5.0,
                satiety: 0.0,
                health: 0.0,
                skill: None,
            },
        ],
    },
];

// 晨间消息（保持单向 toast）
struct Ev {
    text: &'static str,
    ment: f32,
    money: f32,
    energy: f32,
    satiety: f32,
    skill: Option<(usize, f32)>,
}

const MORNING: &[Ev] = &[
    Ev {
        text: "新的一天，冲！",
        ment: 2.0,
        money: 0.0,
        energy: 0.0,
        satiety: 0.0,
        skill: None,
    },
    Ev {
        text: "早高峰地铁人挤人，挤掉了好心情",
        ment: -3.0,
        money: 0.0,
        energy: 0.0,
        satiety: 0.0,
        skill: None,
    },
    Ev {
        text: "今天天气不错",
        ment: 2.0,
        money: 0.0,
        energy: 0.0,
        satiety: 0.0,
        skill: None,
    },
    Ev {
        text: "群里又有人晒 offer，心里酸酸的",
        ment: -4.0,
        money: 0.0,
        energy: 0.0,
        satiety: 0.0,
        skill: None,
    },
    Ev {
        text: "昨晚睡得不错，精力满满",
        ment: 0.0,
        money: 0.0,
        energy: 5.0,
        satiety: 0.0,
        skill: None,
    },
    Ev {
        text: "早饭吃了个包子，元气满满",
        ment: 1.0,
        money: -3.0,
        energy: 0.0,
        satiety: 15.0,
        skill: None,
    },
];

/// 公共结算：把一组数值增量应用到属性（change 夹取到 0..100，技能同样封顶）。
/// 事件（apply_ev / apply_event_choice）都走这里，避免两处重复实现。
fn apply_deltas(
    stats: &mut PlayerStats,
    ment: f32,
    money: f32,
    energy: f32,
    satiety: f32,
    health: f32,
    skill: Option<(usize, f32)>,
) {
    change(&mut stats.mentality, ment);
    stats.money += money;
    change(&mut stats.energy, energy);
    change(&mut stats.satiety, satiety);
    change(&mut stats.health, health);
    if let Some((idx, gain)) = skill {
        stats.skills[idx] = (stats.skills[idx] + gain).min(100.0);
    }
}

fn apply_ev(ev: &Ev, stats: &mut PlayerStats, toast: &mut ToastLog) {
    apply_deltas(stats, ev.ment, ev.money, ev.energy, ev.satiety, 0.0, ev.skill);
    let mut s = ev.text.to_string();
    if ev.ment.abs() > 0.0 {
        s.push_str(&format!("（心态 {:+.0}）", ev.ment));
    }
    if ev.money.abs() > 0.0 {
        s.push_str(&format!("（¥{:+.0}）", ev.money));
    }
    toast.push(s);
}

// ==================== 时钟时段推进 ====================
// 每个时段按真实秒数自动推进（上午 → 工作 → 午饭 → 晚上），
// 晚上是当日终点，停留等待玩家上床睡觉推进天数；
// 进入晚上时有概率（约 70%）触发一次晚间随机事件。
pub fn phase_tick(
    time: Res<Time>,
    mut clock: ResMut<GameClock>,
    mut modal: ResMut<Modal>,
    mut event_state: ResMut<EventState>,
    mut toast: ResMut<ToastLog>,
    mut evening_done: Local<bool>,
) {
    if clock.phase != Phase::Evening {
        *evening_done = false;
    }
    clock.phase_t += time.delta_secs();
    let dur = phase_duration(clock.phase);
    if clock.phase_t < dur {
        return;
    }
    let prev = clock.phase;
    let next = next_phase(prev);
    if next == prev {
        clock.phase_t = 0.0; // 晚上：计时归零避免持续累加，等玩家睡觉
        return;
    }
    let elapsed = clock.phase_t; // 记录实际流逝（归零前）
    clock.phase = next;
    clock.phase_t = 0.0;
    info!(
        "[时段] 第{}周 第{}天 {} → {}（{} 持续 {:.0}s，实际 {:.1}s）",
        clock.week,
        clock.day,
        prev.label(),
        next.label(),
        prev.label(),
        dur,
        elapsed
    );
    toast.push(format!("🕐 {}了", next.label()));
    if next == Phase::Evening && !*evening_done {
        *evening_done = true;
        if modal.kind.is_some() {
            // 已有弹窗（对话/笔试/投简历/通勤）打开：先不触发事件，
            // 标记 pending，等弹窗关闭后再弹，避免顶掉玩家的当前操作。
            event_state.pending = true;
            info!("[时段] 进入晚上，已有弹窗打开，晚间随机事件延后触发");
        } else {
            info!("[时段] 进入晚上，今晚将触发晚间随机事件");
            roll_evening_event(&mut modal, &mut event_state);
        }
    }
}

// ==================== 晚间随机事件 ====================
// 触发时打开选项弹窗（不立即结算，等玩家选择）。
pub fn roll_evening_event(modal: &mut Modal, event: &mut EventState) {
    let mut rng = rand::rng();
    if rng.random::<f32>() < EVENING_EVENT_CHANCE {
        let idx = rng.random_range(0..EVENTS.len());
        let ev = &EVENTS[idx];
        info!(
            "[事件] 晚间触发 #{}「{}」：{}（{} 个选项）",
            idx,
            ev.title,
            ev.desc,
            ev.choices.len()
        );
        event.idx = idx;
        modal.open(ModalKind::Event);
    } else {
        debug!("[事件] 今晚未触发随机事件（30% 概率）");
    }
}

// 应用事件选项的数值影响（由弹窗按钮点击调用）
pub fn apply_event_choice(
    idx: usize,
    choice: usize,
    stats: &mut PlayerStats,
    toast: &mut ToastLog,
) {
    let Some(ev) = EVENTS.get(idx) else {
        warn!("[事件] 结算失败：事件 #{} 不存在", idx);
        return;
    };
    let Some(c) = ev.choices.get(choice) else {
        warn!(
            "[事件] 结算失败：事件 #{}「{}」选项 {} 不存在",
            idx, ev.title, choice
        );
        return;
    };
    // 先记录技能结算前数值，apply_deltas 统一结算（含技能），
    // 之后只生成日志/文案，绝不能再手动加一遍（否则技能会翻倍）。
    let skill_before = c.skill.map(|(idx, _)| stats.skills[idx]);
    apply_deltas(stats, c.ment, c.money, c.energy, c.satiety, c.health, c.skill);
    let mut skill_part = String::new();
    if let Some((skill_idx, _)) = c.skill {
        let before = skill_before.unwrap_or(0.0);
        skill_part = format!(
            " 技能[{}] {:+.1}→{:.1}",
            SKILL_NAMES.get(skill_idx).copied().unwrap_or("?"),
            stats.skills[skill_idx] - before,
            stats.skills[skill_idx]
        );
    }
    info!(
        "[事件] 结算 #{}「{}」选「{}」：心态{:+.0} 钱{:+.0} 精力{:+.0} 饱食{:+.0} 健康{:+.0}{} → 心态{:.0} 钱{:.0}",
        idx,
        ev.title,
        c.label,
        c.ment,
        c.money,
        c.energy,
        c.satiety,
        c.health,
        skill_part,
        stats.mentality,
        stats.money
    );
    let mut s = format!("{}：{}", ev.title, c.label);
    if c.ment.abs() > 0.0 {
        s.push_str(&format!("（心态 {:+.0}）", c.ment));
    }
    if c.money.abs() > 0.0 {
        s.push_str(&format!("（¥{:+.0}）", c.money));
    }
    toast.push(s);
}

// ==================== 晨间消息 ====================
fn morning_message(stats: &mut PlayerStats, toast: &mut ToastLog) {
    let mut rng = rand::rng();
    let idx = rng.random_range(0..MORNING.len());
    apply_ev(&MORNING[idx], stats, toast);
}

// ==================== 推进到新的一天 ====================
// 睡觉后调用：结算房租（周一）、推进周/日、触发章节事件、回到家的清晨。
// 返回 true 表示触发结局（游戏结束）。
#[allow(clippy::too_many_arguments)]
pub fn advance_day(
    clock: &mut GameClock,
    stats: &mut PlayerStats,
    flags: &mut GameFlags,
    toast: &mut ToastLog,
    bonus: &mut WorkBonus,
    over: &mut OverInfo,
    ending: &mut Ending,
    next_state: &mut NextState<GameState>,
    cinematic: &mut Cinematic,
) -> bool {
    // 周一交房租
    if clock.day == 1 {
        let rent = rent_amount(clock.chapter());
        stats.money -= rent;
        info!(
            "[结算] 周一交房租 -¥{rent:.0}（第{}章，余额 ¥{:.0}）",
            clock.chapter(),
            stats.money
        );
        toast.push(format!("💸 周一交房租 -¥{rent:.0}"));
        if stats.money < 0.0 {
            info!("[结局] 交完房租破产（余额 ¥{:.0}）→ 游戏结束", stats.money);
            over.reason = Some(OverReason::Rent);
            ending.title = "流落街头".to_string();
            ending.desc = "银行账户见底，你连下个月的房租都付不起。拖着行李箱站在深夜的街头，你开始怀疑人生。".to_string();
            next_state.set(GameState::GameOver);
            return true;
        }
    }

    // 推进日期
    clock.day += 1;
    if clock.day > 7 {
        clock.day = 1;
        clock.week += 1;
        if clock.week > TOTAL_WEEKS {
            *ending = compute_ending(flags, stats);
            info!("[结局] 26 周结束：{}", ending.title);
            over.reason = Some(OverReason::Finished);
            next_state.set(GameState::GameOver);
            return true;
        }
    }

    // 章节推进事件（大字报演出）
    match clock.chapter() {
        2 if clock.week == 5 => {
            info!("[章节] 进入第二章·实习期（第{}周）", clock.week);
            toast.push("📖 进入第二章·实习期，开始通勤上班");
            if flags.intern_offer {
                toast.push("你拿到了实习 offer，工作日记得去公司！");
                cinematic.play(
                    "第二章 · 实习期",
                    "你拿到了实习 Offer，领到工牌，开始通勤打工",
                );
            } else {
                toast.push("还没实习 offer？快去投简历！");
                cinematic.play("第二章 · 实习期", "实习季开始了，先拿一个 Offer 再说");
            }
        }
        3 if clock.week == 11 => {
            info!(
                "[章节] 进入第三章·秋招季（第{}周），转正答辩 pass={}",
                clock.week,
                flags.intern_offer && (stats.skills[0] + stats.skills[2]) / 2.0 >= INTERN_OK_LINE
            );
            if flags.intern_offer {
                // 转正答辩（依据算法 + 项目平均分）；没投过实习则无答辩资格
                flags.intern_ok = (stats.skills[0] + stats.skills[2]) / 2.0 >= INTERN_OK_LINE;
                if flags.intern_ok {
                    change(&mut stats.mentality, 5.0);
                    toast.push("🎓 转正答辩通过！薪资涨到实习转正档");
                    cinematic.play("第三章 · 秋招季", "转正答辩通过！白天实习，晚上备战秋招");
                } else {
                    change(&mut stats.mentality, -12.0);
                    toast.push("😔 转正答辩没过，实习结束，秋招走起");
                    cinematic.play("第三章 · 秋招季", "转正答辩失利……全力备战秋招");
                }
            } else {
                // 没拿实习 offer：无答辩资格，直接进入秋招
                flags.intern_ok = false;
                change(&mut stats.mentality, -6.0);
                toast.push("你还没拿到实习 offer，直接备战秋招");
                cinematic.play("第三章 · 秋招季", "没有实习经历，全力冲刺秋招");
            }
            toast.push("📖 进入第三章·秋招季！白天实习，晚上刷题");
        }
        4 if clock.week == 19 => {
            info!(
                "[章节] 进入第四章·毕业入职（第{}周），正式 offer={}",
                clock.week, flags.formal_offer
            );
            toast.push("📖 进入第四章·毕业入职");
            if flags.formal_offer {
                change(&mut stats.mentality, 5.0);
                toast.push("带着 offer 入职，起步就是赢家");
                cinematic.play("第四章 · 毕业入职", "带着 Offer 入职新公司，起步就是赢家");
            } else {
                change(&mut stats.mentality, -5.0);
                toast.push("还没正式 offer……快去投简历啊！");
                cinematic.play("第四章 · 毕业入职", "还没正式 Offer？简历还能再抢救一下");
            }
        }
        5 if clock.week == 23 => {
            info!("[章节] 进入第五章·职场日常（第{}周）", clock.week);
            toast.push("📖 进入第五章·职场日常，稳住！");
            cinematic.play("第五章 · 职场日常", "最后冲刺：稳住心态，守住存款");
        }
        _ => {}
    }

    clock.phase = Phase::Morning;
    clock.phase_t = 0.0; // 新的一天从上午开始计时
    bonus.used_today = false;
    info!(
        "[日期] 第{}周 第{}天（{}）",
        clock.week,
        clock.day,
        day_label(clock.day)
    );

    // 每日自然变化 + 晨间消息
    change(&mut stats.satiety, -10.0);
    change(&mut stats.energy, 8.0);
    morning_message(stats, toast);
    false
}

// ==================== 结局判定 ====================
/// 统一失败结局判定：心态归零崩溃 / 存款为负破产。
/// 事件、对话、求职拒绝等任何扣减心态/金钱的路径都应在结算后调用一次，
/// 避免「数值已触发结局条件但游戏没结束」的延迟判定。
/// 返回 true 表示已触发结局（调用方应停止后续结算）。
pub fn check_over(
    stats: &PlayerStats,
    over: &mut OverInfo,
    ending: &mut Ending,
    next_state: &mut NextState<GameState>,
) -> bool {
    if stats.mentality <= 0.0 {
        info!("[结局] 心态归零（0/100）→ 游戏结束");
        over.reason = Some(OverReason::Mentality);
        ending.title = "心态崩溃，回老家考公".to_string();
        ending.desc = "连续的打击终于压垮了你。你打包行李回了老家，开始备考公务员。也许，安稳才是你想要的。"
            .to_string();
        next_state.set(GameState::GameOver);
        return true;
    }
    if stats.money < 0.0 {
        info!("[结局] 存款为负（¥{:.0}）→ 游戏结束", stats.money);
        over.reason = Some(OverReason::Rent);
        ending.title = "流落街头".to_string();
        ending.desc = "银行账户见底，你连下个月的房租都付不起。拖着行李箱，你站在深夜的街头，开始怀疑人生。"
            .to_string();
        next_state.set(GameState::GameOver);
        return true;
    }
    false
}

pub fn compute_ending(flags: &GameFlags, stats: &PlayerStats) -> Ending {
    let avg = stats.skills.iter().sum::<f32>() / 5.0;
    let (title, desc) = if !flags.formal_offer && !flags.intern_offer {
        if stats.money > SAVINGS_MID {
            (
                "灵活就业，自由职业",
                "没等到 offer，但存款还够。你成了一名自由职业者，接点小项目糊口，倒也自在。",
            )
        } else {
            (
                "秋招失利，回炉重造",
                "offer 一个没捞着，你回到图书馆，准备来年再战。失败不可怕，可怕的是放弃。",
            )
        }
    } else if flags.formal_offer {
        match flags.best_tier {
            0 if avg >= SKILL_AVG_GOOD => (
                "AI 时代的冲浪者",
                "你吃透了 AI 工作流，项目扎实，拿下大厂 SSP，成为时代浪尖上的人。",
            ),
            0 => (
                "大厂 SSP，高开稳走",
                "你成功上岸大厂，薪资顶格，朋友圈一片羡慕。",
            ),
            1 => (
                "中厂骨干，稳步成长",
                "你在中厂拿到了满意的 offer，开始积累自己的技术影响力。",
            ),
            _ => (
                "普通企业，安稳上岸",
                "虽然不在大厂，但朝九晚五，生活也算安稳。",
            ),
        }
    } else if stats.money > SAVINGS_HIGH {
        (
            "攒够本钱，辞职创业",
            "实习攒下的钱让你开了间小工作室，雇了两个应届生，当上了小老板。",
        )
    } else {
        (
            "实习转正，稳步积累",
            "你留在了实习公司，虽然薪水一般，但胜在熟悉，慢慢熬吧。",
        )
    };
    Ending {
        title: title.to_string(),
        desc: desc.to_string(),
    }
}

// ==================== 单元测试 ====================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_choice_skill_applies_once_not_twice() {
        // 回归测试：apply_event_choice 曾把技能结算两遍（翻倍）。
        // 「深夜焦虑」选项 0 带 算法 +3，结算后应恰好 +3（18 → 21）。
        let mut stats = PlayerStats::default();
        let mut toast = ToastLog::default();

        apply_event_choice(0, 0, &mut stats, &mut toast);

        assert_eq!(stats.skills[0], 21.0, "技能应只结算一次（+3，而非 +6）");
        assert_eq!(stats.skills[1], 18.0, "其它技能不应受影响");
        assert_eq!(stats.mentality, 78.0, "心态 -2");
        assert_eq!(stats.energy, 72.0, "精力 -8");
        assert_eq!(toast.items.len(), 1, "应弹一条事件 toast");
    }

    #[test]
    fn event_choice_bad_index_is_noop() {
        // 越界防护：非法下标应优雅跳过，不改任何属性
        let mut stats = PlayerStats::default();
        let mut toast = ToastLog::default();

        apply_event_choice(999, 0, &mut stats, &mut toast);
        assert_eq!(stats.mentality, 80.0);
        assert!(toast.items.is_empty());

        apply_event_choice(0, 99, &mut stats, &mut toast);
        assert_eq!(stats.mentality, 80.0);
        assert!(toast.items.is_empty());
    }
}
