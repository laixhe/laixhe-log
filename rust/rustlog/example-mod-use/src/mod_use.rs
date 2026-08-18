//! mod_use.rs：从另一个模块间接调用 utils / models，演示「跨模块间接引用 + 各种 use 形式」。

// ========== 多种 use 导入方式对比 ==========

// 方式 1：绝对路径（推荐新手写——最不容易写错）
use crate::models::User;

// 方式 2：相对路径（super 回到父级，父级就是 crate 根）
use super::utils as tools;  // 顺便 use ... as 重命名，utils 叫 tools 也行

// 方式 3：重导出一个更短的名字（等价）
use crate::utils::helpers;  // 访问 helpers 内部（helpers 是 pub 模块）

pub fn log() {
    // utils::greet 现在可以用重命名后的 tools::greet
    tools::greet("laixhe");

    // User 是用 use crate::models::User 直接导入的
    let user = User::new("laixhe", 18);
    user.print_info();

    // 访问 helpers 级别的具体演示函数（pub use helpers::* 已经把这些导到 utils 了）
    println!("\n---- helpers 路径 / use 重命名演示 ----");
    helpers::demo_paths();
    helpers::demo_use_rename();
    helpers::demo_nested_use();

    // 访问 models 中的 verify_paths 函数（里面反过来通过 crate::utils:: 调 pub(crate) 内容）
    println!("\n---- models/mod.rs 路径校验 ----");
    crate::models::verify_paths();
}
