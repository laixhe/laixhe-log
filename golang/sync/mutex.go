package sync_main

import (
	"fmt"
	"sync"
	"time"
)

// Mutex 互斥锁
// 多个协程同时运行，获得 Mutex 锁者协程优先执行，其余协程阻塞等待
func Mutex() {

	mutex := sync.Mutex{}

	for i := 0; i < 5; i++ {

		go func(idx int) {

			mutex.Lock()         // 锁定
			defer mutex.Unlock() // 解锁

			fmt.Println("idx :=", idx)

			//睡眠 1秒
			time.Sleep(time.Second)

		}(i)

	}

	//睡眠 6 秒
	time.Sleep(6 * time.Second)
	fmt.Println("Func finish.")
}
