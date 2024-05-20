package main

import (
	"fmt"

	"github.com/spf13/viper"
)

// AppConfig 项目
type AppConfig struct {
	Name string `mapstructure:"name"`
	Mode string `mapstructure:"mode"`
	Port int    `mapstructure:"port"`
	Pid  string `mapstructure:"pid"`
}

// MysqlConfig mysql
type MysqlConfig struct {
	Dsn          string `mapstructure:"dsn"`
	MaxIdleCount int    `mapstructure:"max_idle_count"`
	MaxOpenCount int    `mapstructure:"max_open_count"`
	MaxLifeTime  int    `mapstructure:"max_life_time"`
}

type RedisConfig struct {
	RunType  int    `mapstructure:"runType"`
	DbNum    int    `mapstructure:"dbNum"`
	Password string `mapstructure:"password"`
	Single   struct {
		Node string `mapstructure:"node"`
	} `mapstructure:"single"`
	Cluster struct {
		Nodes []string `mapstructure:"nodes"`
	} `mapstructure:"cluster"`
}

type Services struct {
	Host string `mapstructure:"host"`
	Port int    `mapstructure:"port"`
}

// Config 总配置
type Config struct {
	App      AppConfig   `mapstructure:"app"`
	Mysql    MysqlConfig `mapstructure:"mysql"`
	Redis    RedisConfig `mapstructure:"redis"`
	Services []Services  `mapstructure:"services"`
}

func Init() {
	v := viper.New()
	// 设置配置文件的名字
	v.SetConfigName("config")
	// 设置配置文件类型
	v.SetConfigType("yaml") // yaml toml json
	// 添加配置文件所在的路径
	v.AddConfigPath(".")

	// 读取配置文件
	if err := v.ReadInConfig(); err != nil {
		panic(err)
	}

	// 解析
	conf := new(Config)
	if err := v.Unmarshal(conf); err != nil {
		panic(err)
	}

	fmt.Printf("config %+v\n", conf)
}

func main() {
	Init()
}
