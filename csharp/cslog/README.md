# cslog

C# / .NET 标准库学习示例项目（.NET 10 + C#），按主题拆分演示现代 C# 特性，纯 BCL（Base Class Library）、无需第三方依赖。

## 环境要求

| 项目 | 要求 |
|------|------|
| **.NET SDK** | 10.x |
| 查看版本 | `dotnet --version` |

## 项目结构

| 文件 | 主题 | 说明 |
|------|------|------|
| `Program.cs` | 入口 | 依次运行各主题模块 |
| `ControlFlowDemo.cs` | 控制流 | if / switch 表达式 / for / foreach / while / break / continue |
| `ClassDemo.cs` | 面向对象 | 类 / 属性封装 / 继承多态 / 抽象类 / 接口默认实现 / record / struct |
| `EnumDemo.cs` | 枚举 | enum / [Flags] 位标志组合 / 解析与转换 / 遍历 |
| `NumberDemo.cs` | 数值进阶 | 格式化（进制/补零/对齐/千分位）/ 溢出与饱和 / 类型转换 |
| `RandomDemo.cs` | 随机数 | Random / 区间 / 洗牌 / 抽样 / 种子 / 随机字符串 |
| `BitsDemo.cs` | 位运算 | 与或非异或 / 移位 / 奇偶与 2 幂判断 / lowbit / BitArray |
| `StringDemo.cs` | 字符串 | 常用方法 / 码元与码点 / 拼接与格式化 / 词频统计 |
| `NullableDemo.cs` | 可空类型 | int? / ?? 空合并 / ?. 空传播 / NRT 编译期空安全 |
| `CollectionDemo.cs` | 容器进阶 | List / Dictionary / HashSet / Queue / Stack / PriorityQueue / 环形缓冲 / LRU / 分组 |
| `IteratorDemo.cs` | LINQ | Select / Where / Take / Skip / Zip / Aggregate / Any / All / 综合实战 |
| `IteratorAdvDemo.cs` | LINQ 进阶 | GroupBy 分组 / Join 连接 / OrderBy 排序 / 集合运算 / 元素操作 / 延迟执行 / 分页 |
| `TimeDemo.cs` | 日期时间 | DateTime / 格式化 / 解析 / 时区 / 耗时 / 定时器 |
| `ExceptionDemo.cs` | 异常处理 | try-catch-finally / throw / 自定义异常 / using 释放 / 异常链 |
| `FileDemo.cs` | 文件 IO | 一次性读写 / 追加 / 流式读写 / 二进制 / 目录遍历 |
| `RegexDemo.cs` | 正则表达式 | 匹配 / 提取 / 捕获组 / 替换 / 分割 / 常用选项 |
| `GenericDemo.cs` | 泛型 | 泛型方法 / 泛型类 / 约束 where / 默认值 / 协变逆变 |
| `GenericAdvDemo.cs` | 泛型进阶 | 约束全解 / 装箱性能 / IComparable 泛型接口 / 自定义 in-out 协变逆变 / INumber 泛型数学 / 泛型缓存 |
| `DelegateDemo.cs` | 委托与事件 | delegate / Action / Func / 闭包 / 多播委托 / event |
| `PerfDemo.cs` | 性能优化 | Stopwatch 对比：Any/Count、过滤前置、物化、HashSet/Dictionary、预分配、StringBuilder、Array.Sort |
| `SyncDemo.cs` | 并发同步 | Task / lock / Interlocked / Parallel.For / Lazy |
| `JsonDemo.cs` | JSON | System.Text.Json 序列化 / 反序列化 / omitempty / 美化输出 |
| `HttpDemo.cs` | HTTP | HttpListener 服务端 + HttpClient 客户端（GET / POST JSON） |
| `cslog.Tests/` | 测试 | xUnit 冒烟测试：运行全部模块并校验关键输出 |

## 运行

```bash
dotnet run
```

> `Program.cs` 是入口，按顺序运行全部模块；想只看单个主题用 `dotnet run -- Number`（模块名或文件名关键字均可，如 `ControlFlow`、`CollectionDemo`、`Http`）。单个模块异常不会中断其余模块。

## 学习路径建议（从易到难）

1. **控制流**：`ControlFlowDemo`（if / switch 表达式 / 循环 / 跳转）
2. **面向对象**：`ClassDemo`（类 / 继承多态 / 接口 / record / struct）
3. **枚举**：`EnumDemo`（enum / [Flags] 位标志）
4. **字符串**：`StringDemo`（常用方法、码点、插值格式化）
5. **数值进阶**：`NumberDemo`（格式化、checked 溢出、类型转换）
6. **随机数**：`RandomDemo`（区间、洗牌、抽样、种子）
7. **位运算**：`BitsDemo`（与或非异或、移位、常用技巧）
8. **可空类型**：`NullableDemo`（int? / ?? / ?. / NRT 空安全）
9. **容器进阶**：`CollectionDemo`（List / Dictionary / HashSet / 队列栈 / 堆 / LRU / 环形缓冲）
10. **LINQ**：`IteratorDemo`（声明式查询管道）
11. **LINQ 进阶**：`IteratorAdvDemo`（GroupBy、Join、排序、集合运算、延迟执行、分页）
12. **日期时间**：`TimeDemo`（格式化、时区、耗时测量）
13. **异常处理**：`ExceptionDemo`（try-catch-finally、throw、using 释放）
14. **文件 IO**：`FileDemo`（读写、流、目录遍历）
15. **正则表达式**：`RegexDemo`（匹配、捕获组、替换）
16. **泛型**：`GenericDemo`（泛型方法/类、约束、协变逆变）
17. **泛型进阶**：`GenericAdvDemo`（约束全解、装箱性能、泛型接口、泛型数学）
18. **委托与事件**：`DelegateDemo`（Action / Func / 闭包 / event）
19. **性能优化**：`PerfDemo`（LINQ 写法效率、数据结构选型、字符串拼接、预分配容量）
20. **并发同步**：`SyncDemo`（Task、lock、Interlocked）
21. **JSON**：`JsonDemo`（System.Text.Json）
22. **HTTP**：`HttpDemo`（HttpListener + HttpClient）

> 跨语言对照学习建议：同一主题在 Go（`go/golog`）、Rust（`rust/rustlog`）、Java（`java/javalog`）、PHP（`php/phplog`）、Python（`python/pylog`）、Zig（`zig/ziglog`）、TypeScript（`web/tslog`）、C++（`cpp/cppapp`）九份代码中并列阅读，观察语言各自的惯用写法（如 C# List vs Go slice vs Rust Vec vs Java ArrayList vs Python list vs TS Array；C# Task+lock vs Go goroutine+mutex vs Java 线程池 vs PHP Fiber vs Python threading vs C++ thread 等）。

## 测试

```bash
dotnet test
```

> `cslog.Tests`（xUnit）会运行全部 22 个模块，捕获 stdout 并校验关键输出是否与注释中的「预期输出」一致，适合改代码后做回归验证。注意：测试会启动本地 HTTP 服务（默认端口 18080）。

## 常见问题（排障）

| 问题 | 解决办法 |
|------|----------|
| HTTP 端口被占用（`HttpListener` 启动报错） | 换端口重跑：`$env:CSLOG_HTTP_PORT=19090; dotnet run`（Windows PowerShell），或直接关掉占用 18080 的进程 |
| 时区报 `TimeZoneNotFoundException` | `TimeZoneInfo` 的 `"China Standard Time"` 是 Windows 专属 ID，Linux/macOS 改用 `"Asia/Shanghai"`（见 TimeDemo 注释） |
| 性能对比差异不明显 | Debug 模式差异较小，用 `dotnet run -c Release` 或 `dotnet test -c Release` 观察更明显 |
| 只想看单个模块 | `dotnet run -- Number`（模块名或文件名关键字均可，不区分大小写） |
