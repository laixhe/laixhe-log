package main

import "fmt"

// 由关键字```func```声明
// 有可变长参数(不定长参数)
// 没有可选参数
// 没有默认参数值
// 可以返回多个不同类型的值

// 函数可作为参数传递，实现回调
// 函数可作为返回值返回

// 无参数，无返回值
func funcVoid() {
	fmt.Println("void")
}

// 可变长参数(不定长参数)
func funcNames(s ...string) {
	fmt.Println(s)
}

// 有参数，多个返回值
func funcArgs(a int, s string) (int, string) {
	return a, s
}

// 匿名函数，可作为闭包
func funcLambda(){
	// 函数变量
	var sum = func(a int, b int) int {
		return a + b
	}
	fmt.Println(sum(1, 3)) // 结果： 4
}

func FuncMain() {
	funcVoid()

	funcNames("a", "b", "c") // 结果： [a b c]

	max, name := funcArgs(1, "d")
	fmt.Printf("%d, %q\n", max, name) // 结果： 1, "d"

	funcLambda()
}
