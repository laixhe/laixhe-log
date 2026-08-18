package main

import (
	"fmt"
	"testing"
)

/*
基础类型 / 变量 / 常量（对应 TS basics.test.ts / Python basic_types.py）
*/

// 基本数据类型
func TestBasicTypes(t *testing.T) {
	// 整型：int8/16/32/64、uint、byte（= uint8）、rune（= int32，Unicode 码点）
	var i int = 42
	var b byte = 255      // 0~255
	var r rune = '中'     // Unicode 码点，等价 int32
	fmt.Println("int:", i, "byte:", b, "rune:", r, string(r)) // 42 255 20013 中

	// 浮点：float32 / float64
	var f float64 = 3.14
	fmt.Println("float:", f)

	// 布尔
	var ok bool = true
	fmt.Println("bool:", ok)

	// 复数（Go 特有）
	var c complex128 = 3 + 4i
	fmt.Println("complex:", c, "实部:", real(c), "虚部:", imag(c))

	// 零值：声明后不赋值自动为类型零值（区别于 C++ 未初始化 / Java 默认值）
	var zeroInt int
	var zeroStr string
	var zeroBool bool
	fmt.Printf("零值: int=%d string=%q bool=%v\n", zeroInt, zeroStr, zeroBool)
}

// 变量声明方式
func TestVariableDeclare(t *testing.T) {
	// 方式1：var 完整声明
	var name string = "laixhe"
	// 方式2：var 类型推断（省略类型）
	var age = 18
	// 方式3：短变量声明 :=（函数内，最常用）
	score := 88.8
	// 方式4：一次声明多个
	var x, y = 1, 2
	fmt.Println(name, age, score, x, y)

	// 多值交换（无需临时变量，区别于 C++/Java）
	a, b := 1, 2
	a, b = b, a
	fmt.Println("交换后:", a, b) // 2 1
}

// 常量与 iota（自动递增）
func TestConstIota(t *testing.T) {
	const Pi = 3.14159 // 常量（编译期确定，区别于 Java final / C++ constexpr）

	// iota：每行自动 +1，从 0 开始
	const (
		A = iota // 0
		B        // 1
		C        // 2
	)
	fmt.Println("iota:", A, B, C) // 0 1 2

	// iota 应用场景：枚举星期（区别于 Java enum / C# enum）
	const (
		Monday = iota + 1 // 1
		Tuesday           // 2
		Wednesday         // 3
	)
	fmt.Println("星期:", Monday, Tuesday, Wednesday) // 1 2 3

	// 常量与变量区分（常量可参与编译期计算）
	const Double = Pi * 2
	fmt.Println("2π:", Double)
}

// 指针（区别于 C++ 指针 / Java 引用）
func TestPointer(t *testing.T) {
	n := 100
	p := &n // 取地址（对应 C++ &）
	fmt.Println("地址:", p, "值:", *p)

	*p = 200 // 解引用修改（对应 C++ *p）
	fmt.Println("通过指针修改:", n) // 200
}
