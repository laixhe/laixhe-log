package main

import (
	"context"
	"errors"
	"fmt"
	"log"
	"time"

	redis "github.com/redis/go-redis/v9"
)

type RedisClient struct {
	Client        *redis.Client
	ClusterClient *redis.ClusterClient
}

// go-redis 自带连接池
// 在启动阶段如果没有请求的话则不会主动创建连接
func NewClient() *RedisClient {

	// 创建 redis 客户端
	client := redis.NewClient(&redis.Options{
		Addr:     "127.0.0.1:6379",
		Password: "",
		DB:       0,

		// 连接池容量及闲置连接数量
		//PoolSize            // 连接池的大小
		//MinIdleConns        // 连接池容量及闲置连接数量
		// 闲置连接检查
		//IdleCheckFrequency  // 闲置连接检查的周期，-1表示不做周期性检查，只在客户端获取连接时对闲置连接进行处理
		//IdleTimeout         // 闲置超时，-1表示取消闲置超时检查
		//MaxConnAge          // 连接存活时长，从创建开始计时，超过指定时长则关闭连接，默认为0，即不关闭存活时长较长的连接
		// 超时
		//DialTimeout         // 连接建立超时时间
		//ReadTimeout         // 读超时， -1表示取消读超时
		//WriteTimeout        // 写超时，默认等于读超时
		//PoolTimeout         // 当所有连接都处在繁忙状态时，客户端等待可用连接的最大等待时长，默认为读超时+1秒
		// 命令执行失败时的重试策略
		//MaxRetries          // 命令执行失败时，最多重试多少次，默认为0即不重试
		//MinRetryBackoff     // 每次计算重试间隔时间的下限，-1表示取消间隔
		//MaxRetryBackoff     // 每次计算重试间隔时间的上限，-1表示取消间隔

		//钩子函数
		//仅当客户端执行命令时需要从连接池获取连接时，如果连接池需要新建连接时则会调用此钩子函数
		//OnConnect

	})

	// 通过 cient.Ping() 来检查是否成功连接到了 redis 服务器
	_, err := client.Ping(context.Background()).Result()
	if err != nil {
		log.Fatal(err)
	}

	//defer client.Close()

	// 获取redis连接数
	poolStats := client.PoolStats()
	fmt.Printf("总连接数=%d,空闲连接数=%d,已经移除的连接数=%d\n",
		poolStats.TotalConns,
		poolStats.IdleConns,
		poolStats.StaleConns)

	return &RedisClient{Client: client}
}

// 集群
func NewClusterClient() *RedisClient {

	client := redis.NewClusterClient(&redis.ClusterOptions{
		Addrs: []string{":7000", ":7001", ":7002", ":7003", ":7004", ":7005"},
	})

	// 通过 cient.Ping() 来检查是否成功连接到了 redis 服务器
	_, err := client.Ping(context.Background()).Result()
	if err != nil {
		log.Fatal(err)
	}

	//defer client.Close()

	return &RedisClient{ClusterClient: client}
}

// string 字符串
// set  将字符串值 value 关联到 key
// get  返回 key 所关联的字符串值
// del  删除给定的一个或多个 key
// setnx 将 key 的值设为 value ，当且仅当 key 不存在,若给定的 key 已经存在，则 SETNX 不做任何动作
func (this *RedisClient) SetGetDel(key string, value interface{}, expiration time.Duration) error {

	// 第三个参数是过期时间, 如果是0, 则表示没有过期时间
	err := this.Client.Set(context.Background(), key, value, expiration).Err()
	if err != nil {
		return fmt.Errorf("set error: %v\n", err)
	}
	fmt.Printf("%v=%v 设置成功\n", key, value)
	time.Sleep(time.Second)

	data, err := this.Client.Get(context.Background(), key).Result()
	if err != nil {
		if err == redis.Nil {
			return errors.New("key does not exist")
		}
		return fmt.Errorf("get error: %v\n", err)
	}
	fmt.Printf("%v=%v 获取成功\n", key, data)
	time.Sleep(time.Second)

	err = this.Client.Del(context.Background(), key).Err()
	if err != nil {
		return fmt.Errorf("del error: %v\n", err)
	}
	fmt.Printf("%v 删除成功\n", key)
	time.Sleep(time.Second)

	data, err = this.Client.Get(context.Background(), key).Result()
	fmt.Printf("%v=%v err:%v\n", key, data, err)
	time.Sleep(time.Second)

	// 将 key 的值设为 value ，当且仅当 key 不存在
	// 若给定的 key 已经存在，则 SETNX 不做任何动作
	is, err := this.Client.SetNX(context.Background(), key, value, expiration).Result()
	if err != nil {
		return fmt.Errorf("setnx 1 error: %v\n", err)
	}
	fmt.Printf("%v=%v\n", key, is)
	time.Sleep(time.Second)

	is, err = this.Client.SetNX(context.Background(), key, value, expiration).Result()
	if err != nil {
		return fmt.Errorf("setnx 2 error: %v\n", err)
	}
	fmt.Printf("%v=%v\n", key, is)
	time.Sleep(time.Second)
	time.Sleep(time.Second)

	is, err = this.Client.SetNX(context.Background(), key, value, expiration).Result()
	if err != nil {
		return fmt.Errorf("setnx 3 error: %v\n", err)
	}
	fmt.Printf("%v=%v\n", key, is)
	time.Sleep(time.Second)

	return nil
}

// string 字符串
// 自增自减
// incr(key)              自增
// decr(key)              自减
// incrby(key, integer)   设置 key 的 string 增加 integer
// decrby(key, integer)   设置 key 的 string 减少 integer
func (this *RedisClient) IncrDecr(key string) error {

	err := this.Client.Incr(context.Background(), key).Err()
	if err != nil {
		return fmt.Errorf("incr error: %v\n", err)
	}
	fmt.Printf("%v 自增成功\n", key)
	time.Sleep(time.Second)

	data, err := this.Client.Get(context.Background(), key).Result()
	if err != nil {
		if err == redis.Nil {
			return errors.New("key does not exist")
		}
		return fmt.Errorf("get error: %v\n", err)
	}
	fmt.Printf("%v=%v 获取成功\n", key, data)
	time.Sleep(time.Second)

	err = this.Client.Incr(context.Background(), key).Err()
	if err != nil {
		return fmt.Errorf("incr error: %v\n", err)
	}
	fmt.Printf("%v 自增成功\n", key)
	time.Sleep(time.Second)

	data, err = this.Client.Get(context.Background(), key).Result()
	if err != nil {
		if err == redis.Nil {
			return errors.New("key does not exist")
		}
		return fmt.Errorf("get error: %v\n", err)
	}
	fmt.Printf("%v=%v 获取成功\n", key, data)
	time.Sleep(time.Second)

	err = this.Client.Decr(context.Background(), key).Err()
	if err != nil {
		return fmt.Errorf("decr error: %v\n", err)
	}
	fmt.Printf("%v 自减成功\n", key)
	time.Sleep(time.Second)

	data, err = this.Client.Get(context.Background(), key).Result()
	if err != nil {
		if err == redis.Nil {
			return errors.New("key does not exist")
		}
		return fmt.Errorf("get error: %v\n", err)
	}
	fmt.Printf("%v=%v 获取成功\n", key, data)
	time.Sleep(time.Second)

	return nil
}

// list 列表
// lpush  将一个或多个值 value 插入到列表 key 的表头
// rpush  将一个或多个值 value 插入到列表 key 的表尾
// lpop   移除并返回列表 key 的头元素
// rpop   移除并返回列表 key 的尾元素
// lrange 返回列表 key 中指定区间内的元素，区间以偏移量 start 和 stop 指定(0,-1 可以获取列表所有数据)
// llen   返回列表 key 的长度
// blpop  lpop 命令的 block 阻塞版本
// brpop  rpop 命令的 block 阻塞版本
func (this *RedisClient) List() error {

	key := "test_list"

	err := this.Client.LPush(context.Background(), key, 1).Err()
	if err != nil {
		return fmt.Errorf("lpush error: %v\n", err)
	}
	fmt.Printf("为 %v 的 list 头添加一个值为 1 的元素\n", key)
	time.Sleep(time.Second)

	err = this.Client.RPush(context.Background(), key, 2).Err()
	if err != nil {
		return fmt.Errorf("rpush error: %v\n", err)
	}
	fmt.Printf("为 %v 的 list 尾添加一个值为 2 的元素\n", key)
	time.Sleep(time.Second)

	err = this.Client.LPush(context.Background(), key, 3).Err()
	if err != nil {
		return fmt.Errorf("lpush error: %v\n", err)
	}
	fmt.Printf("为 %v 的 list 头添加一个值为 3 的元素\n", key)
	time.Sleep(time.Second)

	array, err := this.Client.LRange(context.Background(), key, 0, -1).Result()
	if err != nil {
		return fmt.Errorf("lrange error: %v\n", err)
	}
	fmt.Printf("获取当前 %v 的 list 所有值：%v\n", key, array)
	time.Sleep(time.Second)

	data, err := this.Client.RPop(context.Background(), key).Result()
	if err != nil {
		return fmt.Errorf("rpop error: %v\n", err)
	}
	fmt.Printf("返回并删除为 %v 的 list 中的尾元素: %v\n", key, data)
	time.Sleep(time.Second)

	data, err = this.Client.LPop(context.Background(), key).Result()
	if err != nil {
		return fmt.Errorf("lpop error: %v\n", err)
	}
	fmt.Printf("返回并删除为 %v 的 list 中的首元素: %v\n", key, data)
	time.Sleep(time.Second)

	array, err = this.Client.LRange(context.Background(), key, 0, -1).Result()
	if err != nil {
		return fmt.Errorf("lrange error: %v\n", err)
	}
	fmt.Printf("获取当前 %v 的 list 所有值：%v\n", key, array)
	time.Sleep(time.Second)

	return nil
}

// set 集合不可重复
// sadd         将一个或多个 member 元素加入到集合 key 当中，已经存在于集合的 member 元素将被忽略
// srem         移除集合 key 中的一个或多个 member 元素，不存在的 member 元素会被忽略
// sismember    判断 member 元素是否集合 key 的成员(判断元素是否在集合中)
// smembers     返回集合 key 中的所有成员(获取指定集合的所有元素)
// spop         移除并返回集合中的一个随机元素
// srandmember  返回集合中的 N 个随机元素
func (this *RedisClient) Set() error {

	// 向 set_bat 中添加元素
	err := this.Client.SAdd(context.Background(), "set_bat", "阿里").Err()
	if err != nil {
		return fmt.Errorf("sadd 1 error: %v\n", err)
	}
	err = this.Client.SAdd(context.Background(), "set_bat", "腾讯", "百度", "阿里").Err()
	if err != nil {
		return fmt.Errorf("sadd 2 error: %v\n", err)
	}

	// 判断元素是否在集合中
	isMember, err := this.Client.SIsMember(context.Background(), "set_bat", "腾讯").Result()
	if err != nil {
		return fmt.Errorf("sismember error: %v\n", err)
	}
	fmt.Println("sismember:", isMember)

	// 获取指定集合的所有元素
	all, err := this.Client.SMembers(context.Background(), "set_bat").Result()
	if err != nil {
		return fmt.Errorf("smembers error: %v\n", err)
	}
	fmt.Println("smembers: ", all)

	err = this.Client.SRem(context.Background(), "set_bat", "腾讯").Err()
	if err != nil {
		return fmt.Errorf("srem error: %v\n", err)
	}
	fmt.Println("srem: ok")

	// 判断元素是否在集合中
	isMember, err = this.Client.SIsMember(context.Background(), "set_bat", "腾讯").Result()
	if err != nil {
		return fmt.Errorf("sismember error: %v\n", err)
	}
	fmt.Println("sismember:", isMember)

	// 获取指定集合的所有元素
	all, err = this.Client.SMembers(context.Background(), "set_bat").Result()
	if err != nil {
		return fmt.Errorf("smembers error: %v\n", err)
	}
	fmt.Println("smembers: ", all)

	return nil
}

// hash 哈希
// hset    将哈希表 key 中的域 field 的值设为 value
// hget    返回哈希表 key 中给定域 field 的值
// hdel    删除哈希表 key 中的一个或多个指定域，不存在的域将被忽略
// hlen    返回哈希表 key 中域的数量
// hexists 查看哈希表 key 中，给定域 field 是否存在
// hmget   返回哈希表 key 中，一个或多个给定域的值
// hmset   同时将多个 field-value (域-值)对设置到哈希表 key 中
func (this *RedisClient) HashTest() error {

	// 向 hash 添加元素
	err := this.Client.HSet(context.Background(), "user", "name", "laixhe").Err()
	if err != nil {
		return fmt.Errorf("hset 1 error: %v\n", err)
	}
	err = this.Client.HSet(context.Background(), "user", "age", 18).Err()
	if err != nil {
		return fmt.Errorf("hset 2 error: %v\n", err)
	}

	// 批量地向名称为 user 的 hash 中添加元素
	err = this.Client.HMSet(context.Background(), "user", map[string]interface{}{"id": 1, "time": "2020-03-31"}).Err()
	if err != nil {
		return fmt.Errorf("hmset error: %v\n", err)
	}

	// 批量获取名为 user 的 hash 中的指定字段的值.
	fields, err := this.Client.HMGet(context.Background(), "user", "name", "age", "test").Result()
	if err != nil {
		return fmt.Errorf("hmget error: %v\n", err)
	}
	fmt.Println("hmget user: ", fields)

	// 获取名为 user 的 hash 中的字段个数
	length, err := this.Client.HLen(context.Background(), "user").Result()
	if err != nil {
		return fmt.Errorf("hlen error: %v\n", err)
	}
	fmt.Println("hlen user: ", length)

	return nil
}
