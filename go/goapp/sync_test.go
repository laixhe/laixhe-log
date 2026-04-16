package main

import (
	"testing"
)

// sync.Map     字典线程安全
// sync.Mutex   互斥锁
// sync.RWMutex 读写锁
// sync.Pool    对象池
// sync.Cond    条件等待锁
// sync/atomic  原子锁操作（不能存储 nil，会 panic）（存储第一个值后，就只能存储这个类型的值，否则会 panic）

// 单例模式
// 只执行一次以后不再触发
// 线程安全
func TestSyncOnce(t *testing.T) {
	// 不带返回值
	// sync.Once
	//          .Do

	// 带返回值
	/*
		var GetConfig = sync.OnceValue(func() *Config {
			return loadConfig()
		})
		cfg := GetConfig()
	*/
}

func TestSyncWaitGroup(t *testing.T) {
	// WaitGroup 组等待
	// Add  增加等待计数
	// Done 减少等待计数
	// GO   动处理 Add(1) 和内部的 Done()，连闭包变量捕获问题都不用再担心
	// Wait 等待所有任务数都完成
	//
	// 当计数为 0 时触发 Wait
	// 用于等待一组例程的结束。主例程在创建每个子例程的时候先调用 Add 增加等待计数，
	// 每个子例程在结束时调用 Done 减少例程计数。
	// 之后，主例程通过 Wait 方法开始等待，直到计数器归零才继续执行
}
