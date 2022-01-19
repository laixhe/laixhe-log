package md5_file

import "testing"

// 结果都一样： 9d32e3c40efb0b749270695d5f0afdfc

func TestReadFile(t *testing.T) {
	// 暴力读取文件
	ReadFile()
}

func TestReadAll(t *testing.T) {
	// 暴力读取文件，和上面 ReadFile 一样的性质
	ReadAll()
}

func TestRead(t *testing.T) {
	// 基本使用 File 方法
	Read()
}

func TestIoCopy(t *testing.T) {
	// 使用 IO 复制
	IoCopy()
}

func TestIoCopyBufio(t *testing.T) {
	// 使用缓存 IO 复制
	IoCopyBufio()
}
