##### 说明
构建约束（build constraints），也称为构建标记，是写在文件顶部的编译指令。
Go 1.17 起推荐使用 `//go:build`，旧的 `// +build` 语法已废弃。

```go
//go:build xxx

```

约束必须出现在 `package` 子句之前，并且后面要跟一个空行，以便与程序包文档区分开。

#### 实例
##### 代码
$GOPATH/test/build/a.go
```go
//go:build a

package build

import "fmt"

func init() {
	fmt.Println("this a")
}

func Say() {
	fmt.Println("I'm a!")
}
```
$GOPATH/test/build/b.go
```go
//go:build b

package build

import "fmt"

func init() {
	fmt.Println("this b")
}

func Say() {
	fmt.Println("I'm b!")
}
```
$GOPATH/test/main.go
```go
package main

import "test/build"

func main() {
	build.Say()
}
```
##### 编译

    go build -o test -tags a main.go
    
##### 执行

    ./test
    // 输出以下结果
    this a
    I'm a!  
    
#### 参考资料

    https://golang.org/pkg/go/build/
