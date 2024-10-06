#### go build

> 有时发布时我们想隐藏所有代码实现相关的信息
```
# -w 去掉DWARF调试信息，得到的程序就不能用 gdb 调试了
# -s 去掉符号表 panic 时候的 stack trace 就没有任何文件名/行号信息了
# 可以减小构建后文件体积
go build -ldflags "-w -s"
# --extldflags "-static -fpic" 完全静态编译
go build -ldflags '-w -s --extldflags "-static -fpic"'
```

#### 配置环境变量并设置代理

```
export GOPROXY=https://proxy.golang.com.cn,direct
export GOROOT=/home/laiki/software/go
export GOPATH=/home/laiki/go
export PATH=$PATH:$GOROOT/bin:$GOPATH/bin
```


> 当一个指针赋值给 interface 类型时，无论此指针是否为 nil，赋值过的 interface 都不为 nil

#### 查看支持哪些平台系统

```
go tool dist list
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
dlv.exe	go 语言调试工具
gocode.exe	go语言代码检查，自动补全
godef.exe 	go语言代码定义和引用的跳转
golint.exe 	go语言代码规范检查
go-outline.exe 	用于在Go源文件中提取JSON形式声明的简单工具
gopkgs.exe 	快速列出可用包的工具
gorename.exe 	在Go源代码中执行标识符的精确类型安全重命名
goreturns.exe 	类似fmt和import的工具，使用零值填充Go返回语句以匹配func返回类型
go-symbols.exe 	从go源码树中提取JSON形式的包符号的工具
gotour.exe 	go语言指南网页版
guru.exe 	go语言源代码有关工具，如代码高亮等
```
