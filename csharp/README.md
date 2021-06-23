## dotnet 基本

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
# 比如新建一个控制台项目
dotnet new console -o xxxx
# 安装第三方库
dotnet add package xxx
```

#### 恢复项目的依赖项和工具
```
dotnet restore
```

#### 项目的运行、构建、测试
```
dotnet run
dotnet build
dotnet test
```
