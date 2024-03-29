### 逃逸分析的代码
> 在 `src/cmd/compile/internal/gc/escape.go` 下

### 逃逸分析原理
- 指向栈对象的指针不能存储在堆中
- 指向栈对象的指针不能超过该对象的存活期，也就说指针不能在栈对象被销毁后依旧存活

> 逃逸分析在编译阶段确定哪些变量可以分配在栈中，哪些变量分配在堆上

> 函数`返回局部`指针变量
```
func Add() *int {
 res := 0
 return &res
}

func main()  {
 Add()
}
```

函数返回的局部变量是一个指针变量，当函数`Add`执行结束后，对应的栈桢就会被销毁，但是引用已经返回到函数之外，在外部解引用地址时就会导致程序访问非法内存，所以`Go编译器`经过逃逸分析后将其在`堆`上分配内存

> `interface`类型逃逸
```
func main()  {
 str := ""
 fmt.Println(str)
}
```

`str`是`main`函数中的一个局部变量，传递给`fmt.Println()`函数后发生了逃逸，这是因为`fmt.Println()`函数的入参是一个`interface{}`类型，如果函数参数为`interface{}`，那么在`编译期间`就很难确定其参数的具体类型，也会发送`逃逸`

> `闭包`产生的逃逸
```
func Increase() func() int {
   n := 0
   return func() int {
      n++
      return n
   }
}

func main() {
   in := Increase()
   fmt.Println(in()) // 结果: 1
}
```

因为函数也是一个指针类型，所以匿名函数当作返回值时也发生了逃逸，在匿名函数中使用外部变量`n`，这个变量`n`会一直存在直到`in`被销毁，所以`n`变量逃逸到了堆上

## 变量大小不确定及栈空间不足引发逃逸
