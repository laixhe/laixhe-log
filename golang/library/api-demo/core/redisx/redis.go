package redisx

import (
	"context"
	"time"

	redis "github.com/redis/go-redis/v9"

	"golang_log/library/api-demo/core/logx"
)

type Config struct {
	RunType     RedisRun // 运行类型
	DbNum       int      // 选择 N 号数据库
	Password    string   // 密码
	PoolSize    int      // 最大链接数
	MinIdleConn int      // 空闲链接数
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

var client redis.Cmdable

// initSingle 单机
func initSingle(c *Config) {
	options := &redis.Options{
		Addr:     c.Single.Node,
		Password: c.Password,
		DB:       c.DbNum,
	}
	if c.PoolSize > 0 {
		options.PoolSize = c.PoolSize
	}
	if c.MinIdleConn > 0 {
		options.MinIdleConns = c.MinIdleConn
	}
	client = redis.NewClient(options)
}

// initSentinel 哨兵主从
func initSentinel(c *Config) {
	options := &redis.FailoverOptions{
		MasterName:    c.Sentinel.Master,
		SentinelAddrs: c.Sentinel.Nodes,
		DB:            c.DbNum,
		Password:      c.Password,
	}
	if c.PoolSize > 0 {
		options.PoolSize = c.PoolSize
	}
	if c.MinIdleConn > 0 {
		options.MinIdleConns = c.MinIdleConn
	}
	client = redis.NewFailoverClient(options)
}

// initCluster 分布式集群
func initCluster(c *Config) {
	options := &redis.ClusterOptions{
		Addrs:    c.Cluster.Nodes,
		Password: c.Password,
	}
	if c.PoolSize > 0 {
		options.PoolSize = c.PoolSize
	}
	if c.MinIdleConn > 0 {
		options.MinIdleConns = c.MinIdleConn
	}
	client = redis.NewClusterClient(options)
}

// Ping 测试
func Ping() error {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()
	err := client.Ping(ctx).Err()
	if err != nil {
		return err
	}
	return nil
}

// DB get redis client
func DB() redis.Cmdable {
	return client
}

func Init(c *Config) {
	logx.Debug("redis 开始初始化...")

	if c.PoolSize < 0 {
		c.PoolSize = 0
	}
	if c.MinIdleConn < 0 {
		c.MinIdleConn = 0
	}

	switch c.RunType {
	case RedisRunSingle:
		logx.Debug("redis 单机初始化...")
		initSingle(c)
	case RedisRunSentinel:
		logx.Debug("redis 哨兵主从开始初始化...")
		initSentinel(c)
	case RedisRunCluster:
		logx.Debug("redis 分布式集群开始初始化...")
		initCluster(c)
	}
	err := Ping()
	if err != nil {
		panic(err)
	}
	logx.Debug("redis 初始化完毕...")
}
