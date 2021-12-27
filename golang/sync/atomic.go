package sync_main

import (
	"fmt"
	"sync"
	"sync/atomic"
)

// 原子锁操作
// 支持的数据类型包括：int32, int64, uint32, uint64, uintptr, unsafe.Pointer
//
// 若需要扩大原子操作的适用范围，可以使用 atomic 包中的 Value，利用它可以实现对任意值进行原子得存储与加载
// atomic.Value 只有两个指针方法：Store、Load
// 使用时需要遵循两个原则： 1.不能存储 nil，会 panic
//                     2.存储第一个值后，就只能存储这个类型的值，否则会 panic
// 最好不要使用 atomic.Value 存储引用类型的值，可能导致数据不是并发安全的

// AtomicAdd 自增
func AtomicAdd() {
	var data int64

	wg := &sync.WaitGroup{}
	wg.Add(10000)

	for i := 0; i < 10000; i++ {

		go func() {
			// data++ // 在无锁的情况下进行自增最终的结果的错误的
			atomic.AddInt64(&data, 1)
			wg.Done()
		}()

	}

	wg.Wait()
	fmt.Println("data:", data)
}
