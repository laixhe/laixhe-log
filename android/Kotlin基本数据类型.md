#### 基本类型
> 包括 Byte、Short、Int、Long、Float、Double 、Boolean、String
>

```
val world = "World"
println("Hello, "+world)
println("Hello, $world")
// 结果： Hello, World

val number = 100           // 默认为 Int 类型
val bigNumber = 8000000000 // 超过 Int 最大值默认为 Long 类型
val longNumber = 20L       // 数字后面显式加 L 表示声明为 Long 类型
val byteNumber: Byte = 1
val doubleNumber = 3.1415928888 // 默认是 Double 类型
val floatNumber = 3.1415928888f // 尾部加 f或F 显式表示这是一个 Float 类型的浮点数，如查赋值超过精度会四舍五入，最多6位小数，所以实际值大小为 3.141593
```

