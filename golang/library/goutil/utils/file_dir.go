package utils

import (
	"errors"
	"io/ioutil"
	"os"
	"os/exec"
	"path/filepath"
)

// RunningDirectory 获取当前执行文件的目录(绝对路径)
func RunningDirectory() (string, error) {

	// 返回绝对路径
	dir, err := filepath.Abs(filepath.Dir(os.Args[0]))
	if err != nil {
		return "", err
	}

	// 将 \ 替换成 /
	// windows 当前路径 D:\go\src\study 转换之后 D:/go/src/study
	dir = filepath.ToSlash(dir)

	return dir, nil
}

// RunningFilePath 获取当前执行文件的路径
func RunningFilePath() (string, error) {

	file, err := exec.LookPath(os.Args[0])
	if err != nil {
		file = os.Args[0]
	}

	file, err = filepath.Abs(file)
	if err != nil {
		return "", err
	}

	return file, nil
}

// IsExist 是否存在这个目录或文件
func IsExist(path string) bool {

	_, err := os.Stat(path)
	if err == nil {
		return true
	}

	if os.IsNotExist(err) {
		return false
	}

	return false
}

// IsFile 是否为文件
func IsFile(f string) bool {

	b, err := os.Stat(f)
	if err != nil {
		return false
	}

	if b.IsDir() {
		return false
	}

	return true
}

// IsDir 是否为目录
func IsDir(path string) bool {

	b, err := os.Stat(path)
	if err != nil {
		return false
	}

	if b.IsDir() {
		return true
	}

	return false
}

// ListDirs 获取目录下所有文件和目录名
func ListDirs(dirPath string) ([]string, error) {

	if !IsExist(dirPath) {
		return nil, errors.New("directory does not exist")
	}

	fs, err := ioutil.ReadDir(dirPath)
	if err != nil {
		return nil, err
	}

	sz := len(fs)
	if sz == 0 {
		return []string{}, nil
	}

	var ret []string
	for i := 0; i < sz; i++ {

		name := fs[i].Name()
		if name != "." && name != ".." {
			ret = append(ret, name)
		}

	}

	return ret, nil
}
