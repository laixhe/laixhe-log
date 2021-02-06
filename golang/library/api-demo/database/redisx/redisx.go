package redisx

import (
	"context"
	"time"

	"github.com/go-redis/redis/v8"
	"github.com/laixhe/goutil/zaplog"

	"golang_log/library/api-demo/config"
)

var client redis.Cmdable

// initSingle 单机
func initSingle() {
	client = redis.NewClient(&redis.Options{
		Addr:     config.Get().Redis.Single.Node,
		Password: config.Get().Redis.Password,
		DB:       config.Get().Redis.DbNum,
	})
}

// initSentinel 哨兵主从
func initSentinel() {
	client = redis.NewFailoverClient(&redis.FailoverOptions{
		MasterName:    config.Get().Redis.Sentinel.Master,
		SentinelAddrs: config.Get().Redis.Sentinel.Nodes,
		DB:            config.Get().Redis.DbNum,
		Password:      config.Get().Redis.Password,
	})
}

// initCluster 分布式集群
func initCluster() {
	client = redis.NewClusterClient(&redis.ClusterOptions{
		Addrs:    config.Get().Redis.Cluster.Nodes,
		Password: config.Get().Redis.Password,
	})
}

// Ping 测试
func Ping() error {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()
	err := client.Ping(ctx).Err()
	if err != nil {
		zaplog.Errorf("redis ping error:%v", err)
		return err
	}

	return nil
}

// DB get redis
func DB() redis.Cmdable {
	return client
}

func init() {
	zaplog.Debug("redis 开始初始化...")

	switch config.RedisRunType() {
	case config.RunTypeSingle:
		zaplog.Debug("redis 单机初始化...")
		initSingle()
	case config.RunTypeSentinel:
		zaplog.Debug("redis 哨兵主从开始初始化...")
		initSentinel()
	case config.RunTypeCluster:
		zaplog.Debug("redis 分布式集群开始初始化...")
		initCluster()
	}

	err := Ping()
	if err != nil {
		panic(err)
	}

	zaplog.Debug("redis 初始化完毕...")
}
