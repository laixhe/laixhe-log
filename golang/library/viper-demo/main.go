package main

import (
	"fmt"

	"github.com/spf13/viper"
)

// Config 配置
type Config struct {
	Mysql ConfigMysql
	Test  []ConfigTest
}

// ConfigMysql 测试大小写
type ConfigMysql struct {
	HostName string // 数据库服务器地址
	HostPort string // 数据库服务器端口
	DataBase string // 数据库名
	UserName string // 数据库用户名
	PassWord string // 数据库密码
	CharSet  string // 数据库编码
}

// ConfigTest 测试各类型
type ConfigTest struct {
	TestBool     bool
	TestName     string
	TestAge      int
	Like         []string
	TestFloat    float32
	TestArrayInt []int
}

func toml(){
	v := viper.New()
	// 设置配置文件的名字
	v.SetConfigName("config")
	// 添加配置文件所在的路径
	v.AddConfigPath("./")
	// 设置配置文件类型
	v.SetConfigType("toml")
	// 读取配置文件
	if err := v.ReadInConfig(); err != nil {
		panic(err)
	}

	// 解析
	conf := new(Config)
	if err := v.Unmarshal(conf); err != nil {
		panic(err)
	}

	fmt.Println("config - toml:", conf)
}

func main() {
	toml()
}

