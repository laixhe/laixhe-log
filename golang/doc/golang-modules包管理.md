
#### Go 1.13 推出了 GOPRIVATE 机制

> 标识出哪些 package 是 private 的，那么对于这个 package 的处理将不会从 proxy 下载

> GOPRIVATE 的值是一个以逗号分隔的列表

> 在最新 release 的 Go 1.13 版本中默认将 GOPROXY 设置为 https://proxy.golang.org，这个对于国内的开发者是无法直接使用的

```
# 配置 GOPROXY 环境变量
go env -w GOPROXY=https://proxy.golang.com.cn,direct
# 或
export GOPROXY=https://proxy.golang.com.cn,direct
```

```
go mod init xxx

go mod [tidy]
init                    初始化 modules，会生成一个 go.mod 文件
tidy                    添加缺失的模块以及移除无用的模块
download                下载 modules 到本地缓存
edit                    提供一种命令行交互修改 go.mod 的方式
graph                   将 module 的依赖图在命令行打印出来
verify                  用来检测依赖包自下载之后是否被改动过
why                     解释为什么需要包或模块

require  语句指定的依赖项模块
replace  语句可以替换依赖项模块
exclude  语句可以忽略依赖项模块

require xxx v0.0.0                     项目需要的依赖包及版本
replace xxx v0.0.0 => xxx v0.0.0       取代当前项目中的某些依赖包
exclude xxx v0.0.0                     排除某些包的特别版本

go get path@version                    添加依赖或修改依赖版本[latest|master][develop][commit id]
go get github.com/laixhe/goimg@v0.1.0
go get github.com/laixhe/goimg@master
go get github.com/laixhe/goimg@77162ddc41707930b4a7a23a9be998d84d1415b0
```

#### 例子

```
module mod-demo

go 1.13

require github.com/gin-gonic/gin v1.4.1

replace (
  golang.org/x/crypto => github.com/golang/crypto master
  golang.org/x/net => github.com/golang/net master
  golang.org/x/sys => github.com/golang/sys master
  golang.org/x/text => github.com/golang/text master
  golang.org/x/tools => github.com/golang/tools master
  golang.org/x/sync => github.com/golang/sync master
  golang.org/x/xerrors => github.com/golang/xerrors master
)
```
