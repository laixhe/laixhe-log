package main

import (
	"fmt"
	"testing"
	"time"
)

/*
goroutine + channel 并发基础（Go 并发模型的核心）
对应 TS async.test.ts（Promise/async-await）/ Python concurrency.py / Java 线程

Go 并发哲学："不要通过共享内存来通信，而要通过通信来共享内存"
- goroutine：轻量级线程（几 KB 栈，可并发数十万）
- channel：goroutine 之间通信的管道（对应 Python queue / Rust channel）
- 与 sync_test.go 互补：sync 包管"锁"，channel 管"通信"
*/

// goroutine 基本用法：go 关键字启动（对应 TS 开 Promise / Java 开线程）
func TestGoroutine(t *testing.T) {
	done := make(chan bool) // 无缓冲 channel

	go func() {
		fmt.Println("goroutine 运行中")
		done <- true // 发送完成信号
	}()

	fmt.Println("主 goroutine 继续")
	<-done // 接收信号（阻塞等待 goroutine 完成）
}

// channel 基础：发送 / 接收（无缓冲，必须同时就绪）
func TestChannel(t *testing.T) {
	ch := make(chan int) // 无缓冲 channel：发送方阻塞直到接收方就绪

	go func() {
		ch <- 42 // 发送
	}()

	v := <-ch // 接收
	fmt.Println("收到:", v) // 42
}

// 缓冲 channel：带容量，发送方在未满时不会阻塞
func TestChannelBuffered(t *testing.T) {
	ch := make(chan int, 3) // 容量 3

	ch <- 1
	ch <- 2
	ch <- 3
	// ch <- 4 // ❌ 缓冲已满，发送会阻塞（在无接收方时会死锁）
	fmt.Println("缓冲长度:", len(ch), "容量:", cap(ch))

	fmt.Println("依次接收:", <-ch, <-ch, <-ch) // 1 2 3
}

// 关闭 channel + range 遍历（对应 Python for 遍历队列）
func TestChannelClose(t *testing.T) {
	ch := make(chan int, 5)
	for i := 1; i <= 3; i++ {
		ch <- i
	}
	close(ch) // 关闭后不能再发送，但可以继续接收完剩余数据

	for v := range ch { // range 自动遍历到关闭为止
		fmt.Println("收到:", v)
	}
	// 判断 channel 是否已关闭
	_, ok := <-ch // 已关闭且无数据 → ok == false
	fmt.Println("channel 已关闭:", !ok) // true
}

// select：多路复用，同时等待多个 channel（对应 TS Promise.race / Java NIO）
func TestChannelSelect(t *testing.T) {
	ch1 := make(chan string, 1)
	ch2 := make(chan string, 1)
	ch1 <- "消息1"

	// select 从就绪的 channel 中随机选一个执行（对应 Go 版 switch）
	select {
	case msg := <-ch1:
		fmt.Println("ch1:", msg)
	case msg := <-ch2:
		fmt.Println("ch2:", msg)
	default: // 没有就绪时走 default（不阻塞）
		fmt.Println("无消息")
	}

	// select 配合超时（对应 TS Promise.race 超时）
	timeout := time.After(10 * time.Millisecond)
	select {
	case <-timeout:
		fmt.Println("等待超时")
	}
}

// channel 作为参数：只读 / 只写限制（对应 TS 泛型约束 / Java 泛型通配）
func TestChannelDirection(t *testing.T) {
	// 生产者：只写 channel（chan<-）
	producer := func(out chan<- int) {
		for i := 1; i <= 3; i++ {
			out <- i
		}
		close(out)
	}
	// 消费者：只读 channel（<-chan）
	consumer := func(in <-chan int) {
		for v := range in {
			fmt.Println("消费:", v)
		}
	}

	ch := make(chan int)
	go producer(ch)
	consumer(ch)
}

// 综合实战：并发求和（分多个 goroutine 计算结果，channel 汇总）
func TestGoroutineSum(t *testing.T) {
	nums := []int{1, 2, 3, 4, 5, 6, 7, 8, 9, 10}
	const workers = 2 // 2 个 worker 并行计算

	results := make(chan int, workers)
	chunk := len(nums) / workers

	for w := 0; w < workers; w++ {
		go func(start, end int) {
			sum := 0
			for _, n := range nums[start:end] {
				sum += n
			}
			results <- sum
		}(w*chunk, (w+1)*chunk)
	}

	total := 0
	for i := 0; i < workers; i++ {
		total += <-results
	}
	fmt.Println("并发求和:", total) // 55
}
