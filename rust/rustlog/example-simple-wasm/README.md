#### WebAssembly 编程 —— 从 0 到在浏览器里跑起来

```
# 1. 添加架构目标（只需安装一次）
rustup target add wasm32-unknown-unknown

# 2. 安装 wasm-pack（只需安装一次）：一键把 Rust 编译成浏览器能用的 WASM + JS 胶水代码
cargo install wasm-pack

# 3. 在 example-simple-wasm 目录下构建（生成 pkg/ 目录）
cd example-simple-wasm
#    target web     —— 现代浏览器原生 ES Module 加载（配合 hello.html 使用）
#    out-name       —— 自定义生成的 .wasm / .js 文件名，方便 hello.html 引用
wasm-pack build --target web --out-dir ./pkg --out-name example-simple-wasm

#    其他 target（按需选择）：
#      --target bundler    —— 给 vite/webpack/rollup 等打包器用
#      --target nodejs     —— Node.js 环境用 require() 加载
#      --target no-modules —— 老浏览器 <script> 标签引入全局变量模式
```

##### 构建产物说明（pkg/ 目录）
| 文件 | 作用 |
|---|---|
| `example-simple-wasm.wasm` | 编译出的 WebAssembly 字节码，真正的 Rust 代码本体 |
| `example-simple-wasm.js`   | `wasm-bindgen` 生成的 JS 胶水代码：负责加载 WASM、字符串/对象互转 |
| `example-simple-wasm.d.ts` | TypeScript 类型声明（TS 项目直接用） |

```
# 4. 配套运行时库 & 工具（了解即可）：

#   wasm-bindgen          —— 核心！Rust ⇄ JS 类型互操作（字符串、对象、函数调用）
#   wasm-opt              —— 优化 WASM 体积（可选，binaryen 套件中的工具，需要额外安装）
#   web-sys               —— 浏览器 API 的 Rust 绑定（DOM / fetch / WebGL / WebSocket 等）
#   js-sys                —— JS 标准对象绑定（Array / Promise / Map / Date 等）
#   console_error_panic_hook —— 把 Rust panic 信息打印到浏览器 console，否则你只会看到 "unreachable"
#   gloo                  —— 高层浏览器 API 封装（比 web-sys 更 Rust 风格）
```

##### 🚀 本地跑 hello.html 看效果

```
# 在项目根目录启动一个静态文件服务器（二选一）
# 方式 A：Python 自带
python -m http.server 8080

# 方式 B：更专业的 Rust 工具（cargo install basic-http-server）
basic-http-server -A 127.0.0.1:8080 .
```

然后浏览器打开：<http://localhost:8080/example-simple-wasm/hello.html>

在输入框里填名字，点按钮 → 浏览器的 JS 就会调用 Rust 编译出来的 `greet()` 函数。
