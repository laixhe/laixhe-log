package sync_main

import (
	"fmt"
	"sync"
	"time"
)

// 条件等待
// 条件等待通过 Wait 让例程等待，通过 Signal 让一个等待的例程继续，
// 通过 Broadcast 让所有等待的例程继续
func Cond() {

	cond := sync.NewCond(&sync.Mutex{})

	cond.L.Lock()         // 1、上锁
	defer cond.L.Unlock() // 解锁

	go func() {

		cond.L.Lock()         // 2、等 Wait 解锁
		defer cond.L.Unlock() // 5、解锁后触发 Wait

		fmt.Println("go locked. 2")

		time.Sleep(time.Second)

		cond.Signal() // 4、触发 Wait 等待解锁

		fmt.Println("go unlock. 3")

	}()

	time.Sleep(time.Second)
	fmt.Println("start wait. 1")

	// 3、可以理解为立刻解锁并触发一个阻塞线程
	// （如果没有阻塞线程则不触发）后立刻再上锁等待 Signal 信号
	cond.Wait()

	fmt.Println("wait finish. 4")

	time.Sleep(time.Second)
	fmt.Println("Func finish. 5")
}
