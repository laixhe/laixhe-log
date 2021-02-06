package main

import "fmt"

// 单链表
type SingleLink struct {
	Name string
	Age  int
	Next *SingleLink
}

func NewSingleLink() *SingleLink {
	return &SingleLink{}
}

// 向单链表的末尾添加一个元素
func (this *SingleLink) Push(next *SingleLink) {

	// 创建一个指向下一个结点元素
	// 最后指向最后元素了
	temp := this
	for {

		// 表示找到最后
		if temp.Next == nil {
			break
		}

		// 让 temp 不断的指向下一个结点
		temp = temp.Next
	}

	// 上面已经找到最后一个了

	// 向单链表的末尾添加一个元素
	temp.Next = next

}

// 向单链表添加一个元素, 根据 age 从小到大插入
func (this *SingleLink) SortPush(next *SingleLink) {

	// 创建一个指向下一个结点元素
	// 找到要插入的元表
	temp := this
	for {

		// 表示找到最后
		if temp.Next == nil {
			break
		} else if temp.Next.Age > next.Age {
			break
		}

		temp = temp.Next

	}

	// 插入
	next.Next = temp.Next
	temp.Next = next

}

// 显示所有结点信息
func (this *SingleLink) Show() {

	// 先判断是否为空的链表
	if this.Next == nil {
		fmt.Println("为空的链表")
		return
	}

	// 创建一个指向下一个结点元素
	// 最后指向最后元素了
	temp := this

	for{

		// 显示当前结点信息
		fmt.Printf("[%s = %d]\n", temp.Next.Name, temp.Next.Age)

		temp = temp.Next
		// 表示找到最后
		if temp.Next == nil {
			break
		}

	}

}

func RunSingleLink() {

	// 创建头结点
	head := NewSingleLink()

	// 创建一个结点
	next := &SingleLink{
		Name: "lai-1",
		Age:  1,
	}

	// 向单链表添加一个元素, 根据 age 从小到大插入
	head.SortPush(next)

	// 创建一个结点
	next = &SingleLink{
		Name: "lai-3",
		Age:  3,
	}

	// 向单链表添加一个元素, 根据 age 从小到大插入
	head.SortPush(next)

	// 创建一个结点
	next = &SingleLink{
		Name: "lai-2",
		Age:  2,
	}

	// 向单链表添加一个元素, 根据 age 从小到大插入
	head.SortPush(next)

	// 创建一个结点
	next = &SingleLink{
		Name: "lai-2-1",
		Age:  2,
	}

	// 向单链表添加一个元素, 根据 age 从小到大插入
	head.SortPush(next)

	// 创建一个结点
	next = &SingleLink{
		Name: "lai-2-2",
		Age:  2,
	}

	// 向单链表添加一个元素, 根据 age 从小到大插入
	head.SortPush(next)

	// 创建一个结点
	next = &SingleLink{
		Name: "lai-5",
		Age:  5,
	}

	// 向单链表添加一个元素, 根据 age 从小到大插入
	head.SortPush(next)

	// 创建一个结点
	next = &SingleLink{
		Name: "lai-4",
		Age:  4,
	}

	// 向单链表添加一个元素, 根据 age 从小到大插入
	head.SortPush(next)

	head.Show()

}
