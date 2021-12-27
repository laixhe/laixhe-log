package common

// Status 状态
type Status = int32

// 状态码
const (
	StatusOK       Status = 0
	StatusSign     Status = 1
	StatusSignForm Status = 2
	StatusSignTime Status = 3
	StatusAuth     Status = 4
	StatusForm     Status = 5
)

// 状态码说明
var statusText = map[Status]string{
	StatusOK:       "success",
	StatusSign:     "sign error",
	StatusSignForm: "sign form error",
	StatusSignTime: "sign time error",
	StatusForm:     "form error",
	StatusAuth:     "authorization error",
}

// StatusText 返回状态码错误说明
func StatusText(code Status) string {
	return statusText[code]
}
