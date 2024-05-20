package osio

import (
	"fmt"
	"os"
)

// CreateFile 创建文件
func CreateFile(str_file string) {

	// 创建文件
	// Create 函数也是调用的 OpenFile
	file, err := os.Create(str_file)
	if err != nil {
		fmt.Println(err)
		return
	}

	fmt.Println(file)

	// 关闭文件
	file.Close()

}
