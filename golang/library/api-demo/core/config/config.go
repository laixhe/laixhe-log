package config

import (
	"errors"
	"fmt"
	"os"
	"strings"

	"github.com/spf13/viper"

	"golang_log/library/api-demo/core/gormx"
	"golang_log/library/api-demo/core/jwtx"
	"golang_log/library/api-demo/core/logx"
	"golang_log/library/api-demo/core/redisx"
)

// AppConfig 项目配置
type AppConfig struct {
	Pid     string `mapstructure:"pid"`     // PID 存放文件
	Version string `mapstructure:"version"` // 版本号
}

// Server 服务器
type Server struct {
	Ip      string `mapstructure:"ip"`      // 运行IP
	Port    uint   `mapstructure:"port"`    // 运行端口
	Timeout uint   `mapstructure:"timeout"` // 超时时间
}

func (s Server) Addr() string {
	return fmt.Sprintf("%s:%d", s.Ip, s.Port)
}

// Servers 服务器组
type Servers struct {
	Http Server `mapstructure:"http"`
	Grpc Server `mapstructure:"grpc"`
}

// Config 总配置
type Config struct {
	App     *AppConfig     `mapstructure:"app"`
	Servers *Servers       `mapstructure:"servers"`
	Mysql   *gormx.Config  `mapstructure:"mysql"`
	Redis   *redisx.Config `mapstructure:"redis"`
	Log     *logx.Config   `mapstructure:"log"`
	Jwt     *jwtx.Config   `mapstructure:"jwt"`
}

var conf *Config

// Pid PID 存放文件
func Pid() string {
	return conf.App.Pid
}

// Get Config
func Get() *Config {
	return conf
}

// splitConfigFile 通过文件路径获取目录、文件名、扩展名
func splitConfigFile(configFile string) (dir string, fileName string, extName string, err error) {
	if len(configFile) == 0 {
		err = errors.New(configFile + " is empty")
		return
	}
	configFiles := strings.Split(configFile, "/")
	lens := len(configFiles) - 1
	if lens == 0 {
		dir = "."
	} else {
		dir = strings.Join(configFiles[:lens], "/")
	}
	files := strings.Split(configFiles[lens], ".")
	if len(files) <= 1 {
		err = errors.New(configFile + " file name is empty")
		return
	}
	fileName = files[0]
	extName = files[1]
	return
}

// InitViper 初始化配置文件
// configFile 配置文件
// isEnv      是否获取环境变量环境
// loadData   装载的数据结构指针类型
func InitViper(configFile string, isEnv bool, loadData interface{}) error {
	dir, fileName, extName, err := splitConfigFile(configFile)
	if err != nil {
		return err
	}

	v := viper.New()
	// 设置配置文件的名字
	v.SetConfigName(fileName)
	// 添加配置文件所在的路径
	v.AddConfigPath(dir)
	// 设置配置文件类型
	v.SetConfigType(extName)
	if err = v.ReadInConfig(); err != nil {
		return err
	}

	// 优先替换环境变量
	if isEnv {
		envConfigs := ListEnvConfig()
		for k := range envConfigs {
			envConfig := envConfigs[k]
			env := os.Getenv(envConfig.env)
			if env != "" {
				v.Set(envConfig.configKey, env) // 替换
			}
		}
	}

	if err = v.Unmarshal(loadData); err != nil {
		return err
	}
	return nil
}

// Init 初始化配置
func Init(configFile string) {
	conf = &Config{
		App:   &AppConfig{},
		Mysql: &gormx.Config{},
		Redis: &redisx.Config{},
		Log:   &logx.Config{},
		Jwt:   &jwtx.Config{},
	}
	if err := InitViper(configFile, true, conf); err != nil {
		panic(err)
	}

	// 初始化日志
	logx.Init(conf.Log)

	if Pid() != "" {
		pid := os.Getpid()
		if err := os.WriteFile(Pid(), []byte(fmt.Sprintf("%d", pid)), 0666); err != nil {
			panic(err)
		}
	}
}
