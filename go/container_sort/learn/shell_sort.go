package learn

import "math"

// ShellSort 希尔排序
func ShellSort(arr []int) []int {
	gap := 1
	for gap < len(arr){
		gap = gap * 3 + 1
	}

	for gap > 0{
		for i:= gap; i < len(arr); i++{
			tmp := arr[i]
			j := i - gap
			for j>=0 && arr[j] > tmp{
				arr[j+gap] = arr[j]
				j -= gap
			}
			arr[j+gap] = tmp
		}
		gap = int(math.Floor(float64(gap / 3)))
	}

	return arr
}

