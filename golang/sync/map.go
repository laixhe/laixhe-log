package sync_main

import (
	"fmt"
	"sync"
)

func Map() {

	var m = new(sync.Map)
	// 存
	m.Store("a", 1)
	// 取
	m.Load("a")
	// 有这个Key就取，没有这个Key就存
	m.LoadOrStore("b", 2)
	// 删
	m.Delete("c")
	// 遍历
	m.Range(func(key, value interface{}) bool {
		fmt.Println(key, value)
		// 返回 true 就继续下次迭代循环
		return true
	})

}
