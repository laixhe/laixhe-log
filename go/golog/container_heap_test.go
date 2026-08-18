package main

import (
	"container/heap"
	"fmt"
	"testing"
)

// container/heap 堆
//
// type Interface interface {
//    Len() int           // 返回堆中元素数量
//    Less(i, j int) bool // 判断索引 i 的元素是否小于索引 j 的元素（决定堆序）
//    Swap(i, j int)      // 交换索引 i 和 j 的元素
//    Push(x any)         // 向堆中添加元素（底层切片执行 `h = append(h, x)`）
//    Pop() any           // 从堆中删除并返回最后一个元素（底层切片执行 `h = h[0 : len(h)-1]`）
// }

// IntHeap 实现 heap.Interface，是最小堆（Less 用 <）
type IntHeap []int

func (h IntHeap) Len() int           { return len(h) }
func (h IntHeap) Less(i, j int) bool { return h[i] < h[j] } // 小于号 = 最小堆（改成 > 则为最大堆）
func (h IntHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *IntHeap) Push(x any) {
	*h = append(*h, x.(int))
}

func (h *IntHeap) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

func TestContainerHeap(t *testing.T) {
	h := &IntHeap{2, 1, 5, 3, 4}
	heap.Init(h) // 建堆（O(n)）

	fmt.Println("堆顶(最小):", (*h)[0]) // 结果: 1

	heap.Push(h, 0) // 入堆
	fmt.Println("入堆 0 后堆顶:", (*h)[0]) // 结果: 0

	// 依次弹出（从小到大）
	fmt.Print("依次弹出: ")
	for h.Len() > 0 {
		fmt.Print(heap.Pop(h), " ") // 结果: 0 1 2 3 4 5
	}
	fmt.Println()
}
