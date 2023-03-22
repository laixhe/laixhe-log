package basictype

import (
	"fmt"
	"testing"
)

func TestGetMRandInt64(t *testing.T) {
	for i := 0; i < 10; i++ {
		fmt.Println(GetMRandInt64(1000, 9999))
	}
}

func TestGetCRandInt64(t *testing.T) {
	for i := 0; i < 10; i++ {
		fmt.Println(GetCRandInt64(1000, 9999))
	}
}

func TestIntArrayToString(t *testing.T) {
	fmt.Println(IntArrayToString([]int{1, 2, 3, 5}, ","))
}

func TestInt32ArrayToString(t *testing.T) {
	fmt.Println(Int32ArrayToString([]int32{1, 2, 3, 5, 4}, ""))
}

func TestInt64ArrayToString(t *testing.T) {
	fmt.Println(Int64ArrayToString([]int64{1, 2, 3, 5, 4, 6}, "|"))
}
