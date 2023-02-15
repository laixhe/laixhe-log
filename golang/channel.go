package main

import (
	"fmt"
	"time"
)

// channel(通道)
// 通道(channel)是用来传递数据的一个数据结构
// 默认情况下，通道是不带缓冲区的。发送端发送数据，同时必须有接收端相应的接收数据。
// 通道可以设置缓冲区，通过 make 的第二个参数指定缓冲区大小。
// 带缓冲区的通道允许发送端的数据发送和接收端的数据获取处于异步状态，
//     就是说发送端发送的数据可以放在缓冲区里面，可以等待接收端去获取数据，
//     而不是立刻需要接收端去获取数据。 不过由于缓冲区的大小是有限的，
//     所以还是必须有接收端来接收数据的，否则缓冲区一满，数据发送端就无法再发送数据了。
//
// 简单的说：
// 通道不带缓冲，发送方会阻塞直到接收方从通道中接收了值(阻塞的)。
// 通道带缓冲，发送方则会阻塞直到发送的值被拷贝到缓冲区内(异步的)，
//     如果缓冲区已满，则意味着需要等待直到某个接收方获取到一个值(阻塞的)
//
// 适合应用于 一收一发 的场景，也就是多个 goroutine 同时只能触发一个接收，但是都可以触发到关闭通知
//
// 只有声明没有初始化( var xxx chan int )是不能进行读写和关闭的，一个 nil 的 channel 读写数据都会造成永远阻塞

// ChanInt 不带缓冲区
func ChanInt() {
	// 在默认情况下，通道是不带缓冲区的。发送端发送数据，同时必须有接收端相应的接收数据
	var chanInt = make(chan int)

	go func(c chan int) {
		// 由于 channel 没有设置长度， 所以是阻塞的，会逐个发送
		c <- 1
		c <- 2
		c <- 3
		fmt.Println("send finished...")
	}(chanInt)

	after := time.After(time.Second)

BreakFor: // 这里会跳出下面的 for 循环
	for {
		select {
		case i := <-chanInt:
			fmt.Println("chanInt", i)
		case <-after:
			fmt.Println("time out...")
			break BreakFor
		}
	}

	fmt.Println("程序退出...")
}

// ChanIntSend 不带缓冲区，但限定了发送
func ChanIntSend() {
	var chanInt = make(chan int)

	go func(c chan<- int) {
		// 当参数设置为 chan<- (只能用于发送) 和 <-chan (只能用于接收)
		c <- 1
		c <- 2
		c <- 3
		fmt.Println("send finished...")
	}(chanInt)

	after := time.After(time.Second)

BreakFor:
	for {
		select {
		case i := <-chanInt:
			fmt.Println("chanInt", i)
		case <-after:
			fmt.Println("time out...")
			break BreakFor
		}
	}

	fmt.Println("程序退出...")
}

// ChanIntClose 不带缓冲区，主动关闭chan
func ChanIntClose() {
	var chanInt = make(chan int)

	go func(c chan<- int) {
		c <- 1
		c <- 2
		c <- 3
		close(c) // 主动关闭chan
		fmt.Println("send finished...")
	}(chanInt)

	after := time.After(time.Second)

BreakFor:
	for {
		select {
		case i, ok := <-chanInt:
			// 如果这里不取出第二个值，那么 i 就会一直得到 chan 类型的默认值，如 int 为 0，永远不会停止
			if ok {
				fmt.Println("chanInt", i)
			} else {
				fmt.Println("channel close...")
				break BreakFor
			}
		case <-after:
			fmt.Println("time out...")
			break BreakFor
		}
	}

	fmt.Println("程序退出...")
}

// ChanTask 不带缓冲区，多个 chan 任务
func ChanTask() {
	upperCh := make(chan bool)
	lowerCh := make(chan bool)

	go func(u chan<- bool, l chan<- bool) {
		if time.Now().Unix()%2 == 0 {
			u <- true
		} else {
			l <- true
		}
	}(upperCh, lowerCh)

	after := time.After(time.Second)

	select {
	case <-upperCh:
		fmt.Println("upper...")
	case <-lowerCh:
		fmt.Println("lower...")
	case <-after:
		fmt.Println("time out...")
	}

	fmt.Println("程序退出...")
}

// ChanBuffer 带缓冲区
func ChanBuffer() {
	var chanInt = make(chan int, 3)

	go func(c chan int) {
		// 由于 channel 设置长度，相当于一个消息队列，在缓冲区没满前是异步的
		// 所以在这里是不会阻塞的
		c <- 1
		c <- 2
		c <- 3
		fmt.Println("send finished...")
	}(chanInt)

	after := time.After(time.Second)

BreakFor: // 这里会跳出下面的 for 循环
	for {
		select {
		case i := <-chanInt:
			fmt.Println("chanInt", i)
		case <-after:
			fmt.Println("time out...")
			break BreakFor
		}
	}

	fmt.Println("程序退出...")
}

// ChanBufferRange 带缓冲区 for range
func ChanBufferRange() {
	var chanInt = make(chan int, 3)

	go func(c chan int) {
		c <- 1
		c <- 2
		c <- 3

		fmt.Println("send finished...")
		time.Sleep(time.Second * 3)
		close(c) // 主动关闭 chan，如果不关闭下面的 for range 是不会退出的
		fmt.Println("channel close...")
	}(chanInt)

	// 一直读取到 chan 主动关闭为止
	for i := range chanInt {
		fmt.Println("chanInt", i)
	}

	fmt.Println("程序退出...")
}
