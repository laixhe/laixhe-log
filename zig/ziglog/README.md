# ziglog

Zig 语言学习项目：用源码文件承载各种基础语法示例，代码含中文注释，可逐个运行观察输出。

## 环境要求

| 项目 | 要求 |
|------|------|
| **Zig 版本** | 0.16（0.16.0 于 2026-04 发布，当前最新稳定版） |
| 查看版本 | `zig version` |

## 项目结构

| 文件 | 主题 | 说明 |
|------|------|------|
| `src/main.zig` | 入口程序 | Juicy Main + std.Io 输出 + 基础类型 / 数组 / 循环 / CLI 参数 |
| `src/basics.zig` | 基础类型 | 整数 / 浮点 / 数组 / 切片 / 枚举 / 结构体 |
| `src/control_flow.zig` | 控制流 | if / switch / for / while |
| `src/strings.zig` | 字符串 | 字面量 / 比较 / 格式化 |
| `src/errors.zig` | 错误处理 | error union / try / catch |
| `src/time.zig` | 时间与日期 | 时间戳 / 格式化 / 字符串解析 / 时间比较（std.time.epoch） |
| `src/regex.zig` | 正则 | 手机号 / 邮箱匹配（第三方库 mvzr） |
| `src/allocator.zig` | 内存管理 | alloc / free / create / ArenaAllocator |
| `src/file.zig` | 文件 I/O | writeFile / readFileAlloc / 临时目录 |
| `src/comptime.zig` | comptime 泛型 | 泛型函数 / 编译期计算 / 类型反射 |
| `src/json.zig` | JSON | 序列化 valueAlloc / 反序列化 parseFromSlice |
| `src/tcp.zig` | 网络 TCP | 客户端连接 / 回显服务器（std.Io.net） |
| `src/number.zig` | 数值进阶 | 格式化（hex/八进制/对齐/精度）、整数溢出（回绕/检测/饱和/检查）、类型转换 |
| `src/collections.zig` | 容器进阶 | ArrayList / StringHashMap / 去重 / DoublyLinkedList / PriorityQueue / 环形缓冲 / LRU |
| `src/sync.zig` | 并发同步 | 线程 / Mutex / 原子操作（对应 Go sync 包） |
| `src/lib/mvzr.zig` | 第三方库 | 轻量正则库 mvzr（单文件、零依赖、MIT） |

## 运行

所有示例通过 `zig run` 或 `zig test` 承载，建议按需运行：

```bash
# 运行入口程序
zig run src/main.zig

# 运行单个主题的测试（推荐）
zig test src/basics.zig
zig test src/control_flow.zig
zig test src/strings.zig
zig test src/errors.zig
zig test src/time.zig
zig test src/regex.zig
zig test src/allocator.zig
zig test src/file.zig
zig test src/comptime.zig
zig test src/json.zig
zig test src/tcp.zig
zig test src/number.zig
zig test src/collections.zig
zig test src/sync.zig
```

> ⚠️ 说明：`src/tcp.zig` 会启动本地服务端线程并监听 `127.0.0.1:12345` 端口，用于演示 TCP 客户端与回显服务。
> ⚠️ 说明：`src/regex.zig` 引用了第三方库 `src/lib/mvzr.zig`，`zig test src/regex.zig` 会连同 mvzr 自带的测试一起运行；只想看本项目用例时可用 `zig test src/regex.zig --test-filter 正则` 过滤。

## 学习路径建议（从易到难）

1. **基础**：`src/basics.zig` → `src/control_flow.zig`
2. **数值进阶**：`src/number.zig`
3. **字符串**：`src/strings.zig` → `src/regex.zig`
4. **时间**：`src/time.zig`
5. **错误处理**：`src/errors.zig`
6. **内存管理**：`src/allocator.zig`
7. **容器进阶**：`src/collections.zig` → `src/comptime.zig`
8. **文件 I/O**：`src/file.zig`
9. **JSON**：`src/json.zig`
10. **并发同步**：`src/sync.zig`
11. **网络 TCP**：`src/tcp.zig`
12. **综合**：`src/main.zig`

## 关于 Zig 0.16

Zig 0.16 引入了若干重要变更：

- **Juicy Main**：入口函数可接收 `std.process.Init` 参数，一次性获取 allocator、Io、命令行参数、环境变量。
- **I/O 接口化**：文件系统、网络、时间、线程等统一到 `std.Io` 接口，通过 `io` 参数注入。

本项目的 `src/main.zig` 演示了 Juicy Main 与 `std.Io` 的标准输出用法。
