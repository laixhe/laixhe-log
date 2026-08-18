//! 结构体与枚举：struct / enum / impl 方法 / 模式匹配。
//!
//! ## 前置知识
//! - `struct` 定义数据（对应 C 结构体 / Go struct / Python class 数据部分）
//! - `enum` 可携带数据，是 Rust 表达"可空"和"多态"的核心（对应 Go iota 枚举 + 联合）
//! - `impl` 给类型加方法（对应类方法 / Go 方法接收者）
//!
//! ## 练习题
//! 1. 定义一个 `Point` 结构体，写方法返回到原点的距离。
//! 2. 定义 `enum Shape { Circle(f64), Rect(f64, f64) }`，用 match 计算面积。

// ============ 结构体 ============
pub fn struct_basic() {
    // 结构体定义 + 使用（对应 Go struct / Python dataclass）
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    // 实例化（字段名初始化，顺序可任意）
    let p = Person {
        name: String::from("laixhe"),
        age: 18,
    };
    println!("结构体: {:?}", p);

    // 字段访问
    println!("名字: {}，年龄: {}", p.name, p.age);

    // 结构体更新语法：.. 复用其他实例字段（对应 Go 手写复制）
    let p2 = Person {
        name: String::from("Alice"),
        ..p // 复用 p 的 age
    };
    println!("更新语法: {:?}", p2);
}

// ============ 方法（impl）============
pub fn struct_method() {
    // impl 块定义方法（对应类方法 / Go 方法接收者）
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        // 关联函数（无 self）：相当于静态方法 / 构造函数（对应 Go NewXxx / C# static）
        fn new(w: u32, h: u32) -> Self {
            Rectangle {
                width: w,
                height: h,
            }
        }

        // 方法：&self 借用（只读）
        fn area(&self) -> u32 {
            self.width * self.height
        }

        // 方法：&mut self（可修改）
        fn scale(&mut self, factor: u32) {
            self.width *= factor;
            self.height *= factor;
        }
    }

    let mut rect = Rectangle::new(3, 4);
    println!("面积: {}", rect.area()); // 12
    rect.scale(2);
    println!("放大后面积: {}", rect.area()); // 48
}

// ============ 枚举 ============
pub fn enums() {
    // 枚举携带数据（对应 Python Enum 增强 / TS 可辨识联合）
    enum Message {
        Quit,                    // 无数据
        Move { x: i32, y: i32 }, // 匿名结构体
        Write(String),           // 单值
        ChangeColor(u8, u8, u8), // 元组
    }

    let msgs = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("hello")),
        Message::ChangeColor(255, 0, 0),
    ];

    // match 穷尽枚举所有变体（编译器强制，对应 TS switch exhaustive）
    for msg in msgs {
        match msg {
            Message::Quit => println!("退出"),
            Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
            Message::Write(text) => println!("写入: {}", text),
            Message::ChangeColor(r, g, b) => println!("变色 ({},{},{})", r, g, b),
        }
    }
}

// ============ Option 枚举（可空值）============
pub fn option_enum() {
    // Option<T>：有值 Some / 无值 None（对应 Go 指针 nil / Java Optional / C# nullable）
    let some: Option<i32> = Some(42);
    let none: Option<i32> = None;

    // match 处理
    for opt in [some, none] {
        match opt {
            Some(v) => println!("有值: {}", v),
            None => println!("无值: None"),
        }
    }

    // 常用方法：unwrap_or 给默认值（对应 Python dict.get / Go 判断 nil）
    println!("默认值: {} {}", some.unwrap_or(0), none.unwrap_or(0)); // 42 0
}

// ============ 练习题参考答案 ============
#[cfg(test)]
mod tests {
    // 练习 1：Point 到原点距离
    #[test]
    fn exercise_1_point_distance() {
        struct Point {
            x: f64,
            y: f64,
        }
        impl Point {
            fn distance_from_origin(&self) -> f64 {
                (self.x * self.x + self.y * self.y).sqrt()
            }
        }
        let p = Point { x: 3.0, y: 4.0 };
        assert_eq!(p.distance_from_origin(), 5.0);
    }

    // 练习 2：enum Shape 面积
    #[test]
    fn exercise_2_shape_area() {
        enum Shape {
            Circle(f64),
            Rect(f64, f64),
        }
        fn area(s: &Shape) -> f64 {
            match s {
                Shape::Circle(r) => std::f64::consts::PI * r * r,
                Shape::Rect(w, h) => w * h,
            }
        }
        let circle = Shape::Circle(1.0);
        let rect = Shape::Rect(2.0, 3.0);
        assert!((area(&circle) - std::f64::consts::PI).abs() < 1e-9);
        assert_eq!(area(&rect), 6.0);
    }
}
