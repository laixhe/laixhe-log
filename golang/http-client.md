### http 客户端
> 注意：客户端必须关闭 response 的 body

##### 简单的 GET 请求
```
package main
import (
    "fmt"
    "io/ioutil"
    "net/http"
)

func main() {

    httpUrl := "http://127.0.0.1:5050/?name=laixhe&age=11"
    
    // 发起 http get 请求
    resp, err := http.Get(httpUrl)
    if err != nil {
        fmt.Println("resp:", err)
        return
    }
    defer resp.Body.Close()

    // 获取请求响应内容
    body, err := ioutil.ReadAll(resp.Body)
    if err != nil {
        fmt.Println("body:", err)
        return
    }

    fmt.Println(string(body))
}
```

##### 参数化的 GET 请求
```
package main
import (
    "fmt"
    "io/ioutil"
    "net/http"
    "net/url"
)

func main(){

    httpurl := "http://127.0.0.1:5050"
    u,_ := url.Parse(httpurl)

    // 添加 GET 参数数据
    q := u.Query()
    q.Set("name", "laixhe")
    q.Set("age", "12")

    u.RawQuery = q.Encode()

    // 发起 http get 请求
    resp, err := http.Get(u.String())
    if err != nil {
        fmt.Println("resp:", err)
        return
    }
    defer resp.Body.Close()

    // 获取请求响应内容
    body, err := ioutil.ReadAll(resp.Body)
    if err != nil {
        fmt.Println("body:", err)
        return
    }

    fmt.Println(string(body))
}
```

##### 简单的 POST 请求
```
package main
import (
    "fmt"
    "io/ioutil"
    "net/http"
    "net/url"
    "strings"
)

func main() {

    httpurl := "http://127.0.0.1"
    // 头信息 Header Content-Type
    contentType := "application/x-www-form-urlencoded"
    // 参数 Body
    body := strings.NewReader("name=Laixhe&age=18")

    // 发起 http post 请求
    resp, err := http.Post(httpurl, contentType, body)
    if err != nil {
        fmt.Println(err)
        return
    }
    defer resp.Body.Close()

    // 获取请求响应内容
    data, err := ioutil.ReadAll(resp.Body)
    if err != nil {
        fmt.Println(err)
        return
    }

    fmt.Println(string(data))
}
```

##### 参数化的 POST 请求
```
package main
import (
    "fmt"
    "io/ioutil"
    "net/http"
    "net/url"
)

func main() {

    httpUrl := "http://127.0.0.1"

    // 添加 POST Body 参数
    payload := url.Values{}
    payload.Set("id", "1")
    payload.Set("name", "laixhe")

    // 发起 http post 请求
    resp, err := http.PostForm(httpUrl, payload)
    if err != nil {
        fmt.Println(err)
        return
    }
    defer resp.Body.Close()

    // 获取请求响应内容
    data, err := ioutil.ReadAll(resp.Body)
    if err != nil {
        fmt.Println(err)
        return
    }

    fmt.Println(string(data))
}
```

##### 自义定请求
```
package main
import (
    "fmt"
    "io/ioutil"
    "net/http"
    "net/url"
    "strings"
)

func main() {

    httpUrl := "http://127.0.0.1:5050"
    method := "POST" // GET

    // 添加 POST Body 参数
    //payload := strings.NewReader(`"id":1,"name":"laixhe"`)
    payload := url.Values{}
    payload.Set("id", "1")
    payload.Set("name", "laixhe")
    bodyData := strings.NewReader(payload.Encode())

    // 准备发起 http 请求
    req, err := http.NewRequest(method, httpUrl, bodyData)
    if err != nil {
        fmt.Println(err)
        return
    }

    // 添加头信息
    req.Header.Add("Content-Type", "application/json")

    // 添加 GET 参数数据
    query := req.URL.Query()
    query.Add("age", "18")
    query.Add("set", "men")
    req.URL.RawQuery = query.Encode()

    // 生成 client 参数为默认
    client := &http.Client{}
    // 发起 http 请求
    res, err := client.Do(req)
    defer res.Body.Close()

    // 获取请求响应内容
    body, err := ioutil.ReadAll(res.Body)

    fmt.Println(string(body))
}
```
