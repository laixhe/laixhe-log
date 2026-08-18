package main

import (
	"fmt"
	"testing"
)

/*
函数（对应 TS functions.test.ts / Python functions.py）
*/

// 多返回值（Go 特色，对应 Python 返回 tuple / Go 惯例返回 (result, error)）
func divide(a, b int) (int, error) {
	if b == 0 {
		return 0, fmt.Errorf("除数不能为 0")
	}
	return a / b, nil // nil = 没有错误
}

// 命名返回值（提前写好名字，函数体内直接赋值，return 时自动返回）
func sum3(a, b, c int) (total int) {
	total = a + b + c // 直接给命名返回值赋值
	return             // 等价 return total
}

// 可变参数（对应 Python *args / Java 可变参数）
func concat(sep string, parts ...string) string { // parts 是切片 []string
	result := ""
	for i, p := range parts {
		if i > 0 {
			result += sep
		}
		result += p
	}
	return result
}

// 函数是一等公民：可赋值给变量、作为参数传递（对应 JS/Python 函数式风格）
func apply(x int, fn func(int) int) int {
	return fn(x)
}

// 闭包：函数捕获外部变量（对应 JS 闭包 / Python 闭包）
func counter() func() int {
	count := 0
	return func() int { // 闭包会持续持有 count
		count++
		return count
	}
}

func TestFunction(t *testing.T) {
	// 多返回值
	q, err := divide(10, 3)
	fmt.Println("10/3 =", q, err) // 3 <nil>
	_, err = divide(1, 0)
	fmt.Println("除以 0:", err) // 除数不能为 0

	// 命名返回值
	fmt.Println("sum3:", sum3(1, 2, 3)) // 6

	// 可变参数（直接传 / 展开切片）
	fmt.Println("concat:", concat("-", "a", "b", "c")) // a-b-c
	words := []string{"x", "y"}
	fmt.Println("展开:", concat(",", words...)) // x,y

	// 函数作为参数（对应 JS Array.map 传入函数）
	doubled := apply(5, func(n int) int { return n * 2 })
	fmt.Println("apply 函数:", doubled) // 10

	// 闭包状态保持
	c := counter()
	fmt.Println("闭包:", c(), c(), c()) // 1 2 3
}

// defer：延迟执行（函数退出前执行，对应 RAII / finally，Go 特色）
func TestDefer(t *testing.T) {
	// defer 是 LIFO（后进先出），常用于资源释放（关闭文件/锁）
	defer fmt.Println("1: defer") // 最后执行
	defer fmt.Println("2: defer") // 先执行
	fmt.Println("函数体")

	// defer 参数在声明时求值，而不是执行时
	x := 10
	defer fmt.Println("defer 捕获的 x:", x) // 10（声明时的值）
	x = 99
	fmt.Println("修改后 x:", x)
}
