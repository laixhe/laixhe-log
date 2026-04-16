package main

import (
	"fmt"
	"slices"
	"testing"
)

// 切片处理
func TestSliceHandle(t *testing.T) {
	// 查找
	fmt.Println(slices.Contains([]int{1, 2, 3}, 2))                // 结果：true
	fmt.Println(slices.Contains([]string{"切片", "查找", "处理"}, "查找")) // 结果：true
	// slices.IndexFunc 处理结构体数组

	// 排序（快速排序算法 Quicksort）（小规模切片会自动切换为插入排序以优化常数时间）
	order := []int{2, 1, 3}
	slices.Sort(order)    // 从小到大（升序）（原地修改切片）（字符串排序基于 Unicode 码点的字典序，例如大写字母会排在小写字母之前）
	fmt.Println(order)    // 结果：[1 2 3]
	slices.Reverse(order) // 从大到小
	fmt.Println(order)    // 结果：[3 2 1]
	// slices.IsSorted 判断切片是否已按升序排列

	// 移除连续重复元素（不是去除所有重复）
	compact := []int{11, 2, 2, 3, 3, 8, 11}
	fmt.Println(slices.Compact(compact)) // 结果：[11 2 3 8 11]
}
