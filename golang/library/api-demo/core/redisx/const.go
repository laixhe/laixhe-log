package redisx

// RedisRun redis运行模式
type RedisRun int

const (
	RedisRunSingle   RedisRun = 1 // 单机
	RedisRunSentinel RedisRun = 2 // 哨兵主从
	RedisRunCluster  RedisRun = 3 // 分布式集群
)
