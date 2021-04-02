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