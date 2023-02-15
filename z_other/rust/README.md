### Hello World

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
```
let tup: (u8, f64, i32) = (1, 2.2, 3000);
```
  - 多种类型 多个值 放一起，长度固定
  - 解构：let (x, y, z) = tup;
  - 索引：let x = tup.0;

- 数组 []
```
let a: [ i32 ;  5 ] = [1, 2, 3, 4, 5];
```
  - 相同类型，长度固定

### 控制流
- if ...
- if ... else ...
- if ... else if ...
- if ... else if ... else ...

### 循环
- loop：无限循环（可以 break 出来）
- while：while 条件 { ... }
- for：for element in array { ... }
