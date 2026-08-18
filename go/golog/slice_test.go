package main

import (
	"cmp"
	"fmt"
	"slices"
	"testing"
)

/*
切片（slice）基础 + 进阶：对应 Python list / C# List / Rust Vec

slice 是数组的"动态视图"：
- 底层是数组，包含 指针 + 长度 len + 容量 cap 三要素
- 切片是引用类型：赋值/传参共享底层数组（区别于 C++ vector 拷贝 / Java ArrayList 引用）
*/

// 声明与初始化（对应 Python list 构造）
func TestSliceDeclare(t *testing.T) {
	// 方式1：字面量（自动推断长度）
	s1 := []int{1, 2, 3}
	fmt.Println("字面量:", s1)

	// 方式2：make（指定长度，零值填充）
	s2 := make([]int, 3) // [0 0 0]
	fmt.Println("make 长度3:", s2)

	// 方式3：make 带容量（第二个参数长度，第三个参数容量）
	s3 := make([]int, 2, 5) // 长度 2，容量 5
	fmt.Printf("make 长度%d 容量%d: %v\n", len(s3), cap(s3), s3)

	// 方式4：从数组切片（切片是数组的引用）
	arr := [5]int{10, 20, 30, 40, 50}
	s4 := arr[1:4]           // 取索引 1~3（左闭右开）
	fmt.Println("数组切片:", s4) // [20 30 40]
}

// 长度 / 容量 / 索引（对应 Python len / C# Count）
func TestSliceLenCap(t *testing.T) {
	s := make([]int, 3, 5)
	fmt.Println("len:", len(s), "cap:", cap(s)) // 3 5

	// 空切片与 nil 切片（都是 len=0，nil 切片 == nil 为 true）
	var nilSlice []int    // nil 切片
	emptySlice := []int{} // 空切片
	fmt.Println("nil:", nilSlice == nil, "空:", emptySlice == nil)
}

// 切片操作：追加 / 拷贝（对应 Python append + copy / C# List.Add + CopyTo）
func TestSliceAppendCopy(t *testing.T) {
	// append：追加元素，容量不足时自动扩容（底层新数组，可能复制）
	s := []int{1, 2, 3}
	s = append(s, 4, 5)          // 追加多个
	fmt.Println("append 多值:", s) // [1 2 3 4 5]

	// append 展开另一个切片（...）
	more := []int{6, 7}
	s = append(s, more...)
	fmt.Println("append 展开:", s) // [1 2 3 4 5 6 7]

	// 扩容演示：cap 翻倍增长（对应 Python list 扩容机制）
	growth := make([]int, 0, 1)
	for i := 0; i < 5; i++ {
		growth = append(growth, i)
		fmt.Printf("len=%d cap=%d\n", len(growth), cap(growth))
	}

	// copy：复制切片（目标长度决定复制个数，多余截断）
	a := []int{1, 2, 3}
	b := make([]int, len(a))
	n := copy(b, a)
	fmt.Println("copy:", n, b) // 3 [1 2 3]
}

// 引用语义：切片赋值/传参共享底层数组（对应 Python 列表引用 / Java 数组引用）
func TestSliceReference(t *testing.T) {
	// 切片切片：多个切片共享底层数组
	base := []int{1, 2, 3, 4, 5}
	sub1 := base[0:3] // [1 2 3]
	sub2 := base[2:5] // [3 4 5]

	sub1[1] = 99               // 修改共享数组
	fmt.Println("base:", base) // [1 99 3 4 5]
	fmt.Println("sub2:", sub2) // [3 4 5]（能看到 sub1 的修改）

	// 传参也是共享底层数组（区别于 C++ 传值拷贝）
	modifyFirst := func(s []int) {
		s[0] = 1000
	}
	nums := []int{1, 2, 3}
	modifyFirst(nums)
	fmt.Println("传参修改:", nums) // [1000 2 3]

	// append 扩容前后的共享关系变化（确定性演示）
	nums2 := []int{1, 2, 3}
	nums3 := nums2[:2]        // len=2 cap=3，与 nums2 共享底层数组
	nums3 = append(nums3, 99) // 未超出容量，仍共享
	nums2[0] = 777
	fmt.Println("未扩容时共享:", nums3[0] == 777) // true

	nums3 = append(nums3, 88) // 超出容量 → 重新分配新数组，不再共享
	nums2[0] = 555
	fmt.Println("扩容后不再共享:", nums3[0]) // 仍是 777（旧数组里的值）
}

// 进阶：切片处理（slices 包）
func TestSliceHandle(t *testing.T) {
	// 查找
	fmt.Println(slices.Contains([]int{1, 2, 3}, 2))                // 结果：true
	fmt.Println(slices.Contains([]string{"切片", "查找", "处理"}, "查找")) // 结果：true

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

	// 结构体数组

	type Person struct {
		name string
		age  int
	}
	people := []Person{
		{name: "Jax", age: 36},
		{name: "TJ", age: 26},
		{name: "Alex", age: 76},
	}

	// 按年龄升序，年龄相同按姓名升序（从小到大）
	sortByAge := func(a, b Person) int {
		ageCmp := cmp.Compare(a.age, b.age) // 交换参数顺序，实现降序（从大到小）
		if ageCmp != 0 {
			return ageCmp
		}
		return cmp.Compare(a.name, b.name) // 年龄相同则比较姓名
	}
	slices.SortFunc(people, sortByAge)
	fmt.Println(people) // 结果：[{TJ 26} {Jax 36} {Alex 76}]
}
