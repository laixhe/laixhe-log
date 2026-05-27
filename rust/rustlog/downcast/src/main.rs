use downcast_rs::{Downcast, impl_downcast};

// 定义支持向下转型的 trait
trait Base: Downcast {}
impl_downcast!(Base);

// 实现该 trait 的具体类型
#[derive(Debug)]
struct Foo(u32);
impl Base for Foo {}

#[derive(Debug)]
struct Bar(f64);
impl Base for Bar {}

fn main() {
    // 创建 trait 对象
    let base: Box<dyn Base> = Box::new(Foo(42));

    // 尝试向下转型
    if let Some(foo) = base.downcast_ref::<Foo>() {
        println!("Found Foo with value: {}", foo.0); // 输出: Found Foo with value: 42
    } else if let Some(bar) = base.downcast_ref::<Bar>() {
        println!("Found Bar with value: {}", bar.0);
    }

    // 检查类型
    assert!(base.is::<Foo>());
}
