package panic_recover

import (
	"fmt"
	"time"
)

// 一个协程出现了panic，如果没有捕获这个协程，就会造成程序的崩溃。
// 可以在 goroutine 中使用 recover 来捕获 panic 不会影响其它线程

// 正常输出
func Hello() {

	for i := 0; i < 5; i++ {
		time.Sleep(time.Second)
		fmt.Println("Hello i=", i)
	}

}

// Panic 有错误的
func Panic() {

	// 使用 defer + recover
	defer func() {

		// 捕获 test 抛出的 panic
		if err := recover(); err != nil {
			fmt.Printf("Panic 发生错误: %v\n", err)
		}
	}()

	var data map[int]string
	data[0] = "golang" // error

}

// PanicRecover 执行
func PanicRecover() {

	go Hello()
	go Panic()

	time.Sleep(10 * time.Second)
}
