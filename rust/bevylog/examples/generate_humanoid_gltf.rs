//! 工具脚本：生成一个多关节的人形 gltf 模型（用于 example_animation_humanoid）。
//!
//! 运行方式：`cargo run --example generate_humanoid_gltf`
//! 产物：
//!   - assets/models/humanoid.gltf
//!   - assets/models/humanoid.bin
//!
//! 模型结构：一个由 15 根骨骼驱动的「火柴人」，含躯干 / 头 / 双臂 / 双腿：
//!   - 躯干 pelvis → spine → head（竖直链）
//!   - 双臂 shoulder → elbow → wrist（左右对称）
//!   - 双腿 hip → knee → ankle（左右对称）
//!
//! 每个肢体是一段网格，关节处顶点绑定「上下两根骨骼各 50% 权重」，
//! 因此骨骼弯曲时网格会在关节处平滑过渡，展示真正的「蒙皮权重混合」效果。
//!
//! 动画：三个 clip，用于对比不同速度 / 幅度下的蒙皮表现：
//!   - Walk（走）：周期 1.0s，肩/髋摆幅 ±30°，肘/膝弯曲 -60°
//!   - Run（跑）：周期 0.6s，肩/髋摆幅 ±50°，肘/膝弯曲 -90°（更快更大幅度）
//!   - Idle（静止）：所有关节 0°，回到站立姿态（配合过渡实现「跑动→静止」）

use std::fs;

// 每根骨骼：名字、父骨骼索引（None 表示根）、绑定姿态下的世界坐标。
struct Joint {
    name: &'static str,
    parent: Option<usize>,
    pos: [f32; 3],
}

// 15 根骨骼，索引 0..14。
const JOINTS: &[Joint] = &[
    Joint {
        name: "pelvis",
        parent: None,
        pos: [0.0, 1.0, 0.0],
    },
    Joint {
        name: "spine",
        parent: Some(0),
        pos: [0.0, 1.4, 0.0],
    },
    Joint {
        name: "head",
        parent: Some(1),
        pos: [0.0, 1.7, 0.0],
    },
    Joint {
        name: "l_shoulder",
        parent: Some(1),
        pos: [-0.45, 1.4, 0.0],
    },
    Joint {
        name: "l_elbow",
        parent: Some(3),
        pos: [-0.45, 1.0, 0.0],
    },
    Joint {
        name: "l_wrist",
        parent: Some(4),
        pos: [-0.45, 0.6, 0.0],
    },
    Joint {
        name: "r_shoulder",
        parent: Some(1),
        pos: [0.45, 1.4, 0.0],
    },
    Joint {
        name: "r_elbow",
        parent: Some(6),
        pos: [0.45, 1.0, 0.0],
    },
    Joint {
        name: "r_wrist",
        parent: Some(7),
        pos: [0.45, 0.6, 0.0],
    },
    Joint {
        name: "l_hip",
        parent: Some(0),
        pos: [-0.22, 1.0, 0.0],
    },
    Joint {
        name: "l_knee",
        parent: Some(9),
        pos: [-0.22, 0.5, 0.0],
    },
    Joint {
        name: "l_ankle",
        parent: Some(10),
        pos: [-0.22, 0.0, 0.0],
    },
    Joint {
        name: "r_hip",
        parent: Some(0),
        pos: [0.22, 1.0, 0.0],
    },
    Joint {
        name: "r_knee",
        parent: Some(12),
        pos: [0.22, 0.5, 0.0],
    },
    Joint {
        name: "r_ankle",
        parent: Some(13),
        pos: [0.22, 0.0, 0.0],
    },
];

// 一段肢体：连接 start/end 两根骨骼，width 是该段网格的宽度。
struct Segment {
    start: usize,
    end: usize,
    width: f32,
}

// 10 段肢体：躯干 + 头 + 左右上臂/前臂 + 左右大腿/小腿。
const SEGMENTS: &[Segment] = &[
    Segment {
        start: 0,
        end: 1,
        width: 0.35,
    }, // 躯干 pelvis → spine
    Segment {
        start: 1,
        end: 2,
        width: 0.28,
    }, // 头 spine → head
    Segment {
        start: 3,
        end: 4,
        width: 0.18,
    }, // 左上臂
    Segment {
        start: 4,
        end: 5,
        width: 0.15,
    }, // 左前臂
    Segment {
        start: 6,
        end: 7,
        width: 0.18,
    }, // 右上臂
    Segment {
        start: 7,
        end: 8,
        width: 0.15,
    }, // 右前臂
    Segment {
        start: 9,
        end: 10,
        width: 0.22,
    }, // 左大腿
    Segment {
        start: 10,
        end: 11,
        width: 0.16,
    }, // 左小腿
    Segment {
        start: 12,
        end: 13,
        width: 0.22,
    }, // 右大腿
    Segment {
        start: 13,
        end: 14,
        width: 0.16,
    }, // 右小腿
];

// 8 个动画通道对应的骨骼索引（顺序固定）：
// l_shoulder, r_shoulder, l_hip, r_hip, l_elbow, r_elbow, l_knee, r_knee
const ANIM_JOINTS: [usize; 8] = [3, 6, 9, 12, 4, 7, 10, 13];

struct Anim {
    name: &'static str,
    // 三个关键帧的时间点（秒），决定动画速度（周期越短越快）。
    times: [f32; 3],
    // 8 个通道 × 3 个关键帧的绕 X 轴旋转角度（度）。
    angles: [[f32; 3]; 8],
}

const ANIMATIONS: &[Anim] = &[
    Anim {
        name: "Walk",
        times: [0.0, 0.5, 1.0],
        angles: [
            [-30.0, 30.0, -30.0], // l_shoulder
            [30.0, -30.0, 30.0],  // r_shoulder
            [30.0, -30.0, 30.0],  // l_hip
            [-30.0, 30.0, -30.0], // r_hip
            [0.0, -60.0, 0.0],    // l_elbow
            [-60.0, 0.0, -60.0],  // r_elbow
            [0.0, -60.0, 0.0],    // l_knee
            [-60.0, 0.0, -60.0],  // r_knee
        ],
    },
    Anim {
        name: "Run",
        times: [0.0, 0.3, 0.6],
        angles: [
            [-50.0, 50.0, -50.0], // l_shoulder
            [50.0, -50.0, 50.0],  // r_shoulder
            [50.0, -50.0, 50.0],  // l_hip
            [-50.0, 50.0, -50.0], // r_hip
            [0.0, -90.0, 0.0],    // l_elbow
            [-90.0, 0.0, -90.0],  // r_elbow
            [0.0, -90.0, 0.0],    // l_knee
            [-90.0, 0.0, -90.0],  // r_knee
        ],
    },
    Anim {
        name: "Idle",
        times: [0.0, 0.5, 1.0],
        angles: [
            [0.0, 0.0, 0.0], // l_shoulder
            [0.0, 0.0, 0.0], // r_shoulder
            [0.0, 0.0, 0.0], // l_hip
            [0.0, 0.0, 0.0], // r_hip
            [0.0, 0.0, 0.0], // l_elbow
            [0.0, 0.0, 0.0], // r_elbow
            [0.0, 0.0, 0.0], // l_knee
            [0.0, 0.0, 0.0], // r_knee
        ],
    },
];

fn push_f32(buf: &mut Vec<u8>, v: f32) {
    buf.extend_from_slice(&v.to_le_bytes());
}

fn push_u16(buf: &mut Vec<u8>, v: u16) {
    buf.extend_from_slice(&v.to_le_bytes());
}

// 绕 X 轴旋转 deg 度对应的四元数 [x, y, z, w]。
fn quat_x(deg: f32) -> [f32; 4] {
    let half = deg.to_radians() * 0.5;
    [half.sin(), 0.0, 0.0, half.cos()]
}

fn main() {
    // 1. 生成网格：每段 3 排 × 2 列 = 6 顶点，中间排权重 0.5/0.5。
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut joints0: Vec<[u16; 4]> = Vec::new();
    let mut weights0: Vec<[f32; 4]> = Vec::new();
    let mut indices: Vec<u16> = Vec::new();

    for seg in SEGMENTS {
        let sp = JOINTS[seg.start].pos;
        let ep = JOINTS[seg.end].pos;
        let cx = (sp[0] + ep[0]) * 0.5;
        let base = positions.len() as u16;

        for row in 0..3u16 {
            let t = row as f32 * 0.5;
            let y = sp[1] + (ep[1] - sp[1]) * t;
            for col in 0..2u16 {
                let x = cx - seg.width * 0.5 + col as f32 * seg.width;
                positions.push([x, y, 0.0]);
                match row {
                    // 顶端：完全绑 start 骨骼
                    0 => {
                        joints0.push([seg.start as u16, 0, 0, 0]);
                        weights0.push([1.0, 0.0, 0.0, 0.0]);
                    }
                    // 中间：start/end 各 50%，实现关节处平滑蒙皮
                    1 => {
                        joints0.push([seg.start as u16, seg.end as u16, 0, 0]);
                        weights0.push([0.5, 0.5, 0.0, 0.0]);
                    }
                    // 底端：完全绑 end 骨骼
                    _ => {
                        joints0.push([seg.end as u16, 0, 0, 0]);
                        weights0.push([1.0, 0.0, 0.0, 0.0]);
                    }
                }
            }
        }

        // 两排四边形共 4 个三角形（顶点顺序：row0 左右、row1 左右、row2 左右）。
        for i in [0u16, 1, 2, 2, 1, 3, 2, 3, 4, 4, 3, 5] {
            indices.push(base + i);
        }
    }

    let vertex_count = positions.len();
    let index_count = indices.len();

    // 2. 计算 POSITION 包围盒（写入 accessor 的 min/max）。
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    for p in &positions {
        for k in 0..3 {
            min[k] = min[k].min(p[k]);
            max[k] = max[k].max(p[k]);
        }
    }

    // 3. 写入二进制 buffer，记录每一段的偏移与长度。
    let mut bin: Vec<u8> = Vec::new();

    // 3.1 POSITION（vec3 float）
    let off_pos = bin.len();
    for p in &positions {
        for v in *p {
            push_f32(&mut bin, v);
        }
    }
    let len_pos = bin.len() - off_pos;

    // 3.2 NORMAL（vec3 float，全部 +Z）
    let off_normal = bin.len();
    for _ in 0..vertex_count {
        for v in [0.0f32, 0.0, 1.0] {
            push_f32(&mut bin, v);
        }
    }
    let len_normal = bin.len() - off_normal;

    // 3.3 JOINTS_0（vec4 uint16）
    let off_joints = bin.len();
    for j in &joints0 {
        for v in *j {
            push_u16(&mut bin, v);
        }
    }
    let len_joints = bin.len() - off_joints;

    // 3.4 WEIGHTS_0（vec4 float）
    let off_weights = bin.len();
    for w in &weights0 {
        for v in *w {
            push_f32(&mut bin, v);
        }
    }
    let len_weights = bin.len() - off_weights;

    // 3.5 索引（uint16）
    let off_indices = bin.len();
    for i in &indices {
        push_u16(&mut bin, *i);
    }
    let len_indices = bin.len() - off_indices;

    // 3.6 inverseBindMatrices（每骨骼一个 mat4，列主序；绑定姿态只有平移 → 逆 = 平移逆）
    let off_ibm = bin.len();
    for j in JOINTS {
        let p = j.pos;
        for v in [
            1.0f32, 0.0, 0.0, 0.0, // 列0
            0.0, 1.0, 0.0, 0.0, // 列1
            0.0, 0.0, 1.0, 0.0, // 列2
            -p[0], -p[1], -p[2], 1.0, // 列3（平移逆）
        ] {
            push_f32(&mut bin, v);
        }
    }
    let len_ibm = bin.len() - off_ibm;

    // 3.7 每个动画：时间轴（3 float）+ 8 通道输出（每通道 3 个 vec4 四元数）
    let mut anim_offsets: Vec<(usize, usize, usize, usize)> = Vec::new(); // (time_off, time_len, out_off, out_each)
    for anim in ANIMATIONS {
        let time_off = bin.len();
        for t in anim.times {
            push_f32(&mut bin, t);
        }
        let time_len = bin.len() - time_off;

        let out_off = bin.len();
        for angles in anim.angles {
            for a in angles {
                for v in quat_x(a) {
                    push_f32(&mut bin, v);
                }
            }
        }
        let out_each = (bin.len() - out_off) / 8;

        anim_offsets.push((time_off, time_len, out_off, out_each));
    }

    let total = bin.len();

    // 4. 组装 gltf JSON。
    // node 0 是挂网格的节点，node 1..=15 对应骨骼 0..14。
    let mut bone_children: Vec<Vec<usize>> = vec![Vec::new(); JOINTS.len()];
    for (i, j) in JOINTS.iter().enumerate() {
        if let Some(p) = j.parent {
            bone_children[p].push(i);
        }
    }

    let mut nodes = String::new();
    nodes.push_str(r#"{ "name":"Humanoid","mesh":0,"skin":0,"children":[1] }"#);
    for i in 0..JOINTS.len() {
        let j = &JOINTS[i];
        let t = match j.parent {
            None => j.pos,
            Some(p) => [
                j.pos[0] - JOINTS[p].pos[0],
                j.pos[1] - JOINTS[p].pos[1],
                j.pos[2] - JOINTS[p].pos[2],
            ],
        };
        nodes.push_str(&format!(
            r#",{{ "name":"{}","translation":[{},{},{}]"#,
            j.name, t[0], t[1], t[2]
        ));
        if !bone_children[i].is_empty() {
            let kids: Vec<String> = bone_children[i]
                .iter()
                .map(|c| (c + 1).to_string())
                .collect();
            nodes.push_str(&format!(r#","children":[{}]"#, kids.join(",")));
        }
        nodes.push_str(" }");
    }

    let joints_list: Vec<String> = (1..=JOINTS.len()).map(|n| n.to_string()).collect();
    let joints_list = joints_list.join(",");

    // 4.1 固定 6 个 bufferView / accessor。
    let mut buffer_views: Vec<String> = Vec::new();
    let mut accessors: Vec<String> = Vec::new();

    buffer_views.push(format!(
        r#"{{ "buffer":0,"byteOffset":{off_pos},"byteLength":{len_pos},"target":34962 }}"#
    ));
    accessors.push(format!(
        r#"{{ "bufferView":0,"componentType":5126,"count":{vertex_count},"type":"VEC3","min":[{},{},{}],"max":[{},{},{}] }}"#,
        min[0], min[1], min[2], max[0], max[1], max[2]
    ));

    buffer_views.push(format!(
        r#"{{ "buffer":0,"byteOffset":{off_normal},"byteLength":{len_normal},"target":34962 }}"#
    ));
    accessors.push(format!(
        r#"{{ "bufferView":1,"componentType":5126,"count":{vertex_count},"type":"VEC3" }}"#
    ));

    buffer_views.push(format!(
        r#"{{ "buffer":0,"byteOffset":{off_joints},"byteLength":{len_joints},"target":34962 }}"#
    ));
    accessors.push(format!(
        r#"{{ "bufferView":2,"componentType":5123,"count":{vertex_count},"type":"VEC4" }}"#
    ));

    buffer_views.push(format!(
        r#"{{ "buffer":0,"byteOffset":{off_weights},"byteLength":{len_weights},"target":34962 }}"#
    ));
    accessors.push(format!(
        r#"{{ "bufferView":3,"componentType":5126,"count":{vertex_count},"type":"VEC4" }}"#
    ));

    buffer_views.push(format!(
        r#"{{ "buffer":0,"byteOffset":{off_indices},"byteLength":{len_indices},"target":34963 }}"#
    ));
    accessors.push(format!(
        r#"{{ "bufferView":4,"componentType":5123,"count":{index_count},"type":"SCALAR" }}"#
    ));

    buffer_views.push(format!(
        r#"{{ "buffer":0,"byteOffset":{off_ibm},"byteLength":{len_ibm} }}"#
    ));
    accessors.push(format!(
        r#"{{ "bufferView":5,"componentType":5126,"count":{},"type":"MAT4" }}"#,
        JOINTS.len()
    ));

    // 4.2 每个动画生成时间轴 + 8 通道输出，以及 samplers / channels。
    let mut animations_json: Vec<String> = Vec::new();
    for (a, anim) in ANIMATIONS.iter().enumerate() {
        let (time_off, time_len, out_off, out_each) = anim_offsets[a];

        // 时间轴 accessor
        let time_bv = buffer_views.len();
        buffer_views.push(format!(
            r#"{{ "buffer":0,"byteOffset":{time_off},"byteLength":{time_len} }}"#
        ));
        let t_min = anim.times.iter().cloned().fold(f32::INFINITY, f32::min);
        let t_max = anim.times.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let time_acc = accessors.len();
        accessors.push(format!(
            r#"{{ "bufferView":{time_bv},"componentType":5126,"count":3,"type":"SCALAR","min":[{t_min}],"max":[{t_max}] }}"#
        ));

        // 8 个输出 accessor
        let mut out_accs: Vec<usize> = Vec::new();
        for k in 0..8 {
            let bv = buffer_views.len();
            buffer_views.push(format!(
                r#"{{ "buffer":0,"byteOffset":{},"byteLength":{out_each} }}"#,
                out_off + k * out_each
            ));
            let acc = accessors.len();
            accessors.push(format!(
                r#"{{ "bufferView":{bv},"componentType":5126,"count":3,"type":"VEC4" }}"#
            ));
            out_accs.push(acc);
        }

        // samplers（8 个，共享时间轴 accessor）
        let mut samplers: Vec<String> = Vec::new();
        for &acc in &out_accs {
            samplers.push(format!(
                r#"{{ "input":{time_acc},"output":{acc},"interpolation":"LINEAR" }}"#
            ));
        }

        // channels（8 个，指向对应骨骼 node）
        let mut channels: Vec<String> = Vec::new();
        for (k, &joint) in ANIM_JOINTS.iter().enumerate() {
            channels.push(format!(
                r#"{{ "sampler":{k},"target":{{ "node":{},"path":"rotation" }} }}"#,
                joint + 1 // 骨骼索引 → node 索引（+1）
            ));
        }

        animations_json.push(format!(
            r#"{{ "name":"{}","samplers":[{}],"channels":[{}] }}"#,
            anim.name,
            samplers.join(","),
            channels.join(",")
        ));
    }

    let gltf = format!(
        r#"{{
  "asset": {{ "version":"2.0","generator":"bevylog humanoid generator" }},
  "scene": 0,
  "scenes": [{{ "nodes":[0] }}],
  "nodes": [ {nodes} ],
  "skins": [{{ "inverseBindMatrices":5,"joints":[{joints_list}],"skeleton":0 }}],
  "meshes": [
    {{
      "name":"HumanoidMesh",
      "primitives": [
        {{
          "attributes": {{ "POSITION":0,"NORMAL":1,"JOINTS_0":2,"WEIGHTS_0":3 }},
          "indices": 4,
          "material": 0
        }}
      ]
    }}
  ],
  "materials": [
    {{
      "name":"SkinMaterial",
      "pbrMetallicRoughness": {{ "baseColorFactor":[0.85,0.65,0.5,1.0],"metallicFactor":0.0,"roughnessFactor":0.85 }},
      "doubleSided": true
    }}
  ],
  "buffers": [{{ "uri":"humanoid.bin","byteLength":{total} }}],
  "bufferViews": [ {buffer_views} ],
  "accessors": [ {accessors} ],
  "animations": [ {animations_json} ]
}}"#,
        buffer_views = buffer_views.join(","),
        accessors = accessors.join(","),
        animations_json = animations_json.join(","),
    );

    fs::create_dir_all("assets/models").unwrap();
    fs::write("assets/models/humanoid.bin", &bin).unwrap();
    fs::write("assets/models/humanoid.gltf", gltf).unwrap();
    println!(
        "已写入 assets/models/humanoid.bin（{} 字节，{} 顶点 / {} 索引 / {} 骨骼 / {} 动画）",
        bin.len(),
        vertex_count,
        index_count,
        JOINTS.len(),
        ANIMATIONS.len()
    );
    println!("已写入 assets/models/humanoid.gltf");
}
