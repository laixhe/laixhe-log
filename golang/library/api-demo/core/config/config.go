package config

import (
	"fmt"
	"os"

	"github.com/spf13/viper"

	"golang_log/library/api-demo/core/gormx"
	"golang_log/library/api-demo/core/httpjwt"
	"golang_log/library/api-demo/core/logx"
	"golang_log/library/api-demo/core/redisx"
)

// AppConfig 项目
type AppConfig struct {
	Name string  `mapstructure:"name"` // 项目名
	Mode AppMode `mapstructure:"mode"` // 运行模式
	Port int     `mapstructure:"port"` // 运行端口
	Pid  string  `mapstructure:"pid"`  // PID 存放文件
}

// Config 总配置
type Config struct {
	App   *AppConfig      `mapstructure:"app"`
	Mysql *gormx.Config   `mapstructure:"mysql"`
	Redis *redisx.Config  `mapstructure:"redis"`
	Log   *logx.Config    `mapstructure:"log"`
	Jwt   *httpjwt.Config `mapstructure:"jwt"`
}

var conf *Config

// AppName 项目名
func AppName() string {
	return conf.App.Name
}

// AppDebug 运行模式
func AppDebug() AppMode {
	if conf.App.Mode == "" {
		conf.App.Mode = AppModeDebug
	}
	return conf.App.Mode
}

// IsAppDebug 是否是调试模式
func IsAppDebug() bool {
	return AppDebug() == AppModeDebug
}

// AppAddr 获取 http 运行端口
func AppAddr() int {
	if conf.App.Port <= 0 || conf.App.Port > 65535 {
		conf.App.Port = 8088
	}
	return conf.App.Port
}

// AppIpAddr 获取 http 运行 IP 端口
func AppIpAddr() string {
	return fmt.Sprintf(":%d", AppAddr())
}

// Pid PID 存放文件
func Pid() string {
	return conf.App.Pid
}

// Get Config
func Get() *Config {
	return conf
}

// initLog 初始化日志
func initLog() {
	conf.Log.ServiceName = AppName()
	logx.Init(conf.Log)
}

func init() {

	v := viper.New()
	// 设置配置文件的名字
	v.SetConfigName("app")
	// 添加配置文件所在的路径
	v.AddConfigPath("./conf")
	// 设置配置文件类型
	v.SetConfigType("yaml")
	if err := v.ReadInConfig(); err != nil {
		panic(err)
	}

	conf = &Config{
		App:   &AppConfig{},
		Mysql: &gormx.Config{},
		Redis: &redisx.Config{},
		Log:   &logx.Config{},
		Jwt:   &httpjwt.Config{},
	}
	if err := v.Unmarshal(conf); err != nil {
		panic(err)
	}

	initLog()

	if Pid() != "" {
		pid := os.Getpid()
		if err := os.WriteFile(Pid(), []byte(fmt.Sprintf("%d", pid)), 0666); err != nil {
			panic(err)
		}
	}
}
