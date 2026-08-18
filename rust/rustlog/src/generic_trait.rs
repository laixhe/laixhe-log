//! 泛型与特征（trait）：泛型函数 / 泛型结构体 / trait 定义与实现 / 特征约束。
//!
//! ## 前置知识
//! - 泛型 `T` 让函数/类型适用于多种类型（对应 Java 泛型 / C# 泛型 / Go 泛型）
//! - `trait` 定义共享行为（对应 Java interface / Go interface / C# interface）
//! - `impl Trait for Type` 为类型实现特征（隐式实现，对应 Go 结构体实现接口）
//!
//! ## 练习题
//! 1. 写泛型函数 `max_of_two`，要求 T 实现 `PartialOrd`。
//! 2. 定义 trait `Describe`（方法 describe），为两个不同类型实现它。

// ============ 泛型函数 ============
pub fn generic_function() {
    // 泛型函数：T 是类型参数（对应 Java <T> / Go [T]）
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let nums = vec![3, 7, 2, 9];
    println!("int 最大值: {}", largest(&nums)); // 9

    let chars = vec!['a', 'z', 'm'];
    println!("char 最大值: {}", largest(&chars)); // z
}

// ============ 泛型结构体 ============
pub fn generic_struct() {
    // 泛型结构体（对应 Java 泛型类 / C# 泛型类型）
    struct Pair<A, B> {
        first: A,
        second: B,
    }

    let p1 = Pair {
        first: String::from("age"),
        second: 18,
    };
    println!("Pair: {} {}", p1.first, p1.second);

    // impl 里声明类型参数
    impl<A, B> Pair<A, B> {
        fn first(&self) -> &A {
            &self.first
        }
    }
    println!("方法访问: {}", p1.first());
}

// ============ trait 定义与实现 ============
pub fn traits() {
    // 定义 trait（对应 Java interface / Go interface）
    trait Speak {
        fn speak(&self) -> String;
    }

    // 为不同类型实现同一 trait（对应 Go 隐式实现 / Java implements）
    struct Dog {
        name: String,
    }
    impl Speak for Dog {
        fn speak(&self) -> String {
            format!("{} 汪汪叫", self.name)
        }
    }

    struct Cat {
        name: String,
    }
    impl Speak for Cat {
        fn speak(&self) -> String {
            format!("{} 喵喵叫", self.name)
        }
    }

    // trait 对象：&dyn Trait 多态调用（对应 Go interface 变量 / Java 接口引用）
    let dog = Dog {
        name: String::from("旺财"),
    };
    let cat = Cat {
        name: String::from("咪咪"),
    };

    let animals: Vec<&dyn Speak> = vec![&dog, &cat];
    for a in animals {
        println!("{}", a.speak());
    }
}

// ============ 特征约束（trait bounds）============
pub fn trait_bounds() {
    // 泛型 + 特征约束：T 必须实现 Display 才能格式化（对应 Java <T extends Comparable>）
    fn print_value<T: std::fmt::Display>(v: T) {
        println!("值: {}", v);
    }
    print_value(42);
    print_value(String::from("hello"));

    // where 子句：约束较多时可读性更好
    fn describe<T>(v: T) -> String
    where
        T: std::fmt::Debug + std::fmt::Display,
    {
        format!("Display: {}，Debug: {:?}", v, v)
    }
    println!("{}", describe(3.14));
}

// ============ 练习题参考答案 ============
#[cfg(test)]
mod tests {
    // 练习 1：泛型 max 函数
    #[test]
    fn exercise_1_generic_max() {
        fn max_of_two<T: PartialOrd>(a: T, b: T) -> T {
            if a > b {
                a
            } else {
                b
            }
        }
        assert_eq!(max_of_two(3, 7), 7);
        assert_eq!(max_of_two('a', 'z'), 'z');
    }

    // 练习 2：trait Describe 多实现
    #[test]
    fn exercise_2_trait_describe() {
        trait Describe {
            fn describe(&self) -> String;
        }
        struct Point {
            x: i32,
            y: i32,
        }
        impl Describe for Point {
            fn describe(&self) -> String {
                format!("Point({}, {})", self.x, self.y)
            }
        }
        struct Name(String);
        impl Describe for Name {
            fn describe(&self) -> String {
                format!("Name: {}", self.0)
            }
        }

        let p = Point { x: 1, y: 2 };
        let n = Name(String::from("laixhe"));
        assert_eq!(p.describe(), "Point(1, 2)");
        assert_eq!(n.describe(), "Name: laixhe");
    }
}
