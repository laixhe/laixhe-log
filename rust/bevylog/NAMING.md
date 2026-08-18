# 示例命名规范（NAMING）

Bevy 0.19 学习项目（bevylog）中 `examples/` 的命名约定。新增用例时请遵循本规范，保持「文件名」「Cargo.toml」「README.md」三处一致。

## 总则

- 单文件示例统一使用 `example_<领域>_<名称>` 命名，全小写、下划线 `_` 分隔。
- 领域前缀参考 Bevy 官方 examples 目录分类，用于一眼识别示例所属领域。
- 工具/生成器脚本使用 `generate_<名称>` 前缀。
- 多文件目录型示例使用 `simple_mini_<名称>` 前缀。
- 项目主入口是 `src/main.rs`（`cargo run` 直接运行，学习编号 #01）。

## 领域前缀速查表

| 前缀 | 领域含义 | 现有示例 |
|------|---------|---------|
| `2d_` | 2D 渲染 / 精灵 / 图元 / 文本 / 拾取 / 拖拽 / 相机 / 碰撞 / 粒子 / 图集 / 物理 / 动画 / 网格 | `2d_sprite`、`2d_movement`、`2d_sprite_properties`、`2d_texture_atlas`、`2d_particles`、`2d_collision`、`2d_lighting`、`2d_custom_material`、`2d_gizmos`、`2d_primitives`、`2d_text`、`2d_mesh`、`2d_picking`、`2d_drag`、`2d_camera_follow`、`2d_sprite_fade`、`2d_parallax`、`2d_gravity`、`2d_rotate_to_mouse`、`2d_bounce`、`2d_wrap`、`2d_scale_pulse`、`2d_spawner`、`2d_color_cycle`、`2d_grid`、`2d_particle_trail` |
| `3d_` | 3D 场景 / 变换 / 程序化网格 / 3D 光照 / 图元 / 层级 / 拾取 / 材质 / 自定义材质 / 物理碰撞 / 纹理 / 雾效 / 泛光 | `3d_scene`、`3d_transform`、`3d_gizmos`、`3d_procedural_mesh`、`3d_lighting`、`3d_custom_mesh`、`3d_primitives`、`3d_hierarchy`、`3d_picking`、`3d_material`、`3d_custom_material`、`3d_gravity`、`3d_collision`、`3d_projectile`、`3d_texture`、`3d_fog`、`3d_bloom` |
| `ui_` | UI 各类控件 / 布局 / 文本 / 样式 / 相机 | `ui`、`ui_grid`、`ui_focus`、`ui_widgets`、`ui_text_input`、`ui_toggle_progress`、`ui_scroll_area`、`ui_radio_group`、`ui_list_box`、`ui_menu`、`ui_layout`、`ui_gradient`、`ui_zindex`、`ui_transform`、`ui_image`、`ui_shadow`、`ui_viewport`、`ui_cursor_position`、`ui_scale`、`ui_text_style`、`ui_span`、`ui_focus_policy`、`ui_overflow`、`ui_text_bounds`、`ui_target_camera`、`ui_interaction` |
| `animation_` | 动画系统（AnimationClip / 骨骼动画播放 / 事件 / 缓动 / 混合） | `animation`、`animation_clip`、`animation_skeletal`、`animation_humanoid`、`animation_multi_curve`、`animation_playback`、`animation_event`、`animation_easing`、`animation_blend` |
| `audio_` | 音频播放 / 控制 / 空间音效 / 全局音量 / 多音源 | `audio`、`audio_control`、`audio_spatial`、`audio_global_volume`、`audio_multi`、`audio_playback` |
| `input_` | 输入设备（鼠标 / 键盘 / 手柄 / 光标） | `input`、`input_gamepad`、`input_cursor`、`input_keyboard`、`input_mouse_motion` |
| `window_` | 窗口 / 多窗口 / 窗口事件 | `window`、`window_events`、`window_multi`、`window_fullscreen`、`window_cursor` |
| `ecs_` | ECS 机制（查询 / 资源 / 系统 / 事件 / 状态 / 消息 / 反射 / 关系 / 层级 / 组件存储 / 子状态 / 计算状态） | `ecs_events`、`ecs_resource`、`ecs_query`、`ecs_query_filter`、`ecs_system_param`、`ecs_system_sets`、`ecs_param_set`、`ecs_exclusive_system`、`ecs_run_condition`、`ecs_deferred`、`ecs_message`、`ecs_observer`、`ecs_reflect`、`ecs_relationship`、`ecs_hierarchy`、`ecs_hierarchy_query`、`ecs_state`、`ecs_performance`、`ecs_storage`、`ecs_sparse_set`、`ecs_storage_benchmark`、`ecs_substate`、`ecs_computed_state`、`ecs_state_transition_event`、`ecs_removed_components`、`ecs_related_query` |
| `time_` | 时间 / 定时器 / 固定步长 | `time_timer`、`time_fixed_timestep`、`time_control`、`time_stopwatch` |
| `asset_` | 资产加载 / 热重载 / 加载状态 / 自定义资产 / 远程加载 | `asset_hot_reload`、`asset_basics`、`asset_load_state`、`asset_custom`、`asset_remote` |
| `async_` | 异步任务（计算池 / I/O 池） | `async_task`、`async_io` |
| （单领域） | 独立且单一的概念，不加额外前缀 | `camera`、`camera_shake`、`camera_bounds`、`camera_zoom_to_cursor`、`camera_orbit_3d`、`camera_scaling_modes`、`camera_rotation_2d`、`camera_first_person_3d`、`camera_third_person_3d`、`logging`、`plugin`、`scene`、`scene_hierarchy`、`scene_patch`、`scene_list`、`scene_system`、`render_layers`、`diagnostics`、`diagnostics_custom`、`random`、`visibility`、`network_request`、`serialization` |

## 特殊前缀

| 前缀 | 用途 | 现有示例 |
|------|------|---------|
| `generate_` | 工具 / 生成器脚本（不参与交互，产出资产文件） | `generate_skeletal_gltf`、`generate_humanoid_gltf` |
| `simple_mini_` | 多文件目录型迷你游戏 / 综合应用（`examples/<name>/main.rs`，组合多个核心概念） | `simple_mini`、`simple_mini_breakout`、`simple_mini_snake`、`simple_mini_2d_star`、`simple_mini_3d_coin`、`simple_mini_ui_todo` |

## 领域前缀选择指引

新增示例时，按示例要演示的核心概念选择前缀：

- 演示 2D 渲染 / 精灵 / 碰撞 / 粒子 / 图集 → `2d_`
- 演示 3D 场景 / 网格 / 光照 → `3d_`
- 演示 UI 控件 / 布局 / 焦点 → `ui_`
- 演示动画系统（AnimationClip / AnimationGraph / 骨骼动画播放）→ `animation_`
- 演示音频 → `audio_`
- 演示输入设备 → `input_`
- 演示窗口相关 → `window_`
- 演示 ECS 机制（Query / Resource / System / Event / State / Message / Observer / Reflect / 关系 / 层级）→ `ecs_`
- 演示时间 / Timer / FixedUpdate → `time_`
- 演示资产加载 / 热重载 → `asset_`
- 演示异步任务 → `async_`
- 概念独立且单一（相机、日志、插件、场景、渲染层）→ 单领域名

> 备注：`custom_mesh` / `custom_material` / `custom_relationship` 这类原本带 `custom_` 前缀的示例，已按实际领域归位为 `3d_custom_mesh` / `2d_custom_material` / `ecs_relationship`。命名时以「演示的是什么领域」为准，而非「是否是自定义实现」。

## 代码写法约定

### `bsn!` 宏 vs `commands.spawn`

- `bsn!`：声明式构建场景，适合「结构固定、无运行时分支」的实体（固定文本、静态布局）。
  例：`commands.spawn_scene(bsn! { Text2d::new("提示") TextColor(...) })`
- `commands.spawn((...))`：命令式，适合「需要条件判断 / 动态计算」的实体。
  例：根据 `has_image` 决定 Sprite 字段时，用 spawn + if 分支。

原则：有运行时分支或动态值 → 用 `spawn`；纯静态声明 → 可用 `bsn!`。

### 中文字体加载（两种写法都正确，按上下文选）

- `bsn!` 里：用 `FontSourceTemplate::Handle(路径字符串)`——模板，构建场景时自动转成 `FontSource`。
- `commands.spawn` 里：用 `FontSource::Handle(asset_server.load(路径))`——真实句柄。

两者是「模板 vs 实例」的同一件事，不是两种可混用的风格，不要跨上下文互换。

## 新增用例 Checklist

按顺序完成以下步骤：

1. **创建文件**：`examples/example_<领域>_<名称>.rs`
   - 文件头用 `//!` 写模块注释，说明「演示什么 + 学习重点 + 操作方式」。
2. **注册示例**：在 `Cargo.toml` 末尾（工具/目录示例除外）追加：
   ```toml
   [[example]]
   name = "example_<领域>_<名称>"
   path = "examples/example_<领域>_<名称>.rs"
   ```
3. **更新 README**：
   - 「运行」代码块加一行：`cargo run --example example_<领域>_<名称>  # 编号 标题  [核心概念：...]`
   - 「建议学习顺序」表格加一行（编号递增）。
4. **格式化与验证**：
   ```sh
   cargo fmt
   cargo check --example example_<领域>_<名称>
   ```

## 注意事项

- 命名全小写、下划线分隔，避免驼峰或连字符。
- 若新示例在代码注释里引用其他示例，务必使用**重命名后的新名**（例如 `example_ecs_events` 而非 `example_events`）。
- 新增编号接在现有最大编号之后（当前到 #161）。
- 若新增示例需要生成资产文件，工具脚本用 `generate_` 前缀，并在 README「资产说明」里补充对应资产条目。
