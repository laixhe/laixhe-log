package main

import (
	"fmt"
	"testing"
)

func TestBubbleSort(t *testing.T) {
	// 冒泡排序
	fmt.Println(BubbleSort([]int{6,2,4,3,6,4,8,6,324,42,76547,43,5466,54}))
}

func TestInsertSort(t *testing.T) {
	// 插入排序
	fmt.Println(InsertSort([]int{6,2,4,3,6,4,8,6,324,42,76547,43,5466,54}))
}

func TestSelectSort(t *testing.T) {
	// 选择排序
	fmt.Println(SelectSort([]int{6,2,4,3,6,4,8,6,324,42,76547,43,5466,54}))
}
