//! user.rs：演示 **结构体字段级可见性** + **impl 方法级可见性** + **Getter**。

// 公共结构体：外部能构造它（但字段不一定能访问）
#[derive(Debug)]
pub struct User {
    // 公共字段：外部可以直接读取 & 修改
    pub name: String,
    // 私有字段：外部无法直接读写——只能通过 public 方法间接操作
    // 这就是 Rust 的「封装」：age 字段的读写权完全由 User 自己掌握
    age: u32,
    // pub(crate) 字段：crate 内部可直接读写，但外部 crate 不可以
    pub(crate) internal_tag: String,
}

impl User {
    /// 公共构造函数（整个 crate 外部都能调）
    ///
    /// 注意：为了示例简洁这里直接返回 Self；
    /// 生产代码建议做合法性校验后返回 `Result<Self, UserError>`。
    pub fn new(name: &str, age: u32) -> Self {
        User {
            name: name.to_string(),
            age,
            internal_tag: String::from("default_tag"),
        }
    }

    /// 公共方法：打印信息（调用内部私有方法 validate_age）
    pub fn print_info(&self) {
        if !self.validate_age() {
            println!("警告：年龄不合法 ({})，跳过打印", self.age);
            return;
        }
        println!("User: name={}, age={}", self.name, self.age);
    }

    /// Getter：私有字段 age 的只读访问入口（外部拿不到 &mut，所以只能读）
    pub fn age(&self) -> u32 {
        self.age
    }

    /// Setter：对外提供修改 age 的渠道，但我们可以在这里加校验
    pub fn set_age(&mut self, new_age: u32) -> Result<(), &'static str> {
        if new_age == 0 || new_age > 150 {
            return Err("年龄必须在 1~150 之间");
        }
        self.age = new_age;
        Ok(())
    }

    // pub(crate)：crate 内任意位置都能调，但外部 crate 调不了
    pub(crate) fn crate_only_method(&self) {
        println!("[crate-only] User::crate_only_method() 被调用了，name={}", self.name);
    }

    // 私有方法：只能在当前 impl / 当前模块内被调用
    fn validate_age(&self) -> bool {
        self.age > 0 && self.age <= 150
    }
}
