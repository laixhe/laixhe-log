package main

import (
	"fmt"
	"net/http"
	"time"
)

func TimeRun() {

	// 以本地机器的时区(当前服务器本地时区)

	// 常量：时 分 秒
	// time.Second
	// time.Minute
	// time.Hour

	// 常量：时区 (全球有 24 个时区)
	// time.UTC
	//
	// 时区的含义
	// UTC 世界协调时间(世界标准时间、世界统一时间)
	// GMT 格林威治标准时间(UTC=GMT)
	// CST 代表 4 个不同的时区[ UT-6:00(美国|中部标准时间) UT+9:30(澳大利亚) UT+8:00(中国) UT-4:00(古巴) ]
	// PRC 中国 (Asia/Shanghai)

	// 常量：格式化
	// time.Layout

	// 获取当前时间对象
	timeNow := time.Now()
	fmt.Println("获取当前时间对象:", timeNow)
	// 结果：2021-12-23 14:22:47.5173704 +0800 CST m=+0.008920401

	// 获取当前时间戳
	timeUnix := time.Now().Unix()
	fmt.Println("获取当前时间戳:", timeUnix)
	// 结果：1640240567
	timeUnixNano := time.Now().UnixNano()
	fmt.Println("获取当前时间戳(包含纳秒数):", timeUnixNano)
	// 结果：1640240567555057000

	// 时间格式化 - Go语言的时间格式化比较奇葩 2006年01月02日15时04分05秒 是固写法 (2006 01 02 15 04 05)
	timeFormat := time.Now().Format("2006-01-02 15:04:05")
	fmt.Println("当前时间格式化:", timeFormat)
	// 结果：2021-12-23 14:22:47

	// 时间格式化 - 使用 GMT 日期格式表示 ( Mon, 02 Jan 2006 15:04:05 GMT )
	timeGMTFormat := time.Now().Format(http.TimeFormat)
	fmt.Println("当前时间GMT格式化:", timeGMTFormat)
	// 结果：Thu, 23 Dec 2021 14:22:47 GMT

	// 时间格式化 - 使用 RFC3339 日期格式表示 ( 2006-01-02T15:04:05Z07:00 )
	timeRFC3339Format := time.Now().Format(time.RFC3339)
	fmt.Println("当前时间RFC3339格式化:", timeRFC3339Format)
	// 结果：2021-12-23T14:22:47+08:00

	// 将时间戳转为时间对象
	timeUnixNow := time.Unix(timeUnix, 0)
	fmt.Println("将时间戳转为时间对象:", timeUnixNow)
	// 结果：2021-12-23 14:22:47 +0800 CST

	// 时间加减操作
	// 3分钟后
	addTime1 := time.Now().Add(time.Second * 180).Format("2006-01-02 15:04:05")
	fmt.Println("3分钟后:", addTime1)
	// 结果：2021-12-23 14:25:47

	// 3分钟前
	addTime2 := time.Now().Add(-(time.Second * 180)).Format("2006-01-02 15:04:05")
	fmt.Println("3分钟前:", addTime2)
	// 结果：2021-12-23 14:19:47

	// 计算运行的时间(计算时间差)(纳秒数)
	nowUnixTime := time.Unix(1640240567, 0)
	timeSub := time.Now().Sub(nowUnixTime)
	fmt.Printf("计算运行的时间相差 %d 秒\n", timeSub/time.Second)
	// 结果：计算运行的时间相差 2127 秒

	// 比较两个时间是否相等
	// now := time.Now()
	// time.Now().Equal(now)

	// 睡眠 1秒
	// time.Sleep(time.Second * 1)
}

// InitTime 自定义时区
func InitTime() {
	// 单次设置
	var cstZone = time.FixedZone("CST", 8*3600) // 东八区
	fmt.Println("自定义时区:", time.Now().In(cstZone).Format("2006-01-02 15:04:05"))

	// 全局设置
	//var cstZone = time.FixedZone("CST", 8*3600) // 东八区
	//time.Local = cstZone
}

// TimeUTC utc
func TimeUTC() {
	timeNow := time.Now()
	fmt.Println("当前本地时间", timeNow)
	timeUnix := timeNow.Unix()
	fmt.Println("获取本地时间戳:", timeUnix)

	timeUTC := time.Now().UTC()
	fmt.Println("当前UTC时间", timeUTC)

	timeUTCFormat := timeUTC.Format("2006-01-02 15:04:05")
	fmt.Println("当前UTC时间格式化", timeUTCFormat)

	timeUTCUnix := timeUTC.Unix()
	fmt.Println("获取UTC本地时间戳:", timeUTCUnix)
}

// TimeParse 时间字符串转换时间
func TimeParse() {
	// time.Parse() 的默认时区是 UTC , time.Format() 的时区默认是本地时区，两个不同的时区转换后时间是不对的
	// 时间字符串转换时间戳 - 先用 time.Parse 对时间字符串进行分析，如果正确会得到一个 time.Time 对象
	parse, _ := time.Parse("2006-01-02 15:04:05", "2018-01-06 16:12:00")
	fmt.Println(parse.Unix()) // 1515255120  2018-01-07 00:12:00

	// 使用 time.ParseInLocation() 而不是 time.Parse()，因为采用了本地时区的
	localTime, _ := time.ParseInLocation("2006-01-02 15:04:05", "2018-01-06 16:12:00", time.Local)
	fmt.Println(localTime.Unix()) // 1515226320  2018-01-06 16:12:00
}

// TimeTick1 定时器
func TimeTick1() {

	// 定时器 - 每隔1秒运行一次，返回时间管道 (周期性)
	ticker1 := time.Tick(time.Second * 1)
	// 定时器 - 3秒后运行一次，返回时间管道 (单次)
	ticker2 := time.After(time.Second * 3)

BreakRor:
	for {
		select {
		case <-ticker1:
			fmt.Println("周期性:", time.Now().Unix())
		case <-ticker2:
			fmt.Println("单次:", time.Now().Unix())
			break BreakRor // 跳出 for
		}
	}

	fmt.Println("定时器结束...")
}

// TimeTick2 定时器
func TimeTick2() {

	// 周期性
	timer1 := time.NewTicker(time.Second * 1)
	// 单次
	timer2 := time.NewTimer(time.Second * 1)

	go func() {
	BreakRor:
		for {
			select {
			case <-timer1.C:
				fmt.Println("周期性:", time.Now().Unix())
			case <-timer2.C:
				fmt.Println("单次:", time.Now().Unix())
				break BreakRor // 跳出 for
			}
		}

		fmt.Println("定时器结束...")
	}()

	// 定时器重置 (只能用于 单次 的)
	timer2.Reset(time.Second * 4)

	// 睡眠 N秒
	time.Sleep(time.Second * 2)

	// 停止定时器
	timer1.Stop()

	// 睡眠 N秒
	time.Sleep(time.Second * 10) // 用于延迟程序的退出
}
