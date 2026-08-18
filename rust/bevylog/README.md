# bevylog

Bevy 0.19 学习项目：每个 example 覆盖一个核心 Bevy 概念，代码含新手友好注释。

## 目录

- [快速开始](#快速开始)
- [开始前：需要了解](#开始前需要了解)
- [新手学习路径（必学清单）](#新手学习路径必学清单)
- [运行示例](#运行)
- [建议学习顺序（全部示例）](#建议学习顺序)
- [常见问题 FAQ](#常见问题-faq)
- [示例命名规范](NAMING.md)
- [Bevy 特性参考](FEATURES.md)

## 快速开始

首次运行前，先把 Rust 工具链升级到最新稳定版（避免版本过低导致编译报错）：

```sh
rustup update stable
```

- **Rust 版本要求**：latest stable（≥ 1.80）。
- **Bevy 版本**：0.19（启用 `dynamic_linking` 特性加快增量编译速度）。
- **代码习惯**：改完代码后运行 `cargo fmt` 格式化、`cargo clippy` 静态检查，保持代码整洁、尽早发现常见错误。
- **命名规范**：示例文件名遵循「领域前缀」约定，新增用例前先看 [NAMING.md](NAMING.md)。
- **Bevy 特性**：调整功能 / 格式 / 平台特性时，参考 [FEATURES.md](FEATURES.md)。

## 开始前：需要了解

### 前置知识

看示例前，建议先具备以下 Rust 基础（不要求精通）：

- 变量、函数、`struct` / `enum`、`match` / `if let`
- 所有权、借用（`&` / `&mut`）、生命周期
- `Result` / `Option` 与 `?` 运算符
- 宏的基本概念（只需会用，不需要会写）

### Bevy 核心术语速查

| 术语 | 一句话解释 | 常见写法 |
|------|-----------|---------|
| Entity | 世界里的一个「对象」ID（本身不含数据） | `Entity`、`commands.spawn(...)` |
| Component | 挂在实体上的数据（位置、外观、标记等） | `#[derive(Component)] struct Health(f32);` |
| System | 每帧运行的逻辑函数 | `fn move_player(...) {}` + `add_systems(Update, ...)` |
| Resource | 全局唯一的单例数据（分数、配置等） | `#[derive(Resource)] struct Score(u32);` |
| Query | 按条件批量访问实体的组件 | `Query<&Transform, With<Player>>` |
| Bundle | 一组组件的「打包」，spawn 时一次性挂上 | `(Sprite, Transform, Health)` |
| Handle | 资产的轻量「句柄」，克隆共享同一份资源 | `Handle<Image>`、`asset_server.load(...)` |
| Plugin | 把一组系统/资源打包成可插拔模块 | `impl Plugin for X` |
| Schedule | 系统运行的「阶段表」 | `Startup` / `Update` / `FixedUpdate` |

## 新手学习路径（必学清单）

第一次接触 Bevy 时，建议按下面顺序跑这 15 个「核心」示例，快速建立整体认知；其余示例按兴趣随时查阅。

```sh
cargo run                                      # 最基础的 App 结构（相机 + 圆形 + 文本）
cargo run --example example_2d_sprite          # 精灵显示（资产加载 + Sprite）
cargo run --example example_2d_movement        # 键盘输入 + 移动
cargo run --example example_ecs_resource       # 资源系统（Res / ResMut）
cargo run --example example_ecs_query          # 查询系统（Query / Single）
cargo run --example example_ecs_events         # 事件系统
cargo run --example example_ui                 # UI 系统
cargo run --example example_input              # 输入系统
cargo run --example example_animation          # 动画系统
cargo run --example example_ecs_state          # 状态机
cargo run --example example_3d_scene           # 3D 基础场景
cargo run --example example_audio              # 音频系统
cargo run --example example_ecs_hierarchy      # 父子层级
cargo run --example example_scene              # 场景系统
cargo run --example simple_mini_breakout       # 综合迷你游戏（融会贯通）
```

> 全部 161 个示例按「建议学习顺序」排列在下方，可随时跳着看。

---

## 运行

所有示例均通过 `cargo run --example <名称>` 启动：

```sh
cargo run                                      # 01 src/main.rs（第 1 课，即项目根入口）：Hello World - 2D 相机 + 黄色圆形 + 底部文本（最基础的应用结构与场景初始化）
cargo run --example example_2d_movement         # 02 简单精灵移动：方向键 / WASD 移动文本玩家  [核心概念：ButtonInput 按键查询、时间无关的速度×dt、玩家 Transform、日志]
cargo run --example example_2d_sprite           # 03 简单精灵显示：加载并显示精灵图片，图片缺失时回退纯色方块  [核心概念：AssetServer 资产加载、Image 缺省回退处理、2D 精灵 Transform]
cargo run --example example_ecs_events          # 04 事件系统：空格触发跳跃，累计 5 次自动退出  [核心概念：#[derive(Event)] 自定义事件、EventWriter 发送 / EventReader 读取、事件计数统计]
cargo run --example example_ui                  # 05 UI 系统：点击按钮计数，颜色随 Hover/Press 状态变化  [核心概念：Node 节点、Button 按钮交互、Interaction 组件查询、Flexbox 布局]
cargo run --example example_input               # 06 输入系统：鼠标左键点击生成圆形，滚轮缩放图形  [核心概念：MouseButton 点击、AccumulatedMouseScroll 滚轮、窗口↔世界坐标转换、PrimaryWindow 查询]
cargo run --example example_animation           # 07 动画系统：太阳脉冲缩放 + 3 颗行星轨道运动 + 自转  [核心概念：Time::elapsed_secs() 时间驱动动画、sin/cos 周期性运动、Transform translation/rotation/scale、Single 查询]
cargo run --example example_ecs_state           # 08 状态系统：菜单 → 游戏中 → 暂停 三态切换（ESC 暂停/继续） [核心概念：#[derive(States)] 状态机、OnEnter/OnExit 调度、in_state 条件运行系统]
cargo run --example example_2d_collision        # 09 物理碰撞：边界反弹 + 球间弹性碰撞响应 [核心概念：圆形碰撞检测（距离平方避免开方）、碰撞法线冲量响应、iter_combinations_mut 两两配对、系统 .chain() 执行顺序]
cargo run --example example_2d_particles        # 10 粒子系统：鼠标左键点击在点击位置放烟花爆炸  [核心概念：实体生命周期 lifetime / despawn、重力模拟、共享 Mesh Handle、LCG 简易随机数、Assets<ColorMaterial> 动态修改]
cargo run --example example_3d_lighting         # 11 光照与阴影：3D 场景 PBR 材质，点光源轨道 + 方向光，空格切换阴影  [核心概念：Camera3d/DirectionalLight/PointLight、shadow_maps_enabled、StandardMaterial PBR、双相机叠加覆盖层]
cargo run --example example_2d_lighting         # 12 2D 光照与阴影：bevy_firefly 点光源轨道 + Occluder2d 遮挡体，空格切换阴影  [核心概念：FireflyPlugin/FireflyConfig 环境光、PointLight2d、Occluder2d 遮挡体、soft_shadows 软阴影]
cargo run --example simple_mini                 # 13 迷你游戏2D：玩家移动 + 空格射击 + 碰撞 + 计分（多文件目录 example）  [核心概念：mod 模块组织、Message/MessageWriter/MessageReader 消息系统、游戏循环整合]
cargo run --example simple_mini_breakout        # 14 上下打砖块(有启动画面)2D：启动画面 → 菜单 → 打砖块游戏  [核心概念：分层 States 状态机、EntityEvent+trigger+Observer 即时事件、bsn! 场景语法、BoundingCircle/Aabb2d 碰撞、拍面反弹角度]
cargo run --example simple_mini_snake           # 15 贪吃蛇2D：Loading→Menu→Playing→GameOver 页面状态机，数据/渲染分离的网格贪吃蛇  [核心概念：States 页面状态机、数据驱动渲染(Block→Sprite)、Follow 链蛇身跟随、MoveTimer tick 移动管线、bsn! 场景语法]
cargo run --example example_time_timer          # 16 定时器系统：Timer 组件 vs Time 资源  [核心概念：Timer::tick/just_finished、TimerMode、Timer 组件（每实体独立）vs Time 资源（全局自动更新）]
cargo run --example example_audio               # 17 音频系统：循环背景音乐 + 一次性音效 + 暂停/继续  [核心概念：AudioPlayer、PlaybackSettings、AudioSink 播放控制、异步加载]
cargo run --example example_ecs_hierarchy       # 18 父子层级：子实体跟随父实体旋转  [核心概念：ChildOf/Children 关系组件、with_children、局部 Transform vs 全局 GlobalTransform]
cargo run --example example_ecs_system_sets     # 19 系统集合：Input→Logic→Render 三阶段有序执行  [核心概念：SystemSet、configure_sets(...).chain()、in_set 分组、显式管理系统顺序]
cargo run --example example_2d_gizmos           # 20 调试绘制：Gizmos 画坐标轴/直线/圆/矩形/箭头  [核心概念：Gizmos、line_2d/circle_2d/rect_2d/arrow_2d、可视化调试]
cargo run --example example_time_fixed_timestep # 21 固定时间步长：球物理用 FixedUpdate 驱动  [核心概念：Time<Fixed>::from_hz、FixedUpdate、固定 dt 与帧率解耦]
cargo run --example example_window              # 22 窗口配置：标题/分辨率/可缩放/位置  [核心概念：WindowPlugin、Window 字段、Changed<Window> 监听尺寸变化]
cargo run --example example_logging             # 23 日志系统：五个日志级别与 RUST_LOG 过滤  [核心概念：trace/debug/info/warn/error 宏、日志级别、RUST_LOG]
cargo run --example example_camera              # 24 相机控制：WASD 移动 + 滚轮缩放  [核心概念：相机 Transform、OrthographicProjection.scale、Projection 枚举]
cargo run --example example_plugin              # 25 自定义插件：Plugin trait 打包系统  [核心概念：impl Plugin、build(&self, app)、插件化组织]
cargo run --example example_2d_texture_atlas    # 26 图集动画：4 帧精灵图集循环播放弹跳小球  [核心概念：TextureAtlasLayout::from_grid、TextureAtlas.index 帧切换、雪碧图]
cargo run --example example_ui_text_input       # 27 文本输入：可编辑文本框 + 回车提交  [核心概念：EditableText、InputFocus 聚焦、value/clear]
cargo run --example example_audio_spatial       # 28 空间音效：3D 声音随位置左右平移  [核心概念：SpatialListener、PlaybackSettings::with_spatial、声源 Transform 定位]
cargo run --example example_3d_custom_mesh      # 29 自定义网格：从顶点数据构造三角形  [核心概念：Mesh::new、insert_attribute 顶点属性、PrimitiveTopology]
cargo run --example example_3d_transform        # 30 3D 变换：平移/旋转/缩放  [核心概念：Transform.translation/rotation/scale、rotate_local_x/y/z、Vec3::splat 缩放]
cargo run --example example_audio_control       # 31 音频控制：实时调节音量/速度  [核心概念：AudioSink、set_volume(Volume::Linear)、set_speed]
cargo run --example example_input_gamepad       # 32 游戏手柄：摇杆移动+按钮动作  [核心概念：ButtonInput<GamepadButton>、Axis<GamepadAxis>]
cargo run --example example_ecs_performance     # 33 ECS 性能优化：查询过滤+变更检测  [核心概念：With/Without 过滤、Changed/Added 变更检测、系统并行]
cargo run --example example_ecs_resource        # 34 资源系统：自定义 Resource + Res/ResMut + 三种初始化方式  [核心概念：Resource 单例、Res/ResMut 读写、init_resource/insert_resource、FromWorld 资源依赖]
cargo run --example example_ecs_query           # 35 查询系统：Single / iter_mut / get / iter 四种查询形式  [核心概念：Query 遍历、Single 单实体、get 精确访问、With/Without 过滤与并行]
cargo run --example example_3d_scene            # 36 3D 基础场景：相机+灯光+立方体/球体/平面+旋转  [核心概念：Camera3d、DirectionalLight、Mesh3d/MeshMaterial3d/StandardMaterial、Cuboid/Sphere/Plane3d]
cargo run --example example_ecs_observer        # 37 观察者/触发器：实体事件 + 全局/实体观察者  [核心概念：EntityEvent、commands.trigger、observe/add_observer、On<T> 当帧处理]
cargo run --example example_time_control        # 38 时间控制：暂停/继续 + 时间缩放  [核心概念：Time<Virtual>、pause/unpause、set_relative_speed、delta_secs 受暂停影响]
cargo run --example example_ecs_run_condition   # 39 运行条件：自定义函数 / resource_changed 控制系统是否运行  [核心概念：run_if、自定义条件函数、resource_changed、chain 排序]
cargo run --example example_ecs_system_param    # 40 自定义系统参数：SystemParam derive 打包 Query+Res  [核心概念：#[derive(SystemParam)]、参数组合、减少签名重复]
cargo run --example example_ecs_message         # 41 消息系统：MessageWriter/MessageReader 双缓冲队列  [核心概念：#[derive(Message)]、write/read、双缓冲队列、chain 保证当帧读取]
cargo run --example example_ecs_reflect         # 42 反射：Reflect derive + 运行时按字段名读取值  [核心概念：#[derive(Reflect)]、register_type、reflect_ref、field/try_downcast_ref]
cargo run --example example_ecs_deferred        # 43 延迟操作：Commands 延迟应用 + chain 自动 ApplyDeferred  [核心概念：Commands 延迟、spawn 后本系统不可见、chain 自动刷新命令]
cargo run --example example_ui_grid             # 44 UI 网格布局：CSS Grid 行列轨道 + 跨行跨列  [核心概念：Display::Grid、grid_template_columns/rows、RepeatedGridTrack::fr、GridPlacement::start_span]
cargo run --example example_animation_clip      # 45 2D 动画：AnimationClip 关键帧曲线 + AnimationPlayer 循环播放  [核心概念：AnimatableCurve/KeyframeCurve、animated_field!、AnimationGraph::from_clip、AnimationTargetId/AnimatedBy]
cargo run --example example_scene               # 46 场景系统：impl Scene 函数复用场景 + spawn_scene  [核心概念：impl Scene、可复用场景函数、spawn_scene、bsn! 复用]
cargo run --example example_asset_hot_reload    # 47 资产热重载：AssetServer watch + AssetEvent 事件  [核心概念：AssetServer::load、AssetEvent、MessageReader、热重载]
cargo run --example example_ecs_param_set       # 48 系统参数集合：ParamSet 一个系统内多次访问同一组件  [核心概念：ParamSet、p0/p1 按索引访问、解决借用冲突]
cargo run --example example_render_layers       # 49 渲染层：RenderLayers + 多相机叠加  [核心概念：RenderLayers::layer、相机分层渲染、order+ClearColorConfig::None]
cargo run --example example_ecs_exclusive_system # 50 排他系统：&mut World 独占访问  [核心概念：&mut World、world.resource/resource_mut、world.entities().len]
cargo run --example example_window_events       # 51 窗口事件：WindowEvent 消息  [核心概念：WindowEvent、MessageReader、WindowResized/Focused/CloseRequested]
cargo run --example example_2d_custom_material  # 52 自定义材质：Material2d + WGSL 着色器  [核心概念：AsBindGroup、Material2d、Material2dPlugin、MeshMaterial2d<M>、自定义 WGSL]
cargo run --example generate_skeletal_gltf      # 工具：生成极简带骨骼 gltf 模型（skeletal_arm.gltf/.bin，供 #53 使用）
cargo run --example example_animation_skeletal  # 53 3D 骨骼动画：加载带骨骼 gltf + AnimationGraph + AnimationPlayer 循环播放  [核心概念：WorldAssetRoot、Gltf.animations、AnimationGraph::from_clips、AnimationGraphHandle、骨骼蒙皮]
cargo run --example generate_humanoid_gltf      # 工具：生成多关节人形 gltf 模型（humanoid.gltf/.bin，供 #54 使用）
cargo run --example example_animation_humanoid  # 54 多关节人形骨骼动画：15 骨骼 + 关节处多权重蒙皮 + 走路/跑动/静止（空格循环平滑过渡）  [核心概念：多骨骼蒙皮权重混合、Gltf.animations、AnimationGraph、AnimationTransitions::play 动画过渡混合]
cargo run --example example_ecs_query_filter    # 55 查询过滤器：Or 组合过滤 + Changed/Added 变更检测  [核心概念：Or<()>、Changed<T>、Added<T>、chain 控制顺序]
cargo run --example example_ecs_hierarchy_query # 56 层级遍历：关系组件 ChildOf/Children + 遍历后代/祖先  [核心概念：iter_descendants、iter_ancestors、related、关系组件]
cargo run --example example_input_cursor        # 57 鼠标光标：CursorIcon 组件动态切换光标样式  [核心概念：CursorIcon、SystemCursorIcon、PrimaryWindow 窗口实体]
cargo run --example example_3d_procedural_mesh  # 58 3D 程序化网格：带索引的立方体  [核心概念：insert_indices、Indices::U16、索引网格复用顶点]
cargo run --example example_2d_sprite_properties # 59 Sprite 高级属性：翻转/着色/缩放/锚点  [核心概念：Sprite.flip_x/flip_y/color/custom_size、Anchor 独立组件]
cargo run --example example_ui_focus            # 60 UI 焦点导航：Tab 键在控件间切换焦点  [核心概念：TabGroup、TabIndex、TabNavigationPlugin、FocusGained/FocusLost 事件]
cargo run --example example_ecs_relationship    # 61 自定义关系组件：Relationship/RelationshipTarget 定义「跟随」关系  [核心概念：#[relationship]、#[relationship_target]、自定义一对多关系]
cargo run --example example_async_task          # 62 异步任务：后台线程池执行耗时计算  [核心概念：AsyncComputeTaskPool、Task、is_finished、block_on/poll_once]
cargo run --example example_window_multi        # 63 多窗口：创建第二个窗口 + 多相机渲染到不同窗口  [核心概念：Window 组件、RenderTarget::Window、WindowRef::Entity、RenderLayers]
cargo run --example example_3d_gizmos           # 64 3D Gizmos：3D 空间绘制坐标轴/球体/圆/箭头  [核心概念：gizmos.line/arrow/sphere/circle、Isometry3d]
cargo run --example example_ui_widgets          # 65 UI 组件：Checkbox 复选框 + Slider 滑块  [核心概念：Checkbox、Slider、SliderValue/SliderRange、ValueChange 事件、checkbox_self_update]
cargo run --example example_ui_toggle_progress  # 66 Toggle 开关 + ProgressBar 进度条  [核心概念：Checkbox 组合自定义视觉、Node.width 百分比动态更新]
cargo run --example example_ui_scroll_area      # 67 ScrollArea 滚动区域：Overflow 滚动容器 + 20 个列表项  [核心概念：ScrollArea、ScrollPosition、Overflow::scroll_y、滚动视图]
cargo run --example example_ui_radio_group      # 68 RadioGroup 单选组：RadioButton + ValueChange 选中互斥  [核心概念：RadioGroup/RadioButton、Checked 组件、ValueChange<Entity>、自定义圆圈视觉]
cargo run --example example_ui_list_box         # 69 ListBox 列表框：ListItem + ValueChange 选中高亮  [核心概念：ListBox/ListItem、Selected 组件、ValueChange<Entity>、ActiveDescendant]
cargo run --example example_ui_menu             # 70 Menu 弹出菜单：MenuButton + MenuPopup + MenuItem 菜单事件  [核心概念：MenuButton/MenuPopup/MenuItem、MenuEvent/MenuAction、MenuFocusState、Activate]
cargo run --example example_ui_layout           # 71 UI 布局单位与盒模型：Val 单位 + UiRect 盒模型 + 绝对定位  [核心概念：px/percent/vw/vh、UiRect(margin/padding/border)、PositionType::Absolute、BorderRadius/Outline]
cargo run --example example_ui_gradient         # 72 UI 渐变背景：线性/径向/锥形渐变  [核心概念：BackgroundGradient、LinearGradient/RadialGradient/ConicGradient、ColorStop/AngularColorStop]
cargo run --example example_ui_zindex           # 73 UI 叠放层级：ZIndex + GlobalZIndex  [核心概念：ZIndex、GlobalZIndex、兄弟节点叠放顺序、跨层级叠加]
cargo run --example example_ui_transform        # 74 UI 变换：UiTransform 旋转/缩放/平移动画  [核心概念：UiTransform、Val2、Rot2、Time 驱动 UI 动画]
cargo run --example example_ui_image            # 75 UI 图片节点：ImageNode 显示/拉伸/平铺/翻转/着色  [核心概念：ImageNode、NodeImageMode::Stretch/Tiled、flip_x、color tint、solid_color]
cargo run --example example_ui_shadow           # 76 UI 阴影/描边：BoxShadow 阴影 + Outline 描边  [核心概念：BoxShadow、ShadowStyle、Outline、多层阴影]
cargo run --example example_ui_viewport         # 77 UI 视口：ViewportNode 小地图/画中画  [核心概念：ViewportNode、Image::new_target_texture、RenderTarget::Image]
cargo run --example example_ui_cursor_position  # 78 UI 相对光标位置：RelativeCursorPosition 跟随鼠标  [核心概念：RelativeCursorPosition、normalized 坐标、UiTransform 跟随]
cargo run --example example_ui_scale            # 79 UI 缩放：UiScale 全局缩放 UI  [核心概念：UiScale 资源、ResMut<UiScale>、+/- 动态缩放]
cargo run --example example_ui_text_style       # 80 UI 文本样式：阴影/背景/对齐/行高/字间距  [核心概念：TextShadow、TextBackgroundColor、TextLayout/Justify、LineHeight、LetterSpacing]
cargo run --example example_ui_span             # 81 UI 富文本：TextSpan 多段样式拼接  [核心概念：TextSpan、多段 TextColor/TextFont、父子文本拼接]
cargo run --example example_ui_focus_policy     # 82 UI 交互穿透：FocusPolicy Block/Pass  [核心概念：FocusPolicy::Block/Pass、遮罩拦截/穿透、Button 点击检测]
cargo run --example example_ui_overflow         # 83 UI 溢出裁剪：Overflow 显示/裁剪/隐藏  [核心概念：Overflow::visible/clip/hidden、溢出内容处理]
cargo run --example example_ui_text_bounds      # 84 UI 文本边界：TextBounds 限定宽度自动换行  [核心概念：TextBounds::new_horizontal、TextLayout/Justify 对齐]
cargo run --example example_ui_target_camera    # 85 UI 指定相机：UiTargetCamera 渲染到第二窗口  [核心概念：UiTargetCamera、RenderTarget::Window、多窗口 UI]
cargo run --example example_ui_interaction      # 86 UI 交互状态：Interaction 三态  [核心概念：Interaction::Pressed/Hovered/None、Changed<Interaction>、状态切换]
cargo run --example example_2d_primitives       # 87 2D 图元集合：圆/矩形/多边形/胶囊/椭圆/环形/三角形  [核心概念：Mesh2d、Circle/Rectangle/RegularPolygon/Capsule2d/Ellipse/Annulus/Triangle2d]
cargo run --example example_2d_text             # 88 2D 世界空间文本：Text2d 渲染与阴影  [核心概念：Text2d、Text2dShadow、Transform 定位/旋转]
cargo run --example example_2d_mesh             # 89 2D 自定义网格：从顶点数据构造 Mesh2d  [核心概念：Mesh::new、insert_attribute、PrimitiveTopology、Mesh2d]
cargo run --example example_2d_picking          # 90 2D 拾取：MeshPicking + Click 事件  [核心概念：MeshPickingPlugin、On<Pointer<Click>>、点击变色]
cargo run --example example_2d_drag             # 91 2D 拖拽：Pointer<Drag> 拖动物体  [核心概念：On<Pointer<Drag>>、Camera::viewport_to_world_2d、屏幕转世界]
cargo run --example example_2d_camera_follow    # 92 2D 相机跟随：WASD 移动 + 相机平滑跟随  [核心概念：指数平滑 lerp、Single 查询、相机跟随目标]
cargo run --example example_2d_sprite_fade      # 93 2D 精灵淡入淡出：透明度动画  [核心概念：Sprite::from_color、Color::set_alpha、sin 周期动画]
cargo run --example example_2d_parallax         # 94 2D 视差滚动：多层背景不同速度  [核心概念：多层 Sprite、速度组件、视差层次]
cargo run --example example_2d_gravity          # 95 2D 重力模拟：下落 + 落地反弹  [核心概念：速度向量、重力加速度、速度积分、阻尼反弹]
cargo run --example example_2d_rotate_to_mouse  # 96 2D 朝向鼠标：物体旋转指向鼠标  [核心概念：cursor_position、viewport_to_world_2d、atan2、Quat::from_rotation_z]
cargo run --example example_2d_bounce           # 97 2D 弹跳球：sin 周期上下弹跳  [核心概念：sin 周期运动、speed/phase 控制速度与相位]
cargo run --example example_2d_wrap             # 98 2D 屏幕环绕：穿出屏幕从对面进入  [核心概念：匀速运动、边界环绕、屏幕边界检测]
cargo run --example example_2d_scale_pulse      # 99 2D 缩放脉冲：Transform.scale 周期动画  [核心概念：Transform.scale、sin 脉冲、phase 错开]
cargo run --example example_2d_spawner          # 100 2D 定时生成：Timer 动态生成/销毁  [核心概念：Timer::Repeating、commands.spawn、despawn 生命周期]
cargo run --example example_2d_color_cycle      # 101 2D 颜色循环：动态修改 ColorMaterial  [核心概念：ColorMaterial 运行时改色、sin RGB 循环、Assets 访问]
cargo run --example example_2d_grid             # 102 2D 网格布局：程序化排列棋盘格  [核心概念：嵌套循环生成、共享 Mesh 句柄、交替颜色]
cargo run --example example_animation_multi_curve # 103 动画：一个 AnimationClip 同时动画位置/旋转/缩放  [核心概念：多曲线剪辑、animated_field!、AnimatableKeyframeCurve]
cargo run --example example_animation_playback  # 104 动画播放控制：暂停/调速/重播  [核心概念：pause_all/resume_all、all_paused、adjust_speeds、rewind_all]
cargo run --example example_animation_event     # 105 动画事件：播放到指定时间触发事件  [核心概念：AnimationEvent derive、AnimationClip::add_event、On<Event> 全局 observer]
cargo run --example example_animation_easing    # 106 缓动动画：四种缓动函数对比  [核心概念：Linear/EaseIn/EaseOut/EaseInOut、时间归一化与进度映射]
cargo run --example example_camera_shake        # 107 相机震动：随机抖动 + 创伤值衰减  [核心概念：Resource 创伤值、trauma² 力度、rand 随机偏移]
cargo run --example example_camera_bounds       # 108 相机边界限制：镜头无法移出地图  [核心概念：可见半视野、clamp 限制相机中心]
cargo run --example example_camera_zoom_to_cursor # 109 相机缩放跟随鼠标：缩放时鼠标处不动  [核心概念：世界点=中心+偏移×scale、缩放补偿]
cargo run --example example_camera_orbit_3d     # 110 3D 轨道相机：绕目标旋转观察  [核心概念：球坐标、look_at、yaw/pitch/distance]
cargo run --example example_camera_scaling_modes # 111 正交投影缩放模式：世界到屏幕的映射  [核心概念：ScalingMode、Fixed/AutoMin/AutoMax/FixedVertical]
cargo run --example example_camera_rotation_2d  # 112 2D 相机旋转：旋转整个画面  [核心概念：相机 Transform.rotate_z、旋转相机 vs 旋转物体]
cargo run --example example_camera_first_person_3d # 113 3D 第一人称：鼠标看四周 + WASD 移动  [核心概念：yaw/pitch、AccumulatedMouseMotion、forward/right]
cargo run --example example_camera_third_person_3d # 114 3D 第三人称跟随：相机在角色后方  [核心概念：角色朝向偏移、look_at、chain 顺序]
cargo run --example example_3d_primitives     # 115 3D 图元集合：各种内置 3D 形状  [核心概念：Cuboid/Sphere/Cylinder/Cone/Capsule3d/Torus、Mesh3d]
cargo run --example example_3d_hierarchy      # 116 3D 父子层级：太阳-行星-卫星  [核心概念：with_children、局部/全局 Transform、嵌套公转]
cargo run --example example_3d_picking        # 117 3D 拾取：点击物体高亮  [核心概念：MeshPickingPlugin、On<Pointer<Click>>、StandardMaterial]
cargo run --example example_3d_material       # 118 3D 材质属性：PBR 材质对比  [核心概念：metallic/roughness/emissive/unlit/alpha]
cargo run --example example_3d_gravity        # 119 3D 重力：下落 + 地面反弹  [核心概念：重力加速度、速度积分、恢复系数]
cargo run --example example_3d_collision      # 120 3D 球体弹性碰撞：封闭盒内  [核心概念：球-球碰撞、边界碰撞、分离 + 交换法线速度]
cargo run --example example_3d_projectile     # 121 3D 抛体运动：抛物线轨迹  [核心概念：初速度、重力只影响 y、动态生成抛体]
cargo run --example example_scene_hierarchy   # 122 场景层级：bsn! 父子关系  [核心概念：Children [...]、局部/全局 Transform]
cargo run --example example_scene_patch       # 123 场景补丁：复用场景 + 字段覆盖  [核心概念：Health { max } 补丁、Added 过滤]
cargo run --example example_scene_list        # 124 场景列表：一次生成多个根场景  [核心概念：bsn_list!、spawn_scene_list]
cargo run --example example_scene_system      # 125 场景作为系统：声明式生成  [核心概念：impl SceneList、scene.spawn()]
cargo run --example example_ecs_storage        # 126 组件存储类型：Table vs SparseSet  [核心概念：#[component(storage)]、默认 Table、稀疏存储]
cargo run --example example_ecs_sparse_set     # 127 SparseSet 实战：频繁增删状态  [核心概念：SparseSet 组件、Commands 延迟、PostUpdate]
cargo run --example example_ecs_storage_benchmark # 128 存储迭代性能：Table vs SparseSet  [核心概念：Instant 计时、缓存友好、遍历耗时]
cargo run --example example_asset_basics       # 129 资源基础：加载 + 句柄 + Assets<T>  [核心概念：AssetServer::load、Handle 克隆共享、Assets<T>::get]
cargo run --example example_asset_load_state   # 130 资源加载状态：异步检测  [核心概念：LoadState、load_state/is_loaded、加载完成再渲染]
cargo run --example example_asset_custom       # 131 自定义资源：Asset + AssetLoader  [核心概念：derive Asset/TypePath、impl AssetLoader、init_asset]
cargo run --example example_3d_texture         # 132 3D 纹理贴图：图片贴到立方体  [核心概念：base_color_texture、UV 坐标]
cargo run --example example_3d_fog             # 133 3D 雾效：距离雾渐隐  [核心概念：DistanceFog、FogFalloff]
cargo run --example example_3d_bloom           # 134 3D 泛光：发光材质光晕  [核心概念：Bloom、emissive HDR、Tonemapping]
cargo run --example example_ecs_substate       # 135 子状态：嵌套状态机  [核心概念：#[derive(SubStates)]、#[source]、add_sub_state]
cargo run --example example_ecs_computed_state # 136 计算状态：自动派生状态  [核心概念：ComputedStates、compute、add_computed_state]
cargo run --example example_ecs_state_transition_event # 137 状态转换事件 + 游戏流程  [核心概念：StateTransitionEvent、MessageReader、分数资源]
cargo run --example example_diagnostics       # 138 诊断系统：FPS/帧时间统计  [核心概念：FrameTimeDiagnosticsPlugin、DiagnosticsStore、smoothed]
cargo run --example example_diagnostics_custom # 139 自定义诊断指标：记录自定义数据  [核心概念：register_diagnostic、Diagnostics、add_measurement]
cargo run --example example_audio_global_volume # 140 全局音量：影响新播放的音频  [核心概念：GlobalVolume、Volume::Linear]
cargo run --example example_audio_multi       # 141 多音源混合：多个 AudioSink 同时播放  [核心概念：多个 AudioPlayer、Query<&AudioSink>]
cargo run --example example_audio_playback    # 142 播放控制：播放/暂停/停止/静音  [核心概念：AudioSinkPlayback、play/pause/stop/toggle_mute]
cargo run --example example_window_fullscreen # 143 窗口全屏切换：F11 切换全屏/窗口  [核心概念：Window.mode、WindowMode::Fullscreen/Windowed]
cargo run --example example_2d_particle_trail # 144 2D 粒子尾迹：鼠标移动生成粒子拖尾  [核心概念：Sprite::from_color、viewport_to_world_2d、生命周期淡出]
cargo run --example example_input_keyboard  # 145 键盘进阶：按键状态/组合键/修饰键  [核心概念：pressed/just_pressed/just_released、Ctrl+R]
cargo run --example example_input_mouse_motion # 146 鼠标移动与拖拽：相对位移光标  [核心概念：AccumulatedMouseMotion、ButtonInput<MouseButton>]
cargo run --example example_time_stopwatch   # 147 秒表：从 0 向上累计计时  [核心概念：Stopwatch、tick、elapsed_secs、pause/unpause]
cargo run --example example_window_cursor    # 148 光标捕获与可见性：隐藏/限制/锁定  [核心概念：CursorOptions、CursorGrabMode]
cargo run --example example_ecs_removed_components # 149 检测组件被移除  [核心概念：RemovedComponents、remove、despawn]
cargo run --example example_random      # 150 随机数：随机生成位置/颜色/大小  [核心概念：rand::rng、random、random_range、random_bool]
cargo run --example example_visibility  # 151 实体可见性：隐藏/显示/继承  [核心概念：Visibility、InheritedVisibility]
cargo run --example example_async_io    # 152 异步 I/O：IoTaskPool 模拟网络下载  [核心概念：IoTaskPool、Task、is_finished]
cargo run --example example_network_request # 153 真实网络请求：ureq 异步 HTTP GET  [核心概念：ureq、IoTaskPool、超时与错误处理]
cargo run --example example_asset_remote  # 154 远程资产加载：AssetServer 加载网络图片  [核心概念：AssetServer、LoadState、https 特性]
cargo run --example example_ecs_related_query # 155 关系查询进阶：related/sources/root  [核心概念：related、relationship_sources、root_ancestor]
cargo run --example example_serialization # 156 序列化存档：serde + ron 存档/读档  [核心概念：Serialize、ron::to_string/from_str]
cargo run --example example_animation_blend # 157 动画混合：平滑切换两个动画  [核心概念：AnimationTransitions、play、交叉淡入淡出]
cargo run --example example_3d_custom_material # 158 自定义 3D 材质：Material trait + WGSL 着色器  [核心概念：Material、AsBindGroup、MeshMaterial3d、绑定组]
cargo run --example simple_mini_2d_star        # 159 组合·2D 综合小游戏「星空收集」：收集星星躲避陨石  [核心概念：状态机+相机跟随+视差+粒子尾迹+图集动画+圆形碰撞+音频+UI 计分]
cargo run --example simple_mini_3d_coin        # 160 组合·3D 综合小游戏「3D 收集金币」：第三人称跳跃收集金币，倒计时内集满获胜  [核心概念：3D 场景+方向光+PBR 金属材质+重力跳跃+3D 碰撞+第三人称相机+UI 叠加]
cargo run --example simple_mini_ui_todo        # 161 组合·UI 综合应用「待办清单」：添加/删除/选中待办 + 设置页调音效/音量/主题 + 存档  [核心概念：TextInput/Button/Checkbox/Slider/RadioGroup/ListBox/ScrollArea 控件+ValueChange+序列化+状态机]
```

## 关于 `bsn!` 宏

多个示例用 `bsn!`（Bevy 0.19 的场景构建宏）以「声明式」方式构建实体：

```rust
commands.spawn_scene(bsn! {
    Text2d::new("提示文字")
    TextColor(Color::WHITE)
    TextFont { font_size: FontSize::Px(30.0), ..default() }
    Transform::from_xyz(0.0, -100.0, 0.0)
});
```

- 效果等价于 `commands.spawn((...))` 传入组件元组，但书写更像「配置清单」，组件较多时更易读。
- `bsn!` 内每个组件都挂到同一个实体上，组件之间用换行分隔（不加逗号）。
- 用到的组件需实现 `Clone + Default`（`bsn!` 内部通过模板反射构造实体）。
- 传统 `spawn((...))` 元组写法同样有效，两者可混用。

**本项目约定（两种写法并存的原因）**：

- 声明式 UI / 文本（`Text2d`、`Text`、`Node`、`Button`、`ImageNode`、`Children` 等）→ 用 `bsn!`，可读性最好。
- 过程式游戏对象（`Mesh2d`、`MeshMaterial2d`、`Sprite`、3D 物体、粒子、灯光等）→ 用 `commands.spawn((...))` 元组。因为这类实体需要在运行时调用 `meshes.add()` / `materials.add()` 创建资产，或依赖条件分支（如精灵图片缺失时回退纯色方块），属于命令式逻辑，无法用 `bsn!` 的声明式写法简洁表达。
- 相机 → 用 `commands.spawn(Camera2d)`（单个组件最简单）。

## 资产说明

- 中文字体：`assets/fonts/Yozai-Regular.ttf`，各示例通过 `FontSourceTemplate::Handle("fonts/Yozai-Regular.ttf")` 加载，避免中文乱码。
- 图片：`assets/images/bevy_bird_dark.png`（[example_2d_sprite](examples/example_2d_sprite.rs) 加载，文件缺失时自动降级为纯色方块）；`assets/images/bevy_logo_bevy.png`（[simple_mini_breakout](examples/simple_mini_breakout/main.rs) 启动画面 Logo）。
- 音频：`assets/audio/bg.wav`（[example_audio](examples/example_audio.rs) 背景音乐）和 `assets/audio/blip.wav`（一次性音效）。这两个 WAV 文件需启用 bevy 的 `wav` 特性才能解码（已在 Cargo.toml 中开启）。
- 图集：`assets/images/ball_spritesheet.png`（[example_2d_texture_atlas](examples/example_2d_texture_atlas.rs) 精灵图集，4 帧弹跳小球，脚本生成）。
- 着色器：`assets/shaders/my_material.wgsl`（[example_2d_custom_material](examples/example_2d_custom_material.rs) 自定义 2D 材质使用的 WGSL 着色器）。
- 3D 模型：`assets/models/skeletal_arm.gltf` + `skeletal_arm.bin`（[generate_skeletal_gltf](examples/generate_skeletal_gltf.rs) 生成的极简带骨骼「手臂」模型，由 [example_animation_skeletal](examples/example_animation_skeletal.rs) 加载并播放骨骼动画）。
- 3D 模型：`assets/models/humanoid.gltf` + `humanoid.bin`（[generate_humanoid_gltf](examples/generate_humanoid_gltf.rs) 生成的多关节「人形」模型，15 骨骼 + 关节处多权重蒙皮 + Walk/Run/Idle 三动画，由 [example_animation_humanoid](examples/example_animation_humanoid.rs) 加载，空格在走/跑/静止间平滑过渡）。
- `assets/images/bevy_logo.png`（[example_ui_image](examples/example_ui_image.rs) 图片节点示例使用的图片）。
- 自定义资产：`assets/level1.level`（[example_asset_custom](examples/example_asset_custom.rs) 自定义 AssetLoader 加载的文本关卡数据，第一行关卡名、第二行敌人数量）。

---

## 建议学习顺序

建议从最基础的 ECS/应用骨架起步，按顺序递进地攻克 Bevy 概念：

| 顺序 | 示例 | 攻克的核心概念 |
|------|------|----------------|
| 1 | `main.rs` | App / Plugins / Startup / Camera2d / Mesh2d / Handle / Commands.spawn |
| 2 | `example_2d_movement` | ButtonInput、Time::delta_secs、帧无关的 dt 更新玩家 Transform |
| 3 | `example_2d_sprite` | AssetServer 加载图片、缺省回退处理、Sprite/材质/组件组合 |
| 4 | `example_ecs_events` | Event、EventWriter、EventReader、事件驱动的系统解耦 |
| 5 | `example_ui` | Node、Button、Interaction 状态机、Flexbox UI 布局 |
| 6 | `example_input` | 鼠标坐标转换、PrimaryWindow 查询、滚轮事件 AccumulatedMouseScroll |
| 7 | `example_animation` | Time::elapsed_secs 驱动动画、Transform 的 translation / rotation / scale |
| 8 | `example_ecs_state` | States 状态机、OnEnter / OnExit / in_state 条件 |
| 9 | `example_2d_collision` | 圆形碰撞检测、弹性响应、iter_combinations_mut、.chain() 顺序 |
| 10 | `example_2d_particles` | lifetime 生命周期管理、重力模拟、Assets 资源、LCG 伪随机数 |
| 11 | `example_3d_lighting` | 3D 相机 / 光照 / 阴影 / PBR StandardMaterial / 双相机叠加 |
| 12 | `example_2d_lighting` | bevy_firefly 第三方库、PointLight2d / Occluder2d / FireflyConfig 环境光 / 软阴影 |
| 13 | `simple_mini` | 多文件 mod 模块组织、Message/MessageWriter/MessageReader 消息系统、游戏循环整合（移动+射击+碰撞+计分） |
| 14 | `simple_mini_breakout` | 分层 States 状态机（GlobalGameState + 嵌套 GameState/MenuState）、EntityEvent+trigger+Observer 即时事件、bsn! 场景语法、BoundingCircle/Aabb2d 碰撞、拍面反弹角度、DespawnOnExit 自动清理 |
| 15 | `simple_mini_snake` | 页面状态机（Loading→Menu→Playing→GameOver）、数据/渲染分离（Block→Sprite 自动渲染）、Follow 链蛇身跟随、MoveTimer/MoveTick tick 移动管线、网格坐标换算 |
| 16 | `example_time_timer` | Timer 组件 vs Time 资源、tick / just_finished、TimerMode（Once/Repeating）、每实体独立计时 |
| 17 | `example_audio` | AudioPlayer、PlaybackSettings（LOOP/DESPAWN）、AudioSink 播放控制、异步资产加载 |
| 18 | `example_ecs_hierarchy` | ChildOf / Children 关系组件、with_children、局部 Transform vs 全局 GlobalTransform |
| 19 | `example_ecs_system_sets` | SystemSet、configure_sets(...).chain()、in_set 分组、显式管理系统执行顺序 |
| 20 | `example_2d_gizmos` | Gizmos 调试绘制、line_2d / circle_2d / rect_2d / arrow_2d |
| 21 | `example_time_fixed_timestep` | Time<Fixed>::from_hz、FixedUpdate、固定 dt 与帧率解耦 |
| 22 | `example_window` | WindowPlugin / Window 字段（title/resolution/resizable/position）、Changed<Window> 监听尺寸变化 |
| 23 | `example_logging` | trace/debug/info/warn/error 日志宏、日志级别、RUST_LOG 过滤 |
| 24 | `example_camera` | 相机 Transform 移动、OrthographicProjection.scale 缩放、Projection 枚举 |
| 25 | `example_plugin` | impl Plugin、build(&self, app)、插件化代码组织 |
| 26 | `example_2d_texture_atlas` | TextureAtlasLayout::from_grid、TextureAtlas.index 帧切换、图集动画 |
| 27 | `example_ui_text_input` | EditableText 可编辑文本、InputFocus 聚焦、value/clear |
| 28 | `example_audio_spatial` | SpatialListener、PlaybackSettings::with_spatial、3D 空间音效 |
| 29 | `example_3d_custom_mesh` | Mesh::new、insert_attribute 顶点属性、PrimitiveTopology 自定义网格 |
| 30 | `example_3d_transform` | Transform.translation/rotation/scale、rotate_local_x/y/z、3D 变换 |
| 31 | `example_audio_control` | AudioSink、set_volume(Volume::Linear)、set_speed 实时音频控制 |
| 32 | `example_input_gamepad` | ButtonInput<GamepadButton>、Axis<GamepadAxis>、手柄输入 |
| 33 | `example_ecs_performance` | With/Without 查询过滤、Changed/Added 变更检测、系统并行调度 |
| 34 | `example_ecs_resource` | Resource 单例、Res/ResMut 读写、init_resource/insert_resource、FromWorld 资源间依赖 |
| 35 | `example_ecs_query` | Query 遍历、Single 单实体、Query::get 精确访问、With/Without 过滤与并行调度 |
| 36 | `example_3d_scene` | Camera3d、DirectionalLight、Mesh3d/MeshMaterial3d/StandardMaterial、Cuboid/Sphere/Plane3d |
| 37 | `example_ecs_observer` | EntityEvent、commands.trigger、observe/add_observer、On<T> 即时事件 |
| 38 | `example_time_control` | Time<Virtual> 暂停/缩放、pause/unpause/set_relative_speed |
| 39 | `example_ecs_run_condition` | run_if、自定义条件函数、resource_changed、chain 排序 |
| 40 | `example_ecs_system_param` | #[derive(SystemParam)]、参数组合（Query/Res/ResMut） |
| 41 | `example_ecs_message` | Message/MessageWriter/MessageReader 双缓冲队列 |
| 42 | `example_ecs_reflect` | #[derive(Reflect)]、register_type、reflect_ref、field/try_downcast_ref |
| 43 | `example_ecs_deferred` | Commands 延迟应用、spawn 后本系统不可见、chain 自动 ApplyDeferred |
| 44 | `example_ui_grid` | Display::Grid、grid_template_columns/rows、RepeatedGridTrack::fr、GridPlacement |
| 45 | `example_animation_clip` | AnimatableCurve/KeyframeCurve、animated_field!、AnimationGraph、AnimationPlayer、AnimationTargetId/AnimatedBy |
| 46 | `example_scene` | impl Scene 可复用场景函数、spawn_scene、bsn! 复用 |
| 47 | `example_asset_hot_reload` | AssetServer::load、AssetEvent、MessageReader、资产热重载 |
| 48 | `example_ecs_param_set` | ParamSet、p0/p1 按索引访问、解决同一系统借用冲突 |
| 49 | `example_render_layers` | RenderLayers::layer、相机分层渲染、多相机叠加 |
| 50 | `example_ecs_exclusive_system` | &mut World 排他系统、world.resource/resource_mut、entities().len |
| 51 | `example_window_events` | WindowEvent、MessageReader、窗口尺寸/焦点/关闭事件 |
| 52 | `example_2d_custom_material` | AsBindGroup、Material2d、Material2dPlugin、自定义 WGSL 着色器 |
| 53 | `example_animation_skeletal` | WorldAssetRoot 加载 gltf 场景、Gltf.animations、AnimationGraph::from_clips、AnimationGraphHandle、AnimationPlayer 骨骼动画 |
| 54 | `example_animation_humanoid` | 多关节人形蒙皮（15 骨骼）、关节处多骨骼权重混合、AnimationTransitions 走/跑/静止平滑过渡 |
| 55 | `example_ecs_query_filter` | Or 组合过滤、Changed/Added 变更检测、chain 控制执行顺序 |
| 56 | `example_ecs_hierarchy_query` | ChildOf/Children 关系组件、iter_descendants/iter_ancestors/related 层级遍历 |
| 57 | `example_input_cursor` | CursorIcon/SystemCursorIcon、PrimaryWindow 窗口实体、动态切换光标 |
| 58 | `example_3d_procedural_mesh` | insert_indices/Indices::U16、索引网格复用顶点、3D 立方体 |
| 59 | `example_2d_sprite_properties` | Sprite.flip_x/flip_y/color/custom_size、Anchor 独立组件 |
| 60 | `example_ui_focus` | TabGroup/TabIndex、TabNavigationPlugin、FocusGained/FocusLost 焦点事件 |
| 61 | `example_ecs_relationship` | #[relationship]/#[relationship_target] 自定义关系、一对多关系自动维护 |
| 62 | `example_async_task` | AsyncComputeTaskPool、Task::is_finished、block_on/poll_once 异步任务 |
| 63 | `example_window_multi` | Window 组件、RenderTarget::Window、WindowRef::Entity、RenderLayers 多窗口 |
| 64 | `example_3d_gizmos` | gizmos.line/arrow/sphere/circle、Isometry3d 定位 3D 调试绘制 |
| 65 | `example_ui_widgets` | Checkbox/Slider 无样式控件、SliderValue/SliderRange、ValueChange 事件 |
| 66 | `example_ui_toggle_progress` | Checkbox 组合自定义视觉、Node.width 百分比动态更新 |
| 67 | `example_ui_scroll_area` | ScrollArea、ScrollPosition、Overflow::scroll_y、滚动容器 |
| 68 | `example_ui_radio_group` | RadioGroup/RadioButton、Checked、ValueChange<Entity>、单选互斥 |
| 69 | `example_ui_list_box` | ListBox/ListItem、Selected、ValueChange<Entity>、ActiveDescendant |
| 70 | `example_ui_menu` | MenuButton/MenuPopup/MenuItem、MenuEvent/MenuAction、MenuFocusState、Activate |
| 71 | `example_ui_layout` | Val 单位（px/percent/vw/vh）、UiRect 盒模型（margin/padding/border）、PositionType::Absolute、BorderRadius/Outline |
| 72 | `example_ui_gradient` | BackgroundGradient、LinearGradient/RadialGradient/ConicGradient、ColorStop/AngularColorStop |
| 73 | `example_ui_zindex` | ZIndex、GlobalZIndex、兄弟叠放顺序、跨层级叠加 |
| 74 | `example_ui_transform` | UiTransform、Val2、Rot2、Time 驱动 UI 变换动画 |
| 75 | `example_ui_image` | ImageNode、NodeImageMode::Stretch/Tiled、flip_x、color tint、solid_color |
| 76 | `example_ui_shadow` | BoxShadow、ShadowStyle、Outline、多层阴影/描边 |
| 77 | `example_ui_viewport` | ViewportNode、Image::new_target_texture、RenderTarget::Image、画中画 |
| 78 | `example_ui_cursor_position` | RelativeCursorPosition、normalized 坐标、UiTransform 跟随鼠标 |
| 79 | `example_ui_scale` | UiScale 资源、ResMut<UiScale>、+/- 动态缩放 UI |
| 80 | `example_ui_text_style` | TextShadow、TextBackgroundColor、TextLayout/Justify、LineHeight、LetterSpacing |
| 81 | `example_ui_span` | TextSpan、多段 TextColor/TextFont、父子文本拼接 |
| 82 | `example_ui_focus_policy` | FocusPolicy::Block/Pass、遮罩拦截/穿透、Button 点击检测 |
| 83 | `example_ui_overflow` | Overflow::visible/clip/hidden、溢出内容处理 |
| 84 | `example_ui_text_bounds` | TextBounds::new_horizontal、TextLayout/Justify、自动换行 |
| 85 | `example_ui_target_camera` | UiTargetCamera、RenderTarget::Window、多窗口 UI |
| 86 | `example_ui_interaction` | Interaction::Pressed/Hovered/None、Changed<Interaction>、状态切换 |
| 87 | `example_2d_primitives` | Mesh2d、Circle/Rectangle/RegularPolygon/Capsule2d/Ellipse/Annulus/Triangle2d |
| 88 | `example_2d_text` | Text2d、Text2dShadow、Transform 定位/旋转 |
| 89 | `example_2d_mesh` | Mesh::new、insert_attribute、PrimitiveTopology、Mesh2d 自定义网格 |
| 90 | `example_2d_picking` | MeshPickingPlugin、On<Pointer<Click>>、点击变色 |
| 91 | `example_2d_drag` | On<Pointer<Drag>>、Camera::viewport_to_world_2d、屏幕转世界 |
| 92 | `example_2d_camera_follow` | 指数平滑 lerp、Single 查询、相机跟随目标 |
| 93 | `example_2d_sprite_fade` | Sprite::from_color、Color::set_alpha、sin 周期动画 |
| 94 | `example_2d_parallax` | 多层 Sprite、速度组件、视差层次 |
| 95 | `example_2d_gravity` | 速度向量、重力加速度、速度积分、阻尼反弹 |
| 96 | `example_2d_rotate_to_mouse` | cursor_position、viewport_to_world_2d、atan2、Quat::from_rotation_z |
| 97 | `example_2d_bounce` | sin 周期运动、speed/phase 控制速度与相位 |
| 98 | `example_2d_wrap` | 匀速运动、边界环绕、屏幕边界检测 |
| 99 | `example_2d_scale_pulse` | Transform.scale、sin 脉冲、phase 错开 |
| 100 | `example_2d_spawner` | Timer::Repeating、commands.spawn、despawn 生命周期 |
| 101 | `example_2d_color_cycle` | ColorMaterial 运行时改色、sin RGB 循环、Assets 访问 |
| 102 | `example_2d_grid` | 嵌套循环生成、共享 Mesh 句柄、交替颜色 |
| 103 | `example_animation_multi_curve` | 多曲线剪辑、animated_field!、AnimatableKeyframeCurve |
| 104 | `example_animation_playback` | pause_all/resume_all、all_paused、adjust_speeds、rewind_all |
| 105 | `example_animation_event` | AnimationEvent derive、AnimationClip::add_event、On<Event> 全局 observer |
| 106 | `example_animation_easing` | Linear/EaseIn/EaseOut/EaseInOut、时间归一化与进度映射 |
| 107 | `example_camera_shake` | Resource 创伤值、trauma² 力度、rand 随机偏移 |
| 108 | `example_camera_bounds` | 可见半视野、clamp 限制相机中心 |
| 109 | `example_camera_zoom_to_cursor` | 世界点=中心+偏移×scale、缩放补偿 |
| 110 | `example_camera_orbit_3d` | 球坐标、look_at、yaw/pitch/distance |
| 111 | `example_camera_scaling_modes` | ScalingMode、Fixed/AutoMin/AutoMax/FixedVertical |
| 112 | `example_camera_rotation_2d` | 相机 Transform.rotate_z、旋转相机 vs 旋转物体 |
| 113 | `example_camera_first_person_3d` | yaw/pitch、AccumulatedMouseMotion、forward/right |
| 114 | `example_camera_third_person_3d` | 角色朝向偏移、look_at、chain 顺序 |
| 115 | `example_3d_primitives` | Cuboid/Sphere/Cylinder/Cone/Capsule3d/Torus、Mesh3d |
| 116 | `example_3d_hierarchy` | with_children、局部/全局 Transform、嵌套公转 |
| 117 | `example_3d_picking` | MeshPickingPlugin、On<Pointer<Click>>、StandardMaterial |
| 118 | `example_3d_material` | metallic/roughness/emissive/unlit/alpha |
| 119 | `example_3d_gravity` | 重力加速度、速度积分、恢复系数 |
| 120 | `example_3d_collision` | 球-球碰撞、边界碰撞、分离 + 交换法线速度 |
| 121 | `example_3d_projectile` | 初速度、重力只影响 y、动态生成抛体 |
| 122 | `example_scene_hierarchy` | Children [...]、局部/全局 Transform |
| 123 | `example_scene_patch` | Health { max } 补丁、Added 过滤 |
| 124 | `example_scene_list` | bsn_list!、spawn_scene_list |
| 125 | `example_scene_system` | impl SceneList、scene.spawn() |
| 126 | `example_ecs_storage` | #[component(storage)]、默认 Table、稀疏存储 |
| 127 | `example_ecs_sparse_set` | SparseSet 组件、Commands 延迟、PostUpdate |
| 128 | `example_ecs_storage_benchmark` | Instant 计时、缓存友好、遍历耗时 |
| 129 | `example_asset_basics` | AssetServer::load、Handle 克隆共享、Assets<T>::get |
| 130 | `example_asset_load_state` | LoadState、load_state/is_loaded、加载完成再渲染 |
| 131 | `example_asset_custom` | derive Asset/TypePath、impl AssetLoader、init_asset |
| 132 | `example_3d_texture` | base_color_texture、UV 坐标 |
| 133 | `example_3d_fog` | DistanceFog、FogFalloff |
| 134 | `example_3d_bloom` | Bloom、emissive HDR、Tonemapping |
| 135 | `example_ecs_substate` | #[derive(SubStates)]、#[source]、add_sub_state |
| 136 | `example_ecs_computed_state` | ComputedStates、compute、add_computed_state |
| 137 | `example_ecs_state_transition_event` | StateTransitionEvent、MessageReader、分数资源 |
| 138 | `example_diagnostics` | FrameTimeDiagnosticsPlugin、DiagnosticsStore、smoothed |
| 139 | `example_diagnostics_custom` | register_diagnostic、Diagnostics、add_measurement |
| 140 | `example_audio_global_volume` | GlobalVolume、Volume::Linear |
| 141 | `example_audio_multi` | 多个 AudioPlayer、Query<&AudioSink> |
| 142 | `example_audio_playback` | AudioSinkPlayback、play/pause/stop/toggle_mute |
| 143 | `example_window_fullscreen` | Window.mode、WindowMode::Fullscreen/Windowed |
| 144 | `example_2d_particle_trail` | Sprite::from_color、viewport_to_world_2d、生命周期淡出 |
| 145 | `example_input_keyboard` | pressed/just_pressed/just_released、组合键、修饰键 |
| 146 | `example_input_mouse_motion` | AccumulatedMouseMotion、ButtonInput<MouseButton> |
| 147 | `example_time_stopwatch` | Stopwatch、tick、elapsed_secs、pause/unpause |
| 148 | `example_window_cursor` | CursorOptions、CursorGrabMode |
| 149 | `example_ecs_removed_components` | RemovedComponents、remove、despawn |
| 150 | `example_random` | rand::rng、random、random_range、random_bool |
| 151 | `example_visibility` | Visibility、InheritedVisibility、toggle_* |
| 152 | `example_async_io` | IoTaskPool、Task、is_finished、poll_once |
| 153 | `example_network_request` | ureq、IoTaskPool、timeout、错误处理 |
| 154 | `example_asset_remote` | AssetServer 远程加载、LoadState、https 特性 |
| 155 | `example_ecs_related_query` | related、relationship_sources、root_ancestor |
| 156 | `example_serialization` | Serialize、ron::to_string/from_str、存档 |
| 157 | `example_animation_blend` | AnimationTransitions、play、平滑过渡 |
| 158 | `example_3d_custom_material` | Material、AsBindGroup、MeshMaterial3d、WGSL 着色器 |
| 159 | `simple_mini_2d_star` | 组合用例：状态机 + 相机平滑跟随 + 视差滚动 + 粒子尾迹/爆炸 + 图集动画 + 圆形碰撞 + 音频 + UI 计分（2D 综合小游戏） |
| 160 | `simple_mini_3d_coin` | 组合用例：3D 场景 + 方向光 + PBR 金属材质 + 重力跳跃 + 3D 距离碰撞 + 第三人称相机 + UI 叠加 + 倒计时（3D 综合小游戏） |
| 161 | `simple_mini_ui_todo` | 组合用例：TextInput/Button/Checkbox/Slider/RadioGroup/ListBox/ScrollArea 控件 + ValueChange 事件 + 数据驱动 UI 重建 + serde/ron 存档（UI 综合应用） |

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
