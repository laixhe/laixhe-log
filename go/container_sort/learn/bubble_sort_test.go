package learn

import (
	"fmt"
	"testing"
)

func TestBubbleSort(t *testing.T) {
	// 冒泡排序
	fmt.Println(BubbleSort([]int{6, 2, 4, 3, 6, 4, 8, 6, 324, 42, 76547, 43, 5466, 54}))
}
