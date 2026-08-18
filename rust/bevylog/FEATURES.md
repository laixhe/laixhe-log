# Bevy 0.19 特性（features）参考

本文整理 `bevy` 主 crate 的全部 Cargo features，说明每个特性的作用，以及哪些默认开启、哪些需要手动开启。

> 依据：`bevy-0.19.1` 的 `Cargo.toml` `[features]` 段。特性之间的「传递依赖」以该文件为准。

## 本项目当前启用的特性

[Cargo.toml](Cargo.toml) 里对 bevy 的配置：

```toml
bevy = { version = "0.19", features = ["dynamic_linking", "wav", "https"] }
```

| 特性 | 为什么启用 |
|------|-----------|
| `dynamic_linking` | 加快增量编译速度（开发期）。发布 exe 时需去掉并改为静态链接 |
| `wav` | 项目音频资产是 `.wav` 格式，默认的 `audio` 只带 `vorbis`(ogg) 解码 |
| `https` | 供 `example_asset_remote` 从 `https://` 加载远程图片 |

## 默认开启的特性

`bevy` 的 `default` 特性只包含 4 个聚合特性，其余靠它们层层传递展开：

```toml
default = ["2d", "3d", "ui", "audio"]
```

## Bevy 0.19 默认支持的资源格式

### 🖼️ 图片格式（默认启用）
| 格式 | Feature | 说明 |
|------|---------|------|
| **PNG** | `png` | ✅ 默认 |
| **HDR** | `hdr` | ✅ 默认 |
| **KTX2** | `ktx2` | ✅ 默认（含 `zstd_rust` 解压支持） |

> 未默认启用的图片格式：BMP、DDS、EXR、Farbfeld、GIF、ICO、JPEG、PNM、QOI、TGA、TIFF、WebP、Basis Universal

### 🎵 音频格式（默认启用）
| 格式 | Feature | 说明 |
|------|---------|------|
| **OGG Vorbis** | `vorbis` | ✅ 默认（通过 lewton 解码） |

> 未默认启用的音频格式：FLAC、MP3、WAV，以及通过 symphonia 后端的 AAC、FLAC、MP3、MP4、OGG/VORBIS、WAV

### 🏗️ 3D 模型格式（默认启用）
| 格式 | Feature | 说明 |
|------|---------|------|
| **GLTF/GLB** | `bevy_gltf` | ✅ 默认 |

### 🔤 字体（默认启用）
| 格式 | Feature | 说明 |
|------|---------|------|
| **TTF**（内置默认字体） | `default_font` | ✅ 默认（仅含 ASCII 字符，约 20KB） |

### 📦 场景格式（默认启用）
| 格式 | Feature | 说明 |
|------|---------|------|
| **BSN**（Bevy Scene Notation） | `bevy_scene` | ✅ 默认（Bevy 0.19 新增的 BSN 格式） |
| **RON**（传统场景格式） | `bevy_scene` | ✅ 默认 |

### 🎨 着色器格式（默认不启用）
| 格式 | Feature | 说明 |
|------|---------|------|
| GLSL | `shader_format_glsl` | ❌ 需手动启用 |
| SPIR-V | `shader_format_spirv` | ❌ 需手动启用 |
| WESL | `shader_format_wesl` | ❌ 需手动启用 |


也就是说，**不加任何 features 时**，等价于开启了下面这些（传递闭包）：

- **聚合维度**：`2d`、`3d`、`ui`、`audio`
- **应用基础设施**：`default_app`、`default_platform`、`common_api`、`async_executor`、`bevy_asset`、`bevy_log`、`bevy_state`、`reflect_auto_register`
- **平台/窗口**：`std`、`multi_threaded`、`bevy_winit`、`bevy_clipboard`、`default_font`、`custom_cursor`、`sysinfo_plugin`、`x11`、`wayland`、`webgl2`、`bevy_gilrs`、`gamepad`
- **渲染相关**：`bevy_render`、`bevy_core_pipeline`、`bevy_post_process`、`bevy_gizmos`、`bevy_gizmos_render`
- **2D**：`2d_api`、`2d_bevy_render`、`bevy_sprite`、`bevy_sprite_render`
- **3D**：`3d_api`、`3d_bevy_render`、`bevy_gltf`、`bevy_pbr`、`bevy_anti_alias`、`bevy_light`、`bevy_mikktspace`、`morph`、`morph_animation`、`gltf_animation`
- **UI**：`ui_api`、`ui_bevy_render`、`bevy_ui`、`bevy_ui_render`、`bevy_ui_widgets`、`bevy_input_focus`
- **音频**：`bevy_audio`、`vorbis`
- **通用 API**：`bevy_animation`、`bevy_camera`、`bevy_color`、`bevy_image`、`bevy_mesh`、`bevy_shader`、`bevy_material`、`bevy_text`、`bevy_window`
- **场景/拾取**：`scene`、`bevy_scene`、`bevy_world_serialization`、`picking`、`bevy_picking`、`mesh_picking`、`sprite_picking`、`ui_picking`
- **格式/纹理**：`png`、`hdr`、`ktx2`、`smaa_luts`、`tonemapping_luts`、`zstd_rust`

> 简单记：默认就「2D + 3D + UI + 音频」全都要，桌面（winit/剪贴板/字体/手柄/拾取）+ PNG/HDR/KTX2 图片 + OGG 音频都带上了。

## 非默认特性（按需手动开启）

下面的特性**不在** `default` 的展开集合里，需要显式加到 `features` 数组才生效。

### 图片格式

| 特性 | 功能 |
|------|------|
| `jpeg` | 解码 JPEG（`.jpg`） |
| `webp` | 解码 WebP |
| `bmp` | 解码 BMP |
| `gif` | 解码 GIF |
| `ico` | 解码 ICO（图标） |
| `tga` | 解码 TGA |
| `tiff` | 解码 TIFF |
| `pnm` | 解码 PNM（PBM/PGM/PPM） |
| `qoi` | 解码 QOI |
| `dds` | 解码 DDS |
| `exr` | 解码 EXR（HDR 高动态范围） |

### 音频格式

| 特性 | 功能 |
|------|------|
| `wav` | 解码 WAV（本项目已开启） |
| `mp3` | 解码 MP3 |
| `flac` | 解码 FLAC |
| `aac` | 解码 AAC |
| `mp4` | 解码 MP4 |
| `symphonia-flac` / `symphonia-vorbis` / `symphonia-wav` | 使用 symphonia 后端的对应格式（底层实现，一般用上面高层特性） |
| `audio-all-formats` | 聚合：一次性开启全部音频格式（aac/flac/mp3/mp4/vorbis/wav） |

### 网络 / 远程资产

| 特性 | 功能 |
|------|------|
| `https` | 从 `https://` URL 加载远程资产（带 TLS，依赖 rustls）——本项目已开启 |
| `http` | 从 `http://` URL 加载远程资产（明文，无 TLS） |
| `web_asset_cache` | 远程资产的本地磁盘缓存 |
| `async-io` | 异步 I/O 运行时 |

### 平台 / 后端

| 特性 | 功能 |
|------|------|
| `web` | 面向浏览器（wasm）的 web 平台支持 |
| `webgpu` | WebGPU 后端（wasm） |
| `android-game-activity` | Android 的 GameActivity 入口 |
| `android-native-activity` | Android 的 NativeActivity 入口 |
| `accesskit_unix` | Linux 无障碍（accessibility）支持 |

### 着色器格式（热重载）

| 特性 | 功能 |
|------|------|
| `shader_format_glsl` | 支持 GLSL 着色器热重载 |
| `shader_format_spirv` | 支持 SPIR-V 着色器热重载 |
| `shader_format_wesl` | 支持 WESL 着色器热重载 |
| `spirv_shader_passthrough` | SPIR-V 着色器直通 |

### 反射增强

| 特性 | 功能 |
|------|------|
| `reflect_auto_register_static` | 静态类型的反射自动注册 |
| `reflect_documentation` | 反射里携带文档字符串 |
| `reflect_functions` | 反射支持函数调用 |

### 调试 / 追踪

| 特性 | 功能 |
|------|------|
| `debug` | 调试信息 |
| `debug_glam_assert` | glam 数学库的断言检查 |
| `glam_assert` | glam 断言 |
| `detailed_trace` | 详细追踪 |
| `trace` | 基础追踪 |
| `trace_chrome` | 输出 Chrome 格式追踪（可在 chrome://tracing 查看） |
| `trace_tracy` | 输出 Tracy 格式追踪 |
| `trace_tracy_memory` | Tracy 内存追踪 |
| `track_location` | 追踪调用位置 |
| `bevy_debug_stepping` | 调试步进 |
| `bevy_dev_tools` | 开发工具合集 |
| `dev` | 聚合：debug + bevy_dev_tools + file_watcher |

### 性能 / 链接

| 特性 | 功能 |
|------|------|
| `dynamic_linking` | 动态链接 bevy（大幅加快开发期编译）——本项目已开启 |

### PBR / 渲染高级

| 特性 | 功能 |
|------|------|
| `pbr_anisotropy_texture` | PBR 各向异性纹理 |
| `pbr_clustered_decals` | 集群贴花 |
| `pbr_light_textures` | 光照纹理 |
| `pbr_multi_layer_material_textures` | 多层材质纹理 |
| `pbr_specular_textures` | 镜面反射纹理 |
| `pbr_transmission_textures` | 透射（次表面）纹理 |
| `experimental_pbr_pcss` | 实验性 PCSS 软阴影 |
| `meshlet` | Meshlet 渲染（Nanite 风格，实验性） |
| `meshlet_processor` | Meshlet 资产处理 |
| `area_light_luts` | 面光源查找表纹理 |
| `dfg_lut` | DFG 查找表纹理 |
| `bluenoise_texture` | 蓝噪声纹理 |
| `dlss` | NVIDIA DLSS 超分辨率 |
| `force_disable_dlss` | 强制禁用 DLSS |
| `statically-linked-dxc` | 静态链接 DXC 着色器编译器 |
| `raw_vulkan_init` | 原始 Vulkan 初始化 |

### 压缩

| 特性 | 功能 |
|------|------|
| `zlib` | zlib 压缩支持 |
| `zstd_c` | zstd（C 实现）压缩 |
| `compressed_image_saver` | 压缩图片保存 |

### 其他模块

| 特性 | 功能 |
|------|------|
| `bevy_remote` | Bevy 远程协议（BRP，远程调试/检查） |
| `bevy_camera_controller` | 内置飞行相机（flycam）控制器 |
| `free_camera` | 自由相机控制器 |
| `pan_camera` | 平移相机控制器 |
| `bevy_feathers` | Feathers UI 控件库 |
| `bevy_settings` | 运行时设置存储 |
| `bevy_solari` | Solari GPU 驱动渲染抽象（实验性） |
| `bevy_ui_debug` | UI 调试工具 |
| `bevy_ci_testing` | CI 测试辅助 |
| `keyboard` / `mouse` / `touch` / `gestures` | 独立输入设备模块（手柄 `gamepad` 已默认开启） |
| `serialize` | serde 序列化支持 |
| `system_font_discovery` | 系统字体自动发现 |
| `system_clipboard` | 系统剪贴板 |
| `clipboard_image` | 剪贴板图片 |
| `hotpatching` | 热补丁（代码热重载） |
| `file_watcher` | 资产文件监听（热重载） |
| `embedded_watcher` | 嵌入式资产监听 |
| `asset_processor` | 资产预处理 |
| `critical-section` / `libm` | no_std 环境支持 |
| `default_no_std` | no_std 默认配置（聚合） |
| `schedule_data` / `type_label_buffers` / `ff` | 内部实现细节 |

## 常用组合速查

| 目标 | 建议 features |
|------|--------------|
| 只做 2D | `["2d"]`（可省掉 3d/ui/audio 减小体积） |
| 只做 3D | `["3d"]` |
| 只做 UI | `["ui"]` |
| 加 JPEG 图片 | 追加 `"jpeg"` |
| 加 MP3 音频 | 追加 `"mp3"` |
| 加远程 https 资产 | 追加 `"https"` |
| 开发期加速编译 | 追加 `"dynamic_linking"` |

> 需要「减小编译体积/运行体积」时，可用 `default-features = false` 关掉默认，再只挑自己需要的维度特性（如 `bevy = { version = "0.19", default-features = false, features = ["2d", "png"] }`）。
