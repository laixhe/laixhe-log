package main

import (
	"fmt"
	"net/http"
	"testing"
)

// net/http 提供了基础的 Web 功能，即监听端口，映射静态路由，解析 HTTP 报文

// 最简单的 HTTP 服务
func TestHttpServe(t *testing.T) {
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintf(w, "Hello Go HTTP")
	})
	http.ListenAndServe(":6060", nil)

	// HandleFunc 注册路由
	// ResponseWriter 用于返回响应
	// Request 表示请求数据
	// ListenAndServe 启动服务器
	//
	// http://localhost:6060
}

// Handler 机制
// 这种设计让代码更加灵活，可以实现中间件、路由系统...

type MyHandler struct{}

func (h MyHandler) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	w.Write([]byte("custom handler"))
}

func TestHttpServeHandler(t *testing.T) {
	http.Handle("/", MyHandler{})
	http.ListenAndServe(":6060", nil)
}

// 请求处理
// 返回响应
func TestHttpServeRequestResponse(t *testing.T) {
	http.HandleFunc("/get", func(w http.ResponseWriter, r *http.Request) {
		name := r.URL.Query().Get("name")
		fmt.Fprintf(w, "http get name=%s", name)
	})
	http.HandleFunc("/post", func(w http.ResponseWriter, r *http.Request) {
		r.ParseForm()
		name := r.FormValue("name")
		fmt.Fprintf(w, "http post name=%s", name)

		// 设置响应头
		// w.Header().Set("Content-Type", "application/json")
		// 设置状态码
		// w.WriteHeader(http.StatusOK)
	})
	http.ListenAndServe(":6060", nil)
}

// http.ServeMux 是 Go 默认提供的多路复用器（路由器），它实现了 Handler 接口，可以看作是一个高级的路由管理器
func TestHttpServeMux(t *testing.T) {
	mux := http.NewServeMux()
	mux.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {})
	http.ListenAndServe(":6060", mux)
}

// 简单文件(静态)服务器
func TestHttpServeFile(t *testing.T) {
	http.Handle("/", http.FileServer(http.Dir("./static")))
	http.ListenAndServe(":6060", nil)
}

/*
#### 简单文件(静态)服务器 embed
```
package main
import (
	"embed"
	"net/http"
)

// 加载 static 目录所有文件
//go:embed static
var static embed.FS

func main() {
    //http.StripPrefix("/static", http.FileServer(http.FS(fs)))
	err := http.ListenAndServe(":6060", http.FileServer(http.FS(static)))
	if err != nil {
		print(err)
	}
}
```
*/

/*
#### 简单加载模板文件 embed
```
package main
import (
	"embed"
	"html/template"
	"net/http"
)

// 加载 templates 目录所有文件
//go:embed templates
var tmpl embed.FS

func main() {
	t, _ := template.ParseFS(tmpl, "templates/*.tmpl")

	http.HandleFunc("/", func(rw http.ResponseWriter, r *http.Request) {
		t.ExecuteTemplate(rw,"index.tmpl",map[string]string{"title":"Golang Embed..."})
	})

	err := http.ListenAndServe(":80",nil)
	if err != nil {
		print(err)
	}
}
```
*/
