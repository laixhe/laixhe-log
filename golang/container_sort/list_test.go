package container_sort

import (
	"container/list"
	"fmt"
	"testing"
)

// 用 list 来实现栈和队列的功能 list_heap_ring

// 用 list 实现 栈 功能
func TestListStack(t *testing.T) {

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

// 用 list 来实现 队列 功能
func TestListQueue(t *testing.T) {

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
