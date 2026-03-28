
#### 新的 go.work 与 go.mod 具有相同的语法
- go work 管理的多模块(项目)结构(一般用于本地多模块调试)
- go mod  管理的模块(项目)

```
# 模块代理
go env -w GOPROXY=https://proxy.golang.com.cn,direct
```

```
# 工作区
go work init [xxx xxx]
init                   初始化 work ，会生成一个 go.work 文件
use                    添加缺失的模块


# 模块区
go mod init xxx
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

go get path@version                    添加依赖或修改依赖版本[latest|master]

```

#### 例子
```
## 工作区
go 1.18

use (
    ./xxx
    ./xxxx
)

replace golang.org/x/net => ../x/net@master


## 模块区
module mod-demo

go 1.18

require github.com/gin-gonic/gin v1.4.1

replace (
  golang.org/x/crypto => github.com/golang/crypto@master
  golang.org/x/net => github.com/golang/net@master
  golang.org/x/sys => github.com/golang/sys@master
  golang.org/x/text => github.com/golang/text@master
  golang.org/x/tools => github.com/golang/tools@master
  golang.org/x/sync => github.com/golang/sync@master
  golang.org/x/xerrors => github.com/golang/xerrors@master
)
```
