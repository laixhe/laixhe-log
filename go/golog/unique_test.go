package main

import (
	"fmt"
	"testing"
)

// 去重：使用 map 记录已出现的元素（结果不保证顺序）
func TestUnique(t *testing.T) {
	nums := []int{3, 1, 2, 1, 3, 2, 4, 5, 4}

	seen := make(map[int]struct{})
	result := make([]int, 0, len(nums))
	for _, n := range nums {
		if _, ok := seen[n]; !ok {
			seen[n] = struct{}{}
			result = append(result, n)
		}
	}
	fmt.Println("map 去重:", result) // 结果: [3 1 2 4 5]（顺序不保证）
}
