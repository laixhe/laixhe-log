### 关于命令行参数处理相关

##### 一、获取程序启动时参数 os.Args
> os.Args 是一个 string 的切片，用来存储所有的命行参数(以空格分隔)

> 从索引 1 开始(索引 0 放的是程序本身的名字)

```
package main
import (
    "fmt"
    "os"
)

func main() {
    // 程序本身的名字 // args[0]=args
    fmt.Println("args[0]=", os.Args[0])

    if len(os.Args) > 1 {
        for key, value := range os.Args {
            if key == 0 {
                continue
            }
            // 第二个参数开始，是用户输入的参数 // args[1]=test
            fmt.Printf("args[%d]=%v\n", key, value)
        }
    }
}

go build -o args
./args test
```

##### 二、获取程序启动时参数 flag
> 需要自定义要获取的参数名称、参数默认值、参数使用方法说明，返回的是指针类型

> 在程序要解析参数时，必须先调用 flag 包的 Parse 函数，就会自动解析参数

```
package main
import (
    "flag"
    "fmt"
)

// 将命令行的参数解析到全局变量
var (
    mode = flag.String("mode", "", "运行模式")
    port = flag.Int("port", 5050, "服务端监听端口")
    file = flag.String("file", "", "文件地址名称")
)

func main(){
    // (必须)解析命令行的参数
    flag.Parse()
    // 打印命令行接收到的参数数据 // dev 5050 test.txt
    fmt.Println(*mode, *port, *file)
}

go build -o flag
./flag -mode dev -port 5050 -file ./test.txt
```

##### 三、交互模式-读取用户的输入

> 用 fmt.Scanf 来捕捉用户输入

> 用 bufio.NewReader 来捕捉用户输入

```
package main
import "fmt"

// 用户
var username string
// 密码
var password string

func main() {

    fmt.Println("请输入用户名:")
    if _, err := fmt.Scanf("%s\n", &username); err != nil {
        fmt.Printf("%s\n", err)
        return
    }

    fmt.Println("请输入密码:")
    if _, err := fmt.Scanf("%s\n", &password); err != nil {
        fmt.Printf("%s\n", err)
        return
    }

    fmt.Printf("参数值:username=%v password=%v\n", username, password)
}
```

```
package main
import (
    "bufio"
    "fmt"
    "os"
    "strings"
)

func main() {

    // 创建一个读取器，并将其与标准输入绑定
    reader := bufio.NewReader(os.Stdin)

    fmt.Println("请输入用户名")
    text, _ := reader.ReadString('\n')
    username := strings.TrimSpace(text)

    fmt.Println("请输入密码")
    text, _ = reader.ReadString('\n')
    password := strings.TrimSpace(text)

    fmt.Printf("参数值:username=%v password=%v\n", username, password)
}
```

```
go build -o test
./test
请输入用户名:
laixhe
请输入密码:
12345
参数值:username=laixhe password=12345
```
