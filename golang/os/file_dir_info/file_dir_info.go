package file_dir_info

import (
	"fmt"
	"io/ioutil"
	"log"
	"os"
	"os/exec"
	"path"
	"path/filepath"
	"strings"
)

// FileDirInfo 文件与目录相关信息
func FileDirInfo() {

	// 获取文件或文件夹的信息
	info, err := os.Stat("./")
	if err != nil {
		// 如果返回的错误类型使用 os.IsNotExist() 判断为 true,说明文件或文件夹不存在
		fmt.Println("Stat:", err)
		return
	}

	log.Println(os.IsNotExist(err))
	log.Println(os.IsExist(err))

	fmt.Println(info.Name())    // 获取文件名
	fmt.Println(info.IsDir())   // 判断是否是目录，返回bool类型
	fmt.Println(info.ModTime()) // 获取文件修改时间
	fmt.Println(info.Mode())    // 文件的权限
	fmt.Println(info.Size())    // 获取文件大小

	// ==============
	dir, _ := os.Getwd()
	fmt.Println("当前的目录:", dir)

	// 切换目录
	//os.Chdir("../")

	// 主机名
	//osName, _ := os.Hostname()
	//fmt.Println("主机名:", osName)

	// 输出所有环境变量
	//path := os.Environ()
	//fmt.Println("输出所有环境变量:", path)
	// 清空环境变量
	//os.Clearenv()

	//GOPATH := os.Getenv("GOPATH")
	//fmt.Println("环境变量GOPATH的值是:", GOPATH)

	// 改变的是文件的权限
	//os.Chmod("./file_dir_info.exe", 0777)
	// 改变文件的时间-访问时间 创建时间
	//os.Chtimes("./file_dir_info.exe", time.Now(), time.Now())
	// 重命名
	//os.Rename("1.go", "2.go")

	fullFilename := "/laixhe/test.txt"
	fmt.Println("fullFilename =", fullFilename)

	filenameWithSuffix := ""
	filenameWithSuffix = path.Base(fullFilename)
	fmt.Println("获取文件名带后缀 =", filenameWithSuffix)

	fileSuffix := ""
	fileSuffix = path.Ext(filenameWithSuffix)
	fmt.Println("获取文件后缀 =", fileSuffix)

	filenameOnly := ""
	filenameOnly = strings.TrimSuffix(filenameWithSuffix, fileSuffix)
	fmt.Println("获取文件名 =", filenameOnly)

	// 遍历文件夹中的所有文件
	rd, err := ioutil.ReadDir("./")
	if err != nil {
		fmt.Println("ReadDir err:", err)
		return
	}

	for _, fi := range rd {
		if !fi.IsDir() {
			fmt.Println("ReadDir: ", fi.Name())
		}
	}

}

// GetCurrPath 获取可执行文件的绝对路径(不包括文件名)
func GetCurrPath() string {

	// 从环境变量中搜索可执行的二进制的文件的路径，
	// 如果 file 中包含一个斜杠，
	// 则直接根据绝对路径或者相对本目录的相对路径去查找
	file, _ := exec.LookPath(os.Args[0])

	// 检测地址是否是绝对地址，是绝对地址直接返回，
	// 不是绝对地址，会添加当前工作路径
	path, _ := filepath.Abs(file)
	return filepath.Dir(path)
}
