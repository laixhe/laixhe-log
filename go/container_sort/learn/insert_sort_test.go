package learn

import (
	"fmt"
	"testing"
)

func TestInsertSort(t *testing.T) {
	// 插入排序
	fmt.Println(InsertSort([]int{6, 2, 4, 3, 6, 4, 8, 6, 324, 42, 76547, 43, 5466, 54}))
}
