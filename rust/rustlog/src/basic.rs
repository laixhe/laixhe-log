//! 基础类型与变量：bool / char / 元组 / 变量绑定 / 常量 / 类型别名 / 遮蔽。
//!
//! ## 前置知识
//! - 变量默认不可变（immutable），`mut` 声明可变（与其他语言默认可变相反）
//! - `char` 是 Unicode 标量值（4 字节），不是字节
//! - 元组 `(a, b)` 可装任意类型，用 `.0 .1` 或解构访问
//!
//! ## 练习题
//! 1. 用元组解构一次性交换两个变量的值。
//! 2. 给 `u64` 起别名 `Speed`，并声明一个 `Speed` 类型变量。

// ============ 基础类型 ============
pub fn basic_types() {
    // bool：只有 true / false（对应 Go bool / C# bool）
    let is_ok: bool = true;
    println!("bool: {}", is_ok); // true

    // char：Unicode 标量值，用单引号（对应 Go rune / Java char）
    let c: char = '中';
    println!("char: '{}'，占用 {} 字节", c, std::mem::size_of::<char>()); // 4 字节

    // 元组：可装不同类型，长度固定（对应 Python tuple / Go 多返回值）
    let tup: (i32, f64, char) = (500, 6.4, '中');
    println!("元组访问: {} {} {}", tup.0, tup.1, tup.2); // 500 6.4 中
    // 解构
    let (x, y, z) = tup;
    println!("元组解构: {} {} {}", x, y, z);

    // 单元类型 ()：函数无返回值的返回类型（对应 C void / Python None）
    let unit: () = ();
    println!("单元类型: {:?}", unit);
}

// ============ 变量绑定 ============
pub fn variables() {
    // 默认不可变：修改会编译报错（对应其他语言默认可变 + final/const 修饰）
    let x = 5;
    println!("不可变变量 x = {}", x);
    // x = 6; // ❌ 编译错误：cannot assign to immutable variable

    // mut 声明可变
    let mut y = 5;
    println!("可变变量 y = {}", y);
    y = 6;
    println!("修改后 y = {}", y);

    // 显式类型标注
    let n: i64 = 100;
    println!("类型标注 n = {}", n);

    // 一次声明多个
    let (a, b) = (1, 2);
    println!("多变量: {} {}", a, b);

    // 类型推断：Rust 会根据使用方式推断（对应 Go := / C# var）
    let inferred = 3.14f64;
    println!("类型推断: {}", inferred);
}

// ============ 常量与类型别名 ============
pub fn consts() {
    // const：编译期常量，必须标注类型（对应 Go const / C# const）
    const MAX_POINTS: u32 = 100_000; // 下划线可读性分隔符
    println!("常量 MAX_POINTS = {}", MAX_POINTS);

    // 类型别名：为复杂类型起短名（对应 Go type / C# using）
    type Speed = u64;
    let s: Speed = 120;
    println!("类型别名 Speed = {}", s);
}

// ============ 变量遮蔽（shadowing）============
pub fn shadowing() {
    // 遮蔽：同名变量可以重新声明，新变量会遮蔽旧变量（对应 JS let 重新声明不同作用域）
    let x = 5;
    let x = x + 1; // 遮蔽：新的 x
    let x = x * 2;
    println!("遮蔽后 x = {}", x); // 12

    // 遮蔽还可以改变类型（mut 不行）
    let spaces = "   "; // 字符串
    let spaces = spaces.len(); // 数字（遮蔽改变类型）
    println!("遮蔽改类型: {}", spaces); // 3
}

// ============ 练习题参考答案 ============
#[cfg(test)]
mod tests {
    // 练习 1：元组解构交换两个变量的值
    #[test]
    fn exercise_1_swap_with_tuple() {
        let (a, b) = (1, 2);
        let (b, a) = (a, b); // 通过解构 + 遮蔽交换
        assert_eq!(a, 2);
        assert_eq!(b, 1);
    }

    // 练习 2：给 u64 起别名 Speed 并声明变量
    #[test]
    fn exercise_2_type_alias() {
        type Speed = u64;
        let s: Speed = 120;
        assert_eq!(s, 120u64);
    }
}
