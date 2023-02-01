package main

import (
	"fmt"
	"sort"
)

// Array 数组
func Array() {
	// 数组
	// 索引从 0 开始

	// 初始化
	//var array [3]int
	//var array = [3]int{}
	//fmt.Println(array) // 结果: [0 0 0]
	// 初始化并赋值
	//array := [3]int{2, 22, 222}
	array := [...]int{2, 22, 222} // 自动推断数组的长度

	array[0] = 1
	array[1] = 11
	array[2] = 111

	fmt.Println(array)

	// 循环
	//for i, v := range array {
	//	fmt.Println("for array key=", i, "value=", v)
	//}
}

// Slice slice(切片)
func Slice() {

	// 源码文件：runtime/slice.go
	/*
	 * type slice struct {
	 *   array unsafe.Pointer
	 *   len   int
	 *   cap   int
	 * }
	 *
	 *	初始化：makeslice
	 *        math.MulUintptr：根据元素大小和容量cap，计算所需的内存空间
	 *        mallocgc: 分配内存 32K 作为一个临界值，小的分配在 P 的 cache 中，大的分配在 heap 堆中
	 *  扩容：growslice
	 *       当长度小于 1024 时容量翻倍，大于 1024 时增加 1.25 倍(1/4)，但这个并不是绝对的，会根据元素的类型尽心过一定的优化
	 *  拷贝：slicecopy
	 *       核心函数为 memmove，from=>to 移动 size 大小的数据，size 为 元素大小 * from 和 to 中长度较小的个数
	 *  拷贝：slicestringcopy
	 *       基本与上面类似，字符串的拷贝
	 */

	// 切片依赖于数组，它的底层就是数组，所以 slice 具有数组的优点
	// 先创建一个有特定长度和数据类型的底层数组
	// 且 slice 支持可以通过 append 向 slice 中追加元素，长度不够时会动态扩展

	// 声明一个未指定大小的数组来定义切片
	// var arr []int
	// 使用 make() 函数来创建切片与指定长度
	// arr := make([]int, len)
	// 使用 make() 函数来创建切片与指定长度和容量
	// arr := make([]int, len, capacity)

	// var arr []int       // 没有初始化的值为 nil，可以进行写操作
	// var arr []int{}     // 初始化，但长度为 0 、容量为 0 、没有元素
	//arr := make([]int, 0, 3) // 初始化，长度为 0 、容量为 3 、没有元素
	//arr := make([]int, 3, 3) // 初始化，长度为 3 、容量为 3 、每个元素的值为类型的默认值 0

	// slice(切片)
	// 索引从 0 开始

	// 初始化并赋值，长度为 3 、容量为 3
	lists := []int{1, 2, 3}

	// 追加元素
	lists = append(lists, 4)
	fmt.Println("lists=", lists) // 结果： [1 2 3 4]

	// 修改某个 索引下标 的值 (没有这个 索引 时会报错)
	lists[2] = 33

	// 查看元素 (没有这个 索引 时会报错) error panic
	// lists[2]

	// 删除元素 (go没有删除项的函数)

	// 长度(个数),容量
	fmt.Println("lists len：", len(lists), "cap:", cap(lists))

	// 清空元素，但里面的容量不变，改变了长度，注意里面的实际数据不会补清空的
	//lists = lists[:0]
	//fmt.Println(lists, len(lists), cap(lists)) // 结果： [] 0 10

	// 排序，默认 升序
	sort.Ints(lists)
	fmt.Println("lists=", lists) // 结果: [1 2 4 33]
	// 排序，降序
	sort.Sort(sort.Reverse(sort.IntSlice(lists)))
	fmt.Println("lists=", lists) // 结果: [33 4 2 1]

	// 循环
	//for i, v := range lists {
	//	fmt.Println("for lists key=", i, "value=", v)
	//}

	fmt.Println("====================================")

	lists = lists[:0] // 清空元素
	// 切片截取
	// 长度为截取的个数，容量为被截取的切片剩下的容量
	// 可以超出 长度 当不能超出 容量，超出补默认值
	// 下面已经超出长度 2位了
	listsSlice := lists[2:6]
	fmt.Println(listsSlice, len(listsSlice), cap(listsSlice)) // 结果： [2 1 0 0] 4 4
	// 注意：上面的结果第一反应应该是 [0 0 0 0] ，上面已经说到实际的数据是不会清空的

	// 拷贝(复制)
	// 用于将内容从一个数组切片复制到另一个数组切片
	// 如果两个切片长度不一样大，就会按其中较小的那个数组切片的元素个数进行复制

	slice1 := []int{1, 2, 3, 4, 5}
	slice2 := []int{6, 7, 8}

	// 拷贝(复制) slice2 的内容到 slice1
	copy(slice1, slice2)          // 只会复制 slice2 的3个元素到 slice1 的前3个位置
	fmt.Println("slice1", slice1) // 结果： [6 7 8 4 5]

}

// Map map(字典)
func Map() {

	// 源码文件：runtime/map.go
	/*
	 * type hmap struct {
	 *   count     int
	 *   ....
	 * }
	 *
	 *	初始化：makemap
	 *	赋值：mapassign
	 *	扩容：hashGrow
	 *	读取：mapaccess
	 *	删除：mapdelete
	 */

	// map(字典)以 键值对 key:value
	// 获取一个不存在的 KEY 时，会返回 value 对应类型的默认值

	// 注意： var mp map[string]int 是没有初始化的值为nil，不能进行写操作，可以读数据

	maps := map[string]int{"a": 1, "b": 2, "c": 3}
	fmt.Println(maps) // 结果：map[a:1 b:2 c:3]

	maps["e"] = 4     // 不存在元素就新增元素，存在则修改
	fmt.Println(maps) // 结果：map[a:1 b:2 c:3 e:4]

	A := maps["A"] // 获取一个不存在的 KEY 时，会返回 value 对应类型的默认值
	fmt.Println(A) // 结果：0

	delete(maps, "b") // 删除元素，即使删除一个没有的元素也不会报错
	fmt.Println(maps) // 结果：map[a:1 c:3 e:4]

	// 判断 key 是否存在
	b, is := maps["a"]
	if is {
		fmt.Println("maps is a=true", b)
	}

	// 循环
	for k, v := range maps {
		fmt.Println("k", k, "v", v)
	}

	// 清空元素 (go没有清空项的函数)
	// 清空 map 的唯一办法就是重新 make 一个新的 map，不用担心垃圾回收的效率
	// 在 Go 中的并行垃圾回收效率比写一个清空函数要高效的多
}

// ToArray 其他转换
func ToArray() {
	// 分隔字符串  (在 string.go 中)
	// 拼接字符串  (在 string.go 中)

	// 分隔字符串 (从 字符串 分隔为 数字类型数组) (需要自己实现)
	// 拼接字符串 (从 数字类型数组 拼接为 字符串) (需要自己实现)
}
