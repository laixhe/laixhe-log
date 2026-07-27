// 公共结构体
#[derive(Debug)]
pub struct User {
    // 公共字段：外部可以直接访问和修改
    pub name: String,
    // 私有字段：外部无法直接访问
    age: u32,
}

impl User {
    // 公共构造函数
    pub fn new(name: &str, age: u32) -> Self {
        User {
            name: name.to_string(),
            age,
        }
    }

    // 公共方法
    pub fn print_info(&self) {
        if self.validate_age() {
            panic!("年龄非法");
        }
        println!("User: name={}, age={}", self.name, self.age);
    }

    // 私有方法，仅当前模块可调用
    fn validate_age(&self) -> bool {
        self.age == 0
    }
}
