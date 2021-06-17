package main

import "fmt"

// 没有初始化的基本类型默认为零值 (数值为 0 | 字符串为 "" | 布尔为 false])
// 可根据值自行判定变量类型

func main() {
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

	var arr []int
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
