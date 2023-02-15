##### 说明
构建约束，也称为构建标记，是开始的行注释

```
// +build xxx

```

列出了在文件中应包含文件的条件。 约束可能会出现在任何类型的源文件中（不仅是Go），但它们必须出现在文件顶部附近，并且只能出现空白行和其他行注释。 这些规则意味着在Go文件中，构建约束必须出现在package子句之前。

为了将构建约束与程序包文档区分开，必须在一系列构建约束后跟空白行。

#### 实例
##### 代码
$GOPATH/test/build/a.go
```
// +build a

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
```
// +build b

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
```
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