package main

import (
	"fmt"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/nsqio/go-nsq"
)

func main() {

	err := initConsumer("order_queue", "first", "127.0.0.1:4161")
	if err != nil {
		fmt.Printf("init consumer failed, err: %v\n", err)
		return
	}

	// 用户发送 INTR 字符(Ctrl+C)触发
	c := make(chan os.Signal)
	signal.Notify(c, syscall.SIGINT)
	<-c

}

// 初始化消费者
func initConsumer(topic string, channel string, addr string) error {

	config := nsq.NewConfig()

	// 设置服务发现的轮时间
	config.LookupdPollInterval = 15 * time.Second
	// 最大允许向两台 Nsqd 服务器接受消息，默认是1，要特别注意
	//config.MaxInFlight=2

	c, err := nsq.NewConsumer(topic, channel, config)
	if err != nil {
		return nil
	}

	// 添加消息处理的具体实现
	consumer := &Consumer{}
	c.AddHandler(consumer)

	// 将消费者连接到具体的 Nsqd - 不推荐
	//err = c.ConnectToNSQD(addr)
	//if err != nil {
	//	return err
	//}

	// 建立 NSQLookupd 连接，再分发给具体的 nsqd
	err = c.ConnectToNSQLookupd(addr)
	if err != nil {
		return err
	}

	return nil
}

// 消费者
type Consumer struct{}

// 处理消费者的消息
func (*Consumer) HandleMessage(msg *nsq.Message) error {

	fmt.Printf("receive=%s id=%x message=%s\n", msg.NSQDAddress, msg.ID, msg.Body)

	return nil
}
