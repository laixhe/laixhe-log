# bevylog

Bevy 0.19 学习项目：每个 example 覆盖一个核心 Bevy 概念，代码含新手友好注释。

## 快速开始

- **Rust 版本要求**：建议使用 latest stable Rust（≥ 1.80），通过 `rustup update stable` 升级即可安装。
- **Bevy 版本**：0.19（启用 `dynamic_linking` 特性加快增量编译速度）。

---

## 运行

所有示例均通过 `cargo run --example <名称>` 启动：

```sh
cargo run                                      # 01 main.rs：Hello World - 2D 相机 + 黄色圆形 + 底部文本（最基础的应用结构与场景初始化）
cargo run --example example_simple_movement    # 02 简单精灵移动：方向键 / WASD 移动文本玩家  [核心概念：ButtonInput 按键查询、时间无关的速度×dt、玩家 Transform、日志]
cargo run --example example_simple_sprite      # 03 简单精灵显示：加载并显示精灵图片，图片缺失时回退纯色方块  [核心概念：AssetServer 资产加载、Image 缺省回退处理、2D 精灵 Transform]
cargo run --example example_events             # 04 事件系统：空格触发跳跃，累计 5 次自动退出  [核心概念：#[derive(Event)] 自定义事件、EventWriter 发送 / EventReader 读取、事件计数统计]
cargo run --example example_ui                 # 05 UI 系统：点击按钮计数，颜色随 Hover/Press 状态变化  [核心概念：Node 节点、Button 按钮交互、Interaction 组件查询、Flexbox 布局]
cargo run --example example_input              # 06 输入系统：鼠标左键点击生成圆形，滚轮缩放图形  [核心概念：MouseButton 点击、AccumulatedMouseScroll 滚轮、窗口↔世界坐标转换、PrimaryWindow 查询]
cargo run --example example_animation          # 07 动画系统：太阳脉冲缩放 + 3 颗行星轨道运动 + 自转  [核心概念：Time::elapsed_secs() 时间驱动动画、sin/cos 周期性运动、Transform translation/rotation/scale、Single 查询]
cargo run --example example_state              # 08 状态系统：菜单 → 游戏中 → 暂停 三态切换（ESC 暂停/继续） [核心概念：#[derive(States)] 状态机、OnEnter/OnExit 调度、in_state 条件运行系统]
cargo run --example example_collision          # 09 物理碰撞：边界反弹 + 球间弹性碰撞响应 [核心概念：圆形碰撞检测（距离平方避免开方）、碰撞法线冲量响应、iter_combinations_mut 两两配对、系统 .chain() 执行顺序]
cargo run --example example_particles          # 10 粒子系统：鼠标左键点击在点击位置放烟花爆炸  [核心概念：实体生命周期 lifetime / despawn、重力模拟、共享 Mesh Handle、LCG 简易随机数、Assets<ColorMaterial> 动态修改]
cargo run --example example_lighting_3d        # 11 光照与阴影：3D 场景 PBR 材质，点光源轨道 + 方向光，空格切换阴影  [核心概念：Camera3d/DirectionalLight/PointLight、shadow_maps_enabled、StandardMaterial PBR、双相机叠加覆盖层]
cargo run --example example_lighting_2d        # 12 2D 光照与阴影：bevy_firefly 点光源轨道 + Occluder2d 遮挡体，空格切换阴影  [核心概念：FireflyPlugin/FireflyConfig 环境光、PointLight2d、Occluder2d 遮挡体、soft_shadows 软阴影]
cargo run --example simple_mini                # 13 迷你游戏2D：玩家移动 + 空格射击 + 碰撞 + 计分（多文件目录 example）  [核心概念：mod 模块组织、Message/MessageWriter/MessageReader 消息系统、游戏循环整合]
cargo run --example simple_mini_breakout       # 14 上下打砖块(有启动画面)2D：启动画面 → 菜单 → 打砖块游戏  [核心概念：分层 States 状态机、EntityEvent+trigger+Observer 即时事件、bsn! 场景语法、BoundingCircle/Aabb2d 碰撞、拍面反弹角度]
```

---

## 建议学习顺序

建议从最基础的 ECS/应用骨架起步，按顺序递进地攻克 Bevy 概念：

| 顺序 | 示例 | 攻克的核心概念 |
|------|------|----------------|
| 1 | `main.rs` | App / Plugins / Startup / Camera2d / Mesh2d / Handle / Commands.spawn |
| 2 | `example_simple_movement` | ButtonInput、Time::delta_secs、帧无关的 dt 更新玩家 Transform |
| 3 | `example_simple_sprite` | AssetServer 加载图片、缺省回退处理、Sprite/材质/组件组合 |
| 4 | `example_events` | Event、EventWriter、EventReader、事件驱动的系统解耦 |
| 5 | `example_ui` | Node、Button、Interaction 状态机、Flexbox UI 布局 |
| 6 | `example_input` | 鼠标坐标转换、PrimaryWindow 查询、滚轮事件 AccumulatedMouseScroll |
| 7 | `example_animation` | Time::elapsed_secs 驱动动画、Transform 的 translation / rotation / scale |
| 8 | `example_state` | States 状态机、OnEnter / OnExit / in_state 条件 |
| 9 | `example_collision` | 圆形碰撞检测、弹性响应、iter_combinations_mut、.chain() 顺序 |
| 10 | `example_particles` | lifetime 生命周期管理、重力模拟、Assets 资源、LCG 伪随机数 |
| 11 | `example_lighting_3d` | 3D 相机 / 光照 / 阴影 / PBR StandardMaterial / 双相机叠加 |
| 12 | `example_lighting_2d` | bevy_firefly 第三方库、PointLight2d / Occluder2d / FireflyConfig 环境光 / 软阴影 |
| 13 | `simple_mini` | 多文件 mod 模块组织、Message/MessageWriter/MessageReader 消息系统、游戏循环整合（移动+射击+碰撞+计分） |
| 14 | `simple_mini_breakout` | 分层 States 状态机（GlobalGameState + 嵌套 GameState/MenuState）、EntityEvent+trigger+Observer 即时事件、bsn! 场景语法、BoundingCircle/Aabb2d 碰撞、拍面反弹角度、DespawnOnExit 自动清理 |

---

## 常见问题 FAQ

### Q1：运行示例时终端出现 `ICU4X data error: No segmentation model for language: ja`，是代码有 Bug 吗？

**A：非致命，功能完全正常。**

- **触发条件**：使用了中文/日文/韩文（CJK）字符，Bevy 0.19 依赖的 ICU4X 国际化库在打包时为了减小二进制体积，没有内置 CJK 语言的文本分段模型数据，遇到 CJK 字符时打印这条警告（上游已知 Bug：Bevy Issue #24094）。
- **对功能的影响**：无。汉字照常显示，场景/交互/动画/光照全部正常，仅终端多一行日志。
- **临时解决方案（屏蔽日志）**：在 App 启动时为 `LogPlugin` 过滤 `icu_provider` 的 warn 级：
  ```rust
  use bevy::log::{DEFAULT_FILTER, LogPlugin};
  App::new()
      .add_plugins(DefaultPlugins.set(LogPlugin {
          filter: format!("{DEFAULT_FILTER},icu_provider=error"),
          ..default()
      }))
      // ...其余设置不变
      .run();
  ```
- **根治方案**：启用 Bevy 的完整 ICU4X 特性，但会显著增加编译时间和二进制体积，个人学习场景不推荐。
