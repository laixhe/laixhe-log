//! example-simple-wasm：演示如何把 Rust 函数编译成 WebAssembly，供浏览器里的 JavaScript 调用。
//!
//! ## 关键点
//! - `#[wasm_bindgen]`：wasm-bindgen 的「标记」。被它标注的 `pub` 函数会暴露给 JS，
//!   参数和返回值会自动做 Rust ⇄ JS 的类型转换，并生成加载 WASM 的胶水代码。
//! - `&str` / `String` 会自动转成 JS 字符串；`i32` 对应 JS 的 number；`Vec<T>` 对应 JS 数组。
//!
//! ## 构建与运行
//! 详见本目录下的 README.md（需要安装 `wasm-pack` 和 `wasm32-unknown-unknown` 目标）。

use wasm_bindgen::prelude::*;

// 字符串进出：JS 传 string，Rust 返回 string
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// 数字互操作：JS 的 number ↔ Rust 的 i32
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 返回 Vec：在 JS 侧会变成一个普通数组
#[wasm_bindgen]
pub fn fib(n: u32) -> Vec<u32> {
    let mut seq = Vec::new();
    let (mut a, mut b) = (0u32, 1u32);
    for _ in 0..n {
        seq.push(a);
        let next = a + b;
        a = b;
        b = next;
    }
    seq
}

// 提示：实际项目建议在模块顶部调用 console_error_panic_hook::set_once()，
// 这样 Rust panic 时会打印到浏览器 console，否则只会看到 "unreachable"。
// 用法（需要先把 console_error_panic_hook 加入依赖）：
//   #[wasm_bindgen]
//   pub fn init() { console_error_panic_hook::set_once(); }

