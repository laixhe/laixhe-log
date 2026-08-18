//! 工具脚本：生成一个极简的带骨骼 gltf 模型（用于 example_animation_skeletal）。
//!
//! 运行方式：`cargo run --example generate_skeletal_gltf`
//! 产物：
//!   - assets/models/skeletal_arm.gltf
//!   - assets/models/skeletal_arm.bin
//!
//! 模型结构：一条「手臂」网格（四边形），由 2 根骨骼驱动：
//!   - Bone0 在原点（固定）
//!   - Bone1 在 (1,0,0)，动画绕 Z 轴来回摆动（0° ↔ 90° ↔ 0°）
//!
//! 左半边顶点绑定 Bone0，右半边绑定 Bone1，从而看到「骨骼带动网格」的形变。

use std::f32::consts::FRAC_1_SQRT_2;
use std::fs;

fn push_f32(buf: &mut Vec<u8>, v: f32) {
    buf.extend_from_slice(&v.to_le_bytes());
}

fn push_u16(buf: &mut Vec<u8>, v: u16) {
    buf.extend_from_slice(&v.to_le_bytes());
}

fn main() {
    let mut bin: Vec<u8> = Vec::new();

    // 1. POSITION（4 顶点 × vec3）
    for p in [
        [0.0f32, -0.1, 0.0],
        [0.0, 0.1, 0.0],
        [2.0, -0.1, 0.0],
        [2.0, 0.1, 0.0],
    ] {
        for v in p {
            push_f32(&mut bin, v);
        }
    }

    // 2. NORMAL（4 顶点 × vec3，均指向 +Z）
    for _ in 0..4 {
        for v in [0.0f32, 0.0, 1.0] {
            push_f32(&mut bin, v);
        }
    }

    // 3. JOINTS_0（4 顶点 × vec4，uint16：左半边=骨骼0，右半边=骨骼1）
    for j in [[0u16, 0, 0, 0], [0, 0, 0, 0], [1, 0, 0, 0], [1, 0, 0, 0]] {
        for v in j {
            push_u16(&mut bin, v);
        }
    }

    // 4. WEIGHTS_0（4 顶点 × vec4，权重 1.0）
    for _ in 0..4 {
        for v in [1.0f32, 0.0, 0.0, 0.0] {
            push_f32(&mut bin, v);
        }
    }

    // 5. 索引（6 × uint16：两个三角形）
    for i in [0u16, 1, 2, 2, 1, 3] {
        push_u16(&mut bin, i);
    }

    // 6. inverseBindMatrices（2 × mat4，列主序）
    // Bone0 在原点 → 逆绑定矩阵 = 单位矩阵
    for v in [
        1.0f32, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ] {
        push_f32(&mut bin, v);
    }
    // Bone1 在 (1,0,0) → 逆绑定矩阵 = 平移(-1,0,0)
    for v in [
        1.0f32, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, -1.0, 0.0, 0.0, 1.0,
    ] {
        push_f32(&mut bin, v);
    }

    // 7. 动画时间轴（3 × float）
    for t in [0.0f32, 0.5, 1.0] {
        push_f32(&mut bin, t);
    }

    // 8. 动画旋转（3 × vec4 四元数，绕 Z 轴：0° → 90° → 0°）
    for q in [
        [0.0f32, 0.0, 0.0, 1.0],
        [0.0, 0.0, FRAC_1_SQRT_2, FRAC_1_SQRT_2],
        [0.0, 0.0, 0.0, 1.0],
    ] {
        for v in q {
            push_f32(&mut bin, v);
        }
    }

    fs::create_dir_all("assets/models").unwrap();
    fs::write("assets/models/skeletal_arm.bin", &bin).unwrap();
    println!(
        "已写入 assets/models/skeletal_arm.bin（{} 字节）",
        bin.len()
    );

    let gltf = r#"{
  "asset": { "version": "2.0", "generator": "bevylog skeletal generator" },
  "scene": 0,
  "scenes": [{ "nodes": [0] }],
  "nodes": [
    { "name": "Armature", "mesh": 0, "skin": 0, "children": [1] },
    { "name": "Bone0", "children": [2] },
    { "name": "Bone1", "translation": [1.0, 0.0, 0.0] }
  ],
  "skins": [
    { "inverseBindMatrices": 5, "joints": [1, 2], "skeleton": 0 }
  ],
  "meshes": [
    {
      "name": "Limb",
      "primitives": [
        {
          "attributes": { "POSITION": 0, "NORMAL": 1, "JOINTS_0": 2, "WEIGHTS_0": 3 },
          "indices": 4,
          "material": 0
        }
      ]
    }
  ],
  "materials": [
    {
      "name": "LimbMaterial",
      "pbrMetallicRoughness": { "baseColorFactor": [0.8, 0.4, 0.2, 1.0], "metallicFactor": 0.0, "roughnessFactor": 0.9 },
      "doubleSided": true
    }
  ],
  "buffers": [{ "uri": "skeletal_arm.bin", "byteLength": 392 }],
  "bufferViews": [
    { "buffer": 0, "byteOffset": 0, "byteLength": 48, "target": 34962 },
    { "buffer": 0, "byteOffset": 48, "byteLength": 48, "target": 34962 },
    { "buffer": 0, "byteOffset": 96, "byteLength": 32, "target": 34962 },
    { "buffer": 0, "byteOffset": 128, "byteLength": 64, "target": 34962 },
    { "buffer": 0, "byteOffset": 192, "byteLength": 12, "target": 34963 },
    { "buffer": 0, "byteOffset": 204, "byteLength": 128 },
    { "buffer": 0, "byteOffset": 332, "byteLength": 12 },
    { "buffer": 0, "byteOffset": 344, "byteLength": 48 }
  ],
  "accessors": [
    { "bufferView": 0, "componentType": 5126, "count": 4, "type": "VEC3", "min": [0.0, -0.1, 0.0], "max": [2.0, 0.1, 0.0] },
    { "bufferView": 1, "componentType": 5126, "count": 4, "type": "VEC3" },
    { "bufferView": 2, "componentType": 5123, "count": 4, "type": "VEC4" },
    { "bufferView": 3, "componentType": 5126, "count": 4, "type": "VEC4" },
    { "bufferView": 4, "componentType": 5123, "count": 6, "type": "SCALAR" },
    { "bufferView": 5, "componentType": 5126, "count": 2, "type": "MAT4" },
    { "bufferView": 6, "componentType": 5126, "count": 3, "type": "SCALAR", "min": [0.0], "max": [1.0] },
    { "bufferView": 7, "componentType": 5126, "count": 3, "type": "VEC4" }
  ],
  "animations": [
    {
      "name": "Swing",
      "samplers": [{ "input": 6, "output": 7, "interpolation": "LINEAR" }],
      "channels": [{ "sampler": 0, "target": { "node": 2, "path": "rotation" } }]
    }
  ]
}"#;

    fs::write("assets/models/skeletal_arm.gltf", gltf).unwrap();
    println!("已写入 assets/models/skeletal_arm.gltf");
}
