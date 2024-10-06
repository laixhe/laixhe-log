package learn

import "testing"

func TestSingleLink(t *testing.T) {

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
