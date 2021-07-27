package learn

import "fmt"

// 稀疏数组 - 把大数组转成小数组

type chessMapNode struct {
	row int
	col int
	val int
}

func SparseArray() {

	// 1、先创建一个原始数组
	var chessMap [11][11]int
	chessMap[1][2] = 1
	chessMap[2][3] = 2

	// 2、输出看看原始的数组
	fmt.Println("原始的数组:")
	for _, v := range chessMap {
		for _, v2 := range v {
			fmt.Printf("%d\t", v2)
		}
		fmt.Println()
	}

	// 3、把数组转成稀疏数组
	// 要转成稀疏数组时，不知道那些数据要转N
	// 思路
	// (1) 遍历 chessMap 发现元素的值不为0，创建一个 chessMapNode 结构体
	// (2) 将其放入对应的切片中

	var sparseArr []chessMapNode

	// 一个标准的稀疏数组应该包含 记录二维数组的规模(行、列、默认值)
	sparseArr = append(sparseArr, chessMapNode{row: len(chessMap), col: len(chessMap[0]), val: 0})

	for i, v := range chessMap {

		for j, v2 := range v {
			if v2 != 0 {

				// 创建一个 chessMapNode 值节点
				node := chessMapNode{
					row: i,
					col: j,
					val: v2,
				}

				sparseArr = append(sparseArr, node)
			}
		}

	}

	fmt.Println("\n转成稀疏数组:")
	for i, v := range sparseArr {
		fmt.Printf("%d: %d %d %d\n", i, v.row, v.col, v.val)
	}

}
