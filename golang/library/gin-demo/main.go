package main

import (
	"log"
	"time"

	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
	"github.com/laixhe/goutil/zaplog"
)

func main() {

	gin.SetMode(gin.DebugMode)

	//r := gin.Default()
	r := gin.New()

	// 中间件
	//r.Use(gin.Logger())
	//r.Use(gin.Recovery())
	r.Use(zaplog.GinLogger())
	r.Use(zaplog.GinRecovery())
	r.Use(Cors())

	r.GET("/ping", ping)
	r.GET("/user/:name", userName)
	r.GET("/user_name", getUserName)

	r.POST("/user_name", postUserName)
	r.POST("/user_names", postUserNames)
	r.POST("/form_file", formFile)   // 单个文件上传
	r.POST("/form_files", formFiles) // 多个文件上传

	group := r.Group("/group")
	group.POST("/json", groupJson) // 接收 json 数据

	err := r.Run(":5501")
	if err != nil {
		log.Fatal(err)
	}
}

// 跨域中间件
func Cors() gin.HandlerFunc {
	return cors.New(cors.Config{
		AllowOrigins:     []string{"*"},                                       // 准许跨域请求网站,多个使用,分开,限制使用* [Access-Control-Allow-Origin]
		AllowMethods:     []string{"GET","POST","DELETE","PUT","OPTIONS"},     // 设置允许请求的方法 [Access-Control-Allow-Methods]
		AllowHeaders:     []string{"Origin", "Content-Type", "Authorization"}, // 设置允许请求的头信息 [Access-Control-Allow-Headers]
		//ExposeHeaders:    []string{"Origin", "Content-Type", "Content-Length", "Authorization"},// 显示的请求表头(可选) [Access-Control-Expose-Headers]
		AllowCredentials: true,                                                // 是否允许需要带 cookie 信息 [Access-Control-Allow-Credentials]
		// 容许跨域的原点网站,可以直接 return true 就万事大吉了
		AllowOriginFunc: func(origin string) bool {
			//return origin == "https://github.com"
			return true
		},
		MaxAge: 12 * time.Hour, // [Access-Control-Max-Age]
	})
}

// curl -X GET http://localhost:4401/ping
// {"message":"pong"}
func ping(c *gin.Context) {
	c.JSON(200, gin.H{
		"message": "pong",
	})
}

// curl -X GET http://localhost:4401/user/laixhe
// {"name":"laixhe"}
func userName(c *gin.Context) {
	name := c.Param("name")
	c.JSON(200, gin.H{
		"name": name,
	})
}

// curl -X GET http://localhost:4401/user_name?name=laixhe
// {"name":"laixhe"}
func getUserName(c *gin.Context) {
	name := c.Query("name") // DefaultQuery
	c.JSON(200, gin.H{
		"name": name,
	})
}

// curl -X POST http://localhost:4401/user_name -d "name=laixhe"
// {"name":"laixhe"}
func postUserName(c *gin.Context) {
	name := c.PostForm("name") // DefaultPostForm
	c.JSON(200, gin.H{
		"name": name,
	})
}

// curl -X POST http://localhost:4401/user_names -d "name_arr=laixhe&name_arr=laiki&name_map[laixhe]=laixhe0&name_map[laiki]=laiki0"
// {"name_arr":["laixhe","laiki"],"name_map":{"laiki":"laiki0","laixhe":"laixhe0"}}
func postUserNames(c *gin.Context) {

	name_arr := c.PostFormArray("name_arr")
	name_map := c.PostFormMap("name_map")
	c.JSON(200, gin.H{
		"name_arr": name_arr,
		"name_map": name_map,
	})
}

// 单个文件上传
// curl -X POST http://localhost:4401/form_file -H "Content-Type:multipart/form-data" -F "form_file=@2.pdf"
// {"file":"2.pdf"}
func formFile(c *gin.Context) {

	file, err := c.FormFile("form_file") // c.Request.FormFile
	if err != nil {
		c.JSON(200, gin.H{
			"form_file_err": err,
		})
		return
	}

	// 将文件存到本地
	err = c.SaveUploadedFile(file, "./"+file.Filename)
	if err != nil {
		c.JSON(200, gin.H{
			"save_uploaded_file": err,
		})
		return
	}

	c.JSON(200, gin.H{
		"file": file.Filename,
	})
}

// 多个文件上传
// curl -X POST http://localhost:4401/form_files -H "Content-Type:multipart/form-data" -F "form_file[]=@2.pdf" -F "form_file[]=@111.sql"
// {"errs":null,"files":["2.pdf","111.sql"]}
func formFiles(c *gin.Context) {

	var fileNames []string
	var errs []error

	form, _ := c.MultipartForm()
	files := form.File["form_file[]"]

	for _, file := range files {
		// 将文件存到本地
		err := c.SaveUploadedFile(file, "./"+file.Filename)
		if err != nil {
			errs = append(errs, err)
		}

		fileNames = append(fileNames, file.Filename)
	}

	c.JSON(200, gin.H{
		"files": fileNames,
		"errs":  errs,
	})
}

// 接收 json 数据
// curl -X POST http://localhost:4401/group/json -H "Content-Type: application/json" -d '{"name":"laixhe","age":18}'
// {"user":{"name":"laixhe","age":18}}
type User struct {
	Name string `json:"name" form:"name" binding:"required"`
	Age  int    `json:"age" form:"age" binding:"required"`
}

func groupJson(c *gin.Context) {

	var user User
	err := c.ShouldBindJSON(&user) // ShouldBind ShouldBindQuery 绑定表单form
	if err != nil {
		c.JSON(200, gin.H{
			"bind_err": err.Error(),
		})
		return
	}

	c.JSON(200, gin.H{
		"user": user,
	})
}
