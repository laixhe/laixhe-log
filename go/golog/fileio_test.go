package main

import (
	"bufio"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

/*
文件读写（IO）：os / bufio / filepath 包（对应 Python 文件操作 / Java IO / C# File）
测试用 t.TempDir() 生成临时目录，测试结束后自动清理，不污染工作区
*/

// 整体读写：os.WriteFile / os.ReadFile（对应 Python open().write() / read()）
func TestFileWriteRead(t *testing.T) {
	dir := t.TempDir()
	path := filepath.Join(dir, "data.txt") // 跨平台拼接路径（对应 Python os.path.join）

	// 写文件（权限 0644：所有者读写、其他只读）
	content := "hello\nworld\n"
	err := os.WriteFile(path, []byte(content), 0644)
	if err != nil {
		t.Fatal(err)
	}

	// 读文件（返回 []byte）
	data, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	fmt.Printf("读出内容: %q\n", string(data)) // "hello\nworld\n"

	// 判断文件是否存在（对应 Python os.path.exists）
	_, err = os.Stat(path)
	fmt.Println("文件存在:", err == nil) // true

	// 文件不存在的情况
	_, err = os.Stat(filepath.Join(dir, "missing.txt"))
	fmt.Println("缺失文件:", os.IsNotExist(err)) // true
}

// 流式读写：os.Open + bufio（逐行处理大文件，对应 Python 逐行读文件）
func TestFileBufio(t *testing.T) {
	dir := t.TempDir()
	path := filepath.Join(dir, "lines.txt")

	// 写入多行内容
	os.WriteFile(path, []byte("go\nrust\njava\n"), 0644)

	// 逐行读取（bufio.Scanner，适合大文件，不一次性载入内存）
	f, err := os.Open(path)
	if err != nil {
		t.Fatal(err)
	}
	defer f.Close() // 关闭文件（对应 Python with open / C# using）

	scanner := bufio.NewScanner(f)
	fmt.Print("逐行读取: ")
	for scanner.Scan() {
		fmt.Print(scanner.Text(), " ") // go rust java
	}
	fmt.Println()
}

// 追加写入：os.OpenFile 带追加标志（对应 Python open(..., "a")）
func TestFileAppend(t *testing.T) {
	dir := t.TempDir()
	path := filepath.Join(dir, "log.txt")

	os.WriteFile(path, []byte("line1\n"), 0644)

	// 以追加模式打开（O_APPEND：写入追加到末尾，对应 Python "a" 模式）
	f, err := os.OpenFile(path, os.O_APPEND|os.O_WRONLY, 0644)
	if err != nil {
		t.Fatal(err)
	}
	f.WriteString("line2\n")
	f.Close()

	data, _ := os.ReadFile(path)
	fmt.Printf("追加后: %q\n", string(data)) // "line1\nline2\n"
}

// 创建目录与重命名删除（对应 Python os.mkdir / os.rename / os.remove）
func TestFileDirOps(t *testing.T) {
	dir := t.TempDir()
	sub := filepath.Join(dir, "sub")

	// 创建目录（MkdirAll 递归创建，对应 Python os.makedirs）
	err := os.MkdirAll(filepath.Join(sub, "a", "b"), 0755)
	fmt.Println("创建目录:", err) // <nil>

	// 重命名（对应 Python os.rename）
	oldPath := filepath.Join(dir, "old.txt")
	newPath := filepath.Join(dir, "new.txt")
	os.WriteFile(oldPath, []byte("x"), 0644)
	os.Rename(oldPath, newPath)
	_, err = os.Stat(newPath)
	fmt.Println("重命名后存在:", err == nil) // true

	// 删除文件（对应 Python os.remove）
	os.Remove(newPath)
	_, err = os.Stat(newPath)
	fmt.Println("删除后存在:", err == nil) // false（不存在）

	// 遍历目录（对应 Python os.listdir）
	os.WriteFile(filepath.Join(dir, "1.txt"), []byte(""), 0644)
	entries, _ := os.ReadDir(dir)
	fmt.Print("目录内容: ")
	for _, e := range entries {
		fmt.Print(e.Name(), " ") // 1.txt sub
	}
	fmt.Println()
}

// 字符串 ↔ 文件流（io.Reader/Writer 接口，对应 Python StringIO）
func TestFileStringIO(t *testing.T) {
	// 用 strings.NewReader 模拟文件内容（io.Reader 接口）
	reader := strings.NewReader("line1\nline2\n")
	scanner := bufio.NewScanner(reader)
	fmt.Print("从字符串读取: ")
	for scanner.Scan() {
		fmt.Print(scanner.Text(), " ")
	}
	fmt.Println()
}
