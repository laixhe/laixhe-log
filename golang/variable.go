package main

import "fmt"

// 没有初始化的基本类型默认为零值 (数值为 0 | 字符串为 "" | 布尔为 false])
// 可根据值自行判定变量类型

// 数据类型
// 布尔型   bool (true 或者 false)
// 数字类型(整型)
//         uint8 (0~255) uint16 (0~65535) uint32 (0~4294967295) uint64 (0~18446744073709551615)
//         int8 (-128~127)  int16 (-32768~32767)  int32 (-2147483648~2147483647)  int64 (-9223372036854775808~9223372036854775807)
//         int uint 数据类型的大小将来自机器的架构，在  x86 为 32 位，在 x64 为 64 位
//         byte (别名 uint8)、rune (别名 int32)
// 数字类型(浮点型)
//        float32
//        float64

func HelloWorld() {
	var world = "World"
	fmt.Println("Hello,", world)
	fmt.Printf("Hello, %s", world)
	// 结果： Hello, World

	var a int
	var b string = "中文"
	var c = true // 推断为 bool 类型
	var d = 1.1  // 推断为 float64 类型
	var e = 2    // 推断为 int 类型
	// 结果：[ 0,"中文",true,1.1,2 ]
	fmt.Printf("[ %v,%q,%v,%v,%v ]\n", a, b, c, d, e)

	var arr []int // 只是 声明 没有 初始化，所以值为 nil
	// 结果：[] true
	fmt.Println(arr, arr == nil)

	// 以下几种类型为 nil
	// var a *int               // 指针 pointer
	// var a []int              // 切片 slice
	// var a map[string] int    // 字典 map
	// var a chan int           // 通道 channel
	// var a func(string) int   // 函数 function
	// var a interface          // 接口 Interface
}
