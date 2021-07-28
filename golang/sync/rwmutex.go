package sync_main

import (
	"fmt"
	"sync"
	"time"
)

// RWMutex 读写锁
// 写请求在读锁和写锁时都必须阻塞等待，读请求只在写锁时阻塞等待
func RWMutex() {

	rwMutex := sync.RWMutex{}

	for i := 0; i < 5; i++ {

		go func(idx int) {
			rwMutex.RLock()         // 写锁定
			defer rwMutex.RUnlock() // 写解锁

			fmt.Println("Read Mutex :", idx)
		}(i)

		go func(idx int) {
			rwMutex.Lock()         // 读锁定
			defer rwMutex.Unlock() // 读解锁

			fmt.Println("Write Mutex :", idx)
			time.Sleep(time.Second)
		}(i)

	}

	time.Sleep(6 * time.Second)
	fmt.Println("Func finish.")
}
