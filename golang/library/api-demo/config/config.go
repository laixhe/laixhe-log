package config

import (
	"errors"
	"fmt"
	"io/ioutil"
	"os"

	"github.com/laixhe/goutil/zaplog"
	"github.com/spf13/viper"
)

// AppConfig 项目
type AppConfig struct {
	Name string // 项目名
	Mode string // 运行模式
	Port int    // 运行端口
	Pid  string // PID 存放文件
}

// MysqlConfig mysql
type MysqlConfig struct {
	Dsn         string // 连接地址
	MaxIdleConn int    // 设置空闲连接池中连接的最大数量
	MaxOpenConn int    // 设置打开数据库连接的最大数量
	MaxLifeTime int    // 设置了连接可复用的最大时间(要比数据库设置连接超时时间少)(单位秒)
}

// RedisConfig redis
type RedisConfig struct {
	RunType  int // 运行类型：1=单机 2=哨兵主从 3=分布式集群
	DbNum    int // 选择 N 号数据库
	Password string
	// 单机
	Single struct {
		Node string
	}
	// 哨兵主从
	Sentinel struct {
		Master string
		Nodes  []string
	}
	// 分布式集群
	Cluster struct {
		Nodes []string
	}
}

// LogsConfig 日志
type LogsConfig struct {
	Path      string // 日志文件路径: 【当为 console 是控制台输出】【当为 ./logs/logs.log 是文件输出】
	LogLevel  string // 日志级别 debug  info  error
	MaxSize   int    // 每个日志文件保存大小 20M
	MaxBackup int    // 保留 N 个备份
	MaxAge    int    // 保留 N 天
}

// JwtConfig jwt
type JwtConfig struct {
	SecretKey string // jwt secret key
	ExpTime   int    // 过期时长(单位秒)
}

// Config 总配置
type Config struct {
	App   AppConfig
	Mysql MysqlConfig
	Redis RedisConfig
	Logs  LogsConfig
	Jwt   JwtConfig
}

var conf *Config

// IsAppDebug 是否是调试模式
func IsAppDebug() bool {
	if conf.App.Mode == "" {
		conf.App.Mode = ModeDebug
	}
	return conf.App.Mode == ModeDebug
}

// DBDsn 获取数据库连接地址
func DBDsn() string {
	if conf.Mysql.Dsn == "" {
		err := errors.New("获取数据库连接地址失败：为空")
		zaplog.Error(err.Error())
		panic(err)
	}

	return conf.Mysql.Dsn
}

// DBMaxIdleConn 获取数据库-空闲连接池中连接的最大数量
func DBMaxIdleConn() int {
	if conf.Mysql.MaxIdleConn <= 0 {
		conf.Mysql.MaxIdleConn = 10
	}

	return conf.Mysql.MaxIdleConn
}

// DBMaxOpenConn 获取数据库-打开数据库连接的最大数量
func DBMaxOpenConn() int {
	if conf.Mysql.MaxOpenConn <= 0 {
		conf.Mysql.MaxOpenConn = 150
	}

	return conf.Mysql.MaxOpenConn
}

// DBMaxLifeTime 获取数据库-设置了连接可复用的最大时间(要比数据库设置连接超时时间少)(单位秒)
func DBMaxLifeTime() int {
	if conf.Mysql.MaxLifeTime <= 0 {
		conf.Mysql.MaxLifeTime = 300
	}

	return conf.Mysql.MaxLifeTime
}

// RedisRunType 获取 redis 运行模式
func RedisRunType() int {
	if (conf.Redis.RunType <= (RunTypeSingle - 1)) || (conf.Redis.RunType >= (RunTypeCluster + 1)) {
		err := errors.New("获取 redis 运行类型失败：1=单机 2=哨兵主从 3=分布式集群 之一")
		zaplog.Error(err.Error())
		panic(err)
	}

	return conf.Redis.RunType
}

// AppAddr 获取 http 运行IP端口
func AppAddr() string {
	if conf.App.Port <= 0 || conf.App.Port > 65535 {
		conf.App.Port = 8088
	}
	return fmt.Sprintf(":%d", conf.App.Port)
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
	switch conf.Logs.LogLevel {
	case LogDebug:
	case LogInfo:
	case LogError:
	default:
		conf.Logs.LogLevel = LogDebug
	}

	if conf.Logs.MaxSize < 0 {
		conf.Logs.MaxSize = 0
	}
	if conf.Logs.MaxBackup < 0 {
		conf.Logs.MaxBackup = 0
	}
	if conf.Logs.MaxAge < 0 {
		conf.Logs.MaxAge = 0
	}

	if conf.Logs.Path == LogRunType {
		zaplog.InitConsole(conf.App.Name, conf.Logs.LogLevel, 1)
	} else {
		zaplog.InitSizeFile(conf.App.Name, conf.Logs.Path, conf.Logs.LogLevel, conf.Logs.MaxSize, conf.Logs.MaxBackup, conf.Logs.MaxAge, 1)
	}
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

	conf = new(Config)
	if err := v.Unmarshal(conf); err != nil {
		panic(err)
	}

	initLog()

	if Pid() != "" {
		pid := os.Getpid()
		if err := ioutil.WriteFile(Pid(), []byte(fmt.Sprintf("%d", pid)), 0666); err != nil {
			panic(err)
		}
	}

	zaplog.Debug("配置与日志初始化完毕...")
}
