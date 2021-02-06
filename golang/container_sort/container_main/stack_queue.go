package container_main

import (
	"container/list"
	"fmt"
)

// 用 list 来实现栈和队列的功能

// StackTest 实现栈
func StackTest() {

	stack := list.New()

	for i := 0; i < 5; i++ {
		// 入栈
		stack.PushBack(i)
	}

	for {

		if stack.Len() == 0 {
			break
		}

		// 取出链表最后一个元素
		ele := stack.Back()

		val := ele.Value.(int)
		fmt.Printf("val: %d\n", val)

		// 将最后一个元素删除，出栈
		stack.Remove(ele)
	}

}

// QueueTest 实现队列
func QueueTest() {

	queue := list.New()

	for i := 0; i < 5; i++ {
		// 入队列
		queue.PushBack(i)
	}

	for {

		if queue.Len() == 0 {
			break
		}

		// 取出链表第一个元素
		ele := queue.Front()

		val := ele.Value.(int)
		fmt.Printf("val: %d\n", val)

		// 将第一个元素删除，出队列
		queue.Remove(ele)
	}

}
