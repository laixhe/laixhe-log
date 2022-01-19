package container_sort

import (
	"fmt"
	"sort"
	"testing"
)

func TestSort(t *testing.T) {

	arrayInt := []int{3, 1, 2, 5, 4}
	arrayFloat := []float64{3.2, 1.8, 1.9, 2.2, 4.3}
	arrayString := []string{"abc", "ab", "bc"}

	fmt.Println("arrayInt:", arrayInt)
	fmt.Println("arrayFloat:", arrayFloat)
	fmt.Println("arrayString", arrayString)
	fmt.Println("===========================")

	// 升序
	sort.Ints(arrayInt)
	sort.Float64s(arrayFloat)
	sort.Strings(arrayString)

	fmt.Println("sort arrayInt:", arrayInt)
	fmt.Println("sort arrayFloat:", arrayFloat)
	fmt.Println("sort arrayString", arrayString)
	fmt.Println("===========================")

	// 降序
	sort.Sort(sort.Reverse(sort.IntSlice(arrayInt)))
	sort.Sort(sort.Reverse(sort.Float64Slice(arrayFloat)))
	sort.Sort(sort.Reverse(sort.StringSlice(arrayString)))

	fmt.Println("sort arrayInt:", arrayInt)
	fmt.Println("sort arrayFloat:", arrayFloat)
	fmt.Println("sort arrayString", arrayString)
}
