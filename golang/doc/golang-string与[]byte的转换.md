### string 与 []byte 的转换
> `string`类型，其实他底层本质就是一个`byte`类型的数组

#### 例子
> 对下面的代码执行: `go tool compile -N -l -S ./main.go` 可以看出 调用过程
```
package main

import (
	"fmt"
)

func main()  {
  str := "laixhe"
  by := []byte(str)   // 调用 runtime.stringtoslicebyte 位于 src/runtime/string.go

  str1 := string(by)  // 调用 runtime.slicebytetostring 位于 src/runtime/string.go
  fmt.Println(str1)
}
```

`string类型 转换 []byte类型` 通过字符串长度来决定是否需要重新分配一块内存，系统预先定义了一个长度为 32 的数组，字符串的长度超过了这个数组的长度，就说明 []byte 不够用了，需要重新分配一块内存了

`[]byte类型 转换 string类型` 会根据 []byte 的长度来决定是否重新分配内存，最后通过 memove 可以拷贝数组到字符串

> 总结: 上面的转换都会发生`内存拷贝`
