//! 模式匹配进阶：解构结构体/元组 / @ 绑定 / 匹配守卫 / matches! 宏。
//!
//! ## 前置知识
//! - `match` 与 `if let` 是 Rust 的模式匹配（基础见 control_flow.rs）
//! - 模式可以**解构**（拆开）结构体、元组、枚举
//! - `@` 绑定：匹配的同时把值绑定到变量
//! - 匹配守卫 `if`：给分支加额外条件
//!
//! ## 练习题
//! 1. 解构一个三元组 `(i32, String, bool)` 并分别使用。
//! 2. 用匹配守卫判断一个数是否 2..=4 且为偶数。

// ============ 解构结构体与元组 ============
pub fn destructure() {
    // 解构结构体（对应 TS 对象解构 / Python 解包）
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 10, y: 20 };

    let Point { x, y } = p; // 解构
    println!("解构 Point: x={} y={}", x, y);

    // 解构时改名 / 忽略字段
    let Point { x: px, y: _ } = p; // y 用 _ 忽略
    println!("改名绑定: px={}", px);

    // 解构元组（对应 Go 多返回值解构 / Python 解包）
    let tup = (1, String::from("hello"), true);
    let (n, s, b) = tup;
    println!("解构元组: {} {} {}", n, s, b);
}

// ============ @ 绑定 ============
pub fn at_binding() {
    // @：匹配范围时把具体值绑定到变量
    match 7 {
        n @ 1..=5 => println!("小数字 {}", n),
        n @ 6..=10 => println!("中数字 {}", n),
        n => println!("大数字 {}", n),
    }

    // Option 场景：匹配 Some 并把内部值绑定
    let maybe: Option<i32> = Some(42);
    match maybe {
        Some(v @ 0..=100) => println!("在范围内: {}", v),
        Some(v) => println!("超出范围: {}", v),
        None => println!("无值"),
    }
}

// ============ 匹配守卫（match guards）============
pub fn match_guard() {
    let num = 4;

    // 守卫：先匹配模式，再用 if 附加条件
    match num {
        n if n > 5 => println!("{} 大于 5", n),
        n if n % 2 == 0 => println!("{} 是偶数（且 ≤5）", n),
        n => println!("{} 是奇数且 ≤5", n),
    }
}

// ============ matches! 宏 ============
pub fn matches_macro() {
    // matches!：只关心"是否匹配"，返回 bool（对应 Go switch 布尔判断）
    let val: Option<i32> = Some(42);
    println!("是 Some: {}", matches!(val, Some(_))); // true
    println!("是 None: {}", matches!(val, None)); // false

    // 解构匹配：判断是否 1..=100 的 Some
    println!("是 Some 且范围内: {}", matches!(val, Some(v) if (1..=100).contains(&v))); // true

    // 场景：遍历过滤（对应 Python 列表推导的条件）
    let items = vec![Some(1), None, Some(3)];
    let some_count = items.iter().filter(|x| matches!(x, Some(_))).count();
    println!("Some 数量: {}", some_count); // 2
}

// ============ if let 解构 ============
pub fn if_let_destructure() {
    // if let 也能解构（对应 Python if x, y = ...）
    let pair = (10, 20);
    if let (a, b) = pair {
        println!("if let 解构: {} {}", a, b);
    }

    // 结构体 + if let
    struct User {
        name: String,
        age: u8,
    }
    let user = User {
        name: String::from("laixhe"),
        age: 18,
    };
    if let User { name, age } = user {
        println!("解构 User: {} {} 岁", name, age);
    }
}

// ============ 练习题参考答案 ============
#[cfg(test)]
mod tests {
    // 练习 1：解构三元组
    #[test]
    fn exercise_1_destructure_tuple() {
        let tup = (3, String::from("hello"), true);
        let (n, s, b) = tup;
        assert_eq!(n, 3);
        assert_eq!(s, "hello");
        assert!(b);
    }

    // 练习 2：匹配守卫
    #[test]
    fn exercise_2_match_guard() {
        let classify = |x: i32| match x {
            n if n >= 2 && n <= 4 && n % 2 == 0 => "2..=4 且偶数",
            _ => "其他",
        };
        assert_eq!(classify(2), "2..=4 且偶数");
        assert_eq!(classify(3), "其他");
    }
}
