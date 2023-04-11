package main

import (
	"context"
	"fmt"
	"log"

	"github.com/redis/go-redis/v9"
)

func main() {
	NewClient().Subscribe()
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

// 订阅
func (this *RedisClient) Subscribe() {

	// 参数1 频道名
	pubsub := this.Client.Subscribe(context.Background(), "message")
	defer pubsub.Close()

	// 接收
	_, err := pubsub.Receive(context.Background())
	if err != nil {
		fmt.Printf("Subscribe Receive err: %v\n", err)
		return
	}

	ch := pubsub.Channel()
	for msg := range ch {
		fmt.Printf("频道名: %s 数据: %s\n", msg.Channel, msg.Payload)
	}

}
