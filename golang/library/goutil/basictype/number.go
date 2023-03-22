package basictype

import (
	"math/big"
	"strconv"
	"strings"
	"time"

	cd "crypto/rand"
	md "math/rand"
)

// GetMRandInt64 生成区间随机数
func GetMRandInt64(min, max int64) int64 {
	//time.Sleep(time.Nanosecond)
	if min >= max || min == 0 || max == 0 {
		return max
	}
	return md.New(md.NewSource(time.Now().UnixNano())).Int63n(max-min) + min
}

// GetCRandInt64 生成区间随机数
func GetCRandInt64(min, max int64) int64 {
	if min >= max || min == 0 || max == 0 {
		return max
	}
	result, _ := cd.Int(cd.Reader, big.NewInt(max-min))
	return result.Int64() + min
}

// Int8ArrayToString 拼接 int8 为 string
func Int8ArrayToString(data []int8, sep string) string {

	dataLen := len(data)
	if dataLen == 0 {
		return ""
	}
	dataLen--

	var sepLen = len(sep)
	var str = strings.Builder{}

	for k, v := range data {
		if k != dataLen {
			str.WriteString(strconv.FormatInt(int64(v), 10))
			if sepLen > 0 {
				str.WriteString(sep)
			}
		} else {
			str.WriteString(strconv.FormatInt(int64(v), 10))
		}
	}

	return str.String()
}

// Uint8ArrayToString 拼接 uint8 为 string
func Uint8ArrayToString(data []uint8, sep string) string {

	dataLen := len(data)
	if dataLen == 0 {
		return ""
	}
	dataLen--

	var sepLen = len(sep)
	var str = strings.Builder{}

	for k, v := range data {
		if k != dataLen {
			str.WriteString(strconv.FormatInt(int64(v), 10))
			if sepLen > 0 {
				str.WriteString(sep)
			}
		} else {
			str.WriteString(strconv.FormatInt(int64(v), 10))
		}
	}

	return str.String()
}

// IntArrayToString 拼接 int 为 string
func IntArrayToString(data []int, sep string) string {

	dataLen := len(data)
	if dataLen == 0 {
		return ""
	}
	dataLen--

	var sepLen = len(sep)
	var str = strings.Builder{}

	for k, v := range data {
		if k != dataLen {
			str.WriteString(strconv.Itoa(v))
			if sepLen > 0 {
				str.WriteString(sep)
			}
		} else {
			str.WriteString(strconv.Itoa(v))
		}
	}

	return str.String()
}

// UintArrayToString 拼接 uint 为 string
func UintArrayToString(data []uint, sep string) string {

	dataLen := len(data)
	if dataLen == 0 {
		return ""
	}
	dataLen--

	var sepLen = len(sep)
	var str = strings.Builder{}

	for k, v := range data {
		if k != dataLen {
			str.WriteString(strconv.FormatInt(int64(v), 10))
			if sepLen > 0 {
				str.WriteString(sep)
			}
		} else {
			str.WriteString(strconv.FormatInt(int64(v), 10))
		}
	}

	return str.String()
}

// Int32ArrayToString 拼接 int32 为 string
func Int32ArrayToString(data []int32, sep string) string {

	dataLen := len(data)
	if dataLen == 0 {
		return ""
	}
	dataLen--

	var sepLen = len(sep)
	var str = strings.Builder{}

	for k, v := range data {
		if k != dataLen {
			str.WriteString(strconv.FormatInt(int64(v), 10))
			if sepLen > 0 {
				str.WriteString(sep)
			}
		} else {
			str.WriteString(strconv.FormatInt(int64(v), 10))
		}
	}

	return str.String()
}

// Uint32ArrayToString 拼接 uint32 为 string
func Uint32ArrayToString(data []uint32, sep string) string {

	dataLen := len(data)
	if dataLen == 0 {
		return ""
	}
	dataLen--

	var sepLen = len(sep)
	var str = strings.Builder{}

	for k, v := range data {
		if k != dataLen {
			str.WriteString(strconv.FormatInt(int64(v), 10))
			if sepLen > 0 {
				str.WriteString(sep)
			}
		} else {
			str.WriteString(strconv.FormatInt(int64(v), 10))
		}
	}

	return str.String()
}

// Int64ArrayToString 拼接 int64 为 string
func Int64ArrayToString(data []int64, sep string) string {

	dataLen := len(data)
	if dataLen == 0 {
		return ""
	}
	dataLen--

	var sepLen = len(sep)
	var str = strings.Builder{}

	for k, v := range data {
		if k != dataLen {
			str.WriteString(strconv.FormatInt(v, 10))
			if sepLen > 0 {
				str.WriteString(sep)
			}
		} else {
			str.WriteString(strconv.FormatInt(v, 10))
		}
	}

	return str.String()
}

// Uint64ArrayToString 拼接 uint64 为 string
func Uint64ArrayToString(data []uint64, sep string) string {

	dataLen := len(data)
	if dataLen == 0 {
		return ""
	}
	dataLen--

	var sepLen = len(sep)
	var str = strings.Builder{}

	for k, v := range data {
		if k != dataLen {
			str.WriteString(strconv.FormatUint(v, 10))
			if sepLen > 0 {
				str.WriteString(sep)
			}
		} else {
			str.WriteString(strconv.FormatUint(v, 10))
		}
	}

	return str.String()
}
