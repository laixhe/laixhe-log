package sync_main

import (
	"fmt"
	"sync"
	"time"
)

// 只执行一次以后不再触发
// 可以用于单例模式
func Once() {

	once := sync.Once{}

	for i := 0; i < 5; i++ {

		go func(idx int) {

			once.Do(func() {
				// 这里只执行一次
				fmt.Println("Do once : ", idx)
			})

			fmt.Println("go : ", idx)
		}(i)
	}

	time.Sleep(2 * time.Second)
	fmt.Println("Func finish.")
}
