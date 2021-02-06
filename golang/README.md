### 引用类型
```
slice       -- 切片，一个切片在未初始化之前默认为 nil
map         -- 映射(集合)(字典)，一个集合在未初始化之前默认为 nil，当为 nil map 不能用来存放键值对的
chan        -- 管道
```

### 内置函数
```
append       -- 把东西增加到slice里面,返回修改后的slice
close        -- 关闭channel
delete       -- 从map中删除key对应的value
panic        -- 停止常规的goroutine
recover      -- 允许程序定义goroutine的panic动作
imag         -- 返回complex的实部
real         -- 返回complex的虚部
make         -- 返回Type本身(只能应用于slice, map, channel)
new          -- 返回指向Type的指针
cap          -- 容量，容积capacity
copy         -- 复制slice，返回复制的数目
len          -- 返回长度

new 和 make 均是用于分配内存：new 用于值类型和用户定义的类型，
如自定义结构，make 用户内置引用类型（切片、map 和管道）
```

### GO语言关键字
```
Go语言的语言符号又称记法元素，共包括5类
标签符 identifier
关键字 keyword
操作符 operator
分隔符 delimiter
字面量 literal

标签符:
所有基本数据类型的名称 string、rune、bool、byte、int/uint、int8/uint8、int16/uint16、
                   int32/uint32、int64/uint64、float32、float64、complex64、complex128
接口类型             error
常量                true、false 以及 iota
所有内建函数的名称    append、cap、close、complex、copy、delete、imag、len、make、
                   new、panic、print、println、real、recover

关键字:
程序声明：           import、package
程序实体声明和定义    chan、const、func、interface、map、struct、type、var
程序流程控制         go、select、break、case、continue、default、defer、else、for、
                   goto、if、range、return、switch、fallthrough

八个复合类型:
Array      数组
Struct     结构体
Function   函数
Interface  接口
Slice      切片
Map        字典
Channel    通道
Pointer    指针
```
