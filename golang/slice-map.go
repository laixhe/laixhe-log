package main

import "fmt"

// SliceMap slice(切片)、map(字典)
func SliceMap() {

	// slice(切片)的索引从 0 开始
	lists := []string{"a", "b", "c"}
	fmt.Println(lists)         // 结果： [a b c]
	lists = append(lists, "e") // 追加元素
	fmt.Println(lists)         // 结果： [a b c e]

	// map(字典)以 键值对 key:value
	maps := map[string]int{"a": 1, "b": 2, "c": 3}
	fmt.Println(maps) // 结果：map[a:1 b:2 c:3]
	maps["e"] = 4     // 不存在元素就新增元素，存在则修改
	fmt.Println(maps) // 结果：map[a:1 b:2 c:3 e:4]
	delete(maps, "b") // 删除元素，即使删除一个没有的元素也不会报错
	fmt.Println(maps) // 结果：map[a:1 c:3 e:4]

}
