# cppapp

C++ 标准库学习示例项目（C++23 + CMake），按类拆分演示现代 C++ 特性，纯标准库、无需第三方依赖。

## 环境要求

| 项目 | 要求 |
|------|------|
| **编译器** | 支持 C++23（GCC / Clang / MSVC） |
| **CMake** | 4.1+ |
| 查看版本 | `g++ --version` / `cmake --version` |

> 本机演示环境：MinGW-w64 g++ 16.2.0（winlibs UCRT + POSIX 线程，官网最新版）+ CMake 4.4.2。

## 项目结构

| 文件 | 主题 | 说明 |
|------|------|------|
| `StdBasic.h/.cpp` | 基础类型 | 整型/浮点/bool/char、auto 推断、const、类型转换、引用 |
| `StdControl.h/.cpp` | 控制流 | if / for / while / do-while / switch（注意 C++ 需要 break） |
| `StdFunction.h/.cpp` | 函数 | 函数重载 / 默认参数 / lambda（捕获）/ 引用参数 |
| `StdClass.h/.cpp` | 类与多态 | 封装 / 构造函数 / 继承 / 虚函数多态（`unique_ptr`） |
| `StdTemplate.h/.cpp` | 模板 | 函数模板 / 类模板 / 概念约束（`std::integral`，C++20） |
| `StdArray.h/.cpp` | 数组 | C 数组 / `std::array`（at 越界检查/值语义）/ 多维数组 |
| `StdEnum.h/.cpp` | 枚举 | `enum class` 强类型枚举 / 底层类型 / switch 配合 |
| `StdAlgorithm.h/.cpp` | STL 算法 | sort / find / find_if / count / transform / reverse / max_element |
| `StdRegex.h/.cpp` | 正则 | 语法速览（字符类/量词/词边界）/ match / search（prefix·suffix）/ 修饰标志 icase / 查找所有 / 捕获分组 / 替换 / 邮箱验证 |
| `StdMove.h/.cpp` | 移动语义 | 左值/右值 / `std::move` / 移动构造 vs 拷贝构造（零拷贝） |
| `StdStringHandle.h/.cpp` | 字符串进阶 | find/contains/starts_with / substr / replace / 大小写 / 分割 / 词频统计 |
| `StdSetOps.h/.cpp` | 集合运算 | 去重（unique/set）/ 交集 / 并集 / 差集 |
| `StdNet.h/.cpp` | 网络 | winsock2 TCP 服务端 + 客户端（本地 echo，Windows 需链接 ws2_32） |
| `StdOptional.h/.cpp` | 可选值与联合 | `std::optional`（对应 Rust Option）/ `std::variant` / `std::tuple` 结构化绑定 |
| `StdRandom.h/.cpp` | 随机数 | `std::random_device` / `mt19937` / 整数/浮点/正态分布 / shuffle / 随机选择 |
| `StdFile.h/.cpp` | 文件读写 | fstream 写入/逐行读取/追加 / filesystem 判断删除 |
| `StdException.h/.cpp` | 异常处理 | try/catch / throw / 自定义异常 / 标准库异常 |
| `Chrono.h/.cpp` | 日期时间 | 时钟（system/steady）/ 时间点与时长运算 / 耗时测量 / C++20 日历（year_month_day）/ 时间点互转 / 时区 zoned_time |
| `StdConst.h/.cpp` | 指针常量 | 常量指针 / 指针常量 / 指向常量的指针常量 |
| `StdPtr.h/.cpp` | 智能指针 | `unique_ptr` / `shared_ptr` / `weak_ptr` |
| `StdString.h/.cpp` | 字符串 | `std::string` 原始字面量 + `std::format` |
| `module/main.cpp` | C++20 模块 | `import std` 独立演示（需手动编译） |
| `StdNumber.h/.cpp` | 数值进阶 | 格式化 / 溢出与饱和运算 / 类型转换（`from_chars`） |
| `StdContainer.h/.cpp` | 容器进阶 | vector / map / set / deque / list / priority_queue / 环形缓冲 / LRU / 去重 / 分组 |
| `StdRange.h/.cpp` | 范围与视图 | `std::ranges` transform / filter / take / drop / zip / accumulate / 综合实战 |
| `StdThread.h/.cpp` | 并发 | `std::thread` / `std::mutex` / `std::atomic` / `call_once` |
| `StdJson.h/.cpp` | JSON | 手写递归下降解析 + 序列化（含美化输出 / omitempty） |
| `StdBits.h/.cpp` | 位运算 | 移位 / 与或异或取反 / 掩码设置清除检查 / `std::bitset` |
| `StdIO.h/.cpp` | 输入输出 | `std::cin` 输入 / iomanip 格式化（setw/setprecision/进制）/ `stringstream` 字符串流 |
| `StdCallback.h/.cpp` | 回调与函数对象 | `std::function` / 函数指针 / lambda 闭包捕获 / `std::bind` |
| `StdStruct.h/.cpp` | 结构体与联合 | struct 聚合初始化 / 位域 / union 内存共享 / alignas 对齐 / struct vs class |
| `StdCast.h/.cpp` | 类型转换 | static_cast / dynamic_cast（运行时类型检查）/ const_cast / reinterpret_cast / bit_cast |
| `StdIter.h/.cpp` | 迭代器基础 | begin/end / 迭代器分类 / 反向迭代器 / back_inserter / advance-distance / 遍历方式对比 |

## 运行

```bash
cmake -B build -G "MinGW Makefiles"
cmake --build build
./build/bin/cppapp
```

> `main.cpp` 是入口，按顺序运行各模块演示；想看单个主题，可注释掉 `main()` 里对应的实例化行。
> `CMakeLists.txt` 用 `aux_source_directory` 自动收集目录下所有 `.cpp`，新增源文件后需重新运行 `cmake -B build` 才会纳入构建。

## C++20 模块示例

`module/main.cpp` 是独立的 C++20 模块演示（`import std`），需手动编译：

```bash
g++ -std=c++23 -fmodules --compile-std-module main.cpp -o app.exe
```

## 学习路径建议（从易到难）

1. **基础语法**：`StdBasic`（类型/变量/auto/引用）→ `StdControl`（控制流）→ `StdFunction`（函数/lambda）→ `StdArray`（数组）→ `StdEnum`（枚举）→ `StdStruct`（结构体/联合/位域）
2. **类与模板**：`StdClass`（封装/继承/多态）→ `StdTemplate`（泛型）→ `StdCast`（四种类型转换）→ `StdMove`（移动语义，对应 Rust 所有权）
3. **算法与正则**：`StdIter`（迭代器基础，begin/end/插入迭代器）→ `StdAlgorithm`（STL 算法）→ `StdRegex`（正则）→ `StdSetOps`（集合运算）→ `StdStringHandle`（字符串进阶）
4. **类型工具**：`StdOptional`（optional/variant/tuple，对应 Rust Option）→ `StdRandom`（随机数）
5. **字符串与指针**：`StdString`（`std::format` 基础）、`StdConst`（指针常量）、`StdPtr`（智能指针）
6. **文件与异常**：`StdFile`（fstream）→ `StdException`（try/catch）
7. **网络**：`StdNet`（winsock TCP 客户端 + 服务端）
8. **数值进阶**：`StdNumber`（格式化、溢出检查、类型转换）→ `StdBits`（位运算，对应 Go/Rust 位运算）→ `StdIO`（输入输出与字符串流）→ `StdCallback`（`std::function` 回调，对应 Go 函数值 / Rust 闭包）
9. **容器进阶**：`StdContainer`（序列容器、关联容器、堆、LRU、环形缓冲）
10. **范围与视图**：`StdRange`（`std::ranges` 声明式管道）
11. **并发**：`StdThread`（线程、互斥锁、原子操作）
12. **JSON**：`StdJson`（手写解析器，理解序列化原理）
13. **日期时间**：`Chrono`（时间点与格式化）
14. **C++20 模块**：`module/main.cpp`（选学）

> 跨语言对照学习建议：同一主题在 Go（`go/golog`）、Rust（`rust/rustlog`）、Java（`java/javalog`）、PHP（`php/phplog`）、Python（`python/pylog`）、Zig（`zig/ziglog`）、TypeScript（`web/tslog`）、C#（`csharp/cslog`）九份代码中并列阅读，观察语言各自的惯用写法（如 C++ 模板 vs Go 泛型 vs Rust 泛型 vs C# 泛型；C++ 异常 vs Go error vs Rust Result vs C# 异常等）。
