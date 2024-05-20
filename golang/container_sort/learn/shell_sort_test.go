package learn

import (
	"fmt"
	"testing"
)

func TestShellSort(t *testing.T) {
	fmt.Println(ShellSort([]int{6, 2, 4, 3, 6, 4, 8, 6, 324, 42, 76547, 43, 5466, 54}))
	fmt.Println(ShellSort([]int{6, 2, 4, 3, 6, 4, 8, 6, 324, 42, 76547, 43, 54}))
}

