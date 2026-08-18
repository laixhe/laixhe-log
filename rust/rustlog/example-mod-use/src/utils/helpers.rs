//! helpers.rs：演示 **函数级可见性** + **use 相对路径** + **pub(crate) 细粒度修饰**。

// ===== 各种可见性修饰符（新手学习重点）=====

// 1) pub：全局公开，外部调用者用 `use utils::greet` 就能用
pub fn greet(name: &str) {
    println!("你好，{}", name);
}

// 2) pub(crate)：只对**整个 crate 内部**公开（crate 外面 import 不了）
//    非常适合写 crate 内部共享的工具函数——对外隐藏实现细节
pub(crate) fn internal_log(msg: &str) {
    println!("[内部日志 crate only] {}", msg);
}

// 3) pub(super)：只对**父模块**公开（父模块即 utils.rs 里能看到，再外面不行）
//    等价于 pub(in super)
pub(super) fn only_parent_module() {
    println!("只有 utils 模块自己和它的父级能看到我");
}

// 4) pub(in crate::utils)：指定**具体路径范围**内可见
//    语法是 pub(in path)，path 必须是当前或父级模块路径
pub(in crate::utils) fn only_utils_tree() {
    println!("只有 crate::utils 下的模块能访问到我");
}

// 5) 无 pub：模块私有（默认）—— 只有 helpers.rs 内部能用
fn really_private() {
    println!("只有 helpers.rs 内部能调用我");
}

// ===== use 路径形式对比（演示用）=====
//
// 在 crate 内使用路径有三种形式：
//   A) 绝对路径：以 `crate::` 开头，从 crate 根出发
//   B) 相对路径：以 `self::` 或 `super::` 开头
//        self::  = 当前模块
//        super:: = 父模块（向上一级）
//
// 下面这组函数在 main.rs 里通过 `pub use helpers::*;` 导出给整个 crate 使用。

/// 演示四种路径的等价写法
pub fn demo_paths() {
    // 调用本模块私有函数 really_private()
    // 等价写法：
    really_private();                 // 最简：直接调用（同模块内不需要前缀）
    self::really_private();           // 显式 self::

    // 调用同模块内的 pub(crate) 函数
    crate::utils::internal_log("路径：crate::utils::internal_log  绝对路径");
    self::internal_log("路径：self::internal_log  相对路径（self=当前helpers）");

    // ================================================
    //  ✅ 在 helpers 模块内部调用 pub(super) / pub(in crate::utils)
    //     刚好符合它们的可见性限制（super=utils，crate::utils 也包含 helpers）
    // ================================================
    self::only_parent_module();        // helpers 内：pub(super) → super(utils) 可见，helpers 当然可见
    self::only_utils_tree();           // helpers 内：pub(in crate::utils) → crate::utils 内，helpers 在里面

    // 注意：helpers 是 utils 的子模块，super 就是 utils。
    // super::internal_log(...)  ❌ 不行！因为 super（utils 模块）
    // 里没有 internal_log 这个名字——internal_log 在 self（helpers）里。
}

/// 演示 use + as 重命名（适合同名符号冲突场景）
pub fn demo_use_rename() {
    // 在函数内部也能写 use！作用域只在这个函数块内
    // 把 std::collections::HashMap 拿进来并改个短名
    use std::collections::HashMap as Map;

    let mut m: Map<&str, i32> = Map::new();
    m.insert("a", 1);
    println!("use ... as 重命名 HashMap 为 Map，演示插入: {:?}", m);
}

/// 嵌套 use 分组写法（Rust 2018+ 支持，import 列表更简洁）
pub fn demo_nested_use() {
    // 把同一路径前缀的多个子项写在一个大括号里
    use std::collections::{
        HashMap, HashSet,    // 两个集合
        BTreeMap as BMap,    // 顺便重命名
    };
    // 上面三行等价于单独写三条 use
    let _h: HashMap<(), ()> = HashMap::new();
    let _s: HashSet<()>     = HashSet::new();
    let _b: BMap<(), ()>    = BMap::new();
    println!("嵌套 use {{ ... }} 写法 OK");
}
