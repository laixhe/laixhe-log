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
func funcLambda() {
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

	fmt.Println("TestLambda1=", TestLambda1())
	fmt.Println("TestLambda2=", TestLambda2())
}

// 关于闭包的问题

func TestLambda1() (val int) { // val 作用域范围大
	defer func() {
		val += 1 // 不管是返回下面是 val 还是 100，这里都 会 影响到的，因为（val 是直接挂载到 调用方 函数上的，作用域范围更大）
	}()

	val = 10
	//return val // val 作用域范围还没结束
	return 100 // 这里也是对 val 的赋值操作
}

func TestLambda2() int {
	val := 10

	defer func() {
		val += 1 // 不管是返回下面是 val 还是 100，这里都 不会 影响到的，因为（val 是 TestLambda2 自己的）
	}()

	return val // val 作用域范围结束了
	//return 100
}
