//! 错误处理：Result / ? 运算符 / panic / 自定义错误。
//!
//! ## 前置知识
//! - `Result<T, E>`：成功 Ok(T) / 失败 Err(E)，必须显式处理（对应 Go error 返回值）
//! - `?` 运算符：错误自动向上传播（对应 Python raise / Java 抛出异常）
//! - `panic!`：不可恢复错误，程序崩溃（对应 Python 未捕获异常 / Go panic）
//!
//! ## 练习题
//! 1. 写函数 `parse_int` 返回 `Result<i32, String>`，非数字返回 Err。
//! 2. 用 `?` 写一个链式调用，两个可能失败的操作。

// ============ Result 基础 ============
pub fn result_basic() {
    // Result：成功 Ok / 失败 Err
    fn divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            return Err(String::from("除数不能为 0"));
        }
        Ok(a / b)
    }

    // match 处理（对应 Go if err != nil）
    match divide(10, 2) {
        Ok(v) => println!("10/2 = {}", v),
        Err(e) => println!("错误: {}", e),
    }

    // 常用方法：unwrap_or 默认值 / is_ok 判断
    println!("默认值: {}", divide(10, 0).unwrap_or(-1)); // -1
    println!("是否成功: {}", divide(10, 2).is_ok()); // true
}

// ============ ? 运算符（错误传播）============
pub fn question_mark() {
    // ? 将 Err 自动返回给调用者（对应 Python raise / Java throw）
    fn parse_and_add(s1: &str, s2: &str) -> Result<i32, std::num::ParseIntError> {
        let a: i32 = s1.parse()?; // 失败则直接 return Err
        let b: i32 = s2.parse()?;
        Ok(a + b)
    }

    match parse_and_add("10", "20") {
        Ok(sum) => println!("10 + 20 = {}", sum),
        Err(e) => println!("解析错误: {}", e),
    }

    // 失败传播
    match parse_and_add("10", "abc") {
        Ok(sum) => println!("sum = {}", sum),
        Err(e) => println!("失败传播: {}", e),
    }
}

// ============ 自定义错误 ============
pub fn custom_error() {
    // 自定义错误类型：实现 Display + Error trait（对应 Go 自定义 error / Python 异常类）
    use std::fmt;

    #[derive(Debug)]
    struct DivideError {
        divisor: i32,
    }

    impl fmt::Display for DivideError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "不能除以 {}", self.divisor)
        }
    }

    impl std::error::Error for DivideError {}

    fn divide(a: i32, b: i32) -> Result<i32, DivideError> {
        if b == 0 {
            return Err(DivideError { divisor: b });
        }
        Ok(a / b)
    }

    match divide(10, 0) {
        Ok(v) => println!("结果: {}", v),
        Err(e) => println!("自定义错误: {}", e), // 自动调用 Display
    }
}

// ============ panic ============
pub fn panic_demo() {
    // panic!：不可恢复错误（对应 Python raise 未捕获 / Go panic）
    println!("panic 示例（会崩溃，这里不实际调用）");
    // panic!("程序崩溃"); // ❌ 会终止程序
    // 生产代码避免 panic，用 Result 显式处理
}

// ============ 练习题参考答案 ============
#[cfg(test)]
mod tests {
    // 练习 1：parse_int 返回 Result
    #[test]
    fn exercise_1_parse_int() {
        fn parse_int(s: &str) -> Result<i32, String> {
            s.parse::<i32>()
                .map_err(|_| format!("'{}' 不是数字", s))
        }
        assert_eq!(parse_int("42"), Ok(42));
        assert!(parse_int("abc").is_err());
    }

    // 练习 2：? 链式调用
    #[test]
    fn exercise_2_question_mark() {
        fn chain(s: &str) -> Result<i32, std::num::ParseIntError> {
            let doubled: i32 = s.parse::<i32>()? * 2;
            let formatted = doubled.to_string();
            let parsed_back: i32 = formatted.parse()?;
            Ok(parsed_back)
        }
        assert_eq!(chain("21").unwrap(), 42);
    }
}
