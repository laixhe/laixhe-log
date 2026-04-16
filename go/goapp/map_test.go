package main

import (
	"fmt"
	"maps"
	"slices"
	"testing"
)

// map       线程不安全
// sync.Map  字典线程安全

// 字典处理
func TestMapHandle(t *testing.T) {

	m := map[string]int{"a": 1, "b": 2, "c": 3}

	// 获取所有 key （获取字典的 keys 迭代器 -> 收集为切片 -> 返回新切片）
	keys := slices.Collect(maps.Keys(m))
	fmt.Println(keys) // 结果：[a c b]

	// 排序
	sortedKeys := slices.Sorted(maps.Keys(m)) // 从小到大（升序）
	fmt.Println(sortedKeys)                   // 结果：[a b c]
}
