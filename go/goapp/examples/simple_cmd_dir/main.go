package main

import (
	"bufio"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"
)

func main() {
	reader := bufio.NewReader(os.Stdin)
	fmt.Println("请输入要处理的目录: ")
	dir, err := reader.ReadString('\n')
	if err != nil {
		panic(err)
	}
	dir = strings.TrimSpace(dir)
	dir = strings.Trim(dir, `"`)

	fmt.Println()
	fmt.Println("工作目录", dir)
	fmt.Println()

    // 打印工作目录下所有文件路径
	if err := filepath.WalkDir(dir, func(path string, info fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if info.IsDir() {
			return nil
		}
		fmt.Println(path)
		return nil
	}); err != nil {
		fmt.Println("目录处理失败: " + err.Error())
		return
	}

    fmt.Println()
	fmt.Println("按任意键结束")
	_, _ = reader.ReadByte()
}
