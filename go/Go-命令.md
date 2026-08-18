#### 配置环境变量并设置代理
- 支持使用 go env -w 去设置对应的 Go 环境变量
```
export GOROOT=/home/laiki/software/go
export PATH=$PATH:$GOROOT/bin

# 模块代理
go env -w GOPROXY=https://proxy.golang.com.cn,direct
```

#### go build
> 有时发布时我们想隐藏所有代码实现相关的信息

```
# -ldflags 链接参数
## -w 禁用调试信息，但不包括符号表，得到的程序就不能用 gdb 调试了
## -s 禁用符号表，在 panic 时候的 stack trace 就没有任何文件名/行号信息了
## -X 修改程序中 string 类型的值

# 可以减小构建后文件体积
go build -ldflags "-w -s"
# 传递版本号给 main 包的全局变量 Version 赋值
go build -ldflags "-X main.Version=0.1.1"

# -gcflags 编译参数
## -N 禁用优化
## -l 禁用函数内联
## -u 禁用 unsafe 代码
## -m 输出优化信息
## -S 输出汇编代码

go build -gcflags "-N -l -u"

# -race 竞态检测
go build -race

```

#### go tool 工具
```
# 查看支持哪些平台系统
go tool dist list
```

#### go clean 清理
```
# 清理缓存
go clean -cache
go clean -modcache
```

#### 交差编译
```
GOOS=linux GOARCH=amd64 go build
GOOS=windows GOARCH=amd64 go build
GOOS=darwin GOARCH=amd64 go build
GOOS=darwin GOARCH=arm64 go build
GOOS=ios GOARCH=arm64 go build
```

#### Go崩溃独立输出
```
gotraceback=system
ulimit -c unlimited
```

```
程序名	程序用途
dlv.exe	go      语言调试工具
gocode.exe  	go语言代码检查，自动补全
godef.exe     	go语言代码定义和引用的跳转
golint.exe 	    go语言代码规范检查
go-outline.exe 	用于在Go源文件中提取JSON形式声明的简单工具
gopkgs.exe 	    快速列出可用包的工具
gorename.exe 	在Go源代码中执行标识符的精确类型安全重命名
goreturns.exe 	类似fmt和import的工具，使用零值填充Go返回语句以匹配func返回类型
go-symbols.exe 	从go源码树中提取JSON形式的包符号的工具
gotour.exe 	    go语言指南网页版
guru.exe     	go语言源代码有关工具，如代码高亮等
```
