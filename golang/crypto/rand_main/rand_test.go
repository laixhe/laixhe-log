package rand_main

import (
	"fmt"
	"testing"
)

func TestRandInt64(t *testing.T) {
	i := RandInt64(10, 50)
	fmt.Println(i)
}
