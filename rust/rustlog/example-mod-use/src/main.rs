//! example-mod-use 的入口：演示 Rust 模块系统（mod / use / 可见性 / pub use 重导出）。
//!
//! ## 学习建议
//! 对照每个文件顶部的模块层级注释阅读，然后运行：`cargo run -p example-mod-use`
//! 看到输出后再回源文件里对应着看「哪个 print 是谁打出来的」，多搞几遍就入门了。

mod utils;
mod models;
mod mod_use;

fn main() {
    // 通过 mod_use 模块间接调用（演示 pub use 重导出后的便利）
    println!("============ 通过 mod_use::log() 间接调用 ============");
    mod_use::log();

    println!("\n--------------------------------------------\n");

    // 直接使用 utils 和 models 模块（演示直接引用）
    println!("============ 直接调用 utils:: / models:: ============");
    utils::greet("laixhe");

    let mut user = models::User::new("laixhe", 18);
    user.print_info();

    // 观察 User 的字段级可见性：
    println!("\n---- User 可见性演示 ----");
    user.name = String::from("NEW_NAME"); // ✅ name 是 pub，可以直接改
    println!("直接读 pub 字段 name = {}", user.name);

    // user.age = 99;                    // ❌ 编译错误！age 是私有字段
    // println!("{}", user.age);         // ❌ 编译错误！age 是私有字段
    println!("通过 getter age() 读 age = {}", user.age());  // ✅ 用 getter 读私有字段
    match user.set_age(200) {
        Ok(_)  => println!("set_age 成功"),
        Err(e) => println!("✅ set_age(200) 合法地被拦下：{}", e),
    }

    // pub(crate) 方法：crate 内能调（如果在别的 crate 里就调不了）
    user.crate_only_method();

    // helpers 下的各种细粒度可见性演示
    println!("\n---- utils 细粒度可见性：pub(crate) / pub(super) / pub(in path) ----");
    // pub(crate)：整个 crate 内部可访问（这里 ok）
    utils::internal_log("main.rs 调用 pub(crate) internal_log() ✅");

    // ┌─ 重要可见性边界演示 ──────────────────────────────────────────┐
    // │ 下面两个函数 pub(super) / pub(in crate::utils) 的可见范围    │
    // │ **不包含 crate 根（main.rs）**，所以 pub use helpers::*     │
    // │ 也无法把它们重导出到 utils 外部。直接写 utils::xxx()        │
    // │ 会报「private function import」。                             │
    // │                                                              │
    // │ 我们已经在 helpers::demo_paths() 里调用过它们（在 helpers     │
    // │ 模块内部，正好符合可见性约束），这里就不重复了。               │
    // └──────────────────────────────────────────────────────────────┘
    println!("（pub(super) only_parent_module 和 pub(in crate::utils) only_utils_tree");
    println!(" 的可见性不允许 crate 根访问，已在 helpers::demo_paths() 内被调用——请查看上方输出）");
}
