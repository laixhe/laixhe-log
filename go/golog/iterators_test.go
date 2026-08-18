package main

import (
	"fmt"
	"iter"
	"slices"
	"strconv"
	"testing"
)

/*
迭代器（Go 1.23+ range over func）：自定义 iter.Seq 迭代器 + 组合操作
对应 Rust rustlog iterators.rs、C# cslog IteratorDemo.cs（LINQ）

Go 1.23 新特性 range over func：
- iter.Seq[V]    单值迭代器   func(yield func(V) bool)
- iter.Seq2[K,V] 双值迭代器   func(yield func(K,V) bool)（对应 map/索引遍历）
- 标准库配套：slices.Collect / slices.Sorted / maps.Keys / maps.Values
*/

// map 变换：对每个元素应用函数（对应 Rust map / C# Select）
func MapSeq[V, R any](seq iter.Seq[V], f func(V) R) iter.Seq[R] {
	return func(yield func(R) bool) {
		for v := range seq {
			if !yield(f(v)) {
				return
			}
		}
	}
}

// filter 过滤：保留满足条件的元素（对应 Rust filter / C# Where）
func FilterSeq[V any](seq iter.Seq[V], keep func(V) bool) iter.Seq[V] {
	return func(yield func(V) bool) {
		for v := range seq {
			if keep(v) && !yield(v) {
				return
			}
		}
	}
}

// take：取前 n 个（对应 Rust take / C# Take）
func TakeSeq[V any](seq iter.Seq[V], n int) iter.Seq[V] {
	return func(yield func(V) bool) {
		i := 0
		for v := range seq {
			if i >= n {
				return
			}
			if !yield(v) {
				return
			}
			i++
		}
	}
}

// drop：跳过前 n 个（对应 Rust skip / C# Skip）
func SkipSeq[V any](seq iter.Seq[V], n int) iter.Seq[V] {
	return func(yield func(V) bool) {
		i := 0
		for v := range seq {
			if i >= n {
				if !yield(v) {
					return
				}
			} else {
				i++
			}
		}
	}
}

// 迭代器：map / filter / take / drop（对应 Rust map/filter、C# Select/Where）
func TestIterMapFilter(t *testing.T) {
	// 1..10 的平方（对应 Rust map / C# Select）
	squares := slices.Collect(MapSeq(slices.Values([]int{1, 2, 3, 4, 5, 6, 7, 8, 9, 10}), func(n int) int {
		return n * n
	}))
	fmt.Println("1..10 平方:", squares) // [1 4 9 16 25 36 49 64 81 100]

	// 过滤长度 ≤ 3 的单词（对应 Rust filter / C# Where）
	words := []string{"go", "c++", "javascript", "rust", "java", "js"}
	shortWords := slices.Collect(FilterSeq(slices.Values(words), func(w string) bool {
		return len(w) <= 3
	}))
	fmt.Println("长度≤3 的单词:", shortWords) // [go c++ js]

	// take / drop（对应 Rust take/skip、C# Take/Skip）
	nums := slices.Values([]int{1, 2, 3, 4, 5, 6, 7, 8, 9, 10})
	fmt.Println("take(3):", slices.Collect(TakeSeq(nums, 3))) // [1 2 3]
	fmt.Println("skip(7):", slices.Collect(SkipSeq(nums, 7))) // [8 9 10]
}

// zip / 展平（对应 Rust zip、C# Zip + SelectMany）
func TestIterZipFlatten(t *testing.T) {
	names := []string{"Alice", "Bob", "Charlie"}
	scores := []int{95, 87, 92}

	// 配对（Seq2 迭代器：yield 返回两个值）
	pairs := slices.Collect(func(yield func(string) bool) {
		for i, name := range names {
			if i >= len(scores) {
				break
			}
			if !yield(fmt.Sprintf("(%s,%d)", name, scores[i])) {
				return
			}
		}
	})
	fmt.Println("zip 配对:", pairs) // [(Alice,95) (Bob,87) (Charlie,92)]

	// 展平嵌套切片（对应 Rust flat_map / C# SelectMany）
	nested := [][]int{{1, 2}, {3, 4}, {5, 6}}
	flattened := slices.Collect(func(yield func(int) bool) {
		for _, arr := range nested {
			for _, v := range arr {
				if !yield(v) {
					return
				}
			}
		}
	})
	fmt.Println("展平:", flattened) // [1 2 3 4 5 6]
}

// reduce / any / all（对应 Rust fold、C# Aggregate/Any/All）
func TestIterReduceAnyAll(t *testing.T) {
	nums := []int{1, 2, 3, 4, 5, 6, 7, 8, 9, 10}

	// reduce 求和（手写循环 + 遍历迭代器）
	sum := 0
	for v := range slices.Values(nums) {
		sum += v
	}
	fmt.Println("sum 1..10 =", sum) // 55

	// any：是否存在 > 10（提前返回）
	anyGreater := false
	for v := range slices.Values(nums) {
		if v > 10 {
			anyGreater = true
			break
		}
	}
	fmt.Println("any > 10?", anyGreater) // false

	// all：是否全部 > 0（出现不满足即返回）
	allPositive := true
	for v := range slices.Values(nums) {
		if v <= 0 {
			allPositive = false
			break
		}
	}
	fmt.Println("all > 0? ", allPositive) // true
}

// 综合实战：R&D 部门 30+ 员工平均月薪（对应 Go/Java/C# 综合示例）
func TestIterAverageSalary(t *testing.T) {
	type Person struct {
		Name   string
		Dept   string
		Age    int
		Salary int
	}
	staff := []Person{
		{Name: "Alice", Dept: "R&D", Age: 28, Salary: 45000},
		{Name: "Bob", Dept: "R&D", Age: 35, Salary: 55000},
		{Name: "Charlie", Dept: "R&D", Age: 32, Salary: 50000},
		{Name: "David", Dept: "HR", Age: 40, Salary: 30000},
		{Name: "Eve", Dept: "R&D", Age: 25, Salary: 35000},
	}

	// 链式组合：过滤 → 提取月薪 → 求平均
	avg := func() int {
		total, count := 0, 0
		for p := range slices.Values(staff) {
			if p.Dept == "R&D" && p.Age >= 30 {
				total += p.Salary
				count++
			}
		}
		if count == 0 {
			return 0
		}
		return total / count
	}()
	fmt.Println("R&D 30+ 员工平均月薪:", avg, "元/月") // 52500
}

// 迭代器组合（filter_map 模式：解析合法数字，对应 Rust filter_map）
func TestIterFilterMap(t *testing.T) {
	maybeNumbers := []string{"123", "abc", "456", "78x", "789"}

	valid := slices.Collect(func(yield func(int) bool) {
		for s := range slices.Values(maybeNumbers) {
			if n, err := strconv.Atoi(s); err == nil {
				if !yield(n) {
					return
				}
			}
		}
	})
	fmt.Println("filter_map 选出合法数字:", valid) // [123 456 789]
}
