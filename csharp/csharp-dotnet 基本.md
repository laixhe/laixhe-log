#### 安装
```
export DOTNET_ROOT=XXX
export PATH=$PATH:$DOTNET
```

```
dotnet --version
dotnet --info
```

#### 创建项目

```
# 能创建什么项目
dotnet new --help
           console # 控制台
           web      # ASP.NET Core 空
           mvc      # ASP.NET Core Web 应用程序 
           webapi   # ASP.NET Core Web API
           grpc     # ASP.NET Core gRPC 服务
           classlib # 
           
           -n          # 指定名字
           -o          # 输出的位置(默认为当前目录)
           --no-https  # 创建不使用 HTTPS 证书运行的应用

# dotnet new console -n xxxx
# dotnet new webapi --no-https


# 安装第三方库
dotnet add package xxx 
                    --version xxx # 指定版本
                    --prerelease  # 更新到最新版本
# 查看所有第三包
dotnet list package
                    --outdated    # 查看所有要更新的包


```

#### 恢复项目的依赖项和工具

    dotnet restore

#### 项目的运行、构建、测试

    dotnet run
    dotnet build
    dotnet test
    dotnet watch run --urls https://localhost:8080

#### 发布(打包)

    # 发布为独立应用
    dotnet publish -r win-x86 -c Release
    # 发布依赖于框架的应用
    # dotnet publish -r linux-x64 --self-contained false -c Release

