//! 行为分发与执行：movement 到达后写入 PendingAction，
//! dispatch_action 消费并按 HotspotKind 执行对应效果
//! （睡觉 / 刷题 / 吃饭 / 上班 / 对话…），最后统一判定失败结局。

use bevy::prelude::*;

use crate::router::GameState;

use super::components::CommuteChoice;
use super::npc::{NPC_LIST_INDEX, NPCS};
use super::progression;
use super::resources::*;
use rand::RngExt;

// ==================== 行为分发 ====================
// movement 到达后写入 PendingAction，这里消费并执行对应行为。
#[allow(clippy::too_many_arguments)]
pub fn dispatch_action(
    mut pending: ResMut<PendingAction>,
    mut stats: ResMut<PlayerStats>,
    mut clock: ResMut<GameClock>,
    mut flags: ResMut<GameFlags>,
    mut toast: ResMut<ToastLog>,
    mut modal: ResMut<Modal>,
    mut dialog: ResMut<DialogueState>,
    mut bonus: ResMut<WorkBonus>,
    mut free_use: ResMut<FreeUse>,
    mut over: ResMut<OverInfo>,
    mut ending: ResMut<Ending>,
    mut next_state: ResMut<NextState<GameState>>,
    mut walk: ResMut<WalkState>,
    mut cinematic: ResMut<Cinematic>,
    mut transit: ResMut<TransitState>,
    mut bike: ResMut<BikeMode>,
) {
    if modal.kind.is_some() {
        // 弹窗打开时暂不消费待执行行为：等弹窗关闭后再触发，避免行为被静默丢弃
        return;
    }
    let Some(kind) = pending.0.take() else {
        return;
    };
    walk.target = None;
    walk.cmd = WalkCmd::Move;
    match kind {
        PendingKind::Npc(idx) => {
            if idx > 0 && idx < NPCS.len() {
                info!("[行为] 与 {} 对话", NPCS[idx].name);
                dialog.npc = idx;
                dialog.node = 0;
                modal.open(ModalKind::Dialogue);
            }
        }
        PendingKind::Hotspot(k) => {
            info!("[行为] 触发 {:?}", k);
            do_action(
                k,
                &mut stats,
                &mut clock,
                &mut flags,
                &mut toast,
                &mut modal,
                &mut dialog,
                &mut bonus,
                &mut free_use,
                &mut over,
                &mut ending,
                &mut next_state,
                &mut cinematic,
                &mut transit,
                &mut bike,
            );
        }
    }
}

// ==================== 具体行为 ====================
/// 免费恢复类热点（无成本）的每日限次：每天首次全额效果，之后效果减半，
/// 防止反复触发无限刷满资源（配合 FreeUse 资源）。
/// 返回 (效果倍率, 是否当天首次)。
fn free_scale(free_use: &mut FreeUse, day: u32, kind: HotspotKind) -> (f32, bool) {
    if free_use.day != day {
        free_use.day = day;
        free_use.used.clear();
    }
    let first = !free_use.used.contains(&kind);
    if first {
        free_use.used.push(kind);
        (1.0, true)
    } else {
        (0.4, false)
    }
}

#[allow(clippy::too_many_arguments)]
fn do_action(
    kind: HotspotKind,
    stats: &mut PlayerStats,
    clock: &mut GameClock,
    flags: &mut GameFlags,
    toast: &mut ToastLog,
    modal: &mut Modal,
    dialog: &mut DialogueState,
    bonus: &mut WorkBonus,
    free_use: &mut FreeUse,
    over: &mut OverInfo,
    ending: &mut Ending,
    next_state: &mut NextState<GameState>,
    cinematic: &mut Cinematic,
    transit: &mut TransitState,
    bike: &mut BikeMode,
) {
    match kind {
        // ============ 家 ============
        HotspotKind::Bed | HotspotKind::DormBed => {
            if clock.phase == Phase::Evening {
                toast.push("🌙 躺下睡觉…");
                progression::advance_day(
                    clock, stats, flags, toast, bonus, over, ending, next_state, cinematic,
                );
            } else if kind == HotspotKind::Bed {
                // 白天补觉免费回精力：每日限一次，防无限刷满精力
                let (s, first) = free_scale(free_use, clock.day, HotspotKind::Bed);
                change(&mut stats.energy, 40.0 * s);
                change(&mut stats.mentality, 5.0 * s);
                toast.push(if first {
                    "补了个回笼觉，精力回复"
                } else {
                    "又躺了一会儿，再睡也缓不过来…效果打折"
                });
            } else {
                // 宿舍床硬一点，补觉效果弱于家里（同样每日限一次）
                let (s, first) = free_scale(free_use, clock.day, HotspotKind::DormBed);
                change(&mut stats.energy, 30.0 * s);
                change(&mut stats.mentality, 3.0 * s);
                toast.push(if first {
                    "在宿舍床上眯了一会儿，精神不少"
                } else {
                    "眯多了反而更累，效果打折…"
                });
            }
        }
        HotspotKind::Desk => {
            let gain = skill_gain(stats, 0, 4.5);
            stats.skills[0] = (stats.skills[0] + gain).min(100.0);
            change(&mut stats.energy, -12.0);
            change(&mut stats.satiety, -4.0);
            toast.push(format!("书桌刷了几道算法题（算法 +{:.1}）", gain));
        }
        HotspotKind::Books => {
            let gain = skill_gain(stats, 1, 4.5);
            stats.skills[1] = (stats.skills[1] + gain).min(100.0);
            change(&mut stats.energy, -10.0);
            change(&mut stats.satiety, -4.0);
            toast.push(format!("啃了一小时八股（八股 +{:.1}）", gain));
        }
        HotspotKind::Kitchen => {
            change(&mut stats.satiety, 45.0);
            stats.money -= 10.0;
            change(&mut stats.energy, -8.0);
            toast.push("做了顿家常菜，吃饱了（-¥10）");
        }
        HotspotKind::Computer => {
            change(&mut stats.energy, -4.0);
            toast.push("打开招聘网站…");
            modal.open(ModalKind::Company);
        }
        HotspotKind::Phone => {
            change(&mut stats.mentality, 1.0);
            dialog.npc = NPC_LIST_INDEX;
            dialog.node = 0;
            modal.open(ModalKind::Dialogue);
        }
        HotspotKind::Tv => {
            change(&mut stats.mentality, 10.0);
            change(&mut stats.energy, -3.0);
            toast.push("窝在沙发追剧，把今天的烦心事都忘掉");
        }
        HotspotKind::Bathroom => {
            // 免费恢复精力/心态/健康：每日限一次，防无限刷满
            let (s, first) = free_scale(free_use, clock.day, HotspotKind::Bathroom);
            change(&mut stats.mentality, 6.0 * s);
            change(&mut stats.energy, 5.0 * s);
            change(&mut stats.health, 3.0 * s);
            toast.push(if first {
                "洗了个热水澡，浑身舒坦"
            } else {
                "刚洗过又洗？效果打折…"
            });
        }
        HotspotKind::Fridge => {
            // 冰箱免费剩饭：每日限一次，防无限刷饱食
            let (s, first) = free_scale(free_use, clock.day, HotspotKind::Fridge);
            change(&mut stats.satiety, 20.0 * s);
            toast.push(if first {
                "从冰箱翻出点剩饭，对付一顿（免费）"
            } else {
                "冰箱里的剩饭翻不出新花样，效果打折…"
            });
        }
        // ============ 校园 ============
        HotspotKind::Track => {
            change(&mut stats.health, 12.0);
            change(&mut stats.energy, -14.0);
            change(&mut stats.mentality, 4.0);
            toast.push("操场跑了三圈，神清气爽");
        }
        HotspotKind::TechGroup => {
            let gain = skill_gain(stats, 3, 4.5);
            stats.skills[3] = (stats.skills[3] + gain).min(100.0);
            change(&mut stats.mentality, 3.0);
            change(&mut stats.energy, -5.0);
            toast.push(format!("在技术群水了几句（社交 +{:.1}）", gain));
        }
        HotspotKind::Library => {
            let gain = skill_gain(stats, 1, 4.5);
            stats.skills[1] = (stats.skills[1] + gain).min(100.0);
            change(&mut stats.energy, -10.0);
            change(&mut stats.mentality, 3.0);
            toast.push(format!("在图书馆啃了一上午八股（八股 +{:.1}）", gain));
        }
        HotspotKind::Lab => {
            let gain = skill_gain(stats, 2, 4.5);
            stats.skills[2] = (stats.skills[2] + gain).min(100.0);
            change(&mut stats.energy, -14.0);
            change(&mut stats.mentality, -2.0);
            toast.push(format!("在实验室调了一下午代码（项目 +{:.1}）", gain));
        }
        HotspotKind::CampusShop => {
            change(&mut stats.satiety, 25.0);
            stats.money -= 8.0;
            change(&mut stats.mentality, 3.0);
            toast.push("小卖部扫了一包零食，快乐加满（-¥8）");
        }
        HotspotKind::DormGame => {
            change(&mut stats.mentality, 12.0);
            change(&mut stats.energy, -5.0);
            toast.push("和室友联机打了两把游戏，好不快活");
        }
        HotspotKind::DormSnack => {
            // 免费蹭零食：每日限一次，防无限刷饱食
            let (s, first) = free_scale(free_use, clock.day, HotspotKind::DormSnack);
            change(&mut stats.satiety, 12.0 * s);
            change(&mut stats.mentality, 2.0 * s);
            toast.push(if first {
                "蹭了室友一包薯片，白嫖快乐（免费）"
            } else {
                "室友的白眼警告：再蹭下去效果打折…"
            });
        }

        // ============ 食堂 ============
        HotspotKind::Canteen1 => {
            change(&mut stats.satiety, 50.0);
            stats.money -= 15.0;
            change(&mut stats.mentality, 3.0);
            toast.push("大众菜，量大管饱（-¥15）");
        }
        HotspotKind::Canteen2 => {
            change(&mut stats.satiety, 60.0);
            stats.money -= 25.0;
            change(&mut stats.mentality, 8.0);
            toast.push("小炒真香！（-¥25）");
        }
        HotspotKind::Microwave => {
            // 免费热饭：每日限一次，防无限刷饱食
            let (s, first) = free_scale(free_use, clock.day, HotspotKind::Microwave);
            change(&mut stats.satiety, 40.0 * s);
            toast.push(if first {
                "微波炉热了昨晚带的饭，省钱！"
            } else {
                "今天带的饭份额用完了，效果打折…"
            });
        }
        HotspotKind::InstantNoodle => {
            change(&mut stats.satiety, 30.0);
            stats.money -= 5.0;
            change(&mut stats.mentality, -5.0);
            change(&mut stats.health, -3.0);
            toast.push("泡面人生，惨并快乐着（-¥5）");
        }
        HotspotKind::DrinkMachine => {
            change(&mut stats.satiety, 12.0);
            change(&mut stats.mentality, 3.0);
            stats.money -= 4.0;
            toast.push("接了一杯酸梅汤，解腻又开胃（-¥4）");
        }
        HotspotKind::MilkTea => {
            change(&mut stats.mentality, 10.0);
            change(&mut stats.satiety, 10.0);
            stats.money -= 12.0;
            toast.push("来了一杯全糖奶茶，快乐加倍（-¥12）");
        }
        HotspotKind::FruitStand => {
            change(&mut stats.health, 5.0);
            change(&mut stats.satiety, 8.0);
            stats.money -= 6.0;
            toast.push("买了点水果补补维生素（-¥6）");
        }

        // ============ 办公室 ============
        HotspotKind::Workstation => {
            // 工资并入「白拿」每日限一次：反复点工位只扣精力/加技能，不再重复发钱，
            // 避免配合咖啡机免费回精力无限刷钱（与下方 toast 文案一致）。
            let income = work_income(flags, clock.chapter());
            let gain = skill_gain(stats, 2, 3.5);
            stats.skills[2] = (stats.skills[2] + gain).min(100.0);
            change(&mut stats.energy, -12.0);
            change(&mut stats.mentality, -2.0);
            if !bonus.used_today {
                bonus.used_today = true;
                if income > 0.0 {
                    info!("[行为] 工位工作 +¥{income:.0}（第{}章）", clock.chapter());
                    stats.money += income;
                }
                change(&mut stats.mentality, 5.0);
                toast.push(format!(
                    "工作顺利，白拿一轮效率加成（每天限一次）💪 +¥{income:.0}"
                ));
            } else {
                change(&mut stats.mentality, -3.0);
                toast.push("老板的余光扫过来了…今天的工作已结算，明天再来吧");
            }
            if income <= 0.0 {
                toast.push("无偿实习，先攒经验");
            }
        }
        HotspotKind::Lounge => {
            change(&mut stats.mentality, 10.0);
            change(&mut stats.energy, 6.0);
            if !bonus.used_today {
                bonus.used_today = true;
                change(&mut stats.mentality, 5.0);
                toast.push("茶水间摸鱼成功，白拿一次（每天限一次）☕");
            } else {
                change(&mut stats.mentality, -3.0);
                toast.push("又被老板看到了…");
            }
        }
        HotspotKind::Slacking => {
            change(&mut stats.mentality, 8.0);
            change(&mut stats.energy, 2.0);
            let mut rng = rand::rng();
            if rng.random::<f32>() < 0.25 {
                change(&mut stats.mentality, -10.0);
                toast.push("摸鱼被抓了个正着，心态崩了😱");
            } else {
                toast.push("摸鱼一时爽，一直摸鱼一直爽");
            }
        }
        HotspotKind::Takeout => {
            change(&mut stats.satiety, 60.0);
            stats.money -= 30.0;
            change(&mut stats.mentality, 5.0);
            change(&mut stats.energy, -2.0);
            toast.push("点了份外卖，工位上开吃（-¥30）");
        }
        HotspotKind::Coffee => {
            // 免费回精力：每日限一次，防配合工位/刷题无限白嫖精力
            let (s, first) = free_scale(free_use, clock.day, HotspotKind::Coffee);
            change(&mut stats.energy, 15.0 * s);
            change(&mut stats.mentality, 4.0 * s);
            toast.push(if first {
                "灌了杯美式，精神起来了"
            } else {
                "咖啡因已经顶不住了，效果打折…"
            });
        }
        HotspotKind::Meeting => {
            let gain = skill_gain(stats, 3, 4.5);
            stats.skills[3] = (stats.skills[3] + gain).min(100.0);
            change(&mut stats.energy, -5.0);
            change(&mut stats.mentality, -2.0);
            toast.push(format!("开完周会，跟团队对齐了需求（社交 +{:.1}）", gain));
        }
        HotspotKind::Printer => {
            let gain = skill_gain(stats, 4, 4.5);
            stats.skills[4] = (stats.skills[4] + gain).min(100.0);
            change(&mut stats.energy, -3.0);
            toast.push(format!("打印了一叠简历资料（简历 +{:.1}）", gain));
        }

        // ============ 公园（免费休闲，每日限一次防刷） ============
        HotspotKind::ParkBench => {
            let (s, first) = free_scale(free_use, clock.day, HotspotKind::ParkBench);
            change(&mut stats.energy, 15.0 * s);
            change(&mut stats.mentality, 8.0 * s);
            toast.push(if first {
                "在公园长椅上歇了会儿，精力恢复"
            } else {
                "再歇下去天都黑了，效果打折…"
            });
        }
        HotspotKind::ParkFountain => {
            let (s, first) = free_scale(free_use, clock.day, HotspotKind::ParkFountain);
            change(&mut stats.mentality, 10.0 * s);
            change(&mut stats.health, 3.0 * s);
            toast.push(if first {
                "绕着喷泉走了两圈，心情舒畅"
            } else {
                "喷泉看腻了，效果打折…"
            });
        }

        // ============ 校园周边探索 ============
        // 夜市：晚上才出摊（其他时段点它会提示打烊）
        HotspotKind::NightMarket => {
            if clock.phase != Phase::Evening {
                toast.push("夜市还没出摊，晚上再来吧");
            } else {
                change(&mut stats.satiety, 20.0);
                change(&mut stats.mentality, 5.0);
                stats.money -= 10.0;
                toast.push("夜市撸了串烤肠，烟火气十足（-¥10）");
            }
        }
        // 观景台：免费登高看风景（每日限一次防刷心态）
        HotspotKind::Lookout => {
            let (s, first) = free_scale(free_use, clock.day, HotspotKind::Lookout);
            change(&mut stats.mentality, 8.0 * s);
            toast.push(if first {
                "登上观景台俯瞰整个城市，心情豁然开朗"
            } else {
                "风景虽好，看多了也腻，效果打折…"
            });
        }
        // 涂鸦墙：随机彩蛋涂鸦（免费回心态，每日限一次防刷）
        HotspotKind::Graffiti => {
            let (s, first) = free_scale(free_use, clock.day, HotspotKind::Graffiti);
            change(&mut stats.mentality, 5.0 * s);
            let mut rng = rand::rng();
            let msgs = [
                "墙上有行大字：「代码跑通了！」",
                "涂鸦画了只会发光的锦鲤，好运+1",
                "墙角的字：「秋招上岸！求好运」",
                "一幅抽象的键盘涂鸦，莫名解压",
                "有人写了「谢谢路过的人，你也很棒」",
            ];
            let i = (rng.random::<f32>() * msgs.len() as f32) as usize % msgs.len();
            toast.push(msgs[i]);
            if !first {
                toast.push("涂鸦看过了，效果打折…");
            }
        }

        // ============ 城市交通 ============
        // 地铁站 / 公交站：选好方式后打开交通面板，选目的地区域
        HotspotKind::SubwayStop | HotspotKind::BusStop => {
            let mode = if kind == HotspotKind::SubwayStop {
                CommuteChoice::Subway
            } else {
                CommuteChoice::Bus
            };
            transit.mode = mode;
            toast.push(format!("🚉 在{}站，去哪儿？", mode.label()));
            modal.open(ModalKind::Commute);
        }
        // 共享单车：骑上 / 停下的骑行状态开关
        HotspotKind::BikeSpot => {
            bike.0 = !bike.0;
            if bike.0 {
                info!("[骑行] 骑上共享单车");
                toast.push("🚲 骑上共享单车，移动速度翻倍！");
            } else {
                info!("[骑行] 停下共享单车");
                toast.push("🅿️ 停好共享单车，下次再骑");
            }
        }
    }

    // 心态归零 / 存款为负 → 触发失败结局（统一判定，见 progression::check_over）
    progression::check_over(stats, over, ending, next_state);
}

// ==================== 单元测试 ====================
// 验证校园三个互动点（图书馆 / 实验室 / 小卖部）的行为效果符合预期：
// 技能增益、资源增减、金额扣除、数值夹取、Toast 提示、不误触失败结局。
#[cfg(test)]
mod tests {
    use super::*;

    // 构造 do_action 需要的全部上下文（属性默认值 + 无待定切换）
    struct Ctx {
        clock: GameClock,
        flags: GameFlags,
        toast: ToastLog,
        modal: Modal,
        dialog: DialogueState,
        bonus: WorkBonus,
        free_use: FreeUse,
        over: OverInfo,
        ending: Ending,
        next_state: NextState<GameState>,
        cinematic: Cinematic,
        transit: TransitState,
        bike: BikeMode,
    }

    impl Default for Ctx {
        fn default() -> Self {
            Self {
                clock: GameClock::default(),
                flags: GameFlags::default(),
                toast: ToastLog::default(),
                modal: Modal::default(),
                dialog: DialogueState::default(),
                bonus: WorkBonus::default(),
                free_use: FreeUse::default(),
                over: OverInfo::default(),
                ending: Ending::default(),
                next_state: NextState::Pending(GameState::Playing),
                cinematic: Cinematic::default(),
                transit: TransitState::default(),
                bike: BikeMode::default(),
            }
        }
    }

    // 以默认上下文执行一次热点行为
    fn act(kind: HotspotKind, stats: &mut PlayerStats, ctx: &mut Ctx) {
        do_action(
            kind,
            stats,
            &mut ctx.clock,
            &mut ctx.flags,
            &mut ctx.toast,
            &mut ctx.modal,
            &mut ctx.dialog,
            &mut ctx.bonus,
            &mut ctx.free_use,
            &mut ctx.over,
            &mut ctx.ending,
            &mut ctx.next_state,
            &mut ctx.cinematic,
            &mut ctx.transit,
            &mut ctx.bike,
        );
    }

    fn has_toast(ctx: &Ctx, keyword: &str) -> bool {
        ctx.toast.items.iter().any(|(s, _)| s.contains(keyword))
    }

    #[test]
    fn library_boosts_interview_skill() {
        let mut stats = PlayerStats {
            energy: 50.0,
            skills: [20.0; 5],
            ..default()
        };
        let mut ctx = Ctx::default();
        let before = stats.skills[1];

        act(HotspotKind::Library, &mut stats, &mut ctx);

        // 八股技能上升（含精力缩放 + 边际递减），且不超 100
        assert!(stats.skills[1] > before, "图书馆应提升八股技能");
        assert!(stats.skills[1] <= 100.0);
        // 消耗精力、心态小幅提升；其他资源不受影响
        assert_eq!(stats.energy, 40.0);
        assert_eq!(stats.mentality, 83.0);
        assert_eq!(stats.money, 2200.0);
        assert_eq!(stats.satiety, 80.0);
        // 有行为提示，且不触发失败结局
        assert!(has_toast(&ctx, "八股"));
        assert!(ctx.over.reason.is_none());
        assert!(!matches!(
            ctx.next_state,
            NextState::Pending(GameState::GameOver)
        ));
    }

    #[test]
    fn lab_boosts_project_skill() {
        let mut stats = PlayerStats::default();
        let mut ctx = Ctx::default();
        let before = stats.skills[2];

        act(HotspotKind::Lab, &mut stats, &mut ctx);

        // 项目技能上升，不超 100；精力消耗、心态小幅下降（赶项目疲劳）
        assert!(stats.skills[2] > before, "实验室应提升项目技能");
        assert!(stats.skills[2] <= 100.0);
        assert_eq!(stats.energy, 66.0);
        assert_eq!(stats.mentality, 78.0);
        // 钱与饱食不变，有提示，无失败结局
        assert_eq!(stats.money, 2200.0);
        assert_eq!(stats.satiety, 80.0);
        assert!(has_toast(&ctx, "项目"));
        assert!(ctx.over.reason.is_none());
        assert!(!matches!(
            ctx.next_state,
            NextState::Pending(GameState::GameOver)
        ));
    }

    #[test]
    fn campus_shop_buys_snacks() {
        let mut stats = PlayerStats {
            satiety: 50.0,
            money: 20.0,
            ..default()
        };
        let mut ctx = Ctx::default();

        act(HotspotKind::CampusShop, &mut stats, &mut ctx);

        // 饱食 +25、心态 +3、扣 ¥8；技能与精力不受影响
        assert_eq!(stats.satiety, 75.0);
        assert_eq!(stats.money, 12.0);
        assert_eq!(stats.mentality, 83.0);
        assert_eq!(stats.energy, 80.0);
        assert_eq!(stats.skills, [18.0; 5]);
        assert!(has_toast(&ctx, "小卖部"));
        // 小卖部属于觅食热点（饱食 < 30 时会被自动寻路到）
        assert!(HotspotKind::CampusShop.is_food());
        // 没饿死没破产
        assert!(ctx.over.reason.is_none());
        assert!(!matches!(
            ctx.next_state,
            NextState::Pending(GameState::GameOver)
        ));
    }

    #[test]
    fn campus_interactions_clamp_at_bounds() {
        // 精力见底时：图书馆扣精力不下 0；小卖部饱食封顶 100
        let mut stats = PlayerStats {
            energy: 5.0,
            satiety: 90.0,
            money: 10.0,
            skills: [99.0; 5],
            ..default()
        };
        let mut ctx = Ctx::default();
        act(HotspotKind::Library, &mut stats, &mut ctx);
        assert_eq!(stats.energy, 0.0, "精力不得低于 0");
        assert!(stats.skills[1] <= 100.0, "技能不得超 100");

        let mut stats2 = PlayerStats {
            satiety: 90.0,
            money: 10.0,
            ..default()
        };
        let mut ctx2 = Ctx::default();
        act(HotspotKind::CampusShop, &mut stats2, &mut ctx2);
        assert_eq!(stats2.satiety, 100.0, "饱食不得超 100");
        assert_eq!(stats2.money, 2.0);
        // 存款不足以买零食会触发破产结局
        let mut broke = PlayerStats {
            satiety: 50.0,
            money: 5.0,
            ..default()
        };
        let mut ctx3 = Ctx::default();
        act(HotspotKind::CampusShop, &mut broke, &mut ctx3);
        assert_eq!(broke.money, -3.0);
        assert!(matches!(ctx3.over.reason, Some(OverReason::Rent)));
        assert!(matches!(
            ctx3.next_state,
            NextState::Pending(GameState::GameOver)
        ));
    }

    #[test]
    fn tv_relaxes_mentality() {
        let mut stats = PlayerStats::default();
        let mut ctx = Ctx::default();

        act(HotspotKind::Tv, &mut stats, &mut ctx);

        // 心态 +10、精力 -3；钱/饱食/健康/技能不受影响
        assert_eq!(stats.mentality, 90.0);
        assert_eq!(stats.energy, 77.0);
        assert_eq!(stats.money, 2200.0);
        assert_eq!(stats.satiety, 80.0);
        assert_eq!(stats.health, 80.0);
        assert_eq!(stats.skills, [18.0; 5]);
        assert!(has_toast(&ctx, "追剧"));
        assert!(ctx.over.reason.is_none());
        assert!(!matches!(
            ctx.next_state,
            NextState::Pending(GameState::GameOver)
        ));
    }

    #[test]
    fn bathroom_restores_energy_health() {
        let mut stats = PlayerStats {
            energy: 30.0,
            health: 40.0,
            ..default()
        };
        let mut ctx = Ctx::default();

        act(HotspotKind::Bathroom, &mut stats, &mut ctx);

        // 心态 +6、精力 +5、健康 +3；不花钱
        assert_eq!(stats.mentality, 86.0);
        assert_eq!(stats.energy, 35.0);
        assert_eq!(stats.health, 43.0);
        assert_eq!(stats.money, 2200.0);
        assert!(has_toast(&ctx, "热水澡"));
        assert!(ctx.over.reason.is_none());
    }

    #[test]
    fn fridge_free_snack_and_clamps() {
        let mut stats = PlayerStats {
            satiety: 50.0,
            ..default()
        };
        let mut ctx = Ctx::default();

        act(HotspotKind::Fridge, &mut stats, &mut ctx);

        // 饱食 +20、免费；冰箱属于觅食热点（饱食 < 30 会自己来翻冰箱）
        assert_eq!(stats.satiety, 70.0);
        assert_eq!(stats.money, 2200.0);
        assert!(HotspotKind::Fridge.is_food());
        assert!(has_toast(&ctx, "冰箱"));
        assert!(ctx.over.reason.is_none());

        // 饱食接近满时封顶 100
        let mut full = PlayerStats {
            satiety: 90.0,
            ..default()
        };
        let mut ctx2 = Ctx::default();
        act(HotspotKind::Fridge, &mut full, &mut ctx2);
        assert_eq!(full.satiety, 100.0, "饱食不得超 100");
    }

    #[test]
    fn coffee_restores_energy() {
        let mut stats = PlayerStats {
            energy: 40.0,
            ..default()
        };
        let mut ctx = Ctx::default();

        act(HotspotKind::Coffee, &mut stats, &mut ctx);

        // 精力 +15、心态 +4；免费
        assert_eq!(stats.energy, 55.0);
        assert_eq!(stats.mentality, 84.0);
        assert_eq!(stats.money, 2200.0);
        assert_eq!(stats.skills, [18.0; 5]);
        assert!(has_toast(&ctx, "美式"));
        assert!(ctx.over.reason.is_none());

        // 精力接近满时封顶 100
        let mut full = PlayerStats {
            energy: 92.0,
            ..default()
        };
        let mut ctx2 = Ctx::default();
        act(HotspotKind::Coffee, &mut full, &mut ctx2);
        assert_eq!(full.energy, 100.0, "精力不得超 100");
    }

    #[test]
    fn meeting_boosts_social_skill() {
        let mut stats = PlayerStats::default();
        let mut ctx = Ctx::default();
        let before = stats.skills[3];

        act(HotspotKind::Meeting, &mut stats, &mut ctx);

        // 社交技能上升、精力 -5、心态 -2（开会疲惫）；不花钱
        assert!(stats.skills[3] > before, "周会应提升社交技能");
        assert!(stats.skills[3] <= 100.0);
        assert_eq!(stats.energy, 75.0);
        assert_eq!(stats.mentality, 78.0);
        assert_eq!(stats.money, 2200.0);
        assert!(has_toast(&ctx, "周会"));
        assert!(ctx.over.reason.is_none());
        assert!(!matches!(
            ctx.next_state,
            NextState::Pending(GameState::GameOver)
        ));
    }

    #[test]
    fn printer_boosts_resume_skill() {
        let mut stats = PlayerStats::default();
        let mut ctx = Ctx::default();
        let before = stats.skills[4];

        act(HotspotKind::Printer, &mut stats, &mut ctx);

        // 简历技能上升、精力 -3；不花钱
        assert!(stats.skills[4] > before, "打印应提升简历技能");
        assert!(stats.skills[4] <= 100.0);
        assert_eq!(stats.energy, 77.0);
        assert_eq!(stats.money, 2200.0);
        assert_eq!(stats.satiety, 80.0);
        assert!(has_toast(&ctx, "简历"));
        assert!(ctx.over.reason.is_none());
        assert!(!matches!(
            ctx.next_state,
            NextState::Pending(GameState::GameOver)
        ));
    }

    #[test]
    fn drink_machine_refreshes() {
        let mut stats = PlayerStats {
            satiety: 50.0,
            ..default()
        };
        let mut ctx = Ctx::default();

        act(HotspotKind::DrinkMachine, &mut stats, &mut ctx);

        // 饱食 +12、心态 +3、扣 ¥4；饮料机属于觅食热点
        assert_eq!(stats.satiety, 62.0);
        assert_eq!(stats.mentality, 83.0);
        assert_eq!(stats.money, 2196.0);
        assert_eq!(stats.health, 80.0);
        assert!(HotspotKind::DrinkMachine.is_food());
        assert!(has_toast(&ctx, "酸梅汤"));
        assert!(ctx.over.reason.is_none());
        assert!(!matches!(
            ctx.next_state,
            NextState::Pending(GameState::GameOver)
        ));
    }

    #[test]
    fn milk_tea_boosts_mentality() {
        let mut stats = PlayerStats::default();
        let mut ctx = Ctx::default();

        act(HotspotKind::MilkTea, &mut stats, &mut ctx);

        // 心态 +10、饱食 +10、扣 ¥12；其他资源不变
        assert_eq!(stats.mentality, 90.0);
        assert_eq!(stats.satiety, 90.0);
        assert_eq!(stats.money, 2188.0);
        assert_eq!(stats.health, 80.0);
        assert_eq!(stats.energy, 80.0);
        assert_eq!(stats.skills, [18.0; 5]);
        assert!(has_toast(&ctx, "奶茶"));
        assert!(ctx.over.reason.is_none());
        assert!(!matches!(
            ctx.next_state,
            NextState::Pending(GameState::GameOver)
        ));
    }

    #[test]
    fn fruit_stand_boosts_health() {
        let mut stats = PlayerStats::default();
        let mut ctx = Ctx::default();

        act(HotspotKind::FruitStand, &mut stats, &mut ctx);

        // 健康 +5、饱食 +8、扣 ¥6；水果摊属于觅食热点
        assert_eq!(stats.health, 85.0);
        assert_eq!(stats.satiety, 88.0);
        assert_eq!(stats.money, 2194.0);
        assert_eq!(stats.mentality, 80.0);
        assert!(HotspotKind::FruitStand.is_food());
        assert!(has_toast(&ctx, "水果"));
        assert!(ctx.over.reason.is_none());
        assert!(!matches!(
            ctx.next_state,
            NextState::Pending(GameState::GameOver)
        ));
    }

    #[test]
    fn dorm_bed_naps_during_day_and_sleeps_at_night() {
        // 白天补觉：效果弱于家里（+30 vs +40）
        let mut stats = PlayerStats {
            energy: 40.0,
            ..default()
        };
        let mut ctx = Ctx::default();
        act(HotspotKind::DormBed, &mut stats, &mut ctx);
        assert_eq!(stats.energy, 70.0);
        assert_eq!(stats.mentality, 83.0);
        assert!(has_toast(&ctx, "宿舍床"));
        assert!(ctx.over.reason.is_none());

        // 晚上睡觉：像家里一样推进天数
        let mut stats2 = PlayerStats {
            energy: 30.0,
            money: 10000.0,
            ..default()
        };
        let mut ctx2 = Ctx::default();
        ctx2.clock.phase = Phase::Evening;
        act(HotspotKind::DormBed, &mut stats2, &mut ctx2);
        assert_eq!(ctx2.clock.day, 2, "晚上在宿舍睡觉应推进天数");
        assert!(has_toast(&ctx2, "躺下睡觉"));
        assert!(ctx2.over.reason.is_none());
        assert!(!matches!(
            ctx2.next_state,
            NextState::Pending(GameState::GameOver)
        ));
    }

    #[test]
    fn dorm_game_boosts_mentality() {
        let mut stats = PlayerStats::default();
        let mut ctx = Ctx::default();

        act(HotspotKind::DormGame, &mut stats, &mut ctx);

        // 心态 +12、精力 -5；免费
        assert_eq!(stats.mentality, 92.0);
        assert_eq!(stats.energy, 75.0);
        assert_eq!(stats.money, 2200.0);
        assert_eq!(stats.satiety, 80.0);
        assert!(has_toast(&ctx, "游戏"));
        assert!(ctx.over.reason.is_none());
        assert!(!matches!(
            ctx.next_state,
            NextState::Pending(GameState::GameOver)
        ));
    }

    #[test]
    fn dorm_snack_free_and_is_food() {
        let mut stats = PlayerStats {
            satiety: 50.0,
            ..default()
        };
        let mut ctx = Ctx::default();

        act(HotspotKind::DormSnack, &mut stats, &mut ctx);

        // 饱食 +12、心态 +2；免费；零食柜属于觅食热点
        assert_eq!(stats.satiety, 62.0);
        assert_eq!(stats.mentality, 82.0);
        assert_eq!(stats.money, 2200.0);
        assert!(HotspotKind::DormSnack.is_food());
        assert!(has_toast(&ctx, "薯片"));
        assert!(ctx.over.reason.is_none());
        assert!(!matches!(
            ctx.next_state,
            NextState::Pending(GameState::GameOver)
        ));
    }

    #[test]
    fn night_market_only_opens_at_evening() {
        // 白天（上午）：夜市打烊，不产生效果、不扣钱
        let mut stats = PlayerStats {
            satiety: 50.0,
            ..default()
        };
        let mut ctx = Ctx::default(); // 默认 phase = Morning
        act(HotspotKind::NightMarket, &mut stats, &mut ctx);
        assert_eq!(stats.satiety, 50.0, "白天夜市打烊，不应回饱食");
        assert_eq!(stats.money, 2200.0, "白天夜市打烊，不应扣钱");
        assert!(has_toast(&ctx, "还没出摊"));

        // 晚上：出摊，饱食 +20、心态 +5、扣 ¥10
        let mut stats2 = PlayerStats {
            satiety: 50.0,
            ..default()
        };
        let mut ctx2 = Ctx::default();
        ctx2.clock.phase = Phase::Evening;
        act(HotspotKind::NightMarket, &mut stats2, &mut ctx2);
        assert_eq!(stats2.satiety, 70.0);
        assert_eq!(stats2.mentality, 85.0);
        assert_eq!(stats2.money, 2190.0);
        assert!(has_toast(&ctx2, "夜市"));
        assert!(ctx2.over.reason.is_none());
        assert!(!matches!(
            ctx2.next_state,
            NextState::Pending(GameState::GameOver)
        ));
    }

    #[test]
    fn lookout_boosts_mentality_free() {
        let mut stats = PlayerStats {
            mentality: 40.0,
            ..default()
        };
        let mut ctx = Ctx::default();

        act(HotspotKind::Lookout, &mut stats, &mut ctx);

        // 心态 +8、免费、其他资源不变
        assert_eq!(stats.mentality, 48.0);
        assert_eq!(stats.money, 2200.0);
        assert_eq!(stats.energy, 80.0);
        assert_eq!(stats.satiety, 80.0);
        assert_eq!(stats.skills, [18.0; 5]);
        assert!(has_toast(&ctx, "观景台"));
        assert!(ctx.over.reason.is_none());
        assert!(!matches!(
            ctx.next_state,
            NextState::Pending(GameState::GameOver)
        ));
    }

    #[test]
    fn graffiti_boosts_mentality_with_random_msg() {
        let mut stats = PlayerStats::default();
        let mut ctx = Ctx::default();

        act(HotspotKind::Graffiti, &mut stats, &mut ctx);

        // 心态 +5、免费；随机彩蛋台词不空
        assert_eq!(stats.mentality, 85.0);
        assert_eq!(stats.money, 2200.0);
        assert_eq!(ctx.toast.items.len(), 1, "应弹出一条涂鸦彩蛋");
        let (msg, _) = &ctx.toast.items[0];
        assert!(!msg.is_empty());
        assert!(ctx.over.reason.is_none());
        assert!(!matches!(
            ctx.next_state,
            NextState::Pending(GameState::GameOver)
        ));
    }
}
