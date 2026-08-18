//! NPC 与对话树数据：7 位 NPC（加上 NPC 0 的「选择聊天对象」列表共 8 个
//! 对话树条目）的颜色 / 标签 / 带分支的对话树，
//! （选项 next >= 1000 表示跳到另一个 NPC 的树），以及笔试题库 QUIZ。
//! 对话的选项 / 节点可以携带数值效果（DlgEffect），选择或读到时会结算属性变化。

use bevy::prelude::*;

// ==================== 对话树数据 ====================
// 特殊约定：选项的 next >= 1000 表示「跳转到另一个 NPC 的对话树」（1000 + NPC 下标）。
// next = None 表示结束对话。

/// 对话效果：选择带效果的选项、或读到带效果的节点时结算一次。
/// mentality = 心态变化；skill = (技能下标, 增量)，下标对应 resources::SKILL_NAMES
/// （0 算法 / 1 八股 / 2 项目 / 3 社交 / 4 简历）。
#[derive(Clone, Copy)]
pub struct DlgEffect {
    pub mentality: f32,
    pub skill: Option<(usize, f32)>,
}

impl DlgEffect {
    pub const fn ment(mentality: f32) -> Self {
        Self {
            mentality,
            skill: None,
        }
    }
    pub const fn skill(idx: usize, delta: f32) -> Self {
        Self {
            mentality: 0.0,
            skill: Some((idx, delta)),
        }
    }
}

pub struct DlgOption {
    pub label: &'static str,
    pub next: Option<usize>,
    pub effect: Option<DlgEffect>, // 选择该选项时结算
}

pub struct DlgNode {
    pub text: &'static str,
    pub options: &'static [DlgOption],
    pub effect: Option<DlgEffect>, // 读到该节点（进入时）结算
}

pub struct Npc {
    pub name: &'static str,
    pub tag: &'static str, // 身份标签
    pub color: Color,
    pub nodes: &'static [DlgNode],
}

pub const NPC_LIST_INDEX: usize = 0; // NPC 0 是「选择聊天对象」列表

pub const NPCS: &[Npc] = &[
    // ===== NPC 0：选择聊天对象（虚拟） =====
    Npc {
        name: "找人聊聊",
        tag: "不消耗行动槽位",
        color: Color::srgb(0.45, 0.45, 0.45),
        nodes: &[DlgNode {
            text: "翻开通讯录，想找谁聊聊？",
            options: &[
                DlgOption {
                    label: "邻居 赖哥（楼道）",
                    next: Some(1000 + 1),
                    effect: None,
                },
                DlgOption {
                    label: "导师 陈教授（校园）",
                    next: Some(1000 + 2),
                    effect: None,
                },
                DlgOption {
                    label: "妈妈（家）",
                    next: Some(1000 + 3),
                    effect: None,
                },
                DlgOption {
                    label: "Mentor 张哥（公司）",
                    next: Some(1000 + 4),
                    effect: None,
                },
                DlgOption {
                    label: "Leader 王总（公司）",
                    next: Some(1000 + 5),
                    effect: None,
                },
                DlgOption {
                    label: "产品 小赵（公司）",
                    next: Some(1000 + 6),
                    effect: None,
                },
                DlgOption {
                    label: "HR 李姐（公司）",
                    next: Some(1000 + 7),
                    effect: None,
                },
                DlgOption {
                    label: "不聊了",
                    next: None,
                    effect: None,
                },
            ],
            effect: None,
        }],
    },
    // ===== NPC 1：邻居赖哥 =====
    Npc {
        name: "赖哥",
        tag: "邻居 · 一起卷",
        color: Color::srgb(0.90, 0.60, 0.30),
        nodes: &[
            DlgNode {
                text: "赖哥拎着钥匙：『你知道吗，楼下二姨又说 AI 要取代程序员了，吓得我昨晚刷题到两点。』",
                options: &[
                    DlgOption {
                        label: "『AI 是工具，不是敌人。』",
                        next: Some(1),
                        effect: None,
                    },
                    DlgOption {
                        label: "『别慌，先卷个实习 offer 再说。』",
                        next: Some(2),
                        effect: None,
                    },
                    DlgOption {
                        label: "（拍拍他肩膀，不聊了）",
                        next: None,
                        effect: None,
                    },
                ],
                effect: None,
            },
            DlgNode {
                text: "『你说得对！我现在每天用 AI 帮我 review 代码，效率翻倍。你也试试？』",
                options: &[DlgOption {
                    label: "『好，回头我也研究下。』",
                    next: None,
                    effect: None,
                }],
                effect: None,
            },
            DlgNode {
                text: "『行，稳住！今晚图书馆见，一起背八股。』",
                options: &[DlgOption {
                    label: "『图书馆见！』",
                    next: None,
                    effect: None,
                }],
                effect: None,
            },
        ],
    },
    // ===== NPC 2：导师陈教授 =====
    Npc {
        name: "陈教授",
        tag: "导师",
        color: Color::srgb(0.35, 0.55, 0.85),
        nodes: &[
            DlgNode {
                text: "陈教授端着保温杯：『最近有个 AI 全流程笔试的内推名额，你要不要试试？项目经历我给你写推荐。』",
                options: &[
                    DlgOption {
                        label: "『要要要！谢谢老师！』（心态+5）",
                        next: Some(1),
                        effect: Some(DlgEffect::ment(5.0)),
                    },
                    DlgOption {
                        label: "『我再想想……』",
                        next: Some(2),
                        effect: None,
                    },
                ],
                effect: None,
            },
            DlgNode {
                text: "『好好准备，算法和项目两手抓。现在面试官人均问 RAG 和 LoRA，记得看看。』",
                options: &[DlgOption {
                    label: "『记住了，老师。』",
                    next: None,
                    effect: None,
                }],
                effect: None,
            },
            DlgNode {
                text: "『年轻人别太焦虑，路是一步一步走出来的。』",
                options: &[DlgOption {
                    label: "『嗯嗯。』",
                    next: None,
                    effect: None,
                }],
                effect: None,
            },
        ],
    },
    // ===== NPC 3：妈妈 =====
    Npc {
        name: "妈妈",
        tag: "家人",
        color: Color::srgb(0.90, 0.45, 0.45),
        nodes: &[
            DlgNode {
                text: "视频那头妈妈在择菜：『钱还够花吗？别老吃泡面，妈给你转点。』",
                options: &[
                    DlgOption {
                        label: "『够的够的，您别操心。』（心态+4）",
                        next: Some(1),
                        effect: Some(DlgEffect::ment(4.0)),
                    },
                    DlgOption {
                        label: "『妈，我最近面试被拒了好几次……』",
                        next: Some(2),
                        effect: None,
                    },
                ],
                effect: None,
            },
            DlgNode {
                text: "『那就好。天冷了多穿点，别熬夜。』",
                options: &[DlgOption {
                    label: "『知道啦，妈。』",
                    next: None,
                    effect: None,
                }],
                effect: None,
            },
            DlgNode {
                text: "『傻孩子，被拒了怕什么，咱家又不是没人疼你。妈给你寄了腊肠。』（心态+6）",
                options: &[DlgOption {
                    label: "『谢谢妈……』（鼻子一酸）",
                    next: None,
                    effect: None,
                }],
                effect: Some(DlgEffect::ment(6.0)),
            },
        ],
    },
    // ===== NPC 4：Mentor 张哥 =====
    Npc {
        name: "张哥",
        tag: "Mentor",
        color: Color::srgb(0.40, 0.70, 0.50),
        nodes: &[
            DlgNode {
                text: "张哥喝了口咖啡：『最近组里上了 AI 编程插件，老代码改起来快多了。你实习的活儿上手了吗？』",
                options: &[
                    DlgOption {
                        label: "『上手了，多亏您带的项目。』（心态+3）",
                        next: Some(1),
                        effect: Some(DlgEffect::ment(3.0)),
                    },
                    DlgOption {
                        label: "『有点吃力，需求改来改去……』",
                        next: Some(2),
                        effect: None,
                    },
                ],
                effect: None,
            },
            DlgNode {
                text: "『不错！转正答辩时把这段经历讲清楚，稳的。』",
                options: &[DlgOption {
                    label: "『张哥，转正答辩您可得帮我把把关。』",
                    next: Some(3),
                    effect: None,
                }],
                effect: None,
            },
            DlgNode {
                text: "『正常，产品改需求是常态。记住：先做核心链路，别被边角料拖死。』（心态+4）",
                options: &[DlgOption {
                    label: "『学到了。』",
                    next: None,
                    effect: None,
                }],
                effect: Some(DlgEffect::ment(4.0)),
            },
            DlgNode {
                text: "『没问题，答辩前咱们过一遍 PPT。加油！』（社交+2）",
                options: &[DlgOption {
                    label: "『谢谢张哥！』",
                    next: None,
                    effect: None,
                }],
                effect: Some(DlgEffect::skill(3, 2.0)),
            },
        ],
    },
    // ===== NPC 5：Leader 王总 =====
    Npc {
        name: "王总",
        tag: "Leader",
        color: Color::srgb(0.55, 0.45, 0.75),
        nodes: &[
            DlgNode {
                text: "王总路过工位：『小同学，最近公司搞降本增效，测试组缩编了一半。你那边效率还行吗？』",
                options: &[
                    DlgOption {
                        label: "『我尽量用自动化把重复活扛下来。』（社交+2）",
                        next: Some(1),
                        effect: Some(DlgEffect::skill(3, 2.0)),
                    },
                    DlgOption {
                        label: "『（心里一紧）还、还行……』",
                        next: Some(2),
                        effect: None,
                    },
                ],
                effect: None,
            },
            DlgNode {
                text: "『有想法！会用工具的人走到哪都不怕。好好干。』（心态+4）",
                options: &[DlgOption {
                    label: "『谢谢王总！』",
                    next: None,
                    effect: None,
                }],
                effect: Some(DlgEffect::ment(4.0)),
            },
            DlgNode {
                text: "『年轻人别慌，把你该做的做好就行。』（心态+2）",
                options: &[DlgOption {
                    label: "『好的……』",
                    next: None,
                    effect: None,
                }],
                effect: Some(DlgEffect::ment(2.0)),
            },
        ],
    },
    // ===== NPC 6：产品小赵 =====
    Npc {
        name: "小赵",
        tag: "产品",
        color: Color::srgb(0.85, 0.60, 0.90),
        nodes: &[
            DlgNode {
                text: "小赵抱着一摞文档：『又改需求了……这次要把「AI 助手」塞进老模块。你看还有救吗？』",
                options: &[
                    DlgOption {
                        label: "『我看看，能救。先定最小闭环。』（项目+1）",
                        next: Some(1),
                        effect: Some(DlgEffect::skill(2, 1.0)),
                    },
                    DlgOption {
                        label: "『……我选择沉默。』",
                        next: Some(2),
                        effect: None,
                    },
                ],
                effect: None,
            },
            DlgNode {
                text: "『靠谱！需求评审请你喝奶茶。』（心态+3）",
                options: &[DlgOption {
                    label: "『那我可记下了。』",
                    next: None,
                    effect: None,
                }],
                effect: Some(DlgEffect::ment(3.0)),
            },
            DlgNode {
                text: "『别这样，改需求是工作，改完需求是生活。』（心态+2）",
                options: &[DlgOption {
                    label: "『有道理……个鬼。』",
                    next: None,
                    effect: None,
                }],
                effect: Some(DlgEffect::ment(2.0)),
            },
        ],
    },
    // ===== NPC 7：HR 李姐 =====
    Npc {
        name: "李姐",
        tag: "HR",
        color: Color::srgb(0.85, 0.50, 0.55),
        nodes: &[
            DlgNode {
                text: "李姐端着咖啡：『小伙子简历写得不错！不过面试官反馈说 AI 工具这块要再多准备准备。』",
                options: &[
                    DlgOption {
                        label: "『谢谢李姐，我回去补补课。』（心态+3）",
                        next: Some(1),
                        effect: Some(DlgEffect::ment(3.0)),
                    },
                    DlgOption {
                        label: "『李姐，秋招这边有内推吗？』",
                        next: Some(2),
                        effect: None,
                    },
                ],
                effect: None,
            },
            DlgNode {
                text: "『态度不错，机会留给有准备的人。』",
                options: &[DlgOption {
                    label: "『嗯！』",
                    next: None,
                    effect: None,
                }],
                effect: None,
            },
            DlgNode {
                text: "『有！你这种踏实肯干的，我第一个推。简历记得突出项目里的 AI 实践。』（社交+2）",
                options: &[DlgOption {
                    label: "『太感谢了！』",
                    next: None,
                    effect: None,
                }],
                effect: Some(DlgEffect::skill(3, 2.0)),
            },
        ],
    },
];

// ==================== 笔试题库 ====================
pub struct QuizQ {
    pub q: &'static str,
    pub opts: [&'static str; 3],
    pub correct: usize,
}

pub const QUIZ: &[QuizQ] = &[
    QuizQ {
        q: "Transformer 中自注意力（Self-Attention）的核心计算是？",
        opts: [
            "Q·Kᵀ 再 softmax 后乘 V",
            "直接对输入做卷积",
            "按位置逐元素相加",
        ],
        correct: 0,
    },
    QuizQ {
        q: "「Token」在 LLM 语境下通常指？",
        opts: ["一段加密口令", "文本被切分后的最小单元", "模型权重文件"],
        correct: 1,
    },
    QuizQ {
        q: "RAG（检索增强生成）主要解决 LLM 的什么问题？",
        opts: ["训练太慢", "幻觉与知识时效性不足", "显存占用过高"],
        correct: 1,
    },
    QuizQ {
        q: "Embedding 向量的主要作用是什么？",
        opts: [
            "把离散词映射为稠密向量以计算语义相似度",
            "压缩模型体积",
            "加密用户数据",
        ],
        correct: 0,
    },
    QuizQ {
        q: "LoRA 微调相比全量微调的优势是？",
        opts: ["效果一定更好", "只训练少量低秩参数，省显存", "不需要数据"],
        correct: 1,
    },
    QuizQ {
        q: "KV Cache 用于优化什么？",
        opts: ["生成阶段的重复注意力计算", "训练阶段的梯度下降", "模型量化"],
        correct: 0,
    },
    QuizQ {
        q: "temperature 参数调高，模型输出会？",
        opts: ["更保守、更确定", "更随机、更多样", "变长"],
        correct: 1,
    },
    QuizQ {
        q: "快排的平均时间复杂度是？",
        opts: ["O(n)", "O(n log n)", "O(n²)"],
        correct: 1,
    },
    QuizQ {
        q: "TCP 三次握手的目的是？",
        opts: ["确认双方收发能力并同步序列号", "加密传输", "压缩数据"],
        correct: 0,
    },
    QuizQ {
        q: "「微调 vs RAG」通常如何选择？",
        opts: [
            "RAG 能更新知识、成本低，微调更适合固定风格/能力",
            "微调总能替代 RAG",
            "两者完全等价",
        ],
        correct: 0,
    },
];
