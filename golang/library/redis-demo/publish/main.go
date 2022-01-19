package main

import (
	"context"
	"fmt"
	"log"
	"time"

	"github.com/go-redis/redis/v8"
)

func main() {
	NewClient().Publish()
}

type RedisClient struct {
	Client *redis.Client
}

func NewClient() *RedisClient {

	// 创建 redis 客户端
	client := redis.NewClient(&redis.Options{
		Addr:     "127.0.0.1:6379",
		Password: "",
		DB:       0,
	})

	// 通过 cient.Ping() 来检查是否成功连接到了 redis 服务器
	_, err := client.Ping(context.Background()).Result()
	if err != nil {
		log.Fatal(err)
	}

	return &RedisClient{Client: client}
}

// 发布
func (this *RedisClient) Publish() error {

	for {
		time.Sleep(time.Second)
		// 参数1 频道名
		// 参数2 发送的内容
		err := this.Client.Publish(context.Background(), "message", fmt.Sprintf("%v", time.Now().Unix())).Err()
		if err != nil {
			return err
		}
	}

}
