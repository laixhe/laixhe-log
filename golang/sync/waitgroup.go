package sync_main

import (
	"fmt"
	"sync"
)

// 组等待
// Add 增加等待计数
// Done 减少等待计数
// 当计数为 0 时触发 Wait
// 用于等待一组例程的结束。主例程在创建每个子例程的时候先调用 Add 增加等待计数，
// 每个子例程在结束时调用 Done 减少例程计数。
// 之后，主例程通过 Wait 方法开始等待，直到计数器归零才继续执行
func WaitGroup() {

	//创建任务组
	wg := sync.WaitGroup{}
	//添加任务数(设置等待任务数)
	wg.Add(2)

	for i := 0; i < 2; i++ {

		go func() {

			fmt.Println("完成")

			//每完成一个任务，删除一个（标记一个）
			wg.Done()
		}()

	}

	//等待所有任务数都完成
	wg.Wait()
	fmt.Println("所有任务数都完成")
}
