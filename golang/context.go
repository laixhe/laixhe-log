package main

import (
	"context"
	"fmt"
	"time"
)

// CtxCancel 通过 cancel 主动关闭
func CtxCancel() {

	c, cancel := context.WithCancel(context.Background())

	go func(cc context.Context) {
		// 单次定时器
		after := time.After(time.Second * 3)

		for {
			select {
			case <-cc.Done():
				fmt.Println("收到关闭通知:", cc.Err())
				return
			case <-after:
				fmt.Println("定时器...")
				return
			}
		}

	}(c)

	time.Sleep(time.Second)
	cancel() // 主动关闭

}

// CtxTimeout 通过超时，自动触发
func CtxTimeout() {
	c, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel() // 主动执行 cancel 也会让协程收到消息

	go func(cc context.Context) {
		// 单次定时器
		after := time.After(time.Second * 3)

		for {
			select {
			case <-cc.Done():
				fmt.Println("收到关闭通知1:", cc.Err())
				return
			case <-after:
				fmt.Println("定时器1...")
				return
			}
		}

	}(c)

	go func(cc context.Context) {
		// 单次定时器
		after := time.After(time.Second * 2)

		for {
			select {
			case <-cc.Done():
				fmt.Println("收到关闭通知2:", cc.Err())
				return
			case <-after:
				fmt.Println("定时器2...")
				return
			}
		}

	}(c)

	time.Sleep(3 * time.Second)
}

// CtxDeadline 通过设置截止时间，触发超时
func CtxDeadline() {
	c, cancel := context.WithDeadline(context.Background(), time.Now().Add(2*time.Second)) // 也就是 2秒后 发出通知

	defer cancel() // 主动执行 cancel 也会让协程收到消息

	go func(cc context.Context) {
		// 单次定时器
		after := time.After(time.Second * 2)

		for {
			select {
			case <-cc.Done():
				fmt.Println("收到关闭通知:", cc.Err())
				return
			case <-after:
				fmt.Println("定时器...")
				return
			}
		}

	}(c)

	time.Sleep(3 * time.Second)
}

// CtxValue 用 key/value 传递参数，可以浅浅封装一层，转化为自已想要的结构体
func CtxValue() {
	c := context.WithValue(context.Background(), "user_name", "laixhe")
	c = context.WithValue(c, "user_age", 18)

	go func(cc context.Context) {
		userName, ok := cc.Value("user_name").(string)
		if ok {
			fmt.Println("user_name=", userName)
		}

		userAge, ok := cc.Value("user_age").(int)
		if ok {
			fmt.Println("user_age=", userAge)
		}
	}(c)

	time.Sleep(3 * time.Second)
}
