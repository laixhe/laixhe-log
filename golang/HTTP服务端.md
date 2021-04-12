#### 使用标准库 net/http
> ```net/http``` 提供了基础的 Web 功能，即监听端口，映射静态路由，解析 HTTP 报文
> 
> http://localhost:5050
```
package main
import (
	"fmt"
	"log"
	"net/http"
)
func main() {
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintf(w, "url:%q\n", r.URL.Path)
	})
	if err := http.ListenAndServe(":5050", nil); err != nil {
		log.Panic(err)
	}
}
```

#### 实现 http.Handler 接口
```
package main
import (
	"fmt"
	"log"
	"net/http"
)
type Engine struct{}
func (engine *Engine) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	switch r.URL.Path {
	case "/":
		fmt.Fprintf(w, "url:%q\n", r.URL.Path)
	default:
		fmt.Fprintf(w, "404 NOT FOUND: %s\n", r.URL)
	}
}
func main() {
	engine := new(Engine)
	if err := http.ListenAndServe(":5050", engine); err != nil {
		log.Panic(err)
	}
}
```


#### 简单文件(静态)服务器
```
package main

import (
    "net/http"
)

func main() {
    // 设置文件服务
    http.Handle("/", http.FileServer(http.Dir("./")))
    err := http.ListenAndServe(":80", nil)
    if err != nil {
        panic(err)
    }
}
```