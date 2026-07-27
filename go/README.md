### 标准库
```
fmt       格式化输入输出
io        通用 I/O 接口（Reader, Writer）
os        操作系统交互（文件、目录、进程）
time      时间与日期处理
sync      并发同步原语（WaitGroup协程同步, Mutex RWMutex互斥锁, Once单例模式）
strings   字符串处理
strconv   类型转换
math      数学运算与随机数
regexp    正则表达式
sort      排序（切片 Ints, Strings）
flag      命令行参数解析
crypto    加密
hash      哈希
context   上下文管理
container 容器数据结构（heap堆实现, list双向链表, ring环形链表）
```

#### 默认值
```
没有初始化的基本类型默认为零值 (数值为 0 | 字符串为 "" | 布尔为 false)
可根据值自行判定变量类型
```

#### 第一个例子
```
package main

import (
	"fmt"
)

// GetAdd 定义一个相加的函数
func GetAdd(num1, num2 int) int {
	return num1 + num2
}

func main() {
	// 声明变量并初始化
	var hello string = "Hello"
	// 定义变量并根据值自动推断类型（简短声明方式（只能在函数内部使用））
	world := "World"
	s1 := fmt.Sprintf("%s %s", hello, world)
	s2 := hello + " " + world
	fmt.Println(s1) // 结果: Hello World
	fmt.Println(s2) // 结果: Hello World

	// 定义变量并调用相加函数
	numberInt := GetAdd(1, 2)
	fmt.Println(numberInt) // 结果: 3
}
```
