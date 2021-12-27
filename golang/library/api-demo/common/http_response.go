package common

// 响应请求的公共模型
type ResponseModel struct {
	Code Status      `json:"code"` // 响应码
	Msg  string      `json:"msg"`  // 响应信息
	Data interface{} `json:"data"` // 数据
}
