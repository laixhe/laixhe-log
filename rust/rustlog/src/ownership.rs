//! 所有权与借用（Rust 最核心的基础）：移动语义 / Copy / clone / 借用 & 与 &mut。
//!
//! ## 前置知识
//! - 每个值只有一个所有者（owner），所有者离开作用域时值被释放（对应 C++ RAII）
//! - 赋值/传参会**移动**所有权（move），移动后原变量不可再用（区别于 C++ 拷贝 / Go 引用）
//! - `Copy` 类型的赋值是复制而非移动（如 i32、bool、char 等标量）
//! - 借用 `&` 只读、`&mut` 可写，借用不转移所有权，且同一时刻只能有一个 `&mut` 或任意多个 `&`
//!
//! ## 练习题
//! 1. 判断下面哪种写法合法：`let s1 = String::from("a"); let s2 = s1;` 后还能用 `s1` 吗？
//! 2. 写一个函数接收 `&mut Vec<i32>` 并在尾部追加元素。

// ============ 移动语义（move）============
pub fn move_semantics() {
    // String 不是 Copy，赋值会移动所有权
    let s1 = String::from("hello");
    let s2 = s1; // s1 的所有权移动到 s2
    // println!("{}", s1); // ❌ 编译错误：s1 已被移动（borrow of moved value）
    println!("移动后: {}", s2); // 合法

    // 标量类型实现 Copy：赋值是复制，原变量仍可用
    let a = 42;
    let b = a; // Copy：复制
    println!("Copy 后两者都可用: {} {}", a, b);

    // 传参也会移动所有权
    fn take_ownership(s: String) {
        println!("接收所有权: {}", s);
    }
    let s3 = String::from("world");
    take_ownership(s3); // s3 被移动进函数
    // println!("{}", s3); // ❌ 编译错误：s3 已被移动

    // 需要继续用时用 clone（深拷贝，对应 Python deepcopy）
    let s4 = String::from("clone me");
    let s5 = s4.clone();
    println!("clone 后两者都可用: {} | {}", s4, s5);
}

// ============ 借用（& 与 &mut）============
pub fn borrow() {
    let mut s = String::from("hello");

    // &：只读借用，不转移所有权（对应 C++ const 引用 / Go 读）
    let len = calculate_len(&s);
    println!("借用计算长度: {}（s 仍可用: {}）", len, s);

    // &mut：可变借用（对应 C++ 非 const 引用）
    append_world(&mut s);
    println!("可变借用后: {}", s);

    // 借用规则演示（编译期强制，这里只展示合法用法）：
    // 1. 任意多个不可变借用可以共存（读不冲突）
    let r1 = &s;
    let r2 = &s;
    println!("多个只读借用: {} {}", r1, r2);
    // 2. 不可变借用存在时不能有可变借用（编译报错，已注释）
    // let m = &mut s; // ❌ 与 r1/r2 冲突

    // 值在借用结束后才可再次移动
    let owner = String::from("temp");
    let borrowed = &owner;
    println!("借用中: {}", borrowed);
    println!("借用结束后所有者仍可用: {}", owner);
}

fn calculate_len(s: &String) -> usize {
    s.len() // 只读借用，可读取不可修改
}

fn append_world(s: &mut String) {
    s.push_str(", world"); // 可变借用，可修改
}

// ============ 引用 vs 值的传参对比 ============
pub fn borrow_compare() {
    // 值传递：移动（所有权转移）
    fn consume(v: Vec<i32>) {
        println!("消费 Vec: {:?}", v);
    }
    // 引用传递：借用（所有权保留）
    fn inspect(v: &Vec<i32>) {
        println!("只读检查: {:?}", v);
    }
    // 可变引用：借用但可修改
    fn double(v: &mut Vec<i32>) {
        for x in v.iter_mut() {
            *x *= 2;
        }
    }

    let mut nums = vec![1, 2, 3];
    inspect(&nums);  // 借用：nums 仍可用
    double(&mut nums); // 可变借用：nums 被修改
    println!("可变借用后: {:?}", nums); // [2, 4, 6]

    consume(nums); // 移动：之后 nums 不可用
    // println!("{:?}", nums); // ❌ 编译错误
}

// ============ 练习题参考答案 ============
#[cfg(test)]
mod tests {
    // 练习 1：String 移动后原变量不可用
    #[test]
    fn exercise_1_move_invalidates() {
        let s1 = String::from("a");
        let s2 = s1;
        // let _ = s1; // 会编译失败：s1 已被移动
        assert_eq!(s2, "a");
    }

    // 练习 2：&mut Vec 追加元素
    #[test]
    fn exercise_2_mut_borrow() {
        fn append(v: &mut Vec<i32>, x: i32) {
            v.push(x);
        }
        let mut v = vec![1, 2];
        append(&mut v, 3);
        assert_eq!(v, vec![1, 2, 3]);
    }
}
