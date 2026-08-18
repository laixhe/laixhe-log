package main

import (
	"fmt"
	"testing"
)

/*
数组（array）：定长、值类型（对应 Java/Python 数组基础）
与 slice 的区别（见 slice_test.go）：
- 数组：长度固定、值语义（赋值/传参会拷贝整个数组）
- 切片：长度可变、引用语义（赋值/传参共享底层数组）
实际开发中数组很少直接用，多用切片；数组主要用于固定长度的场景（如 hash 桶）
*/

// 声明与初始化
func TestArrayDeclare(t *testing.T) {
	// 方式1：var 声明（零值填充）
	var a [3]int
	fmt.Println("var 声明:", a) // [0 0 0]

	// 方式2：字面量
	b := [3]int{1, 2, 3}
	fmt.Println("字面量:", b)

	// 方式3：省略长度（由编译器推断，...）
	c := [...]int{1, 2, 3, 4}
	fmt.Println("省略长度:", c, "长度:", len(c))

	// 方式4：指定下标初始化（其余零值）
	d := [5]int{1: 100, 3: 300} // [0 100 0 300 0]
	fmt.Println("指定下标:", d)
}

// 数组是值类型：赋值/传参整体拷贝（区别于 slice 引用语义）
func TestArrayValueSemantics(t *testing.T) {
	a := [3]int{1, 2, 3}

	b := a    // 拷贝整个数组
	b[0] = 99 // 修改副本
	fmt.Println("原数组不变:", a) // [1 2 3]

	// 传参也是拷贝
	modifyFirst := func(arr [3]int) {
		arr[0] = 1000
	}
	modifyFirst(a)
	fmt.Println("传参后原数组不变:", a) // [1 2 3]

	// 需要传引用时用指针（对应 C 语言数组退化为指针，Go 数组必须显式取地址）
	modifyFirstPtr := func(arr *[3]int) {
		arr[0] = 1000
	}
	modifyFirstPtr(&a)
	fmt.Println("传指针后修改:", a) // [1000 2 3]
}

// 多维数组（对应 Python 嵌套列表 / C 二维数组）
func TestArrayMulti(t *testing.T) {
	// 二维数组：固定行列
	var matrix [2][3]int // 2 行 3 列
	for i := 0; i < 2; i++ {
		for j := 0; j < 3; j++ {
			matrix[i][j] = i*10 + j
		}
	}
	fmt.Println("二维数组:", matrix) // [[0 1 2] [10 11 12]]

	// 字面量初始化
	matrix2 := [2][2]int{{1, 2}, {3, 4}}
	fmt.Println("字面量:", matrix2[1][0]) // 3
}

// 遍历（range 返回 下标 + 值）
func TestArrayRange(t *testing.T) {
	arr := [...]string{"go", "rust", "java"}

	// 下标 + 值
	for i, v := range arr {
		fmt.Println("下标:", i, "值:", v)
	}

	// 忽略下标
	for _, v := range arr {
		fmt.Println("仅值:", v)
	}
}
