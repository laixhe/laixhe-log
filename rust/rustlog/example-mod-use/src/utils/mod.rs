//! utils/mod.rs：演示模块声明 + pub use 重导出。
//!
//! 模块层级关系：
//! ```
//! crate
//! └── utils              ← 此文件（utils/mod.rs 等价于 utils.rs）
//!     └── helpers        ← pub mod helpers;  声明子模块
//!         ├── greet                pub
//!         ├── internal_log         pub(crate)
//!         ├── only_parent_module   pub(super)
//!         ├── only_utils_tree      pub(in crate::utils)
//!         └── really_private       私有
//! ```

// 声明 helpers 子模块。不加 pub 的话，外面（main.rs / 其他 crate）连 `utils::helpers` 这个路径都看不到
pub mod helpers;

// ===== pub use：重导出 =====
//
// helpers.rs 里有一堆函数。用户想用必须写 `utils::helpers::greet(...)`，层级深打字多。
// `pub use helpers::*;` 把 helpers 中所有 pub 项**重导出**到 utils 这一层，
// 于是调用方直接写 `utils::greet(...)` 就够了——内部实现拆分成多个文件，但对外 API 仍然扁平。
pub use helpers::*;
