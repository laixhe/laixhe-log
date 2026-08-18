# sfmlapp

SFML 图形示例项目（C++23 + CMake + FetchContent），演示窗口创建、事件处理、图形绘制等。

## 依赖（通过 FetchContent 自动下载）

| 库 | 用途 |
|----|------|
| nlohmann/json | JSON 解析 |
| spdlog | 日志 |
| SFML | 图形与窗口 |

## 运行

```bash
cmake -B Build -D CMAKE_BUILD_TYPE=Debug -G "MinGW Makefiles"
cmake --build Build --config Debug -j
./Build/bin/sfmlapp
```

> 首次构建会自动下载并编译上述依赖，耗时较长。

## SFML 模块

- `Graphics` — 2D 图形渲染
- `Window` — 窗口管理和事件处理
- `Audio` — 音频播放和录制
- `System` — 系统工具（线程、时钟等）
- `Network` — 网络通信

源码 `src/main.cpp` 里含大量注释，覆盖：窗口 / 事件系统 / 角度系统 / 资源管理（纹理、精灵、字体、音乐）等。
