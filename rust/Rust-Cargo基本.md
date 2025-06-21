### Cargo 基本命令
```
cargo
      init    xxx  # 初始化当前所在目录
      new     xxx  # 创建项(初始化一个新的目录)
      install xxx  # 安装依赖
      add     xxx  # 添加依赖关系
      remove  xxx  # 删除依赖关系
      build   xxx  # 编译项目
      run     xxx  # 运行项目
      update  xxx  # 更新依赖
      clean   xxx  # 清理构建目录(默认 target)
      check   xxx  # 检查语法
      fmt     xxx  # 格式代码
      test    xxx  # 测试
      bench        # 分析代码性能
      tree         # 查看库依赖关系
      doc          # 生成文档
      publish      # 发布模块

--version          # 版本
--lib         xxx  # 指定为库
--release          # 发布构建(默认 debug)
--target      xxx  # 指定架构库
--features xxx[full]  # 指定库的特征
-p            xxx  # 指定库
```

```
# 指定编译架构库
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
name = "xxx"
version = "0.1.0"
edition = "2021"
authors = ["laixhe <laixhe@laixhe.com>"]

# 依赖关系
[dependencies]
## 从本地指定依赖
my-lib = { path = "../my-lib" }
## 从 git 仓库指定依赖
rand = { git = "https://github.com/rust-lang-nursery/rand", version = "0.8" }
## 从 crates.io 指定依赖项(默认)
rand = "0.8"
tokio = { version = "1.23", features = ["full"] }
```
