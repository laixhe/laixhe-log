package basictype

import (
	"fmt"
	"testing"
)

func TestSubstr(t *testing.T) {
	str := Substr("123456789", 0, 6)
	fmt.Println(str)
}

func TestGetRandomString(t *testing.T) {
	str := GetRandomString(20, 1)
	fmt.Println(str, len(str))
}

func TestStringToIntArray(t *testing.T) {
	fmt.Println(StringToIntArray("12,23,45,2147483647,2147483648", ","))
}

func TestStringToInt32Array(t *testing.T) {
	fmt.Println(StringToInt32Array("12,23,45,56,2147483647,2147483648", ","))
}

func TestStringToInt64Array(t *testing.T) {
	fmt.Println(StringToInt64Array("12,23,45,56,78,2147483647,2147483648", ","))
}
