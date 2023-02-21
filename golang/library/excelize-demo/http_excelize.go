package main

import (
	"fmt"
	_ "image/gif"
	_ "image/jpeg"
	_ "image/png"
	"io/ioutil"
	"net/http"
	"strings"

	"github.com/xuri/excelize/v2"
)

func HttpXls(w http.ResponseWriter, req *http.Request) {

	w.Header().Set("Content-Disposition", "attachment; filename=用户列表.xlsx")
	w.Header().Set("Content-Type", "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")

	categories := map[string]string{"A1": "姓名", "B1": "年龄", "C1": "其它", "D1": "备注"}
	values1 := map[string]string{"A2": "Laixhe", "B2": "18", "C2": "code", "D2": "https://www.laixhe.com/static/img/002.png"}
	values2 := map[string]string{"A3": "Laiki", "B3": "19", "C3": "", "D3": "没有"}

	f := excelize.NewFile()
	// 创建一个工作表
	sheet1, err := f.NewSheet("Sheet1")
	if err != nil {
		fmt.Println("new file sheet error", err)
		return
	}
	// 设置工作簿的默认工作表
	f.SetActiveSheet(sheet1)

	// 设置单元格的值
	for k, v := range categories {
		f.SetCellValue("Sheet1", k, v)
	}

	// 设置单元格的值
	for k, v := range values1 {
		// 是否以 https 开头
		if strings.HasPrefix(v, "https") {
			buf, err := HttpGet(v)
			if err != nil {
				fmt.Println("http get:", err)
				continue
			}
			// 插入图片，并设置图片的缩放、外部超链接
			err = f.AddPictureFromBytes("Sheet1", k, `{"x_scale":0.1,"y_scale":0.1,"hyperlink":"`+v+`","hyperlink_type":"External"}`, ".png", buf, nil)
			if err != nil {
				fmt.Println("add picture:", err)
				continue
			}
		} else {
			f.SetCellValue("Sheet1", k, v)
		}
	}

	// 设置单元格的值
	for k, v := range values2 {
		f.SetCellValue("Sheet1", k, v)
	}

	// 输出到网页
	f.WriteTo(w)

	// 保存文件
	//f.SaveAs("用户列表.xlsx")
}

// 远程获取网页
func HttpGet(u string) ([]byte, error) {
	resp, err := http.Get(u)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	//获取网页的所有内容
	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return nil, err
	}

	return body, nil
}
