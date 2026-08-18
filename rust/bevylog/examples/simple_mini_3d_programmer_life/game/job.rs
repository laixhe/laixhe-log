//! 求职管线：12 家公司（三个档次）的数据、投递 → 笔试 → 面试 → Offer
//! 的逐日推进逻辑（job_advance_system，每天在 advance_day 中触发）。

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::router::GameState;

use super::npc::QUIZ;
use super::progression;
use super::resources::*;
use rand::RngExt;

// ==================== 求职判定参数（调参看这里） ====================
// 面试通过率 = 基础概率 + 技能贡献（技能分 / 100 × 权重）- 公司档次惩罚，夹取到 [MIN, MAX]。
const STAGE_BASE_SUBMIT: f32 = 0.999; // 投递阶段：必进笔试
const IV1_BASE: f32 = 0.35; // 技术一面基础通过率
const IV2_BASE: f32 = 0.30; // 项目二面基础通过率
const HR_BASE: f32 = 0.35; // HR 面基础通过率
const IV1_W_ALGO: f32 = 0.30; // 一面：算法贡献权重
const IV1_W_BAGU: f32 = 0.20; // 一面：八股贡献权重
const IV1_W_PROJ: f32 = 0.20; // 一面：项目贡献权重
const IV2_W_PROJ: f32 = 0.35; // 二面：项目贡献权重
const IV2_W_SOCIAL: f32 = 0.20; // 二面：社交贡献权重
const HR_W_SOCIAL: f32 = 0.30; // HR 面：社交贡献权重
const HR_W_MENT: f32 = 0.20; // HR 面：心态贡献权重
const HR_W_RESUME: f32 = 0.20; // HR 面：简历贡献权重
const TIER_PENALTY_IV1: f32 = 0.05; // 公司每高一档，一面通过率降低
const TIER_PENALTY_IV2: f32 = 0.04;
const TIER_PENALTY_HR: f32 = 0.03;
const STAGE_PROB_MIN: f32 = 0.05; // 通过率下限（保底）
const STAGE_PROB_MAX: f32 = 0.95; // 通过率上限
const WRITTEN_W_ALGO: f32 = 0.30; // 笔试：算法贡献权重
const WRITTEN_W_BAGU: f32 = 0.25; // 笔试：八股贡献权重
const WRITTEN_W_RESUME: f32 = 0.15; // 笔试：简历贡献权重
const WRITTEN_PASS_BONUS: f32 = 0.25; // 笔试答对通过率加成
const WRITTEN_FAIL_PENALTY: f32 = -0.15; // 笔试答错惩罚

// ==================== 12 家公司（三个档次） ====================
#[derive(Clone, Copy)]
pub struct Company {
    pub name: &'static str,
    pub tier: u32,    // 0 大厂 / 1 中厂 / 2 小厂
    pub salary: f32,  // 正式 offer 日薪
    pub written: f32, // 笔试基准通过率
    pub reply: u32,   // 投递后回复天数
}

pub const TIER_NAMES: [&str; 3] = ["大厂", "中厂", "小厂"];

pub const COMPANIES: &[Company] = &[
    // 大厂：钱多，但难
    Company {
        name: "鹅厂互动",
        tier: 0,
        salary: 800.0,
        written: 0.30,
        reply: 2,
    },
    Company {
        name: "字杰跳动",
        tier: 0,
        salary: 800.0,
        written: 0.28,
        reply: 2,
    },
    Company {
        name: "拼夕夕",
        tier: 0,
        salary: 850.0,
        written: 0.25,
        reply: 2,
    },
    Company {
        name: "米哈油",
        tier: 0,
        salary: 900.0,
        written: 0.22,
        reply: 2,
    },
    // 中厂
    Company {
        name: "网一互娱",
        tier: 1,
        salary: 600.0,
        written: 0.45,
        reply: 2,
    },
    Company {
        name: "快守科技",
        tier: 1,
        salary: 550.0,
        written: 0.48,
        reply: 1,
    },
    Company {
        name: "大疆智造",
        tier: 1,
        salary: 650.0,
        written: 0.42,
        reply: 2,
    },
    Company {
        name: "商汤智脑",
        tier: 1,
        salary: 620.0,
        written: 0.44,
        reply: 2,
    },
    // 小厂
    Company {
        name: "金蝶云",
        tier: 2,
        salary: 450.0,
        written: 0.60,
        reply: 2,
    },
    Company {
        name: "用友网络",
        tier: 2,
        salary: 430.0,
        written: 0.62,
        reply: 2,
    },
    Company {
        name: "深信服",
        tier: 2,
        salary: 480.0,
        written: 0.58,
        reply: 1,
    },
    Company {
        name: "紫光云",
        tier: 2,
        salary: 400.0,
        written: 0.65,
        reply: 1,
    },
];

// ==================== 投递状态 ====================
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum AppStage {
    Submitted,  // 已投递，等回复
    Written,    // 笔试
    Interview1, // 技术一面
    Interview2, // 项目二面
    Hr,         // HR 三面
    Offer,      // 拿到 offer
    Rejected,   // 已拒绝
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Application {
    pub company: usize,
    pub stage: AppStage,
    pub wait: u32, // 当前阶段已等待天数
}

#[derive(Resource, Default, Clone, Serialize, Deserialize)]
pub struct JobPipeline {
    pub apps: Vec<Application>,
}

fn stage_reply_days(stage: AppStage, comp: &Company) -> u32 {
    match stage {
        AppStage::Submitted => comp.reply, // 投递回复天数随公司档次不同
        AppStage::Written => 2,
        AppStage::Interview1 => 1,
        AppStage::Interview2 => 1,
        AppStage::Hr => 2,
        _ => 0,
    }
}

impl Application {
    pub fn status_text(&self) -> String {
        let comp = &COMPANIES[self.company];
        match self.stage {
            AppStage::Submitted => {
                let remain = stage_reply_days(self.stage, comp).saturating_sub(self.wait);
                format!("等待回复（{remain}d）")
            }
            AppStage::Written => "笔试中…".to_string(),
            AppStage::Interview1 => "一面待安排".to_string(),
            AppStage::Interview2 => "二面待安排".to_string(),
            AppStage::Hr => "HR 面待安排".to_string(),
            AppStage::Offer => format!("✅ Offer（日薪 ¥{}）", comp.salary),
            AppStage::Rejected => "❌ 已拒绝".to_string(),
        }
    }
}

impl JobPipeline {
    pub fn applied_to(&self, company: usize) -> Option<usize> {
        self.apps
            .iter()
            .position(|a| a.company == company && a.stage != AppStage::Rejected)
    }
}

// ==================== 投递 ====================
pub fn apply_to(
    company: usize,
    pipeline: &mut JobPipeline,
    flags: &mut GameFlags,
    toast: &mut ToastLog,
    stats: &mut PlayerStats,
) {
    if pipeline.applied_to(company).is_some() {
        toast.push("你已经投过这家了");
        return;
    }
    pipeline.apps.push(Application {
        company,
        stage: AppStage::Submitted,
        wait: 0,
    });
    flags.applied_count += 1;
    change(&mut stats.energy, -4.0);
    info!("[求职] 投递【{}】", COMPANIES[company].name);
    toast.push(format!(
        "已向【{}】投出简历（{}），等回复…",
        COMPANIES[company].name, TIER_NAMES[COMPANIES[company].tier as usize]
    ));
}

// ==================== 每日推进求职进展 ====================
// 每天晨间推进一次：等待天数 +1，到期的阶段出结果。
// 笔试阶段弹答题弹窗（答完再判定）；其余阶段按技能概率自动判定。
pub fn advance_apps(
    clock: &GameClock,
    pipeline: &mut JobPipeline,
    stats: &mut PlayerStats,
    flags: &mut GameFlags,
    toast: &mut ToastLog,
    modal: &mut Modal,
    quiz: &mut QuizState,
) {
    let mut rng = rand::rng();
    let chapter = clock.chapter();

    for app in pipeline.apps.iter_mut() {
        if matches!(app.stage, AppStage::Offer | AppStage::Rejected) {
            continue;
        }
        app.wait += 1;
    }

    // 从最早的投递开始，一次只处理一个需要答题的笔试
    for i in 0..pipeline.apps.len() {
        if matches!(pipeline.apps[i].stage, AppStage::Offer | AppStage::Rejected) {
            continue; // 已结束的投递不再推进
        }
        let due = pipeline.apps[i].wait
            >= stage_reply_days(pipeline.apps[i].stage, &COMPANIES[pipeline.apps[i].company]);
        if !due {
            continue;
        }
        let stage = pipeline.apps[i].stage;
        if stage == AppStage::Written {
            // 笔试：弹答题弹窗（答完再判定）
            quiz.app = i;
            quiz.q = rng.random_range(0..QUIZ.len());
            modal.open(ModalKind::Quiz);
            toast.push(format!(
                "【{}】笔试链接已发来，快答题！",
                COMPANIES[pipeline.apps[i].company].name
            ));
            return;
        }
        let outcome = roll_stage(stage, &COMPANIES[pipeline.apps[i].company], stats, &mut rng);
        advance_stage(i, outcome, pipeline, flags, toast, stats, chapter);
    }
}

// 非笔试阶段的判定：返回 true 表示通过
fn roll_stage(
    stage: AppStage,
    comp: &Company,
    stats: &PlayerStats,
    rng: &mut rand::rngs::ThreadRng,
) -> bool {
    let s = &stats.skills;
    let prob = match stage {
        AppStage::Submitted => STAGE_BASE_SUBMIT, // 投递必进笔试
        AppStage::Interview1 => (IV1_BASE + (s[0] * IV1_W_ALGO + s[1] * IV1_W_BAGU + s[2] * IV1_W_PROJ) / 100.0
            - comp.tier as f32 * TIER_PENALTY_IV1)
            .clamp(STAGE_PROB_MIN, STAGE_PROB_MAX),
        AppStage::Interview2 => {
            (IV2_BASE + (s[2] * IV2_W_PROJ + s[3] * IV2_W_SOCIAL) / 100.0 - comp.tier as f32 * TIER_PENALTY_IV2)
                .clamp(STAGE_PROB_MIN, STAGE_PROB_MAX)
        }
        AppStage::Hr => (HR_BASE + (s[3] * HR_W_SOCIAL + stats.mentality * HR_W_MENT + s[4] * HR_W_RESUME) / 100.0
            - comp.tier as f32 * TIER_PENALTY_HR)
            .clamp(STAGE_PROB_MIN, STAGE_PROB_MAX),
        _ => 0.0,
    };
    rng.random::<f32>() < prob
}

// 笔试判定：答对 +25% 通过率，答错 -15%
pub fn resolve_written(
    app_idx: usize,
    correct: bool,
    pipeline: &mut JobPipeline,
    stats: &mut PlayerStats,
    flags: &mut GameFlags,
    toast: &mut ToastLog,
    chapter: u32,
) {
    let comp = COMPANIES[pipeline.apps[app_idx].company];
    let s = &stats.skills;
    let quiz_bonus = if correct { WRITTEN_PASS_BONUS } else { WRITTEN_FAIL_PENALTY };
    let prob = (comp.written
        + (s[0] * WRITTEN_W_ALGO + s[1] * WRITTEN_W_BAGU + s[4] * WRITTEN_W_RESUME) / 100.0
        + quiz_bonus)
        .clamp(STAGE_PROB_MIN, STAGE_PROB_MAX);
    let mut rng = rand::rng();
    let pass = rng.random::<f32>() < prob;
    let name = comp.name;
    info!("[求职] 【{name}】笔试判定 pass={pass} prob={prob:.2}");
    toast.push(if correct {
        "笔试作答完成，答案漂亮！".to_string()
    } else {
        "笔试有一题答错了，影响发挥……".to_string()
    });
    advance_stage(app_idx, pass, pipeline, flags, toast, stats, chapter);
    if pass {
        toast.push(format!("【{name}】笔试通过，进入技术一面！"));
    } else {
        toast.push(format!("【{name}】笔试未通过，继续加油。"));
    }
}

// 统一推进：通过 → 下一阶段 / 拿 offer；失败 → 拒绝
fn advance_stage(
    idx: usize,
    pass: bool,
    pipeline: &mut JobPipeline,
    flags: &mut GameFlags,
    toast: &mut ToastLog,
    stats: &mut PlayerStats,
    chapter: u32,
) {
    let stage = pipeline.apps[idx].stage;
    let comp = COMPANIES[pipeline.apps[idx].company];
    if !pass {
        pipeline.apps[idx].stage = AppStage::Rejected;
        flags.rejected_count += 1;
        change(&mut stats.mentality, -8.0);
        info!("[求职] 【{}】{:?} 未通过，已拒绝", comp.name, stage);
        toast.push(format!("【{}】挂了，心态 -8。", comp.name));
        return;
    }
    info!("[求职] 【{}】{:?} 通过", comp.name, stage);
    let next = match stage {
        AppStage::Submitted => Some(AppStage::Written),
        AppStage::Written => Some(AppStage::Interview1),
        AppStage::Interview1 => Some(AppStage::Interview2),
        AppStage::Interview2 => Some(AppStage::Hr),
        AppStage::Hr => {
            // 拿到 offer
            pipeline.apps[idx].stage = AppStage::Offer;
            if chapter <= 2 {
                flags.intern_offer = true;
                info!("[求职] 🎉 收到【{}】实习 Offer", comp.name);
                toast.push(format!("🎉 收到【{}】实习 Offer！（日薪 ¥150）", comp.name));
            } else {
                flags.formal_offer = true;
                if comp.tier < flags.best_tier || flags.best_tier == u32::MAX {
                    flags.best_tier = comp.tier;
                }
                flags.salary = flags.salary.max(comp.salary);
                info!(
                    "[求职] 🎉 收到【{}】正式 Offer，日薪 ¥{}",
                    comp.name, comp.salary
                );
                toast.push(format!(
                    "🎉 收到【{}】正式 Offer！（日薪 ¥{}）",
                    comp.name, comp.salary
                ));
            }
            return;
        }
        _ => None,
    };
    if let Some(n) = next {
        pipeline.apps[idx].stage = n;
        pipeline.apps[idx].wait = 0;
    }
}

// ==================== 求职进展推进系统（每天执行一次） ====================
// 用 JobAdvanceStamp 资源记录上次处理的 (force, week, day)，避免每帧重复推进；
// force 保证重开新档也会推进。之所以存 Resource（并随存档序列化）而非 Local：
// Local 在进程重启（读档）后会归零，导致读档当天再推进一次。
#[allow(clippy::too_many_arguments)]
pub fn job_advance_system(
    clock: Res<GameClock>,
    force: Res<SceneForce>,
    mut pipeline: ResMut<JobPipeline>,
    mut stats: ResMut<PlayerStats>,
    mut flags: ResMut<GameFlags>,
    mut toast: ResMut<ToastLog>,
    mut modal: ResMut<Modal>,
    mut quiz: ResMut<QuizState>,
    mut stamp: ResMut<JobAdvanceStamp>,
    mut over: ResMut<OverInfo>,
    mut ending: ResMut<Ending>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let key = (force.0, clock.week, clock.day);
    if stamp.0 == Some(key) {
        return;
    }
    stamp.0 = Some(key);
    debug!(
        "[求职] 每日推进 第{}周 第{}天（共 {} 家申请）",
        clock.week,
        clock.day,
        pipeline.apps.len()
    );
    advance_apps(
        &clock,
        &mut pipeline,
        &mut stats,
        &mut flags,
        &mut toast,
        &mut modal,
        &mut quiz,
    );
    // 求职被拒心态 -8，可能扣到 0 → 立即判定结局（避免延迟到下次行为）
    progression::check_over(&stats, &mut over, &mut ending, &mut next_state);
}
