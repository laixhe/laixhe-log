package http_client

import (
	"fmt"
	"io/ioutil"
	"net/http"
	"net/url"
	"strings"
)

// HttpPost 简单的 POST 请求
func HttpPost() {

	httpurl := "http://127.0.0.1"
	// 头信息 Header Content-Type
	contentType := "application/x-www-form-urlencoded"
	// 参数 Body
	body := strings.NewReader("name=Laixhe&age=18")

	// 发起 http post 请求
	resp, err := http.Post(httpurl, contentType, body)
	if err != nil {
		fmt.Println(err)
		return
	}
	defer resp.Body.Close()

	// 获取请求响应内容
	data, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		fmt.Println(err)
		return
	}

	fmt.Println(string(data))
}

// HttpPostValues 参数化的 POST 请求
func HttpPostValues() {

	httpUrl := "http://127.0.0.1"

	// 添加 POST Body 参数
	payload := url.Values{}
	payload.Set("id", "1")
	payload.Set("name", "laixhe")

	// 发起 http post 请求
	resp, err := http.PostForm(httpUrl, payload)
	if err != nil {
		fmt.Println(err)
		return
	}
	defer resp.Body.Close()

	// 获取请求响应内容
	data, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		fmt.Println(err)
		return
	}

	fmt.Println(string(data))
}
