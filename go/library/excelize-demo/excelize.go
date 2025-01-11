package main

import (
	"log"
	"strconv"

	excelize "github.com/xuri/excelize/v2"
)

// 对 表格 文件进行写入和读取

// 目录结构
type Knowledge struct {
	Press   int    //出版社
	Grade   int    //年级
	Subject int    //学科
	Book    int    //册
	Serial  int    //序号
	Level   int    //层次
	Name    string //目录名
}

func ReadExcelize(fileName string) {

	//打开一个 excel 文件资源
	f, err := excelize.OpenFile(fileName)
	if err != nil {
		log.Println(err.Error())
	}

	//目录结构所有数据
	knowledgeArray := make([]*Knowledge, 0, 1000)

	//循环文件中所有工作表
	for _, sheet := range f.GetSheetMap() {

		// 获取 工作表 上所有单元格
		rows, err := f.GetRows(sheet)
		if err != nil {
			log.Println("GetRows err:", err)
			continue
		}

		//当列数据错误时返回到行
	BREAKSHEET:

		//循环对应工作表中行数
		for _, row := range rows {

			//每行的列的个数
			rowLen := len(row)
			if rowLen >= 7 {

				//目录结构每行数据
				knowledge := new(Knowledge)

				//循环工作表行数的每一列
				for cellKey, cell := range row {
					//获取每列的字符串

					//switch
					switch cellKey {

					case 0: //出版社
						press, iserr := getStr(cell)
						if !iserr {
							continue BREAKSHEET
						}
						knowledge.Press = press

					case 1: //年级
						grade, iserr := getStr(cell)
						if !iserr {
							continue BREAKSHEET
						}
						knowledge.Grade = grade

					case 2: //学科
						subject, iserr := getStr(cell)
						if !iserr {
							continue BREAKSHEET
						}
						knowledge.Subject = subject

					case 3: //册
						book, iserr := getStr(cell)
						if !iserr {
							continue BREAKSHEET
						}
						knowledge.Book = book

					case 4: //序号
						serial, iserr := getStr(cell)
						if !iserr {
							continue BREAKSHEET
						}
						knowledge.Serial = serial

					case 5: //层次
						level, iserr := getStr(cell)
						if !iserr {
							continue BREAKSHEET
						}
						knowledge.Level = level

					case 6: //目录名
						if cell == "" {
							continue BREAKSHEET
						}
						knowledge.Name = cell

					} //----switch

				} //--------循环工作表行数的每一列

				//存放每一行数据
				knowledgeArray = append(knowledgeArray, knowledge)

			} //------------每行的列的个数

		} //----------------循环对应工作表中行数

	} //--------------------循环文件中所有工作表

	for _, v := range knowledgeArray {
		log.Println(v)
	}

}

// 将数字字符串转为数型
func getStr(str string) (int, bool) {
	if str == "" {
		return 0, false
	}

	strId, err := strconv.Atoi(str)
	if err != nil {
		return 0, false
	}

	return strId, true
}
