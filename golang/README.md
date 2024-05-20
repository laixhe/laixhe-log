## 需要 Golang 1.20 以上
```
# 设置代理
export GOPROXY=https://proxy.golang.com.cn,direct
# 当前项目下
go mod tidy
```

### 运行单个文件
```
go run var.go
```

> Go语言的参数传递只有值传递，且函数也是一个指针类型

### 引用类型
```
slice       -- 切片，一个切片在未初始化之前默认为 nil
map         -- 映射(集合)(字典)，一个集合在未初始化之前默认为 nil，当为 nil map 不能用来存放键值对的
chan        -- 管道(Channel)
```

### 可以为 nil 的类型
```
指针 pointer
切片 slice
字典 map
通道 channel
函数 function
接口 Interface
```

### 内置函数
> new 和 make 均是用于分配内存
> 
> new  主要为类型申请内存空间，并返回指针类型，用于值类型和用户定义的类型(如自定义结构)
> 
> make 主要初始化类型所需的内存空间和结构，用户内置引用类型 (slice(切片)、map、channel(管道))

```
append       -- 把东西增加到 slice 里面,返回修改后的 slice
close        -- 关闭 channel
delete       -- 从map中删除 key 对应的 value
panic        -- 停止常规的 goroutine(处理不当会导致程序退出)
recover      -- 允许程序定义 goroutine 的 panic 动作
imag         -- 返回 complex 的实部
real         -- 返回 complex 的虚部
make         -- 返回 Type 本身(只能应用于 slice, map, channel)
new          -- 返回指向 Type 的指针
cap          -- 容量、容积 capacity
copy         -- 复制 slice 返回复制的数目
len          -- 返回长度
```

### 八个复合类型
```
Array      数组
Struct     结构体
Function   函数
Interface  接口
Slice      切片
Map        字典
Channel    通道
Pointer    指针
```

### 对 `nil` 的定义

`nil` 是一个预先声明的标识符(不是关键字)，是没有`默认类型`的，代表指针(`pointer`)、通道(`channel`)、函数(`func`)、接口(`interface`)、`map`、切片(`slice`)。也可以这么理解：指针、通道、函数、接口、map、切片的零值就是 `nil`，就像布尔类型的零值是 `false`、整型的零值是 `0`
```
# 两个 nil 是不能比较的，无法编译
fmt.Println( nil== nil )
```