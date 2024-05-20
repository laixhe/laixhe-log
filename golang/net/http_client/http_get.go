package http_client

import (
	"fmt"
	"io/ioutil"
	"net/http"
	"net/url"
)

// HttpGet 简单的 GET 请求
func HttpGet() {

	httpUrl := "http://127.0.0.1:5050/?name=laixhe&age=11"

	// 发起 http get 请求
	resp, err := http.Get(httpUrl)
	if err != nil {
		fmt.Println("resp:", err)
		return
	}
	defer resp.Body.Close()

	// 获取请求响应内容
	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		fmt.Println("body:", err)
		return
	}

	fmt.Println(string(body))
}

// HttpGetQuery 参数化的 GET 请求
func HttpGetQuery() {

	httpurl := "http://127.0.0.1:5050"
	u, _ := url.Parse(httpurl)

	// 添加 GET 参数数据
	q := u.Query()
	q.Set("name", "laixhe")
	q.Set("age", "12")

	u.RawQuery = q.Encode()

	// 发起 http get 请求
	resp, err := http.Get(u.String())
	if err != nil {
		fmt.Println("resp:", err)
		return
	}
	defer resp.Body.Close()

	// 获取请求响应内容
	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		fmt.Println("body:", err)
		return
	}

	fmt.Println(string(body))
}
