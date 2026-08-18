# Rust 模块系统学习指南（新手向）

> 本指南配套项目 [`example-mod-use`](./example-mod-use)，代码和说明一一对应。
> 建议边看边运行：`cargo run -p example-mod-use`，然后回到源码里逐个对照「哪个输出是谁打的」。

---

## 目录

1. [为什么需要模块系统](#1-为什么需要模块系统)
2. [前置概念：crate、包、模块](#2-前置概念crate包模块)
3. [用 `mod` 声明模块 + 文件组织](#3-用-mod-声明模块--文件组织)
4. [路径与 `use` 导入](#4-路径与-use-导入)
5. [可见性控制（核心）](#5-可见性控制核心)
6. [`pub use` 重导出](#6-pub-use-重导出)
7. [`use` 进阶写法](#7-use-进阶写法)
8. [结构体字段级可见性与封装](#8-结构体字段级可见性与封装)
9. [完整实例走读](#9-完整实例走读)
10. [新手常见坑](#10-新手常见坑)
11. [练习建议](#11-练习建议)

---

## 1. 为什么需要模块系统

当代码只有几十行时，全写在一个文件里没问题。但项目一旦变大，就会遇到：

- 函数、结构体越来越多，**名字容易冲突**；
- 一个文件上千行，**难以阅读和维护**；
- 有些函数只给内部用，**不想让外部随便调**。

Rust 的模块系统就是用来解决这三件事的：

| 需求 | 对应机制 |
|------|----------|
| 组织代码、分层 | `mod` 声明模块 |
| 引用别处代码 | `use` + 路径 |
| 控制谁能访问 | 可见性（`pub` / `pub(crate)` 等） |
| 对外提供简洁 API | `pub use` 重导出 |

---

## 2. 前置概念：crate、包、模块

先分清三个词，新手最容易混淆：

- **包（package）**：`cargo new` 创建出来的整个项目，对应 `Cargo.toml`。
- **crate**：编译单元。一个包里通常有一个**二进制 crate**（`src/main.rs`）或一个**库 crate**（`src/lib.rs`）。
  - `main.rs` 是**二进制 crate 的根**；
  - `lib.rs` 是**库 crate 的根**。
- **模块（module）**：crate 内部的代码分组单位，用 `mod` 关键字声明。

> 记忆：**包 > crate > 模块**。
> 路径里的 `crate::` 就是从「当前 crate 根」开始找的意思。

本项目的 `example-mod-use` 是一个二进制 crate，根是 [`src/main.rs`](./example-mod-use/src/main.rs)。

---

## 3. 用 `mod` 声明模块 + 文件组织

### 3.1 声明模块

在 crate 根里用 `mod` 声明子模块：

```rust
// src/main.rs
mod utils;    // 声明一个叫 utils 的模块
mod models;   // 声明一个叫 models 的模块
mod mod_use;  // 声明一个叫 mod_use 的模块
```

`mod xxx;` 会告诉编译器「去磁盘上找 `xxx` 对应的文件」。**没写 `mod`，模块就不存在**。

### 3.2 模块对应的文件长什么样（2018 edition 及以后）

一个模块 `foo` 有两种组织方式：

| 方式 | 文件 | 说明 |
|------|------|------|
| 单文件 | `src/foo.rs` | 模块内容直接写在这里 |
| 目录 | `src/foo/mod.rs` | `foo/` 目录 + `mod.rs` 入口 |

本项目两种都用到了：

```
example-mod-use/src/
├── main.rs          # crate 根
├── mod_use.rs       # 模块 mod_use（单文件形式）
├── utils/
│   ├── mod.rs       # 模块 utils（目录形式入口）
│   └── helpers.rs   # utils 的子模块 helpers
└── models/
    ├── mod.rs       # 模块 models（目录形式入口）
    └── user.rs      # models 的子模块 user
```

### 3.3 声明子模块

在 [`utils/mod.rs`](./example-mod-use/src/utils/mod.rs) 里再声明子模块：

```rust
// src/utils/mod.rs
pub mod helpers;   // 声明 helpers 子模块
```

> 注意这里的 `pub`：**模块本身也要 `pub`，外部才看得到这个路径**。
> 如果写成 `mod helpers;`（不加 pub），那 `utils::helpers` 这个路径在外面就不可见。

同理，[`models/mod.rs`](./example-mod-use/src/models/mod.rs) 声明了：

```rust
// src/models/mod.rs
pub mod user;   // 声明 user 子模块
```

---

## 4. 路径与 `use` 导入

要调用别处的函数，有两种写法：

1. **直接用完整路径**调用（不 import）；
2. **先 `use` 导入**，再用短名字调用。

### 4.1 路径的三种形式

Rust 路径分「绝对」和「相对」两大类，常用三种写法：

| 写法 | 含义 | 示例 |
|------|------|------|
| `crate::` | 绝对路径，从 crate 根出发 | `crate::models::User` |
| `self::` | 相对路径，从当前模块出发 | `self::user::User` |
| `super::` | 相对路径，从父模块出发（向上一级） | `super::utils` |

在 [`models/mod.rs`](./example-mod-use/src/models/mod.rs) 里有这三者的等价对照：

```rust
// 三种等价写法（实际选一种即可）：
use crate::models::user::User;   // A. 绝对路径，推荐新手用
use self::user::User;            // B. 相对路径（self = 当前 models 模块）
use super::models::user::User;   // C. super 路径（super = crate 根，再往下找）
```

### 4.2 `use` 导入

```rust
use crate::models::User;   // 之后就能直接写 User，不用写全路径
```

导入后，函数里就能直接用 `User::new(...)`。

### 4.3 路径 vs `use` 的取舍

- **路径层级浅、用得少**：直接用 `crate::xxx::yyy()` 就行；
- **用得频繁、路径很长**：`use` 导入一次，后面写短名。

---

## 5. 可见性控制（核心）

**默认所有项都是私有的**：函数、结构体、字段、模块，不加 `pub` 外面就访问不了。

### 5.1 修饰符速查表

| 修饰符 | 访问范围 |
|--------|----------|
| 默认（无修饰符） | 私有：仅当前模块及其子模块可访问 |
| `pub` | 完全公开：任何模块都能访问 |
| `pub(crate)` | 仅当前 crate 内可访问（外部 crate 不行） |
| `pub(super)` | 仅父模块可访问 |
| `pub(in path::to::mod)` | 仅指定路径的模块内可访问 |

### 5.2 项目里的完整演示

[`utils/helpers.rs`](./example-mod-use/src/utils/helpers.rs) 一次性演示了五种可见性：

```rust
// 1) pub：全局公开
pub fn greet(name: &str) {
    println!("你好，{}", name);
}

// 2) pub(crate)：只对整个 crate 内部公开
pub(crate) fn internal_log(msg: &str) {
    println!("[内部日志 crate only] {}", msg);
}

// 3) pub(super)：只对父模块（utils）公开
pub(super) fn only_parent_module() {
    println!("只有 utils 模块自己和它的父级能看到我");
}

// 4) pub(in crate::utils)：指定路径范围内可见
pub(in crate::utils) fn only_utils_tree() {
    println!("只有 crate::utils 下的模块能访问到我");
}

// 5) 无 pub：模块私有（默认）
fn really_private() {
    println!("只有 helpers.rs 内部能调用我");
}
```

对应的模块层级图（见 [`utils/mod.rs`](./example-mod-use/src/utils/mod.rs)）：

```
crate
└── utils
    └── helpers
        ├── greet                pub
        ├── internal_log         pub(crate)
        ├── only_parent_module   pub(super)
        ├── only_utils_tree      pub(in crate::utils)
        └── really_private       私有
```

### 5.3 一个关键点

[`main.rs`](./example-mod-use/src/main.rs) 里有一段注释专门说明：`pub(super)` 和 `pub(in crate::utils)` 的可见范围**不包含 crate 根**，所以 `main.rs` 里无法调用它们。这正是可见性约束在起作用。

---

## 6. `pub use` 重导出

`use` 是「在本模块里导入一个名字」；`pub use` 是「导入**并把它重新导出**，让外面也能用」。

### 6.1 为什么要重导出

看 [`models/mod.rs`](./example-mod-use/src/models/mod.rs)：

```rust
pub mod user;

// 不重导出时，外部要写很长的路径：
//   use crate::models::user::User;

// pub use 把 User 提到 models 这一层：
pub use user::User;
// 外部就能写：
//   use crate::models::User;
```

### 6.2 批量重导出

[`utils/mod.rs`](./example-mod-use/src/utils/mod.rs) 用 `pub use helpers::*;` 把 helpers 里所有 pub 项一次性提到 utils 层：

```rust
pub mod helpers;

// 把 helpers 里所有 pub 项重导出到 utils 这一层
pub use helpers::*;
```

于是调用方直接写 `utils::greet(...)` 即可，不用写 `utils::helpers::greet(...)`。

> **好处**：内部可以随意拆分文件（比如 `utils/` 下拆十几个文件），对外 API 仍然扁平简洁。

---

## 7. `use` 进阶写法

### 7.1 `use ... as` 重命名

解决同名冲突，或给长名字起个短名。见 [`mod_use.rs`](./example-mod-use/src/mod_use.rs)：

```rust
use super::utils as tools;   // utils 改叫 tools

// 之后用 tools::greet(...)
```

[`helpers.rs`](./example-mod-use/src/utils/helpers.rs) 里也有：

```rust
use std::collections::HashMap as Map;
let mut m: Map<&str, i32> = Map::new();
```

### 7.2 嵌套 `use` 分组

同一路径前缀的多个项，可以写在一个大括号里：

```rust
use std::collections::{
    HashMap, HashSet,    // 两个集合
    BTreeMap as BMap,    // 顺便重命名
};
// 等价于单独写三条 use
```

### 7.3 `use` 也可以写在函数内部

作用域只限该函数块，见 [`demo_use_rename`](./example-mod-use/src/utils/helpers.rs)：

```rust
pub fn demo_use_rename() {
    use std::collections::HashMap as Map;
    // 只在函数内部有效
}
```

---

## 8. 结构体字段级可见性与封装

可见性不仅能修饰模块和函数，也能修饰**结构体的字段**和**方法**。这是 Rust 封装（encapsulation）的体现。见 [`models/user.rs`](./example-mod-use/src/models/user.rs)：

```rust
#[derive(Debug)]
pub struct User {
    pub name: String,           // 公开：外部可直接读 & 改
    age: u32,                   // 私有：外部无法直接读写
    pub(crate) internal_tag: String, // crate 内可读写，外部 crate 不行
}
```

私有字段 `age` 只能通过**方法**间接访问：

```rust
impl User {
    // Getter：只读入口
    pub fn age(&self) -> u32 { self.age }

    // Setter：带校验的写入入口
    pub fn set_age(&mut self, new_age: u32) -> Result<(), &'static str> {
        if new_age == 0 || new_age > 150 {
            return Err("年龄必须在 1~150 之间");
        }
        self.age = new_age;
        Ok(())
    }
}
```

[`main.rs`](./example-mod-use/src/main.rs) 里演示了私有字段的访问限制：

```rust
user.name = String::from("NEW_NAME");  // ✅ name 是 pub，可以直接改
// user.age = 99;                       // ❌ 编译错误！age 是私有字段
println!("通过 getter age() 读 age = {}", user.age());   // ✅ 用 getter 读
```

---

## 9. 完整实例走读

建议按这个顺序对照源码看一遍：

1. **[main.rs](./example-mod-use/src/main.rs)**：crate 根，`mod` 声明三个模块，然后调用。
2. **[mod_use.rs](./example-mod-use/src/mod_use.rs)**：演示「跨模块间接引用 + 多种 use 形式」。
3. **[utils/mod.rs](./example-mod-use/src/utils/mod.rs)**：声明子模块 + `pub use helpers::*` 重导出。
4. **[utils/helpers.rs](./example-mod-use/src/utils/helpers.rs)**：函数级可见性 + use 路径/重命名/嵌套。
5. **[models/mod.rs](./example-mod-use/src/models/mod.rs)**：`pub use user::User` 重导出 + 路径验证。
6. **[models/user.rs](./example-mod-use/src/models/user.rs)**：结构体字段级可见性 + getter/setter。

跑一遍看输出，再回到源码里找「每个 print 对应哪行代码」：

```bash
cargo run -p example-mod-use
```

---

## 10. 新手常见坑

| 现象 | 原因 | 解决 |
|------|------|------|
| `unresolved module` / 找不到模块 | 忘了写 `mod xxx;` | 在父模块里声明 `mod xxx;` |
| `module xxx is private` | 子模块没加 `pub` | 写成 `pub mod xxx;` |
| `function is private` / `private function import` | 函数没加 `pub`，或可见性不够 | 检查修饰符（`pub` / `pub(crate)` 等） |
| 路径写错层级 | 绝对/相对路径混用 | 新手优先用 `crate::` 绝对路径 |
| 私有字段直接赋值报错 | 字段是私有的 | 用 getter/setter 间接访问 |
| 路径太长打字累 | 没做重导出 | 用 `pub use` 把常用项提到上层 |

---

## 11. 练习建议

1. 在 `utils/` 下新建一个模块 `math.rs`，写一个 `pub fn add(a: i32, b: i32) -> i32`，在 `main.rs` 里调用它。
2. 给 `math.rs` 里的函数分别改成 `pub(crate)`、`pub(super)`、默认私有，观察哪些调用会报错。
3. 在 `models` 里新建一个结构体，尝试「私有字段 + getter/setter」的封装模式。
4. 用 `pub use` 把 `math::add` 重导出到 `utils` 层，让外部能 `utils::add(...)` 调用。
5. 故意写一个 `use self::` 和 `use super::` 的路径，对照理解它们指向哪里。

---

## 相关链接

- 项目总览：[README.md](./README.md)
- 模块演示项目：[example-mod-use](./example-mod-use)
- 官方文档：[Rust Reference — Items & Modules](https://doc.rust-lang.org/reference/items/modules.html)
