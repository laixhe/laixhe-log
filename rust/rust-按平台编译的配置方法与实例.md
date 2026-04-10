
> `Rust` 提供了一些平台检测宏，比如 `cfg(target_os)` 可以检测操作系统。

```
fn main() {
    // 如果是 Windows 系统
    #[cfg(target_os = "windows")]
    {
        println!("Windows...");
    }
    // 如果是 Linux 系统
    #[cfg(target_os = "linux")]
    {
        println!("Linux...");
    }
}
```

> 配置不同平台的依赖

在 `Windows` 上编译时，会引入 `windows-sys` 依赖
在 `Linux` 上编译时，会引入 `libc` 依赖

```
[package]
name = "test"
version = "0.1.0"
edition = "2024"

[dependencies]
# 通用依赖
serde = { version = "1", features = ["derive"] }

# 不同平台的依赖
[target.'cfg(target_os = "windows")'.dependencies]
windows-sys = "0.61"

[target.'cfg(target_os = "linux")'.dependencies]
libc = "0.2"
```
