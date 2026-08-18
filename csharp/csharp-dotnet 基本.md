# dotnet CLI 基本

.NET 的命令行工具，用于创建、构建、运行、测试、发布项目。

## 安装与验证

```bash
# 设置环境变量（Linux/macOS）
export DOTNET_ROOT=/path/to/dotnet
export PATH=$PATH:$DOTNET

# 查看版本与详细信息
dotnet --version
dotnet --info
```

## 创建项目

```bash
# 查看能创建哪些项目模板
dotnet new --help
# 常用模板：
#   console     控制台应用
#   web         ASP.NET Core 空项目
#   mvc         ASP.NET Core Web 应用（MVC）
#   webapi      ASP.NET Core Web API
#   grpc        ASP.NET Core gRPC 服务
#   classlib    类库

# 常用参数：
#   -n          指定项目名
#   -o          指定输出目录（默认当前目录）
#   --no-https  创建不使用 HTTPS 证书运行的应用

# 示例
dotnet new console -n MyApp
dotnet new webapi --no-https
```

## 安装第三方包（NuGet）

```bash
dotnet add package <包名>     # 安装最新稳定版
    --version <版本号>         # 指定版本
    --prerelease               # 安装预览版

dotnet list package            # 查看已安装的包
    --outdated                 # 查看可更新的包
```

## 恢复 / 运行 / 构建 / 测试

```bash
dotnet restore                 # 恢复依赖（根据 .csproj 下载 NuGet 包）
dotnet run                     # 编译并运行
dotnet build                   # 仅编译
dotnet test                    # 运行单元测试
dotnet watch run --urls https://localhost:8080  # 监听文件变化自动重跑
```

## 发布（打包）

```bash
# 发布为独立应用（自带运行时，目标机器无需安装 .NET）
dotnet publish -r win-x86 -c Release

# 发布为依赖框架的应用（目标机器需安装 .NET）
dotnet publish -r linux-x64 --self-contained false -c Release
```
