package osio

import (
	"fmt"
	"os"
)

// RemoveFile 删除文件
func RemoveFile(str_file string) {

	// 删除文件
	del := os.Remove(str_file)
	if del != nil {
		fmt.Println(del)
	}

}

// RemoveAllFile 删除指定目录下的所有文件(包括本身)
func RemoveAllFile(str_dir string) {

	// 删除指定 path 下的所有文件(包括本身)
	delDir := os.RemoveAll(str_dir)
	if delDir != nil {
		fmt.Println(delDir)
	}

}
