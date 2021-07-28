package sync_main

import (
	"fmt"
	"sync"
	"time"
)

// Cond 条件等待
// 条件等待通过 Wait 让例程等待，通过 Signal 让一个等待的例程继续，
// 通过 Broadcast 让所有等待的例程继续
// 适合应用于 多收一发 的场景，也是多个 goroutine 同时都能触发到

// Cond 条件等待-单播通知
func Cond() {

	cond := sync.NewCond(&sync.Mutex{})

	// 在主协程上先获得锁
	cond.L.Lock() // 1、上锁

	go func() {

		// 协程一开始无法获得锁(阻塞的)
		cond.L.Lock()         // 2、等 Wait 解锁
		defer cond.L.Unlock() // 4.1、解锁后触发 Wait

		fmt.Println("go locked. 2")

		time.Sleep(time.Second)

		cond.Signal() // 4、触发 Wait 等待解锁(单播)

		fmt.Println("go unlock. 3")

	}()

	time.Sleep(time.Second)
	fmt.Println("start wait. 1")

	// 3、可以理解为立刻解锁并触发一个阻塞线程
	// （如果没有阻塞线程则不触发）后立刻再上锁等待 Signal 信号
	cond.Wait()

	fmt.Println("wait finish. 4")

	time.Sleep(time.Second)
	cond.L.Unlock() // 5、解锁后触发 Wait
	fmt.Println("Func finish. 5")
}

// CondBroadcast 条件等待-广播通知
func CondBroadcast() {

	cond := sync.NewCond(&sync.Mutex{})

	for i := 0; i < 5; i++ {
		go func(ii int, c *sync.Cond) {

			fmt.Printf("No.#%d\n", ii)
			c.L.Lock()
			fmt.Printf("No.#%d lock\n", ii)

			c.Wait()
			fmt.Printf("No.#%d wait\n", ii)

			c.L.Unlock()

		}(i, cond)
	}

	time.Sleep(2 * time.Second)

	// 广播
	cond.Broadcast()

	time.Sleep(time.Second)
}
