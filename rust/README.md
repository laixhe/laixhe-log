# Rust 学习笔记

个人 Rust 学习笔记整理：基础语法速查 + 专题深入 + 两个实战项目。

## 目录

### 语法速查与专题笔记

| 文件 | 主题 |
|------|------|
| `rust-基本安装.md` | 环境安装、国内镜像、工具链、跨平台编译、编译加速 |
| `rust-cargo基本.md` | rustup / cargo 命令、workspace / package / profile 配置 |
| `rust-match 匹配.md` | match 匹配控制流 |
| `rust-用 if let 和 while let 简化控制流.md` | if let / while let 简化匹配 |
| `rust-option 枚举.md` | Option 枚举 |
| `rust-显式丢弃 result 或 option.md` | 显式丢弃 Result / Option |
| `rust-善用 const 和 static.md` | const / static 的选用 |
| `rust-const generics 将值参数化.md` | const generics |
| `rust-被低估的 vecdeque 双端队列.md` | VecDeque 双端队列 |
| `rust-隐藏实现细节 impl trait 返回值.md` | impl Trait 返回值 |
| `rust-phantomdata 类型系统的幽灵.md` | PhantomData |
| `rust-按平台编译的配置方法与实例.md` | 按平台编译配置 |
| `rust-杂项.md` | 杂项 |

### 项目

| 项目 | 说明 |
|------|------|
| [rustlog](./rustlog/README.md) | Rust 基础语法 + 生态库示例（serde / tokio / rayon 等） |
| [bevylog](./bevylog/README.md) | Bevy 0.19 游戏引擎入门示例 |

---

### 命名规则

> 使用蛇形命名 `user_name`
>
> 常量使用全大写加下划线 `MAX_SIZE`
> 
> 类型名使用驼峰命名 `MyStruct`
> 

### 第一个例子

> 单文件

```rust
// main.rs
fn main() {
  println!("Hello, world!");
}
```

```
$ rustc main.rs
$ ./main
Hello, world!
```

> cargo 包管理

```
# 新建项目
cargo new hello

cd hello
vim src/main.rs
fn main() {
    println!("Hello, world!");
}

cargo run              # 运行代码
cargo build            # 编译：debug
cargo build --release  # 编译：release 
```

### 变量

- 常量：`const` 始终不可变
- 变量：
  - `let`：默认不可变
  - `let mut`：只可变，类型不可变

### 复合类型

- 元组 ()
  - 多种类型 多个值 放一起，长度固定
  - 解构：let (x, y, z) = tup;
  - 索引：let x = tup.0;

```rust
let tup: (u8, f64, i32) = (1, 2.2, 3000);
```

- 数组 []
  - 相同类型，长度固定

```rust
let a: [ i32 ;  5 ] = [1, 2, 3, 4, 5];
```


- 动态数组 Vec<T>
  - 在堆上分配一块连续内存，并允许在运行时增长或收缩
  - 尾部操作高效：push 和 pop 的均摊时间复杂度为 O(1)
  - 中间插入/删除：时间复杂度 O(n)，因为需要移动后续元素

- 双端队列 VecDeque<T>
  - 支持在头部和尾部以 O(1) 时间复杂度高效地插入或删除元素
  - 非常适合实现队列（先进先出）或双端队列（两端操作）场景

- 集合 HashMap<K, V>
  - 基于哈希表实现，提供平均 O(1) 的插入、查找和删除操作

- 有序集合 BTreeMap<K, V>
  - 基于 B 树的有序键值映射
  - 根据键的顺序（通过 Ord trait）存储元素，支持高效的范围查询、有序遍历和日志时间复杂度的插入、删除、查找

- 无序集合 HashSet<T>
  - 基于哈希表的无序集合
  - 存储不重复的值，并支持平均 O(1) 的插入、删除和查找操作

- 有序集合 BTreeSet<T>
  - 基于 B 树的有序集合
  - 存储不重复的值，并且始终按照元素的顺序（通过 Ord trait）进行排序)

- 优先级队列 BinaryHeap<T>
  - 基于二叉堆的优先级队列
  - 是一个最大堆（max-heap），意味着堆顶始终是集合中的最大值

- 双向链表 LinkedList<T>
  - 是一个线性集合，每个元素都包含指向前一个和后一个元素的指针

> **重要提示：** 在绝大多数 Rust 使用场景中，`LinkedList` 性能不佳，通常应优先使用 `Vec<T>` 或 `VecDeque<T>`。只有在需要频繁的中间插入/删除且同时需要在链表两端进行高效操作时，才考虑使用链表。即便如此，现代 CPU 缓存特性使得连续内存的 `VecDeque` 往往更快。


### 控制流
- if ...
- if ... else ...
- if ... else if ...
- if ... else if ... else ...

### 循环
- loop：无限循环（可以 break 出来）
- while：while 条件 { ... }
- for：for element in array { ... }

### 匹配控制流
- match

