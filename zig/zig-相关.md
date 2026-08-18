# Zig 相关

> 本笔记基于 Zig 0.16（最低要求），Zig 0.16.0 于 2026-04 发布，是当前最新稳定版。

## 什么是 Zig

Zig 是一种系统级编程语言，目标是取代 C，主要特点：

- **手动内存管理**：没有隐式内存分配，内存操作更可控。
- **编译期求值（comptime）**：可在编译期执行代码，用于泛型与元编程。
- **无隐式类型转换**：类型转换需显式声明，减少隐藏 bug。
- **显式错误处理**：通过 error union 与 `try`/`catch` 处理错误。
- **交叉编译简单**：内置 libc 与目标平台支持，方便为不同平台构建。

## Zig 0.16 关键变化

0.16（代号 "Juicy Main"）引入两项重要变化：

- **Juicy Main**：`main` 函数可接收 `std.process.Init` 参数，一次性获取 allocator、Io、命令行参数、环境变量。
- **I/O 接口化**：文件系统、网络、时间、线程等统一到 `std.Io` 接口，通过 `io` 参数注入。

代码示例见 [ziglog](./ziglog/) 学习项目。

## 常用命令

```bash
zig version             # 查看 Zig 版本
zig targets             # 查看当前支持的平台（目标三元组）
zig init                # 初始化一个项目（生成 build.zig 等）
zig build               # 构建项目
zig build run           # 构建并运行项目
zig run file.zig        # 直接编译并运行单个文件
zig build-exe file.zig  # 编译单个文件为可执行文件
zig test file.zig       # 运行测试
zig fmt                 # 格式化源码
```

## 查看当前支持的平台

```bash
zig targets
```
