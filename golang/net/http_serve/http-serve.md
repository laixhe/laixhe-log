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
	err := http.ListenAndServe(":80", http.FileServer(http.FS(static)))
	if err != nil {
		print(err)
	}
}
```

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

#### 简单 Gin 静态文件服务 embed
```
package main
import (
	"embed"
	"net/http"
	"github.com/gin-gonic/gin"
)

// 加载 static 目录所有文件
//go:embed static
var static embed.FS

func main() {
	r := gin.Default()
	//r.StaticFS("/", http.FS(static))
	
	r.Any("/*filepath", func(c *gin.Context) {
		staticServer := http.FileServer(http.FS(fs))
		staticServer.ServeHTTP(c.Writer, c.Request)
	})

	err := r.Run(":80")
	if err != nil {
		print(err)
	}
}

```

#### 简单 Gin HTML 模板 embed
```
package main
import (
	"embed"
	"html/template"
	"github.com/gin-gonic/gin"
)

// 加载 templates 目录所有文件
//go:embed templates
var tmpl embed.FS

func main() {
	r := gin.Default()

	t, _ := template.ParseFS(tmpl, "templates/*.tmpl")
	r.SetHTMLTemplate(t)

	r.GET("/", func(ctx *gin.Context) {
		ctx.HTML(200, "index.tmpl", gin.H{"title": "Golang Embed..."})
	})

	err := r.Run(":80")
	if err != nil {
		print(err)
	}
}
```
