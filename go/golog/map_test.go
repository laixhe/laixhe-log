package main

import (
	"fmt"
	"maps"
	"slices"
	"testing"
)

/*
字典（map）基础 + 进阶：对应 Python dict / C# Dictionary / Java HashMap

map 特性：
- 引用类型：声明后必须 make 或字面量初始化，否则是 nil map（写入会 panic）
- 键值对无序：遍历顺序随机（对应 Python 3.7+ 保持插入顺序，Go 不保证）
- 键类型要求可比较（int/string/struct 等，slice/map 不能做键）
- map 线程不安全，并发写要用 sync.Map（见 sync_test.go）
*/

// 声明与初始化（对应 Python dict 构造）
func TestMapDeclare(t *testing.T) {
	// 方式1：字面量
	m1 := map[string]int{"a": 1, "b": 2}
	fmt.Println("字面量:", m1)

	// 方式2：make（空 map，可写入）
	m2 := make(map[string]int)
	m2["a"] = 1
	fmt.Println("make:", m2)

	// 方式3：var 声明得到 nil map（只读可以，写入会 panic）
	var m3 map[string]int
	fmt.Println("nil map:", m3 == nil, "len:", len(m3)) // true 0
	// m3["a"] = 1 // ❌ 写入 nil map 会 panic: assignment to entry in nil map
}

// 增删改查（对应 Python dict 增删改查）
func TestMapCRUD(t *testing.T) {
	m := make(map[string]int)

	// 增 / 改（都是直接赋值）
	m["apple"] = 5
	m["banana"] = 3
	fmt.Println("增:", m) // map[apple:5 banana:3]

	m["apple"] = 10
	fmt.Println("改:", m) // map[apple:10 banana:3]

	// 查：两个返回值（值 + 是否存在）（对应 Python dict.get / in 判断）
	v, ok := m["apple"]
	fmt.Println("查 apple:", v, ok)    // 10 true
	v2, ok2 := m["orange"]            // 不存在的键
	fmt.Println("查 orange:", v2, ok2) // 0 false（零值 + false）

	// 查单值：不存在时返回零值（无法区分 0 和不存在）
	fmt.Println("单值查不存在:", m["orange"]) // 0

	// 删：delete（对应 Python del dict[key]）
	delete(m, "banana")
	fmt.Println("删:", m) // map[apple:10]

	// 长度
	fmt.Println("len:", len(m)) // 1
}

// 遍历（对应 Python for k,v in dict.items()）
func TestMapRange(t *testing.T) {
	m := map[string]int{"a": 1, "b": 2, "c": 3}

	// 键 + 值（顺序随机）
	for k, v := range m {
		fmt.Println("k =", k, "v =", v)
	}

	// 仅键 / 仅值
	for k := range m {
		fmt.Println("only key:", k)
	}

	// 注意：Go 遍历 map 顺序随机，需要有序输出请先收集 key 再排序（见下方 TestMapHandle 的 slices.Sorted）
}

// map 是引用类型：赋值/传参共享（对应 Python dict 引用）
func TestMapReference(t *testing.T) {
	original := map[string]int{"a": 1}

	copied := original // 共享底层数据
	copied["b"] = 2
	fmt.Println("引用共享:", original) // map[a:1 b:2]

	// 需要独立副本时使用 maps.Clone（Go 1.21+，浅拷贝）
	clone := maps.Clone(original)
	clone["c"] = 3
	fmt.Println("Clone 后原 map 不变:", original) // map[a:1 b:2]

	// 嵌套结构：map 的值为 slice（对应 Python dict 值为 list）
	counts := map[string][]int{
		"偶数": {2, 4, 6},
		"奇数": {1, 3, 5},
	}
	fmt.Println("值为切片:", counts["偶数"]) // [2 4 6]

	// 嵌套 map：二维字典（对应 Python dict of dict）
	matrix := map[string]map[string]int{
		"row1": {"x": 1, "y": 2},
	}
	// 写入嵌套 map 需先初始化内层
	inner, ok := matrix["row1"]
	if !ok {
		inner = make(map[string]int)
		matrix["row1"] = inner
	}
	inner["z"] = 3
	fmt.Println("嵌套 map:", matrix) // map[row1:map[x:1 y:2 z:3]]
}

// 进阶：获取 key 迭代器（maps 包 + slices 包）
func TestMapHandle(t *testing.T) {
	m := map[string]int{"a": 1, "b": 2, "c": 3}

	// 获取所有 key（获取字典的 keys 迭代器 -> 收集为切片 -> 返回新切片）
	keys := slices.Collect(maps.Keys(m))
	fmt.Println(keys) // 结果：[a c b]（顺序随机）

	// 排序
	sortedKeys := slices.Sorted(maps.Keys(m)) // 从小到大（升序）
	fmt.Println(sortedKeys)                   // 结果：[a b c]
}
