### 时间相关
> 以本地机器的时区(当前服务器本地时区)
> 
> 常量：时 分 秒
> > time.Second
> > time.Minute
> > time.Hour
>
> 常量：时区 (全球有 24 个时区)
> > time.UTC
> > UTC 世界协调时间(世界标准时间、世界统一时间)
> > GMT 格林威治标准时间(UTC = GMT)
> > CST 代表 4 个不同的时区[ UT-6:00(美国|中部标准时间) UT+9:30(澳大利亚) UT+8:00(中国) UT-4:00(古巴) ]
> > PRC 中国 (Asia/Shanghai)
>
> 常量：格式化
> > time.Layout

#### 基本使用
```
package main

import (
	"fmt"
	"net/http"
	"time"
)

func main() {
	// 获取当前时间对象
	timeNow := time.Now()
	// 获取当前时间戳 (不管 Time 变量存储的是什么时区，其 Unix() 方法返回的都是距离 UTC 时间戳)
	timeUnix := timeNow.Unix()
	// 时间格式化 - Go语言的时间格式化比较奇葩 2006年01月02日15时04分05秒 是固写法 (2006 01 02 15 04 05)
	timeFormat := timeNow.Format("2006-01-02 15:04:05")
	// 时间格式化 - 使用 GMT 日期格式表示 ( Mon, 02 Jan 2006 15:04:05 GMT )
	timeGMTFormat := timeNow.Format(http.TimeFormat)
    // 将时间戳转为时间对象
	timeUnixNow := time.Unix(timeUnix, 0)

	fmt.Println("获取当前时间对象:", timeNow)          // 结果: 2025-06-21 17:18:39.1714361 +0800 CST m=+0.002067001
	fmt.Println("获取当前时间戳:", timeUnix)          // 结果: 1750497519
	fmt.Println("当前时间格式化:", timeFormat)        // 结果: 2025-06-21 17:18:39
	fmt.Println("当前时间GMT格式化:", timeGMTFormat)  // 结果: Sat, 21 Jun 2025 17:18:39 GMT
    fmt.Println("将时间戳转为时间对象:", timeUnixNow)  // 结果: 2025-06-21 17:18:39 +0800 CST
}
```

#### 时间字符串转换时间
```
package main

import (
	"fmt"
	"time"
)

func main() {
	// time.Parse() 的默认时区是 UTC , time.Format() 的时区默认是本地时区，两个不同的时区转换后时间是不对的
	parse, _ := time.Parse("2006-01-02 15:04:05", "2018-01-06 16:12:00")
	fmt.Println(parse.Unix()) // 结果: 1515255120

	// 使用 time.ParseInLocation() 而不是 time.Parse()，因为采用了本地时区的
	localTime, _ := time.ParseInLocation("2006-01-02 15:04:05", "2018-01-06 16:12:00", time.Local)
	fmt.Println(localTime.Unix()) // 结果: 1515226320
}
```

#### 自定义时区
```
package main

import (
	"fmt"
	"time"
)

func main() {
	// 单次设置
	var cstZone = time.FixedZone("CST", 8*3600) // 东八区
	fmt.Println("自定义时区1:", time.Now().In(cstZone).Format("2006-01-02 15:04:05"))

	// 全局设置
	//var cstZone = time.FixedZone("CST", 8*3600) // 东八区
	//time.Local = cstZone

	// 单次设置(不推荐)
	var cstSh, _ = time.LoadLocation("Asia/Shanghai") // 亚洲/上海 Asia/Shanghai 不能写错，区分大小写
	fmt.Println("自定义时区2:", time.Now().In(cstSh).Format("2006-01-02 15:04:05"))
	// LoadLocation 有个问题，它依赖于 IANA Time Zone Database (tzdata) 这个数据库
	// 一般 linux 系统都带了，但是 windows 系统就没带。
	// 没有 tzdata 就会从 $GOROOT/lib/time/ZONEINFO.zip 中找。对于没有安装 go 环境的 windows 系统来讲，就没办法经过 LoadLocation 设置时区

	// Asia/Shanghai       亚洲/上海
	// Asia/Chongqing      亚洲/重庆
}
```

#### 定时器 1
```
package main

import (
	"fmt"
	"time"
)

func main() {
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
```

#### 定时器 2
```
package main

import (
	"fmt"
	"time"
)

func main() {
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
```
