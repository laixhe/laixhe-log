### 正则相关
> regexp.MustCompile 编译正则表达式，如果失败直接 panic（适用于全局变量初始化）
>
> regexp.Compile     编译正则表达式，如果失败返回错误信息

```
package main
import (
	"fmt"
	"regexp"
)
// 将 [正则操作什么] 替换为 [正则操作替换]
func main() {
	reg := regexp.MustCompile("什么")
    // 替换所有匹配项
	str := reg.ReplaceAllString("正则操作什么", "替换")
	fmt.Println(str) // 结果: 正则操作替换
}
```

#### 中英文之间加空格
```
package main
import (
	"log"
	"regexp"
)
func main() {
	text := "中a英bc文之空格"
	g1 := regexp.MustCompile("([\u4e00-\u9fa5])(\\w)")
	g2 := regexp.MustCompile("(\\w)([\u4e00-\u9fa5])")
	g3 := g2.ReplaceAllString(g1.ReplaceAllString(text, "$1 $2"), "$1 $2")
	log.Println(g3) // 结果: 中 a 英 bc 文之空格
}
```

#### 获取匹配的内容
```
package main
import (
	"fmt"
	"regexp"
)
func main() {
	str := "abc azc a7c aac 888 a9c taa1c"
	reg1 := regexp.MustCompile(`a\dc`)
    // 查找所有匹配的字符串
	strs := reg1.FindAllString(str, -1)
	fmt.Println(strs) // 结果: [a7c a9c a1c]
}
```

#### 常用正则
```
package main
import "regexp"

var (
	emailExp  = regexp.MustCompile(`^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$`)
	mobileExp = regexp.MustCompile(`^1[0-9]{10}$`)
	numberExp = regexp.MustCompile(`^[0-9]+$`)
)

// MatchEmail 是否为邮箱
func MatchEmail(s string) bool {
	if s == "" {
		return false
	}
	return emailExp.MatchString(s)
}

// MatchMobile 是否为手机号
func MatchMobile(s string) bool {
	if s == "" {
		return false
	}
	return mobileExp.MatchString(s)
}

// MatchNumber 是否为数整数
func MatchNumber(s string) bool {
	if s == "" {
		return false
	}
	return numberExp.MatchString(s)
}
```
