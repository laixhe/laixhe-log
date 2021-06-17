// 面向对象的三个基本特征：封装(Encapsulation)、继承(Inheritance)、多态(Polymorphism)

// Rust 并不是一个纯面向对象编程语言，没有提供类（class）这个关键字，提供了结构体（struct）类型
// 而 struct 的成员默认都是 private 的，除非加上 pub关 键词做修饰，没有构造函数、没有析构函数

//

// trait(特质) 与接口类似 (给结构体添加定义的行为)
trait BaseTrait {
    fn show(&self);
}

// 定义结构体(结构体的声明仅包含了它内部的数据结构)
// 相关方法的实现是放在额外的 impl 语句块中的
#[derive(Debug)]
struct Base {
    id: i32,
    name: String
}

// 定义属于 Base 的方法
impl Base{
    // 相当于 静态方法
    pub fn new(id: i32, name: String) -> Base {
        Base{
            id: id,
            name: name
        }
    }
    // 实例方法
    fn set_name(&mut self, name: String) {
        // 拼接字符串
        self.name = format!("set name={}", name);
    }
}

// 实现 BaseTrait (trait(与接口类似))
impl BaseTrait for Base {
    fn show(&self) {
        println!("base show id={},name={}", self.id, self.name)
    }
}

fn main() {
    let mut base = Base{id:10, name: "laixhe".to_string()};
    println!("{:?}", base); // 结果： Base { id: 10, name: "laixhe" }
    base.set_name("lai".to_string());
    println!("{:?}", base); // 结果： Base { id: 10, name: "set name=lai" }

    let base_new = Base::new(88, String::from("base new..."));
    println!("{:?}", base_new); // 结果： Base { id: 88, name: "base new..." }
    base_new.show(); // 结果： base show id=88,name=base new...
}
