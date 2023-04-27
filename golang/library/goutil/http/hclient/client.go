package hclient

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"io/ioutil"
	"net/http"
	"net/url"
	"reflect"
	"strings"
)

// 相关头部信息
const (
	UserAgent      = "User-Agent"
	UserAgentValue = "Goutil/1.0.0 Laixhe/1.0.0"
)

// 相关头部信息
const (
	ContentType               = "Content-Type"
	ApplicationFormUrlencoded = "application/x-www-form-urlencoded"
	ApplicationJson           = "application/json"
)

// 相关头部信息
const (
	Authorization         = "Authorization"
	AuthorizationValue    = "Basic "
	AuthorizationValueLen = 6
)

// httpClient HTTP客户端
type httpClient struct {
	method     string
	httpURL    *url.URL
	header     map[string]string
	body       io.Reader
	Error      error
	RespBody   []byte
	RespHeader map[string]string
	RespCode   int
}

// reflectStruct 解析结构体
func (hc *httpClient) reflectStruct(v *reflect.Value) map[string]string {
	data := make(map[string]string)

	// 包含所有字段的数量(取所有字段的信息)
	for i := 0; i < v.NumField(); i++ {
		// 取出当前字段
		field := v.Type().Field(i)
		// 取出当前字段名
		name := field.Name
		// 取出当前字段的值
		val := v.Field(i).Interface()
		// 取出当前字段的tag
		tag := field.Tag.Get("json")
		// 提取tag信息
		var tags []string
		if tag != "" {
			tags = strings.Split(tag, ",")
			name = tags[0]
		}

		// 类型检测
		switch field.Type.Kind() {
		case reflect.String:
			value := val.(string)
			if len(tags) >= 2 {
				if tags[1] == "omitempty" {
					if value == "" {
						break
					}
				}
			}
			data[name] = value
		case reflect.Int64, reflect.Int32, reflect.Int, reflect.Int8,
			reflect.Uint64, reflect.Uint32, reflect.Uint, reflect.Uint8:
			value := fmt.Sprintf("%d", val)
			if len(tags) >= 2 {
				if tags[1] == "omitempty" {
					if value == "0" {
						break
					}
				}
			}
			data[name] = value
		case reflect.Float64, reflect.Float32:
			value := fmt.Sprintf("%v", val)
			if len(tags) >= 2 {
				if tags[1] == "omitempty" {
					if value == "0" {
						break
					}
				}
			}
			data[name] = value
		case reflect.Bool:
			value := fmt.Sprintf("%v", val)
			if len(tags) >= 2 {
				if tags[1] == "omitempty" {
					if value == "false" {
						break
					}
				}
			}
			data[name] = value
		}
	}

	return data
}

// reflectHeader 解析请求头信息
func (hc *httpClient) reflectHeader(header interface{}) {
	// 进行类型断言
	if header != nil {
		switch header.(type) {
		case map[string]string:
			headerMap := header.(map[string]string)
			if len(headerMap) > 0 {
				hc.header = headerMap
			}
		default:
			headerReflect := reflect.ValueOf(header)
			// 判断是否为结构类型
			if headerReflect.Kind() == reflect.Struct {
				headerMap := hc.reflectStruct(&headerReflect)
				if len(headerMap) > 0 {
					hc.header = headerMap
				}
			}
		}
	}
}

// httpCall 发起 http 请求
func (hc *httpClient) httpCall() {
	if hc.Error != nil {
		return
	}

	// 准备发起 http 请求
	req, err := http.NewRequest(hc.method, hc.httpURL.String(), hc.body)
	if err != nil {
		hc.Error = err
		return
	}

	// 添加头信息
	req.Header.Set(UserAgent, UserAgentValue)
	for k, v := range hc.header {
		req.Header.Set(k, v)
	}

	// 生成 client 参数为默认
	client := &http.Client{}

	// 发起 http 请求
	resp, err := client.Do(req)
	if err != nil {
		hc.Error = err
		return
	}
	defer resp.Body.Close()

	hc.RespCode = resp.StatusCode
	if resp.StatusCode == http.StatusNotFound {
		hc.Error = fmt.Errorf("resource not found: %s", hc.httpURL.String())
		return
	}

	// 获取 http body 内容
	data, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		hc.Error = err
		return
	}
	hc.RespBody = data
}

// Json 将请求数据序列化为json
// data 必须为指针，将返回的 json 数据进行反序列化存储
func (hc *httpClient) Json(data interface{}) *httpClient {
	hc.httpCall()

	if hc.Error != nil {
		return hc
	}

	if len(hc.RespBody) == 0 {
		hc.Error = errors.New("data not found")
	} else {
		err := json.Unmarshal(hc.RespBody, data)
		if err != nil {
			hc.Error = err
		}
	}

	return hc
}

// Request 请求
func (hc *httpClient) Request() *httpClient {
	hc.httpCall()
	return hc
}

// Log 日志
func (hc *httpClient) Log(f func(data []byte, header map[string]string, err error)) *httpClient {
	if f == nil {
		fmt.Printf("data=%s\n", hc.RespBody)
		fmt.Printf("header=%v\n", hc.RespHeader)
		fmt.Printf("err=%v\n", hc.Error)
	}
	return hc
}

// HttpGet 发起 GET 请求
// header 头信息，只接受 map[string]string、struct(只接受基本类型)
// query 请求体，只接受 string、[]byte、map[string]string、struct(只接受基本类型)
func HttpGet(httpURL string, header interface{}, query interface{}) *httpClient {
	hc := &httpClient{
		method: http.MethodGet,
		header: make(map[string]string),
	}
	// 生成要访问的 url
	urlParse, err := url.Parse(httpURL)
	if err != nil {
		hc.Error = err
		return hc
	}
	hc.httpURL = urlParse

	// 进行类型断言
	if query != nil {
		switch query.(type) {
		case map[string]string:
			queryMap := query.(map[string]string)
			if len(queryMap) > 0 {
				hcQuery := hc.httpURL.Query()
				for k, v := range queryMap {
					if v != "" {
						hcQuery.Add(k, v)
					}
				}
				hc.httpURL.RawQuery = hcQuery.Encode()
			}
		case string:
			queryString := query.(string)
			if queryString != "" {
				if queryValues, err1 := url.ParseQuery(queryString); err1 != nil {
					hc.Error = err1
					return hc
				} else {
					hcQuery := hc.httpURL.Query()
					for k, v := range queryValues {
						if len(v) > 0 {
							for _, s := range v {
								if s != "" {
									hcQuery.Add(k, s)
								}
							}
						}
					}
					hc.httpURL.RawQuery = hcQuery.Encode()
				}
			}
		case []byte:
			queryBytes := query.([]byte)
			if len(queryBytes) > 0 {
				if queryValues, err1 := url.ParseQuery(string(queryBytes)); err1 != nil {
					hc.Error = err1
					return hc
				} else {
					hcQuery := hc.httpURL.Query()
					for k, v := range queryValues {
						if len(v) > 0 {
							for _, s := range v {
								if s != "" {
									hcQuery.Add(k, s)
								}
							}
						}
					}
					hc.httpURL.RawQuery = hcQuery.Encode()
				}
			}
		default:
			queryReflect := reflect.ValueOf(query)
			// 判断是否为结构类型
			if queryReflect.Kind() == reflect.Struct {
				queryMap := hc.reflectStruct(&queryReflect)
				if len(queryMap) > 0 {
					hcQuery := hc.httpURL.Query()
					for k, v := range queryMap {
						if v != "" {
							hcQuery.Add(k, v)
						}
					}
					hc.httpURL.RawQuery = hcQuery.Encode()
				}
			}
		}
	}

	hc.reflectHeader(header)
	hc.header[ContentType] = ApplicationJson
	return hc
}

// HttpPost 发起 POST 请求
// header 头信息，只接受 map[string]string、struct(只接受基本类型)
// query 请求体，只接受 string、[]byte、map[string]string、struct(只接受基本类型)
func HttpPost(httpURL string, header interface{}, body interface{}) *httpClient {
	hc := &httpClient{
		method: http.MethodPost,
		header: make(map[string]string),
	}
	// 生成要访问的 url
	urlParse, err := url.Parse(httpURL)
	if err != nil {
		hc.Error = err
		return hc
	}
	hc.httpURL = urlParse

	// 进行类型断言
	if body != nil {
		switch body.(type) {
		case map[string]string:
			bodyMap := body.(map[string]string)
			if len(bodyMap) > 0 {
				bodyV := url.Values{}
				for k, v := range bodyMap {
					bodyV.Set(k, v)
				}
				hc.body = strings.NewReader(bodyV.Encode())
			}
		case string:
			bodyString := body.(string)
			hc.body = strings.NewReader(bodyString)
		case []byte:
			bodyBytes := body.([]byte)
			hc.body = bytes.NewReader(bodyBytes)
		default:
			bodyReflect := reflect.ValueOf(body)
			// 判断是否为结构类型
			if bodyReflect.Kind() == reflect.Struct {
				bodyMap := hc.reflectStruct(&bodyReflect)
				if len(bodyMap) > 0 {
					bodyV := url.Values{}
					for k, v := range bodyMap {
						bodyV.Set(k, v)
					}
					hc.body = strings.NewReader(bodyV.Encode())
				}
			}
		}
	}

	hc.reflectHeader(header)
	hc.header[ContentType] = ApplicationFormUrlencoded
	return hc
}

// HttpPostJson -
func HttpPostJson(httpURL string, header interface{}, body interface{}) *httpClient {
	var err error
	bodyReflect := reflect.ValueOf(body)
	// 判断是否为结构类型
	if bodyReflect.Kind() == reflect.Struct {
		body, err = json.Marshal(body)
	}

	hc := HttpPost(httpURL, header, body)
	if hc.Error == nil && err != nil {
		hc.Error = errors.New("body request can only be map[string]string or struct")
	}

	hc.header[ContentType] = ApplicationJson
	return hc
}
