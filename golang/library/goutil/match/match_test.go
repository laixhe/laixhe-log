package match

import (
	"fmt"
	"testing"
)

func TestMatchEmail(t *testing.T) {
	fmt.Println(IsEmail("laixhe@laixhe.com"))
	fmt.Println(IsEmail("laixhe#laixhe.cn"))
}

func TestMatchMobile(t *testing.T) {
	fmt.Println(IsMobile("13000000000"))
	fmt.Println(IsMobile("138001380000"))
}

func TestMatchNumber(t *testing.T) {
	fmt.Println(IsNumber("123"))
	fmt.Println(IsNumber("a123"))
}
