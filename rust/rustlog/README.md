# RustLog

Rust 学习笔记项目，涵盖 Rust 基础语法和常用生态库的使用示例。适合 Rust 初学者参考学习。

---

## ⚠️ 重要前置说明

### 环境要求（请务必满足）

| 项目 | 要求 | 说明 |
|------|------|------|
| **Rust 版本** | **1.85+**（稳定版或 nightly） | 本项目使用 **Edition 2024**，低于此版本将无法编译 |
| 安装方式 | `rustup update stable` | 或使用 `rustup install nightly` |

> 查看当前 Rust 版本：`rustc --version`
>
> 如暂时无法升级，可将 [Cargo.toml](file:///e:/code/github.com/laixhe/laixhe-log/rust/rustlog/Cargo.toml) 中的 `edition = "2024"` 改为 `edition = "2021"`（大部分示例仍可运行）。

---

### 关于代码中 `.unwrap()` 的说明

> **为了示例简洁易懂，本项目大量使用了 `.unwrap()` 直接处理 `Result` / `Option`。**
>
> ❌ 这**不是**生产环境的推荐写法！
>
> ✅ 生产代码中应使用：
> - `match` / `if let` 显式处理错误分支
> - `?` 操作符向上传播错误
> - `unwrap_or(...)` / `unwrap_or_default(...)` / `unwrap_or_else(...)` 提供兜底值
>
> 后续示例会尽量用注释标出「这里正确的生产写法应该是……」。

---

## 项目结构

```
src/                  # 核心学习模块：基础语法、集合、字符串操作
  basic.rs            # 基础类型与变量：bool/char/元组、变量绑定、常量、类型别名、遮蔽
  control_flow.rs     # 控制流：if 表达式、for/while/loop 循环、match 模式匹配
  function.rs         # 函数与闭包：函数定义、多返回值（元组）、闭包、高阶函数
  struct_enum.rs      # 结构体与枚举：struct、impl 方法、enum 携带数据、Option
  ownership.rs        # ⭐ 所有权与借用：move 移动语义、Copy/clone、& 与 &mut 借用规则
  pattern.rs          # 模式匹配进阶：解构、@ 绑定、匹配守卫、matches! 宏
  generic_trait.rs    # 泛型与特征：泛型函数/结构体、trait 定义与实现、特征约束
  error.rs            # 错误处理：Result、? 运算符、自定义错误、panic
  file_io.rs          # 文件读写：std::fs 读写、逐行读取、目录操作
  concurrency.rs      # 并发基础：thread、Mutex、mpsc 通道
  number.rs           # 数值类型操作：类型转换、溢出安全处理、格式化
  char_string.rs      # 字符与字符串：char、String 的常用方法、String vs &str
  array_map.rs        # 集合类型：数组、元组、Vec、VecDeque、HashMap、BTreeMap 等
  iterators.rs        # ⭐ 迭代器专题：map/filter/fold/collect 等核心适配器
  main.rs             # 入口，串联所有模块

examples/             # 第三方库使用示例
example-mod-use/      # Rust 模块系统演示（mod / use / 可见性 / pub use 重导出）
example-simple-wasm/  # WebAssembly 编译示例（含配套 HTML Demo）
```

---

## 集合类型选型速查表

| 需求场景 | 推荐类型 | 时间复杂度（平均） | 说明 |
|---|---|---|---|
| 存一组同类型元素，主要在尾部增删 | **`Vec<T>`** | push/pop O(1) 均摊，中间插入 O(n) | 最常用的集合，默认首选 |
| 需要在**头部和尾部**都高效增删（队列/栈） | **`VecDeque<T>`** | 两端 O(1) | 内部环形缓冲区，适合 FIFO/LIFO |
| 键值对存储，**不需要有序** | **`HashMap<K, V>`** | O(1) 插入/查找/删除 | 基于哈希表，无序 |
| 键值对存储，**需要按 key 有序遍历 / 范围查询** | **`BTreeMap<K, V>`** | O(log n) | 基于 B 树，按 Ord 排序 |
| 存一组**不重复**的值，不需要有序 | **`HashSet<T>`** | O(1) | HashMap 的 value=() 版本 |
| 存一组**不重复**的值，需要有序遍历 | **`BTreeSet<T>`** | O(log n) | BTreeMap 的 value=() 版本 |
| 优先级队列（每次取最大/最小值） | **`BinaryHeap<T>`** | O(log n) 入堆/出堆 | 最大堆，配合 Reverse 可做最小堆 |
| 双向链表 | **`LinkedList<T>`** | 理论上中间 O(1) | ⚠️ **绝大多数场景应优先用 Vec / VecDeque**（CPU 缓存友好，实际更快） |

---

## 运行

### 主程序（基础语法示例）

```bash
cargo run
```

<details>
<summary>📄 预期输出（部分关键行）</summary>

```
============= number::number_to_string =============
i=666
f1=88.89
f2=88.00
666 hex=0x29A  octal=0o1232  binary=0b1010011010
666 with leading zeros: 00000666
left=|666       | center=|   666    | right=|       666|
positive= +666  negative= -888
============= number::overflow =============
checked_add:   255+1 = 溢出了（返回 None）
saturating_add: 255+1 = 255（饱和，卡在 u8::MAX）
wrapping_add:   255+1 = 0（回绕到 0）
overflowing_add: 255+1 = 0, 溢出？true
============= char_string::std_char =============
c1 = A c2 = 中 c3 = 😀 c4 = 1 c5 = !
'A' 是字母？true
...
============= array_map::std_binary_heap (新增) =============
最大堆依次 pop: 50 30 20 10
最小堆依次 pop: 10 20 30 50
============= iterators::practice (综合实战) =============
R&D 30+ 员工平均月薪: 52500 元/月
```
</details>

---

### 模块系统演示

```bash
cargo run -p example-mod-use
```

<details>
<summary>📄 预期输出</summary>

```
============ 通过 mod_use::log() 间接调用 ============
你好，laixhe
User: name=laixhe, age=18

---- helpers 路径 / use 重命名演示 ----
只有 helpers.rs 内部能调用我
[内部日志 crate only] 路径：crate::utils::internal_log  绝对路径
use ... as 重命名 HashMap 为 Map，演示插入: {"a": 1}
嵌套 use { ... } 写法 OK

--------------------------------------------

============ 直接调用 utils:: / models:: ============
你好，laixhe
User: name=laixhe, age=18

---- User 可见性演示 ----
直接读 pub 字段 name = NEW_NAME
通过 getter age() 读 age = 18
✅ set_age(200) 合法地被拦下：年龄必须在 1~150 之间
[crate-only] User::crate_only_method() 被调用了，name=NEW_NAME
```
</details>

---

### 第三方库示例

```bash
# JSON 序列化 / 反序列化（serde + serde_json）
cargo run --example example_json
#   → 序列化结果: {"name":"laixhe","age":18,"admin":"super_admin","active":true,"userRole":"editor"}
#   → 反序列化结果: User { name: "laixhe", age: 18, admin: Some("admin"), active: true, user_role: "owner" }

# RON 序列化 / 反序列化（ron + serde，Rust 风格配置格式）
cargo run --example example_ron
#   → RON 序列化 (紧凑): (name:"my_app",timeout:30,mode:Debug,...)
#   → RON 反序列化: Config { name: "my_app", timeout: 30, mode: Debug, ... }
#   → 如预期失败（缺字段）: 1:46: Unexpected missing field named `database`

# JWT 令牌签发与验证（jsonwebtoken）
cargo run --example example_jwt
#   → ✅ 签发成功 JWT: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...
#   → 🎉 Token 验证通过: 用户名=user_123, 角色=admin, 签发时间戳=...

# 日期时间处理（std::time + jiff）
cargo run --example example_date_time
#   → (休眠 1 秒后)
#   → 本地时区当前时间: 202x-xx-xxTxx:xx:xx+08:00[Asia/Shanghai]
#   → 格式化（中文友好）: 202x年xx月xx日 xx:xx:xx 星期x (CST +0800)
#   → 当前时间戳(秒): 1786612078
#   → 当前时间格式化: 2026-08-13 17:07:58
#   → 解析字符串 → 日期时间: 2026-08-13 12:13:14

# 日志与追踪（log + tracing + tracing-subscriber）
cargo run --example example_log_tracing
#   → [ERROR example_log_tracing] ❌ error!：严重错误...
#   →  INFO checkout: 开始结算 user_id=42
#   →  INFO process_order: 处理订单中... order_id=1024 amount=99.5

# 错误处理（thiserror + anyhow）
cargo run --example example_thiserror_anyhow
#   → ❌ thiserror 捕获: 解析错误: invalid digit found in string
#   →    → 具体类型: ParseIntError(invalid digit found in string)
#   → ❌ anyhow 捕获: 解析错误: invalid digit found in string
#   →    错误链: [0] 解析错误...  [1] invalid digit found in string

# 随机数（rand）
cargo run --example example_rand
#   → ✅ 随机整数 1..=100: 41  (每次结果不同)
#   → ✅ shuffle 打乱: [5, 2, 4, 3, 1]
#   → ✅ 随机密码 (长度 16): 5=fZj76OVTTAF0R^

# 正则表达式（regex）
cargo run --example example_regex
#   → ✅ find_iter: ["3", "15"]
#   → ✅ 完整匹配: 2024-12-25  → 年: 2024  月: 12  日: 25
#   → user@example.com     → ✅ 合法
#   → invalid-email        → ❌ 不合法
#   → 手机号 13812345678 → ✅ 合法
#   → 邮箱 LAIXHE@EXAMPLE.COM → ✅ 合法（大小写不敏感）

# 并行计算（rayon）
cargo run --example example_rayon
#   → ✅ 并行 map (平方): [1, 4, 9, 16, 25, 36, 49, 64]
#   → ✅ 性能对比 (1百万次 x^2 求和): 顺序 ~7ms  并行 ~1.7ms  加速比: ~4x

# 异步运行时（tokio）
cargo run --example example_tokio
#   → concurrent_tasks：3 个任务并发执行（总耗时 ~150ms 而非 300ms）
#   → channel_demo：mpsc 通道演示 → 收到: 消息 #0 #1 #2
#   → mutex_demo：5 个任务各自 +1，最终计数 = 5

# HTTP 客户端（reqwest，需要网络）
cargo run --example example_reqwest
#   → ✅ GET ...posts/1  → 状态码: 200 OK  → title: sunt aut facere...
#   → ✅ POST ...posts   → 状态码: 201 Created → 分配的 id: 101

# 无锁队列 SegQueue（crossbeam-queue，多生产者多消费者）
cargo run --example example_crossbeam_queue_segqueue
#   → [消费 #1] Log received: Worker 0: message #1
#   → [消费 #2] Log received: Worker 0: message #2  (顺序可能不同)
#   → ✅ 断言通过：所有消息都被消费

# 类型向下转型（downcast-rs，实现类似多态）
cargo run --example example_downcast
#   → ✅ downcast_ref 为 Foo → Foo(42)
#   → ✅ downcast_ref 为 Bar → Bar(3.14)

# 加密与哈希（ring SHA / bcrypt / BLAKE3）
cargo run --example example_encrypt
#   → SHA256('123456') -> 8D969EEF6ECAD3C29A3A629280E686CF0C3F5D5A86AFF3CA12020C923ADC6C92
#   → bcrypt(cost=10) hash = "$2b$10$..."
#   → ✅ 使用正确密码验证: verified = true
#   → BLAKE3('123456') -> 7adb787627ad5ee341fa0ba46a956e78fd85c39e195119bb260d5181b4f1e4ba

# 环形缓冲区（ringbuf，SPSC 单生产者单消费者）
cargo run --example example_ringbuf
#   → 容量 2：push 10, 20 → 已满（剩余空位=0）
#   → push 第 3 个(30) → 如预期失败，被拒值 = Some(30)
#   → ✅ 所有 ringbuf 断言通过

# 同步内存缓存（moka sync）
cargo run --example example_moka_sync
#   → 获取缓存 user:1 → 张三
#   → remove user:1 → 被删除的值 = Some("张三")
#   → 当前 entry_count = 0  /  构建时 max_capacity 设置为 1000

# 异步内存缓存（moka future，需要 tokio）
cargo run --example example_moka_future
#   → 异步 get user:100 → 赵六
#   → 原子填充 user:101 → 孙七

# ⚠️ TCP 示例：**必须先启动 server，再启动 client！**
# 终端 1（保持运行）：
cargo run --example example_std_net_tcp_server
#   → ✅ TCP Server 已启动，监听 127.0.0.1:5050 ...

# 终端 2（另开窗口）：
cargo run --example example_std_net_tcp_client
#   → 客户端 recv(35 字节): server write: hello from tcp server!
```

---

## 涉及的主要 crate

| crate | 用途 | 对应示例 |
|-------|------|----------|
| serde / serde_json | 序列化与反序列化 | `example_json` |
| ron | Rusty Object Notation（Rust 风格可读配置格式） | `example_ron` |
| jsonwebtoken | JWT 令牌签发与验证 | `example_jwt` |
| log + tracing | 日志与追踪（span / 结构化日志） | `example_log_tracing` |
| thiserror + anyhow | 错误处理（自定义错误类型 / 错误链） | `example_thiserror_anyhow` |
| rand | 随机数（整数/浮点/选择/打乱/字节） | `example_rand` |
| regex | 正则表达式（匹配/捕获组/替换/分割） | `example_regex` |
| rayon | 并行计算（并行迭代器/排序/reduce） | `example_rayon` |
| tokio | 异步运行时（spawn/channel/select/Mutex） | `example_tokio` |
| reqwest | HTTP 客户端（GET/POST/JSON） | `example_reqwest` |
| moka | 内存缓存（同步/异步 + TTL/TTI） | `example_moka_sync` / `example_moka_future` |
| ring + bcrypt + blake3 | SHA 哈希 / 密码哈希 / BLAKE3 快速哈希 | `example_encrypt` |
| crossbeam-queue | 无锁队列（SegQueue / ArrayQueue） | `example_crossbeam_queue_segqueue` |
| ringbuf | 环形缓冲区（SPSC 高性能） | `example_ringbuf` |
| jiff | 日期时间处理（时区、格式化、计算） | `example_date_time` |
| downcast-rs | trait 对象向下转型 | `example_downcast` |
| wasm-bindgen | Rust ⇄ WebAssembly/JS 互操作 | `example-simple-wasm` |

---

## 学习路径建议（从易到难）

1. **基础语法**：先运行 `cargo run` 看 [basic.rs](file:///e:/code/github.com/laixhe/laixhe-log/rust/rustlog/src/basic.rs) → [control_flow.rs](file:///e:/code/github.com/laixhe/laixhe-log/rust/rustlog/src/control_flow.rs) → [function.rs](file:///e:/code/github.com/laixhe/laixhe-log/rust/rustlog/src/function.rs)
2. **类型系统**：[struct_enum.rs](file:///e:/code/github.com/laixhe/laixhe-log/rust/rustlog/src/struct_enum.rs)（结构体/枚举/Option）→ [ownership.rs](file:///e:/code/github.com/laixhe/laixhe-log/rust/rustlog/src/ownership.rs)（⭐ 所有权/借用，Rust 核心）→ [pattern.rs](file:///e:/code/github.com/laixhe/laixhe-log/rust/rustlog/src/pattern.rs)（模式匹配进阶）→ [generic_trait.rs](file:///e:/code/github.com/laixhe/laixhe-log/rust/rustlog/src/generic_trait.rs)（泛型/trait）
3. **错误与 IO**：[error.rs](file:///e:/code/github.com/laixhe/laixhe-log/rust/rustlog/src/error.rs)（Result/?）→ [file_io.rs](file:///e:/code/github.com/laixhe/laixhe-log/rust/rustlog/src/file_io.rs)（文件读写）
4. **并发基础**：[concurrency.rs](file:///e:/code/github.com/laixhe/laixhe-log/rust/rustlog/src/concurrency.rs)（thread/Mutex/mpsc）
5. **标准库核心**：[number.rs](file:///e:/code/github.com/laixhe/laixhe-log/rust/rustlog/src/number.rs) → [char_string.rs](file:///e:/code/github.com/laixhe/laixhe-log/rust/rustlog/src/char_string.rs) → [array_map.rs](file:///e:/code/github.com/laixhe/laixhe-log/rust/rustlog/src/array_map.rs) → [iterators.rs](file:///e:/code/github.com/laixhe/laixhe-log/rust/rustlog/src/iterators.rs)
6. **模块系统**：`cargo run -p example-mod-use`，对照 [example-mod-use/](file:///e:/code/github.com/laixhe/laixhe-log/rust/rustlog/example-mod-use/) 目录理解 `mod` / `use` / 可见性
7. **序列化**：`example_json` → `example_ron`
8. **错误处理进阶**：`example_thiserror_anyhow` → `example_log_tracing`
9. **常用库**：`example_date_time` → `example_encrypt` → `example_jwt` → `example_rand` → `example_regex`
10. **并发/异步**：`example_tokio` → `example_rayon` → `example_crossbeam_queue_segqueue` → `example_ringbuf` → `example_moka_sync` → `example_moka_future`
11. **网络**：`example_std_net_tcp_server` + `client` → `example_reqwest`（HTTP 客户端）
12. **进阶**：`example_downcast`（trait 对象向下转型）→ `example-simple-wasm`（WASM 跨语言调用）

> 每个 src 模块都附带 `#[cfg(test)]` 练习题参考答案，可用 `cargo test` 验证。
