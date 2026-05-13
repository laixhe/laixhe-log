package main

import (
	"container/list"
	"fmt"
	"testing"
)

// container/list 包提供了一套完整的双向链表实现，成为处理有序数据、频繁插入删除场景的核心工具
// 值类型封装   通过 interface{} 实现泛型存储，支持任意数据类型
// 指针语义     插入/删除操作返回的Element指针需妥善保存，避免悬空引用
// 无索引访问   链表不支持随机访问，需通过指针逐节点移动
// 安全迭代     遍历过程中删除节点时，需记录下一个节点指针，避免 panic
//
// type List struct {
//    root Element // 哨兵节点，首尾相连形成环形链表
//    len  int     // 链表元素数量
// }
//
// type Element struct {
//    Value interface{} // 存储实际数据
//    next  *Element    // 后驱节点指针
//    prev  *Element    // 前驱节点指针
// }
//
// Front()                返回第一个元素
// Back()                 返回最后一个元素
// PushFront(v any)       在链表头部插入节点，返回新节点        O(1)
// PushBack(v any)        在链表尾部插入节点，返回新节点        O(1)
// InsertBefore(v, mark)  在指定节点 mark 前插入新节点        O(1)
// InsertAfter(v, mark)   在指定节点 mark 后插入新节点        O(1)
// MoveAfter(e, mark)     将节点 e 移动到 mark 节点之后
// MoveBefore(e, mark)    将节点 e 移动到 mark 节点之前
// MoveToFront(e)         将节点 e 移动到链表头部
// MoveToBack(e)          将节点 e 移动到链表尾部
// Remove(e *Element)     从链表中删除节点 e 返回存储的 Value  O(1)
//
// 正向遍历
// for e := l.Front(); e != nil; e = e.Next() {
//     fmt.Println(e.Value)
// }
// 反向遍历
// for e := l.Back(); e != nil; e = e.Prev() {
//     fmt.Println(e.Value)
// }
// 遍历所有元素
// l.Do(func(v interface{}) {
//     fmt.Println(v)
// })

func TestContainerList(t *testing.T) {
	l := list.New()

	// 尾部插入 elem1
	elem1 := l.PushBack("first")
	// 头部插入 elem2
	_ = l.PushFront("last")
	// 在 elem1 前插入
	l.InsertBefore("middle", elem1)

	fmt.Println("链表长度:", l.Len())         // 输出: 3
	fmt.Println("头元素值:", l.Front().Value) // 输出: last
	fmt.Println("尾元素值:", l.Back().Value)  // 输出: first

	fmt.Println("=============================")

	queue := list.New()
	queue.PushBack("a")                       // 队尾：a
	queue.PushBack("b")                       // 队尾：b
	fmt.Println("队首元素：", queue.Front().Value) // 输出：a

	for queue.Len() > 0 {
		e := queue.Front()
		fmt.Println(queue.Remove(e)) // 弹出a, b
	}

}
