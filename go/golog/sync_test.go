package main

import (
	"fmt"
	"sync"
	"sync/atomic"
	"testing"
)

// sync.Map     字典线程安全
// sync.Mutex   互斥锁
// sync.RWMutex 读写锁（读并发、写互斥）
// sync.Once    只执行一次
// sync.WaitGroup 等待一组 goroutine 完成
// sync/atomic  原子操作（无锁的并发安全计数，适合简单场景）

// 单例模式：sync.Once 保证函数只执行一次（线程安全）
func TestSyncOnce(t *testing.T) {
	var once sync.Once
	var count int

	// 即使循环多次调用，Do 中的函数也只会执行一次
	for i := 0; i < 5; i++ {
		once.Do(func() {
			count++
			fmt.Println("初始化执行（只会执行一次）")
		})
	}
	fmt.Println("count =", count) // 结果: count = 1
}

// OnceValue：带返回值的「只执行一次」（Go 1.21+）
func TestSyncOnceValue(t *testing.T) {
	var getValue = sync.OnceValue(func() string {
		return "laixhe"
	})

	fmt.Println(getValue()) // 结果: laixhe
	fmt.Println(getValue()) // 结果: laixhe（后续直接返回缓存值）
}

// WaitGroup：等待一组 goroutine 完成
func TestSyncWaitGroup(t *testing.T) {
	var wg sync.WaitGroup

	for i := 0; i < 5; i++ {
		wg.Add(1) // 每启动一个 goroutine，计数 +1
		go func(n int) {
			defer wg.Done() // goroutine 结束时计数 -1
			fmt.Println("goroutine", n)
		}(i)
	}

	wg.Wait() // 阻塞直到计数归零
	fmt.Println("所有 goroutine 完成")
}

// Mutex：互斥锁，保护共享变量
func TestSyncMutex(t *testing.T) {
	var mu sync.Mutex
	var count int

	var wg sync.WaitGroup
	for i := 0; i < 1000; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			mu.Lock()   // 加锁
			count++     // 临界区：同一时刻只有一个 goroutine 能执行
			mu.Unlock() // 解锁
		}()
	}
	wg.Wait()
	fmt.Println("count =", count) // 结果: count = 1000（不加锁会得到错误结果）
}

// RWMutex：读写锁，读可并发、写互斥
func TestSyncRWMutex(t *testing.T) {
	var rw sync.RWMutex
	data := "初始值"

	// 写锁：独占，与所有读/写互斥
	rw.Lock()
	data = "新值"
	rw.Unlock()

	// 读锁：多个 goroutine 可同时持有
	rw.RLock()
	fmt.Println("读取:", data) // 结果: 读取: 新值
	rw.RUnlock()
}

// atomic：无锁的原子操作，比 Mutex 更轻量（适合简单计数）
func TestSyncAtomic(t *testing.T) {
	var count atomic.Int64

	var wg sync.WaitGroup
	for i := 0; i < 1000; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			count.Add(1) // 原子自增，无需加锁
		}()
	}
	wg.Wait()
	fmt.Println("count =", count.Load()) // 结果: count = 1000
}
