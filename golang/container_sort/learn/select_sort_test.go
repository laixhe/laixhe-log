package learn

import (
	"fmt"
	"testing"
)

func TestSelectSort(t *testing.T) {
	// 选择排序
	fmt.Println(SelectSort([]int{6, 2, 4, 3, 6, 4, 8, 6, 324, 42, 76547, 43, 5466, 54}))
}
