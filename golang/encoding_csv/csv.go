package encoding_csv

import (
	"encoding/csv"
	"fmt"
	"os"
	"sort"
)

// CreateCsv 创建
func CreateCsv() {

	// 不存在则创建;存在则清空;读写模式;
	file, err := os.Create("csv_test.csv")
	if err != nil {
		fmt.Println("open file is failed, err: ", err)
		return
	}
	// 延迟关闭
	defer file.Close()

	// 写入UTF-8 BOM，防止中文乱码
	//file.WriteString("\xEF\xBB\xBF")

	w := csv.NewWriter(file)

	// Map写入
	m := make(map[int][]string)
	m[0] = []string{"学生编号", "学生姓名", "学生特长"}
	m[1] = []string{"s1", "学生1", "乾坤大挪移1"}
	m[2] = []string{"s2", "学生2", "乾坤大挪移2"}
	m[3] = []string{"s3", "学生3", "乾坤大挪移3"}
	m[4] = []string{"s4", "学生4", "乾坤大挪移4"}
	m[5] = []string{"s5", "学生5", "乾坤大挪移5"}
	m[6] = []string{"s6", "学生6", "乾坤大挪移6"}
	m[7] = []string{"s7", "学生7", "乾坤大挪移7"}
	m[8] = []string{"s8", "学生8", "乾坤大挪移8"}
	m[9] = []string{"s9", "学生9", "乾坤大挪移9"}
	m[10] = []string{"s10", "学生10", "乾坤大挪移10"}

	// 按照key排序
	var keys []int
	for k := range m {
		keys = append(keys, k)
	}

	sort.Ints(keys)

	for _, key := range keys {
		err = w.Write(m[key])
		if err != nil {
			fmt.Println(" write err: ", err)
			break
		}
	}

	// 刷新缓冲
	w.Flush()
}
