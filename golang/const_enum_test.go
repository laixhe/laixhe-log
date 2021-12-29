package main

import (
	"fmt"
	"testing"
)

func TestConst(t *testing.T) {
	fmt.Println(uid) // 结果： 10

	fmt.Println(read, green, blue) // 结果： 0 1 2
}
