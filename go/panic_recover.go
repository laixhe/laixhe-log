package main

import (
	"fmt"
	"time"
)

/**
>
> panic 不应跨域 package
>
> recover 只能在 defer 中起作用
>
> panic 只能由当前 goroutine recover
>
> 如果 panic 没有被 recover，那么有可能会挂起整个程序(panic 并没有被解决，一直抛到最上层，使整个程序崩溃)
>
 */

// 一个协程出现了panic，如果没有捕获这个协程，就会造成程序的崩溃。
// 可以在 goroutine 中使用 recover 来捕获 panic 不会影响其它线程

// PanicHello 正常输出
func PanicHello() {

	for i := 0; i < 3; i++ {
		fmt.Println("Hello i=", i)
		time.Sleep(time.Second)
	}

}

// PanicError 有错误的
func PanicError() {

	// 使用 defer + recover
	defer func() {

		// 捕获 test 抛出的 panic
		if err := recover(); err != nil {
			fmt.Printf("Panic 发生错误: %v\n", err)
		}
	}()
	time.Sleep(time.Second)
	var data map[int]string
	data[0] = "golang" // error

}

// PanicRecover 执行
func PanicRecover() {

	go PanicHello()
	go PanicError()

	time.Sleep(5 * time.Second)
}
