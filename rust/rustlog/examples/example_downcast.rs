//! 类型向下转型（downcast-rs）：在 trait 对象的基础上恢复具体类型。
//!
//! ## 前置知识：为什么需要向下转型？
//!
//! Rust 的 trait 对象（`dyn Base`）是「多态」：只暴露 `Base` trait 的方法，
//! 调用者**不知道**内部实际是 `Foo` 还是 `Bar`。但有时业务需要：
//!
//! > 「如果它是 Foo，我要调 `Foo::special_foo_method()`；
//! > 如果它是 Bar，我要调 `Bar::special_bar_method()`。」
//!
//! 标准库 `Any` trait 也能做，但 API 比较啰嗦（需要 `&'static` 等约束）。
//! **downcast-rs** crate 用宏帮你把这部分样板代码生成出来。
//!
//! 注意：如果所有分支都能抽出同一个接口，应该直接在 Base trait 里加方法；
//! 只有当「不同类型行为完全不同」时才需要 downcast。
//!
//! ## 练习题
//! 1. 新增一个具体类型 `Baz(String)`，实现 Base，然后在 main 里 Box::new(Baz(...))，
//!    观察 `downcast_ref::<Baz>` 能不能拿到内部的 String。
//! 2. 试试 `downcast_mut` 和 `downcast`（后者拿走所有权）的区别。
//! 3. 对比：如果不用 trait 对象，用 `enum Shape { Circle(f64), Square(f64) }` + match，
//!    哪种写法更适合你的业务？（enum 在类型封闭、穷尽匹配上更有优势。）

use downcast_rs::{impl_downcast, Downcast};

// ===== Step 1：声明支持向下转型的 trait =====
//
// Base trait 继承 Downcast，再用 impl_downcast 宏生成 downcast 相关方法。
trait Base: Downcast {
    /// 所有 Base 实现者都共有的方法
    fn describe(&self) -> &'static str;
}
impl_downcast!(Base); // ⭐ 自动生成 is::<T>() / downcast_ref::<T>() 等方法

// ===== Step 2：定义具体类型并实现 Base =====

#[derive(Debug)]
struct Foo(u32);
impl Base for Foo {
    fn describe(&self) -> &'static str { "I'm a Foo (holds u32)" }
}

#[derive(Debug)]
struct Bar(f64);
impl Base for Bar {
    fn describe(&self) -> &'static str { "I'm a Bar (holds f64)" }
}

/// 新增的 Baz 类型，演示扩展更多类型
#[derive(Debug)]
struct Baz(String);
impl Base for Baz {
    fn describe(&self) -> &'static str { "I'm a Baz (holds String)" }
}

fn main() {
    // 把各种 Base 实现者装到同一个 Vec 里——这就是「多态」：异构集合
    let objs: Vec<Box<dyn Base>> = vec![
        Box::new(Foo(42)),
        Box::new(Bar(3.14)),
        Box::new(Baz("hello rust".into())),
    ];

    for (idx, base) in objs.iter().enumerate() {
        println!("\n-- 对象 #{}: {} --", idx, base.describe());

        // ===== 1) is::<T>()：先判断是不是目标类型 =====
        println!("  is::<Foo>? {}  is::<Bar>? {}  is::<Baz>? {}",
                 base.is::<Foo>(), base.is::<Bar>(), base.is::<Baz>());

        // ===== 2) downcast_ref::<T>()：拿到不可变引用 =====
        if let Some(foo) = base.downcast_ref::<Foo>() {
            println!("  ✅ downcast_ref 为 Foo → Foo({})", foo.0);
        } else if let Some(bar) = base.downcast_ref::<Bar>() {
            println!("  ✅ downcast_ref 为 Bar → Bar({})", bar.0);
        } else if let Some(baz) = base.downcast_ref::<Baz>() {
            println!("  ✅ downcast_ref 为 Baz → Baz(\"{}\")", baz.0);
        }
    }

    // ===== 3) downcast_mut::<T>()：拿到可变引用，能修改内部值 =====
    println!("\n==== downcast_mut 演示 ====");
    let mut boxed: Box<dyn Base> = Box::new(Foo(0));
    if let Some(foo_mut) = boxed.downcast_mut::<Foo>() {
        foo_mut.0 = 999;
        println!("修改后 Foo 的值: Foo({})", foo_mut.0);
    }

    // ===== 4) downcast::<T>()：把 Box<dyn Base> 转回 Box<具体类型> =====
    // （拿走所有权；失败会把原来的 Box<dyn Base> 还给你）
    println!("\n==== downcast 所有权转移 演示 ====");
    let bx:  Box<dyn Base> = Box::new(Bar(2.71828));
    match bx.downcast::<Bar>() {
        Ok(bar_box)  => println!("成功把 Box<dyn Base> 转成 Box<Bar> → {:?}", bar_box),
        Err(_orig)   => unreachable!("这里实际装的是 Bar，所以不可能 Err"),
    }
}
