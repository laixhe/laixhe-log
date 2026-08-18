package main

import (
	"fmt"
	"testing"
)

/*
泛型（Go 1.18+）：对应 TS generics.test.ts / Java 泛型 / C# 泛型
- 泛型函数 / 泛型结构体 / 类型约束（constraints）
- 用 [] 声明类型参数，类型推断可省略
*/

// 泛型函数：对任意类型切片去重并保持顺序（对应 TS 泛型函数）
func uniqueGeneric[T comparable](input []T) []T {
	seen := make(map[T]struct{}, len(input))
	result := make([]T, 0, len(input))
	for _, v := range input {
		if _, ok := seen[v]; !ok {
			seen[v] = struct{}{}
			result = append(result, v)
		}
	}
	return result
}

// 泛型函数：返回切片中最大值（约束为可排序，对应 TS 泛型约束）
func maxValue[T ~int | ~int8 | ~int16 | ~int32 | ~int64 | ~uint | ~float32 | ~float64](s []T) T {
	m := s[0]
	for _, v := range s[1:] {
		if v > m {
			m = v
		}
	}
	return m
}

// 泛型结构体：键值对容器（对应 TS 泛型接口 / Java 泛型类）
type Pair[K comparable, V any] struct {
	Key   K
	Value V
}

// 泛型方法不能定义（Go 只允许泛型类型的方法使用类型参数），这里演示泛型结构体方法
func (p Pair[K, V]) String() string {
	return fmt.Sprintf("(%v, %v)", p.Key, p.Value)
}

// 泛型约束接口（对应 TS 泛型约束 extends）
type Number interface {
	~int | ~int8 | ~int16 | ~int32 | ~int64 | ~uint | ~uint8 | ~uint16 | ~uint32 | ~uint64 | ~float32 | ~float64
}

// 使用约束接口：求和
func sumGeneric[T Number](s []T) T {
	var total T // 泛型零值：var x T
	for _, v := range s {
		total += v
	}
	return total
}

func TestGenericFunction(t *testing.T) {
	// 类型推断：int 切片
	fmt.Println("int 去重:", uniqueGeneric([]int{3, 1, 2, 3, 1})) // [3 1 2]
	// 类型推断：string 切片（同一泛型函数复用）
	fmt.Println("string 去重:", uniqueGeneric([]string{"a", "b", "a"}))

	// 显式指定类型参数（通常可省略）
	fmt.Println("显式指定:", uniqueGeneric[int]([]int{1, 1, 2}))

	// 泛型函数：最大值
	fmt.Println("int 最大值:", maxValue([]int{3, 7, 2}))    // 7
	fmt.Println("float 最大值:", maxValue([]float64{1.5, 3.2, 2.1})) // 3.2
}

func TestGenericStruct(t *testing.T) {
	// 泛型结构体：K 可比较，V 任意
	p1 := Pair[string, int]{Key: "age", Value: 18}
	p2 := Pair[int, string]{Key: 1, Value: "one"}
	fmt.Println(p1.String()) // (age, 18)
	fmt.Println(p2.String()) // (1, one)
}

func TestGenericConstraint(t *testing.T) {
	// 约束接口：只接受数值类型
	fmt.Println("int 求和:", sumGeneric([]int{1, 2, 3}))       // 6
	fmt.Println("float 求和:", sumGeneric([]float64{1.5, 2.5})) // 4
	// sumGeneric([]string{"a"}) // ❌ string 不满足 Number 约束，编译报错
}
