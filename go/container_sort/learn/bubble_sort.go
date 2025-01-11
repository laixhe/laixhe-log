package learn

// BubbleSort 冒泡排序
// 比较相邻的元素，如果第一个比第二个大，就交换他们两个(重复步骤)
//
// 第一轮开始
// arr[0] ---- 第一位和
// arr[1] ---- 第二位进行比较(第一位大于(或小于)第二位)，就进行交换
// arr[2] ---- 以此类推 (第二位和第三位比较)
// arr[3] ---- 以此类推 (第三位和第四位比较)
// ... 第一轮循环后 - 最后一位就是最大（或最小）的一个元素
// 开始第二轮
// ...
// 主要是把最大(最小)数放到后面
func BubbleSort(arr []int) []int {

	// 获取数组的大小(个数)
	arrLen := len(arr)

	for i := 0; i < arrLen; i++ {

		for j := 0; j < arrLen-i-1; j++ {

			// 所大的数往最右边排，从小到大排列
			if arr[j] > arr[j+1] {
				arr[j], arr[j+1] = arr[j+1], arr[j]
			}

		}

	}

	return arr
}
