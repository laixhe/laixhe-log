package main

import "fmt"

// Slice slice(切片)
func Slice() {

	// 切片依赖于数组，它的底层就是数组，所以 slice 具有数组的优点
	// 先创建一个有特定长度和数据类型的底层数组
	// 且 slice 支持可以通过 append 向 slice 中追加元素，长度不够时会动态扩展

	// slice 的索引从 0 开始
	// 声明一个未指定大小的数组来定义切片
	// var arr []int
	// 使用 make() 函数来创建切片与指定长度
	// arr := make([]int, len)
	// 使用 make() 函数来创建切片与指定长度和容量
	// arr := make([]int, len, capacity)

	// 注意： var arr []int   是没有初始化的值为 nil，可以进行写操作
	//       var arr []int{} 初始化
	lists := make([]int, 0, 10)

	// 追加元素
	lists = append(lists, 1)
	lists = append(lists, 2)
	lists = append(lists, 3)
	lists = append(lists, 4)
	fmt.Println(lists) // 结果： [1 2 3 4]

	// 清空元素，但里面的容量不变，改变了长度，注意里面的实际数据不会补清空的
	lists = lists[:0]
	fmt.Println(lists, len(lists), cap(lists)) // 结果： [] 0 10

	// 不能获取超出长度
	// lists[11] // error panic

	// 切片截取
	// 长度为截取的个数，容量为被截取的切片剩下的空量
	// 可以超出 长度 当不能超出 容量，超出补默认值
	// 下面已经超出长度 2位了
	lists_1 := lists[2:6]
	fmt.Println(lists_1, len(lists_1), cap(lists_1)) // 结果： [3 4 0 0] 4 8
	// 注意：上面的结果第一反应应该是 [0 0 0 0] ，上面已经说到实际的数据是不会清空的

	// 拷贝(复制)
	// 用于将内容从一个数组切片复制到另一个数组切片
	// 如果加入的两个数组切片不一样大，就会按其中较小的那个数组切片的元素个数进行复制

	slice1 := []int{1, 2, 3, 4, 5}
	slice2 := []int{6, 7, 8}

	// 拷贝(复制) slice1 的内容到 slice2
	copy(slice1, slice2)          // 只会复制 slice2 的3个元素到 slice1 的前3个位置
	fmt.Println("slice1", slice1) // 结果： [6 7 8 4 5]
}

// Map map(字典)
func Map() {
	// map(字典)以 键值对 key:value
	// 获取一个不存在的 KEY 时，会返回 value 对应类型的默认值

	// 注意： var mp map[string]int 是没有初始化的值为nil，不能进行写操作
	maps := map[string]int{"a": 1, "b": 2, "c": 3}
	fmt.Println(maps) // 结果：map[a:1 b:2 c:3]

	maps["e"] = 4     // 不存在元素就新增元素，存在则修改
	fmt.Println(maps) // 结果：map[a:1 b:2 c:3 e:4]

	delete(maps, "b") // 删除元素，即使删除一个没有的元素也不会报错
	fmt.Println(maps) // 结果：map[a:1 c:3 e:4]

	A := maps["A"] // 获取一个不存在的 KEY 时，会返回 value 对应类型的默认值
	fmt.Println(A) // 结果：0
}
