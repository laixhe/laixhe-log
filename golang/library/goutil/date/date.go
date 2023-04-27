package date

import (
	"errors"
	"net/http"
	"strings"
	"time"

	"golang_log/library/goutil/match"
)

// (这是个奇葩,必须是这个时间点, 据说是go诞生之日, 记忆方法:6-1-2-3-4-5)
// 2006-01-02 03:04:05 PM
// 2006-01-02 15:04:05
// time.Now().Unix() 获取时间戳

// 时间格式化格式
const (
	DateTimeFormat      = "2006-01-02 15:04:05"
	DateFormat          = "2006-01-02"
	DateShortFormat     = "2006-1-2"
	ShortDateTimeFormat = "20060102150405"
	ShortDateFormat     = "20060102"
	DateTimeFormatB     = "2006/01/02 15:04:05"
	DateFormatB         = "2006/01/02"
	DateShortFormatB    = "2006/1/2"
)

var invalidTimeFormat = errors.New("无效的时间格式！")

// 时间戳
var TimeUnix = time.Now().Unix()

// SameDayTime 一天的时间秒数
const SameDayTime int64 = 86400

// 新时间模型
type newTime struct {
	t time.Time
}

// NewTime 初始化当前时间
func NewTime() *newTime {
	return &newTime{t: time.Now()}
}

// Unix 获取时间戳
func (t *newTime) Unix() int64 {
	return t.t.Unix()
}

// StartUnix 获取当天起始的时间戳
func (t *newTime) StartUnix() int64 {
	return t.StartTime().Unix()
}

// EndUnix 获取当天结束的时间戳
func (t *newTime) EndUnix() int64 {
	return t.EndTime().Unix()
}

// DateFormat 格式化时间 ( 2006-01-02 15:04:05 )
func (t *newTime) DateFormat() string {
	return t.t.Format(DateTimeFormat)
}

// TimeFormat 格式化时间 使用 GMT 日期格式表示 ( Mon, 02 Jan 2006 15:04:05 GMT )
func (t *newTime) TimeFormat() string {
	return t.t.UTC().Format(http.TimeFormat)
}

// StartTime 获取当天的起始的时间戳
func (t *newTime) StartTime() *newTime {
	return &newTime{t: time.Date(t.t.Year(), t.t.Month(), t.t.Day(), 0, 0, 0, 0, t.t.Location())}
}

// EndTime 获取当天的结束的时间戳
func (t *newTime) EndTime() *newTime {
	return &newTime{t: time.Date(t.t.Year(), t.t.Month(), t.t.Day(), 23, 59, 59, 0, t.t.Location())}
}

// YesterdayTime 获取昨天的时间
func (t *newTime) YesterdayTime() *newTime {
	return &newTime{t: t.t.AddDate(0, 0, -1)}
}

// TomorrowTime 获取明天的时间
func (t *newTime) TomorrowTime() *newTime {
	return &newTime{t: t.t.AddDate(0, 0, 1)}
}

// WeekStartTime 获取本周的起始的时间戳
func (t *newTime) WeekStartTime() *newTime {
	// 返回当前星期几
	week := t.t.Weekday()
	i := 0
	switch week {
	case time.Sunday:
		i = 6
	case time.Monday:
		i = 0
	case time.Tuesday:
		i = 1
	case time.Wednesday:
		i = 2
	case time.Thursday:
		i = 3
	case time.Friday:
		i = 4
	case time.Saturday:
		i = 5
	}

	return &newTime{t: time.Date(t.t.Year(), t.t.Month(), t.t.Day()-i, 0, 0, 0, 0, t.t.Location())}
}

// WeekEndTime 获取本周的结束的时间戳
func (t *newTime) WeekEndTime() *newTime {
	// 返回当前星期几
	week := t.t.Weekday()
	i := 0
	switch week {
	case time.Sunday:
		i = 0
	case time.Monday:
		i = 6
	case time.Tuesday:
		i = 5
	case time.Wednesday:
		i = 4
	case time.Thursday:
		i = 3
	case time.Friday:
		i = 2
	case time.Saturday:
		i = 1
	}

	return &newTime{t: time.Date(t.t.Year(), t.t.Month(), t.t.Day()+i, 23, 59, 59, 0, t.t.Location())}
}

// MonthStartTime 获取本月的起始的时间戳
func (t *newTime) MonthStartTime() *newTime {
	return &newTime{t: time.Date(t.t.Year(), t.t.Month(), 1, 0, 0, 0, 0, t.t.Location())}
}

// MonthEndTime 获取本月的结束的时间戳
func (t *newTime) MonthEndTime() *newTime {
	return &newTime{t: time.Date(t.t.Year(), t.t.Month(), 1, 23, 59, 59, 0, t.t.Location()).AddDate(0, 1, -1)}
}

// YearStartTime 获取本年的起始的时间戳
func (t *newTime) YearStartTime() *newTime {
	return &newTime{t: time.Date(t.t.Year(), 1, 1, 0, 0, 0, 0, t.t.Location())}
}

// YearEndTime 获取本年的结束的时间戳
func (t *newTime) YearEndTime() *newTime {
	return &newTime{t: time.Date(t.t.Year(), 12, 31, 23, 59, 59, 0, t.t.Location())}
}

// Add 以秒数改变(增减)时间
func (t *newTime) Add(d int64) *newTime {
	return &newTime{t: t.t.Add(time.Duration(d) * time.Second)}
}

// NewTimeUnix 以时间戳创建时间模型
// ts 时间戳 1611244800
func NewTimeUnix(ts int64) *newTime {
	return &newTime{t: time.Unix(ts, 0)}
}

// NewTimeParse 以字符串的日期时间创建时间模型
// str 字符串的日期时间
// 有效的日期格式：2006-01-02 15:04:05
//
//	2006-01-02
//	2006-1-2
//	2006/01/02 15:04:05
//	2006/01/02
//	2006/1/2
//	20060102150405
//	20060102
//	2006-01-02T15:04:05+08:00
func NewTimeParse(str string) (*newTime, error) {
	if "" == str {
		return nil, invalidTimeFormat
	}
	layout := DateTimeFormat
	strLen := len(str)
	switch strLen {
	case 8:
		if match.IsNumber(str) {
			layout = ShortDateFormat
		} else if strings.Count(str, "-") == 2 {
			layout = DateShortFormat
		} else if strings.Count(str, "/") == 2 {
			layout = DateShortFormatB
		}
	case 9:
		if strings.Count(str, "-") == 2 {
			layout = DateShortFormat
		} else if strings.Count(str, "/") == 2 {
			layout = DateShortFormatB
		}
	case 10:
		if strings.Count(str, "-") == 2 {
			layout = DateFormat
		} else if strings.Count(str, "/") == 2 {
			layout = DateFormatB
		}
	case 14:
		if match.IsNumber(str) {
			layout = ShortDateTimeFormat
		}
	case 19:
		if strings.Count(str, "/") == 2 {
			layout = DateTimeFormatB
		}
	}
	if strings.Index(str, "T") == 10 {
		layout = time.RFC3339
	}
	// 在对应时区转化为 time.time 类型
	parse, err := time.ParseInLocation(layout, str, time.Local)
	if err != nil {
		return nil, err
	}
	return &newTime{t: parse}, nil
}
