//! 函数与闭包：函数定义 / 返回多值 / 闭包 / 高阶函数。
//!
//! ## 前置知识
//! - 函数参数与返回值都要标注类型
//! - 最后一个表达式即返回值（无 `return` 更 Rust 风格）
//! - 闭包可捕获环境变量（对应 Go 闭包 / JS 箭头函数）
//!
//! ## 练习题
//! 1. 写一个 `max` 函数，返回两个数中较大的一个。
//! 2. 用闭包捕获外部变量，实现一个累加器（返回闭包）。

// ============ 函数基础 ============
pub fn basics() {
    // 参数带类型，返回值用 -> 标注
    fn add(a: i32, b: i32) -> i32 {
        a + b // 最后一个表达式就是返回值（不能加分号）
    }
    println!("add(1, 2) = {}", add(1, 2));

    // 单元类型返回：没有返回值
    fn greet(name: &str) {
        println!("你好，{}", name);
    }
    greet("laixhe");

    // 显式 return 提前返回
    fn early(x: i32) -> i32 {
        if x > 0 {
            return x * 2;
        }
        x
    }
    println!("early(5) = {}", early(5));
}

// ============ 多返回值（元组）============
pub fn multi_return() {
    // Rust 用元组返回多个值（对应 Go 多返回值 / Python tuple）
    fn div_mod(a: i32, b: i32) -> (i32, i32) {
        (a / b, a % b)
    }

    let (q, r) = div_mod(10, 3);
    println!("10/3 商 {} 余 {}", q, r); // 3 1
}

// ============ 闭包 ============
pub fn closures() {
    // 闭包定义：|参数| 表达式（对应 Go func literal / JS 箭头函数）
    let add_one = |x: i32| x + 1;
    println!("闭包 add_one(5) = {}", add_one(5)); // 6

    // 闭包捕获外部变量（对应 Go 闭包）
    let factor = 3;
    let multiply = |x: i32| x * factor; // 捕获 factor
    println!("闭包捕获: 4 * 3 = {}", multiply(4)); // 12

    // 闭包作为函数参数（高阶函数，对应 Go 传入 func / C# Func<>）
    fn apply_twice(f: impl Fn(i32) -> i32, x: i32) -> i32 {
        f(f(x))
    }
    let double = |x: i32| x * 2;
    println!("高阶函数: apply_twice(double, 3) = {}", apply_twice(double, 3)); // 12

    // 闭包返回闭包：累加器（对应 Go 闭包计数器）
    fn counter() -> impl FnMut() -> i32 {
        let mut count = 0;
        move || {
            count += 1;
            count
        }
    }
    let mut c = counter();
    println!("累加器: {} {} {}", c(), c(), c()); // 1 2 3
}

// ============ 练习题参考答案 ============
#[cfg(test)]
mod tests {
    // 练习 1：max 函数
    #[test]
    fn exercise_1_max() {
        fn max(a: i32, b: i32) -> i32 {
            if a > b {
                a
            } else {
                b
            }
        }
        assert_eq!(max(3, 7), 7);
        assert_eq!(max(-1, -5), -1);
    }

    // 练习 2：闭包累加器
    #[test]
    fn exercise_2_accumulator() {
        fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
            move |x| x + n
        }
        let add10 = make_adder(10);
        assert_eq!(add10(5), 15);
    }
}
