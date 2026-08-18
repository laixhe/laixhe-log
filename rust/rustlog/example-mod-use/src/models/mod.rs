//! models/mod.rs：声明子模块 + pub use 重导出 + **use 路径演示**。

// 声明子模块 models::user，加 pub 否则外部看不到
pub mod user;

// ===== 多种 use 导入方式（等价，写哪种看团队风格）=====

// ⚠️ 演示说明（下面三行是三种等价写法，实际代码里选一种即可，不要都写否则会冲突）：
//   A. 绝对路径：  use crate::models::user::User;   ← ✅ 推荐新手用这种，从 crate 根出发，最不容易写错
//   B. 相对路径：  use self::user::User;            ← 路径短，但需要知道 self 是当前模块
//   C. super 路径： use super::models::user::User;  ← super 回到父级再往下找，层级深时容易绕晕

// pub use：把 User 这个名字直接挂到 models 这一层，
// 外部调用者就可以写 `use crate::models::User`，而不是更长的 `use crate::models::user::User`
pub use user::User;

// 下面用 super 和 crate:: 路径做一下等价性验证：给一个公共函数在内部用两种方式调
pub fn verify_paths() {
    // super::utils —— super 是 crate 根，所以 super::utils == crate::utils
    // 我们用 crate::utils 里重导出的 internal_log（pub(crate) 所以当前 crate 内能访问）
    crate::utils::internal_log("models/mod.rs 通过 crate::utils:: 路径调用成功");
}
