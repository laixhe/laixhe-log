package main

import (
	"container/ring"
	"fmt"
	"testing"
)

// container/ring 环形链表
//
// type Ring struct {
//    Next, Prev *Ring // 双向指针，形成环形结构
//    Value      any   // 存储用户数据
// }
//
// New(n int) *Ring        创建一个具有 n 个元素的新环形链表
// Next() *Ring            获取当前元素的下一个元素
// Prev() *Ring            获取当前元素的前一个元素
// Move(n int) *Ring       向前或向后移动 n 个位置
// Link(s *Ring) *Ring     在当前元素之后插入另一个环形链表
// Unlink(n int) *Ring     从当前元素开始删除n个元素的子环
// Do(f func(interface{})) 遍历所有元素

func TestContainerRing(t *testing.T) {
	// 创建包含 5 个节点的环
	r := ring.New(5)
	// 为每个节点赋值（0-4）
	for i := 0; i < r.Len(); i++ {
		r.Value = i
		r = r.Next() // 移动到下一个节点
	}

	// 遍历环（从第一个节点开始）
	r.Do(func(v any) {
		fmt.Print(v, " ")
	})
	// 输出：0 1 2 3 4
}
