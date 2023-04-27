package redisx

import (
	"context"
	"errors"
	"strings"
	"time"

	redis "github.com/redis/go-redis/v9"

	"golang_log/library/api-demo/core/logx"
)

type Config struct {
	Addr        string `mapstructure:"addr"`          // 多个地址是以 , 分割
	DbNum       uint   `mapstructure:"db_num"`        // 选择N号数据库
	Password    string `mapstructure:"password"`      // 密码
	PoolSize    uint   `mapstructure:"pool_size"`     // 最大链接数
	MinIdleConn uint   `mapstructure:"min_idle_conn"` // 空闲链接数
	Master      string `mapstructure:"master"`        // 哨兵主从(当有值时为:主从模式)(当没值时且addr有多个时为:分布式集群)
}

// Redisx 客户端
type Redisx struct {
	client redis.Cmdable
}

// Ping 判断服务是否可用
func (r *Redisx) Ping() error {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()
	err := r.client.Ping(ctx).Err()
	if err != nil {
		return err
	}
	return nil
}

// Client get redis client
func (r *Redisx) Client() redis.Cmdable {
	return r.client
}

var db *Redisx

// DB get redisx
func DB() *Redisx {
	return db
}

// Client get redis client
func Client() redis.Cmdable {
	return db.client
}

// Ping 判断服务是否可用
func Ping() error {
	return db.Ping()
}

// initSingle 单机
func initSingle(c *Config) redis.Cmdable {
	options := &redis.Options{
		Addr:     c.Addr,
		Password: c.Password,
		DB:       int(c.DbNum),
	}
	if c.PoolSize > 0 {
		options.PoolSize = int(c.PoolSize)
	}
	if c.MinIdleConn > 0 {
		options.MinIdleConns = int(c.MinIdleConn)
	}
	return redis.NewClient(options)
}

// initSentinel 哨兵主从
func initSentinel(c *Config) redis.Cmdable {
	options := &redis.FailoverOptions{
		MasterName:    c.Master,
		SentinelAddrs: strings.Split(c.Addr, ","),
		DB:            int(c.DbNum),
		Password:      c.Password,
	}
	if c.PoolSize > 0 {
		options.PoolSize = int(c.PoolSize)
	}
	if c.MinIdleConn > 0 {
		options.MinIdleConns = int(c.MinIdleConn)
	}
	return redis.NewFailoverClient(options)
}

// initCluster 分布式集群
func initCluster(c *Config) redis.Cmdable {
	options := &redis.ClusterOptions{
		Addrs:    strings.Split(c.Addr, ","),
		Password: c.Password,
	}
	if c.PoolSize > 0 {
		options.PoolSize = int(c.PoolSize)
	}
	if c.MinIdleConn > 0 {
		options.MinIdleConns = int(c.MinIdleConn)
	}
	return redis.NewClusterClient(options)
}

// Connect 连接数据库
func Connect(c *Config) (*Redisx, error) {
	r := &Redisx{}

	addrs := strings.Split(c.Addr, ",")
	if c.Master == "" && len(addrs) == 1 {
		r.client = initSingle(c) // 单机
	} else if c.Master == "" && len(addrs) > 1 {
		r.client = initCluster(c) // 分布式集群
	} else if c.Master != "" && len(addrs) > 1 {
		r.client = initSentinel(c) // 哨兵主从
	} else {
		return nil, errors.New("redis 运行类型不符合")
	}
	err := r.Ping()
	if err != nil {
		return nil, err
	}
	return r, nil
}

func Init(c *Config) {
	logx.Debugf("redis Config=%v", c)
	logx.Debug("redis 开始初始化...")

	var err error
	db, err = Connect(c)
	if err != nil {
		panic(err)
	}

	logx.Debug("redis 初始化完毕...")
}
