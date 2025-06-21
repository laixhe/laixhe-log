##### 安装前设置安装目录(可选)
```
# cargo的安装路径
export CARGO_HOME=xxx
# rustc的安装路径
export RUSTUP_HOME=xxx
```

##### 安装前设置 rustup-init为在线安装工具为设置国内镜像(可选)
```
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static 
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
```

##### 安装地址
> https://www.rust-lang.org/learn/get-started

##### 为第三方库设置国内镜像
> 为 $HOME/.cargo 目录下建一个名为 config 的文本文件( ~/.cargo/config )

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
