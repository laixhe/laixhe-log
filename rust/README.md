### 第一个例子

- 单文件

```
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

- cargo 包管理

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

```
let tup: (u8, f64, i32) = (1, 2.2, 3000);
```

- 数组 []
  - 相同类型，长度固定

```
let a: [ i32 ;  5 ] = [1, 2, 3, 4, 5];
```

- 向量 Vec<T>
  - Vector（向量）是一种动态数组类型
  - push(element: T)：向 Vector 的末尾添加一个元素
  - pop()：移除并返回 Vector 的末尾元素
  - get(index: usize) -> Option<&T>：根据索引获取 Vector 中的元素，返回一个 Option 类型的引用
  - len() -> usize：获取 Vector 的长度（元素个数）
  - is_empty() -> bool：判断 Vector 是否为空
  - contains(&item) -> bool：判断 Vector 是否包含指定元素
  - iter()：返回一个迭代器，用于遍历 Vector 中的元素


```
# 定义和创建
let numbers: Vec<i32> = Vec::new();
let numbers = vec![1, 2, 3, 4, 5];
```

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

