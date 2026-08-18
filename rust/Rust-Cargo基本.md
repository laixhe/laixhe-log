### rustup 基本命令

```
rustup
    target  # 添加目标架构
            add xxx
```

### cargo 基本命令

```
cargo
    init    xxx  # 初始化当前所在目录
    new     xxx  # 创建项(初始化一个新的目录)
            [--lib 库]
    install xxx  # 安装依赖
    add     xxx  # 添加依赖关系
    remove  xxx  # 删除依赖关系
    build        # 编译项目
            [--release 发布构建(默认 debug)]
            [--target xxx 指定架构库]
    run          # 运行项目
    update       # 更新依赖
            [-p xxx 指定库]
    clean   xxx  # 清理构建目录(默认 target)
    check   xxx  # 检查语法
    fmt     xxx  # 格式代码
    test    xxx  # 测试
    bench        # 分析代码性能
    tree         # 查看库依赖关系
    doc          # 生成文档
    publish      # 发布模块

--version        # 版本
--lib       xxx  # 指定为库
--features  xxx[full]  # 指定库的特征
```

```
# 加依赖，直接编辑 Cargo.toml 或者用命令
cargo add serde --features derive
cargo add serde_json
# 编译运行
cargo run --release
```

### 目标架构

```
# 添加目标架构
rustup target add x86_64-unknown-linux-gnu
# 构建发布指定编译架构库
cargo build --target x86_64-apple-ios --release
# 指定更新依赖
cargo update -p time
```

### 工作空间(workspace)

- vim Cargo.toml

```
[workspace]

members = [
  "tcpserver", 
  "tcpclient"
]

```

### 项目空间(package)

- vim Cargo.toml

```
# 配置项目信息
[package]
name = "testrust"
version = "0.1.0"
edition = "2024"
authors = ["laixhe <laixhe@laixhe.com>"]

# 依赖关系
[dependencies]
## 从本地指定依赖
my-lib = { path = "../my-lib" }
## 从 git 仓库指定依赖
bevy = { git = "https://github.com/bevyengine/bevy", version = "0.19" }
bevy = { git = "https://github.com/bevyengine/bevy", branch = "main" }
bevy = { git = "https://github.com/bevyengine/bevy", rev = "ef69ba45b1404c08f31514c210e72fbc19a41314" }

bevy = { version = "0.19", default-features = false, features = ["default_app"] }

## 从 crates.io 指定依赖项(默认)
# Serde 核心库，启用 derive 宏，自动生成 trait 实现
serde = { version = "1.0", features = ["derive"] }
# JSON 格式适配器
serde_json = "1.0"
```

### 使用 release 配置编译

- vim Cargo.toml

```
[profile.release]
# 最高优化级别
opt-level=3
# 剥离调试符号，减小二进制体积
strip=true
# 开启链接时优化（让编译器在链接阶段对整个程序进行全局优化）
lto=true
# 使用单个代码生成单元，提升优化效果（会拖慢编译速度，但能让优化器做出更好的决策）
codegen-units=1
```

### 使用 dev 配置编译

- vim Cargo.toml

```
[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3
```
