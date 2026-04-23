package main

import (
	"bytes"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"strings"
	"testing"

	"github.com/quic-go/quic-go/http3"
)

// HTTP/3 客户端
func TestHttp3Client(t *testing.T) {
	client := &http.Client{
		Transport: &http3.Transport{},
	}

	resp, err := client.Get("https://example.com") // 简单的 GET 请求
	if err != nil {
		t.Fatal(err)
		return
	}
	defer resp.Body.Close()
}

func TestHttpClientGet(t *testing.T) {
	//http.Get("https://example.com") // 简单的 GET 请求

	// 创建一个新的 HTTP 客户端
	client := &http.Client{}

	// 创建一个新的 HTTP 请求
	req, err := http.NewRequest("GET", "https://example.com", nil)
	if err != nil {
		fmt.Println("Error creating request:", err)
		return
	}
	// 设置 User-Agent Accept 头
	req.Header.Set("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
	req.Header.Set("Accept", "application/json")

	// 发送请求
	resp, err := client.Do(req)
	if err != nil {
		fmt.Println("Error sending request:", err)
		return
	}
	defer resp.Body.Close()
	// 读取响应体
	body, err := io.ReadAll(resp.Body)
	if err != nil {
		fmt.Println("Error reading response body:", err)
		return
	}
	// 打印响应体
	fmt.Println(string(body))
}

func TestHttpClientPost(t *testing.T) {
	//http.Post("https://example.com", "text/plain", strings.NewReader("Hello World")) // 简单的 POST 请求

	data := url.Values{}
	data.Add("age", "18")
	_, _ = http.Post("https://example.com", "application/x-www-form-urlencoded", strings.NewReader(data.Encode()))
	// 或
	//http.PostForm("https://example.com", data)

	// 创建一个新的HTTP客户端
	client := &http.Client{}

	jsonData := []byte(`{"age":"18"}`)
	// 创建一个新的HTTP请求
	req, err := http.NewRequest("POST", "https://jsonplaceholder.typicode.com/posts", bytes.NewBuffer(jsonData))
	if err != nil {
		fmt.Println("Error creating request:", err)
		return
	}

	// 设置 Content-Type User-Agent Accept 头
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
	req.Header.Set("Accept", "application/json")

	// 发送请求
	resp, err := client.Do(req)
	if err != nil {
		fmt.Println("Error sending request:", err)
		return
	}
	defer resp.Body.Close()
	// 读取响应体
	body, err := io.ReadAll(resp.Body)
	if err != nil {
		fmt.Println("Error reading response body:", err)
		return
	}
	// 打印响应体
	fmt.Println(string(body))
}
