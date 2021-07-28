package sync_main

import (
	"fmt"
	"sync"
)

// WaitGroup 组等待
// Add 增加等待计数
// Done 减少等待计数
// 当计数为 0 时触发 Wait
// 用于等待一组例程的结束。主例程在创建每个子例程的时候先调用 Add 增加等待计数，
// 每个子例程在结束时调用 Done 减少例程计数。
// 之后，主例程通过 Wait 方法开始等待，直到计数器归零才继续执行
func WaitGroup() {

	// Add与Done应该放在哪？
	// Add放在Goroutine外，Done放在Goroutine中，逻辑复杂时建议用defer保证调用

	n := 16
	//创建任务组
	wg := &sync.WaitGroup{}
	//添加任务数(设置等待任务数)
	wg.Add(n)

	for i := 0; i < n; i++ {

		go func(j int, w *sync.WaitGroup) {

			fmt.Println("完成", j)

			//每完成一个任务，删除一个（标记一个）
			w.Done()
		}(i, wg)

	}

	//等待所有任务数都完成
	wg.Wait()
	fmt.Println("所有任务数都完成")
}
