package array

import (
	"fmt"
	"testing"
)

func TestFilterRepeatString(t *testing.T) {
	fmt.Println(FilterRepeatString([]string{"123", "456", "789", "456", "123"}))
}

func TestFilterRepeatInt(t *testing.T) {
	fmt.Println(FilterRepeatInt([]int{123, 456, 789, 456, 123}))
}
