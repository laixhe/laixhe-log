package main

import (
	"errors"
	"fmt"
)

// 数组模拟队列

// 使用一个结构管理队列
type ArrayQueue struct {
	maxSize int    // 队列大小，默认值：5
	array   [5]int // 数组=>模拟列，默认值：5
	front   int    // 表示指向队列首部
	rear    int    // 表示指向队列尾部
}

func NewArrayQueue() {

	queue := &ArrayQueue{
		maxSize: 5,
		front:   0,
		rear:    0,
	}

	var key string
	var val int

	for {
		fmt.Println("1. 输入 add 表示添加数据到队列")
		fmt.Println("2. 输入 get 表示获取数据到队列")
		fmt.Println("3. 输入 show 表示显示队列")
		fmt.Println("4. 输入 exit 表示退出队列")

		fmt.Scanln(&key)
		switch key {
		case "add":

			fmt.Println("输入你要入队列的数")
			fmt.Scanln(&val)
			err := queue.addQueue(val)
			if err != nil {
				fmt.Println(err)
			} else {
				fmt.Println("加入队列ok")
			}

		case "get":
			value, err := queue.getQueue()
			if err != nil {
				fmt.Println(err)
			} else {
				fmt.Println("从队列中取出数据: ", value)
			}

		case "show":
			queue.showQueue()
		case "exit":
			return
		}

	}

}

// 添加数据到队列
func (this *ArrayQueue) addQueue(value int) error {

	if this.isFull() {
		return errors.New("queue full")
	}

	this.array[this.rear] = value
	this.rear = (this.rear + 1) % this.maxSize

	return nil
}

// 从队列中取出数据
func (this *ArrayQueue) getQueue() (int, error) {

	if this.isEmpty() {
		return 0, errors.New("queue empty")
	}

	value := this.array[this.front]
	this.front = (this.front + 1) % this.maxSize

	return value, nil
}

// 判断队列是否满了
func (this *ArrayQueue) isFull() bool {
	return (this.rear+1)%this.maxSize == this.front
}

// 判断队列是否为空
func (this *ArrayQueue) isEmpty() bool {
	return this.rear == this.front
}

// 获取队列有多少个元素
func (this *ArrayQueue) size() int {
	return (this.rear + this.maxSize - this.front) % this.maxSize
}

// 显示队列
func (this *ArrayQueue) showQueue() {

	size := this.size()
	if size == 0 {
		fmt.Println("queue empty")
		return
	}

	tempHead := this.front
	for i := 0; i < size; i++ {
		fmt.Printf("arr[%d]=%d\n", tempHead, this.array[tempHead])
		tempHead = (tempHead + 1) % this.maxSize
	}

	fmt.Println()
}
