# guiapp

ImGui + SFML 的 GUI 示例项目（C++23 + CMake + FetchContent）。

## 依赖（通过 FetchContent 自动下载）

| 库 | 用途 |
|----|------|
| nlohmann/json | JSON 解析 |
| spdlog | 日志 |
| SFML | 窗口与图形 |
| imgui + ImGui-SFML | 即时模式 GUI |

## 运行

```bash
cmake -B Build -D CMAKE_BUILD_TYPE=Debug -G "MinGW Makefiles"
cmake --build Build --config Debug -j
./Build/bin/guiapp
```

> 首次构建会自动下载并编译上述依赖，耗时较长；后续构建走缓存。
