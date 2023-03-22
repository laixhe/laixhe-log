package basictype

import (
	"errors"
	"math/rand"
	"strconv"
	"strings"
	"time"
)

var RandomString = []byte("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz")
var RandomStringToLower = []byte("0123456789abcdefghijklmnopqrstuvwxyz")
var RandomStringInt = []byte("0123456789")

// Substr 字符串截取
func Substr(str string, start, length int) string {
	rs := []rune(str)
	rl := len(rs)
	end := 0

	if start < 0 {
		start = rl - 1 + start
	}
	end = start + length

	if start > end {
		start, end = end, start
	}

	if start < 0 {
		start = 0
	}
	if start > rl {
		start = rl
	}
	if end < 0 {
		end = 0
	}
	if end > rl {
		end = rl
	}

	return string(rs[start:end])
}

// GetRandomString 生成随机字符串
// l int 随机的长度
// is int 0=不包含大写 1=包含大写 2=只有数字 (默认为 0)
func GetRandomString(l int, is ...int) string {

	bytes := RandomStringToLower
	if len(is) > 0 {
		if is[0] == 1 {
			bytes = RandomString
		}
		if is[0] == 2 {
			bytes = RandomStringInt
		}
	}

	bytesLen := len(bytes)
	result := make([]byte, 0, l)

	r := rand.New(rand.NewSource(time.Now().UnixNano()))
	for i := 0; i < l; i++ {
		result = append(result, bytes[r.Intn(bytesLen)])
	}
	return string(result)
}

// StringToInt 字符串转整型
func StringToInt(s string) int {
	if s == "" {
		return 0
	}
	id, err := strconv.ParseInt(s, 10, 0)
	if err != nil {
		return 0
	}
	return int(id)
}

// StringToUint 字符串转整型
func StringToUint(s string) uint {
	if s == "" {
		return 0
	}
	id, err := strconv.ParseUint(s, 10, 64)
	if err != nil {
		return 0
	}
	return uint(id)
}

// StringToInt32 字符串转整型
func StringToInt32(s string) int32 {
	if s == "" {
		return 0
	}
	id, err := strconv.ParseInt(s, 10, 0)
	if err != nil {
		return 0
	}
	return int32(id)
}

// StringToUint32 字符串转整型
func StringToUint32(s string) uint32 {
	if s == "" {
		return 0
	}
	id, err := strconv.ParseUint(s, 10, 64)
	if err != nil {
		return 0
	}
	return uint32(id)
}

// StringToInt64 字符串转整型
func StringToInt64(s string) int64 {
	if s == "" {
		return 0
	}
	id, err := strconv.ParseInt(s, 10, 64)
	if err != nil {
		return 0
	}
	return id
}

// StringToUint64 字符串转整型
func StringToUint64(s string) uint64 {
	if s == "" {
		return 0
	}
	id, err := strconv.ParseUint(s, 10, 64)
	if err != nil {
		return 0
	}
	return id
}

// StringToFloat 字符串转浮点型
func StringToFloat(s string) float64 {
	if s == "" {
		return 0
	}
	f, err := strconv.ParseFloat(s, 64)
	if err != nil {
		return 0
	}
	return f
}

// StringToBool 字符串转浮布尔
func StringToBool(s string) bool {
	if s == "" {
		return false
	}
	b, _ := strconv.ParseBool(s)
	return b
}

// StringToInt8Array 分割字符串为 int8 数组
func StringToInt8Array(s, sep string) ([]int8, error) {
	if s == "" {
		return nil, nil
	}

	if sep == "" {
		return nil, errors.New("StringToInt8Array err: sep empty")
	}

	split := strings.Split(s, sep)
	data := make([]int8, 0, len(split))

	for _, v := range split {
		i, err := strconv.Atoi(v)
		if err != nil {
			return nil, errors.New("StringToInt8Array err: " + err.Error())
		}

		data = append(data, int8(i))
	}

	return data, nil
}

// StringToUint8Array 分割字符串为 uint8 数组
func StringToUint8Array(s, sep string) ([]uint8, error) {
	if s == "" {
		return nil, nil
	}

	if sep == "" {
		return nil, errors.New("StringToUint8Array err: sep empty")
	}

	split := strings.Split(s, sep)
	data := make([]uint8, 0, len(split))

	for _, v := range split {
		i, err := strconv.Atoi(v)
		if err != nil {
			return nil, errors.New("StringToUint8Array err: " + err.Error())
		}

		data = append(data, uint8(i))
	}

	return data, nil
}

// StringToIntArray 分割字符串为 int 数组
func StringToIntArray(s, sep string) ([]int, error) {
	if s == "" {
		return nil, nil
	}

	if sep == "" {
		return nil, errors.New("StringToIntArray err: sep empty")
	}

	split := strings.Split(s, sep)
	data := make([]int, 0, len(split))

	for _, v := range split {
		i, err := strconv.Atoi(v)
		if err != nil {
			return nil, errors.New("StringToIntArray err: " + err.Error())
		}

		data = append(data, i)
	}

	return data, nil
}

// StringToUintArray 分割字符串为 uint 数组
func StringToUintArray(s, sep string) ([]uint, error) {
	if s == "" {
		return nil, nil
	}

	if sep == "" {
		return nil, errors.New("StringToUintArray err: sep empty")
	}

	split := strings.Split(s, sep)
	data := make([]uint, 0, len(split))

	for _, v := range split {
		i, err := strconv.ParseUint(v, 10, 32)
		if err != nil {
			return nil, errors.New("StringToUintArray err: " + err.Error())
		}

		data = append(data, uint(i))
	}

	return data, nil
}

// StringToInt32Array 分割字符串为 int32 数组
func StringToInt32Array(s, sep string) ([]int32, error) {
	if s == "" {
		return nil, nil
	}

	if sep == "" {
		return nil, errors.New("StringToInt32Array err: sep empty")
	}

	split := strings.Split(s, sep)
	data := make([]int32, 0, len(split))

	for _, v := range split {
		i, err := strconv.ParseInt(v, 10, 32)
		if err != nil {
			return nil, errors.New("StringToInt32Array err: " + err.Error())
		}

		data = append(data, int32(i))
	}

	return data, nil
}

// StringToUint32Array 分割字符串为 uint32 数组
func StringToUint32Array(s, sep string) ([]uint32, error) {
	if s == "" {
		return nil, nil
	}

	if sep == "" {
		return nil, errors.New("StringToUint32Array err: sep empty")
	}

	split := strings.Split(s, sep)
	data := make([]uint32, 0, len(split))

	for _, v := range split {
		i, err := strconv.ParseUint(v, 10, 32)
		if err != nil {
			return nil, errors.New("StringToUint32Array err: " + err.Error())
		}

		data = append(data, uint32(i))
	}

	return data, nil
}

// StringToInt64Array 分割字符串为 int64 数组
func StringToInt64Array(s, sep string) ([]int64, error) {
	if s == "" {
		return nil, nil
	}

	if sep == "" {
		return nil, errors.New("StringToInt64Array err: sep empty")
	}

	split := strings.Split(s, sep)
	data := make([]int64, 0, len(split))

	for _, v := range split {
		i, err := strconv.ParseInt(v, 10, 64)
		if err != nil {
			return nil, errors.New("StringToInt64Array err: " + err.Error())
		}

		data = append(data, i)
	}

	return data, nil
}

// StringToUint64Array 分割字符串为 uint64 数组
func StringToUint64Array(s, sep string) ([]uint64, error) {
	if s == "" {
		return nil, nil
	}

	if sep == "" {
		return nil, errors.New("StringToUint64Array err: sep empty")
	}

	split := strings.Split(s, sep)
	data := make([]uint64, 0, len(split))

	for _, v := range split {
		i, err := strconv.ParseUint(v, 10, 64)
		if err != nil {
			return nil, errors.New("StringToUint64Array err: " + err.Error())
		}

		data = append(data, i)
	}

	return data, nil
}
