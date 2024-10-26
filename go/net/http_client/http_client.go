package http_client

import (
	"fmt"
	"io/ioutil"
	"net/http"
	"net/url"
	"strings"
)

// HttpClient 自义定请求
func HttpClient() {

	httpUrl := "http://127.0.0.1:5050"
	method := "POST" // 请求方法

	// 添加 POST Body 参数
	//payload := strings.NewReader(`"id":1,"name":"laixhe"`)
	payload := url.Values{}
	payload.Set("id", "1")
	payload.Set("name", "laixhe")
	bodyData := strings.NewReader(payload.Encode())

	// 准备发起 http 请求
	req, err := http.NewRequest(method, httpUrl, bodyData)
	if err != nil {
		fmt.Println("net request error", err)
		return
	}

	// 添加头信息
	req.Header.Add("Content-Type", "application/json")

	// 添加 GET 参数数据
	query := req.URL.Query()
	query.Add("age", "18")
	query.Add("set", "men")
	req.URL.RawQuery = query.Encode()

	// 生成 client 参数为默认
	client := &http.Client{}
	// 发起 http 请求
	res, err := client.Do(req)
	if err != nil {
		fmt.Println("client do error", err)
		return
	}
	defer res.Body.Close()

	// 获取请求响应内容
	body, err := ioutil.ReadAll(res.Body)
	if err != nil {
		fmt.Println("read all error", err)
		return
	}

	fmt.Println(string(body))
}
