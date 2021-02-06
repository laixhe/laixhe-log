package main

// 插入排序
// 1、从第一个元素开始，该元素可以认为已经被排序
// 2、取出下一个元素，在已经排序的元素序列中从后向前扫描
// 3、如果该元素（已排序）大于新元素，将该元素移到下一位置
// 4、重复步骤3，直到找到已排序的元素小于或者等于新元素的位置
// 5、将新元素插入到下一位置中
// 6、重复步骤2~5
func InsertSort(arr []int) []int {

	// 获取数组的大小(个数)
	arrLen := len(arr)

	for i := 1; i < arrLen; i++ {
		for j := i; j > 0; j-- {

			if arr[j] < arr[j-1] {
				arr[j], arr[j-1] = arr[j-1], arr[j]
			} else {

				// 如果是大于相等，就跳出当前循环(j)
				break
			}

		}
	}

	return arr
}
