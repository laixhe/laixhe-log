```
cargo run -p date_time
cargo run -p json
cargo run -p mod_use
cargo run -p tcpserver
cargo run -p tcpclient
```

### 可见性控制

> 在 Rust 中，模块内的项（如函数、结构体、枚举等）默认私有，仅能被当前模块及其子模块访问。

```
   修饰符                       访问范围说明
默认（无修饰符）	        私有：仅当前模块及其子模块可访问
pub	                    完全公开：可被任何模块访问
pub(crate)              Crate 级公开：仅能在当前 crate 内访问
pub(super)              父模块级公开：仅能被当前模块的父模块访问
pub(in path::to::mod)   指定路径公开：仅能在指定路径的模块内访问（如 pub(in crate::models)）
```

```
// 绝对路径
crate::
// 相对路径（当前模块）
self::
// 相对路径（父模块）
super::
```
