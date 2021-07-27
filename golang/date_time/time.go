package date_time

import (
	"fmt"
	"time"
)

// 获取当前时间戳
// time.Now().Unix()
// time.Now().UnixNano() // 包含纳秒数

// 睡眠 1秒
// time.Sleep(time.Second * 1)

// 时间加减操作
// 3分钟后
// time.Now().Add(time.Second * 180).Format("2006-01-02 15:04:05")
// 3分钟前
// time.Now().Add(-(time.Second * 180)).Format("2006-01-02 15:04:05")

// 时间格式化 - Go语言的时间格式化比较奇葩 2006年01月02日15时04分05秒 是固写法
// time.Now().Format("2006-01-02 15:04:05")

// 时间格式化 - 使用 GMT 日期格式表示 ( Mon, 02 Jan 2006 15:04:05 GMT )
// time.Now().UTC().Format(http.TimeFormat)

// 将时间戳转为时间对象
// time.Unix(timStamp, 0)

// 计算运行的时间(计算时间差)(纳秒数)
// now := time.Now()
// time.Now().Sub(now)

// 比较两个时间是否相等
// now := time.Now()
// time.Now().Equal(now)

// 自定义时区
func initTime(){
	// 单次设置
	//var cstZone = time.FixedZone("CST", 8*3600) // 东八区
	//fmt.Println("SH : ", time.Now().In(cstZone).Format("2006-01-02 15:04:05"))

	// 全局设置
	//var cstZone = time.FixedZone("CST", 8*3600) // 东八
	//time.Local = cstZone
}

// TimeParse 时间字符串转换时间
func TimeParse(){
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
		case <- ticker2:
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
