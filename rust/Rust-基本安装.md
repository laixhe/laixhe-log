#### 安装前设置安装目录
```
# cargo的安装路径
export CARGO_HOME=xxx
# rustc的安装路径
export RUSTUP_HOME=xxx
```

#### 安装前设置 rustup-init为在线安装工具为设置国内镜像
```
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
```

#### 为第三方库设置国内镜像
> 为 $HOME/.cargo 目录下建一个名为 config 的文本文件( ~/.cargo/config.toml )

```
[source.crates-io] 
registry="https://github.com/rust-lang/crates.io-index"

# 替换成你偏好的镜像源
replace-with='rsproxy'

# 字节跳动
[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"
[registries.rsproxy]
index = "https://rsproxy.cn/crates.io-index"

# 清华大学
[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

# 中国科学技术大学
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"

# 上海交通大学
[source.sjtu]
registry = "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index"

# rustcc社区
[source.rustcc]
registry = "git://crates.rustcc.cn/crates.io-index"

# 阿里云
[source.aliyun]
registry = "https://code.aliyun.com/rustcc/crates.io-index"

[net]
git-fetch-with-cli = true
```

#### 工具链架构
```
# 更新工具链
rustup update [stable]
# 更新 rustup 本身
rustup self update 
# 卸载 rust 所有程
rustup self uninstall

# 列出架构目标
rustup target list
# 添加架构目标
rustup target add aarch64-apple-ios x86_64-apple-ios
```

#### 简化跨平台编译
```
cargo install cargo-zigbuild
cargo zigbuild --target x86_64-unknown-linux-gnu --release
```

#### 让 Rust 编译更快
```
# 将已构建的内容缓存到本地
cargo install sccache --locked
# 有提供二进制文件 https://github.com/mozilla/sccache/releases

# 检查状态
sccache --show-stats

vim $HOME/.cargo/config.toml
[build]
rustc-wrapper = ".cargo/bin/sccache"
```

```
# 在 ~/.cargo/config.toml 中添加
[build]
# 设置最大并行编译单元数（建议设为 CPU 核心数）
jobs = 8
# 缓存编译
rustc-wrapper = "sccache"


# 在 Cargo.toml 中添加
[profile.dev]
# 启用增量编译
incremental = true
# 自定义优化级别减少最终构建差异（0~3）
opt-level = 0
# 并行代码生成单元数（1~16）
codegen-units = 16
# 是否保留调试信息
debug = true

[profile.dev.package."*"]
# 依赖包适度优化，平衡性能与速度
opt-level = 2
 
[profile.release]
# 启用最高级别优化
opt-level = 3
# 减少代码生成单元数量，提高优化效果
codegen-units = 1
# 是否链接时优化，提升性能
lto = true
# 是否剥离调试符号，减少二进制体积
strip = true
```
