package main

import (
	"fmt"
	"strconv"
	"time"

	"github.com/IBM/sarama"
)

func main() {

	config := sarama.NewConfig()

	// 等待服务器所有副本都保存成功后的响应
	// 赋值为 -1 时这意味着发送完数据需要 leader 和 follow 都确认
	config.Producer.RequiredAcks = sarama.WaitForAll
	// 写到随机分区(partition)中，默认设置8个分区
	// 随机的分区类型：返回一个分区器，该分区器每次选择一个随机分区
	config.Producer.Partitioner = sarama.NewRandomPartitioner
	// 是否等待成功和失败后的响应,只有上面的 RequireAcks 设置不是 NoReponse 这里才有用
	config.Producer.Return.Successes = true
	config.Producer.Return.Errors = true

	// 使用给定代理地址和配置创建一个同步生产者(同步模式)
	client, err := sarama.NewSyncProducer([]string{"127.0.0.1:9092"}, config)
	if err != nil {
		fmt.Println("producer close, err:", err)
		return
	}

	defer client.Close()

	for {
		time.Sleep(time.Second)
		unixTime := time.Now().Unix()

		// 构建发送的消息
		msg := &sarama.ProducerMessage{}
		msg.Topic = "test_data"

		if (unixTime % 2) == 0 {
			msg.Value = sarama.ByteEncoder("将字符串转换为字节数组:" + strconv.Itoa(int(unixTime)))
		} else {
			// 将字符串转换为字节数组
			msg.Value = sarama.StringEncoder(strconv.Itoa(int(unixTime)))
		}

		// SendMessage：该方法是生产者生产给定的消息
		// 生产成功的时候返回该消息的分区和所在的偏移量
		// 生产失败的时候返回 error
		partition, offset, err := client.SendMessage(msg)
		if err != nil {
			fmt.Println("send message failed,", err)
			return
		}

		fmt.Printf("partition:%v offset:%v\n", partition, offset)

	}

}

// 异步生产者
func SaramaProducer() {

	config := sarama.NewConfig()
	config.Producer.RequiredAcks = sarama.WaitForAll
	config.Producer.Partitioner = sarama.NewRandomPartitioner
	config.Producer.Return.Successes = true
	config.Producer.Return.Errors = true

	fmt.Println("start make producer")
	// 使用配置,新建一个异步生产者
	client, e := sarama.NewAsyncProducer([]string{"127.0.0.1:9092"}, config)
	if e != nil {
		fmt.Println(e)
		return
	}
	defer client.AsyncClose()

	// 循环判断哪个通道发送过来数据.
	fmt.Println("start goroutine")
	go func(c sarama.AsyncProducer) {
		for {
			select {
			case suc := <-c.Successes():
				// 开启 config.Producer.Return.Successes = true 后一定要监听这个chan，默认大小256 如果满了就阻塞掉
				fmt.Println("partitions: ", suc.Partition, "Successes offset: ", suc.Offset, "timestamp: ")
			case fail := <-c.Errors():
				// 开启 config.Producer.Return.Errors = true 后一定要监听这个chan，默认大小256 如果满了就阻塞掉
				fmt.Println("Errors err: ", fail.Err)
			}
		}
	}(client)

	for {
		time.Sleep(time.Second)
		unixTime := time.Now().Unix()

		// 构建发送的消息
		msg := &sarama.ProducerMessage{}
		msg.Topic = "test_data"

		if (unixTime % 2) == 0 {
			msg.Value = sarama.ByteEncoder("字符串1:" + strconv.Itoa(int(unixTime)))
		} else {
			// 将字符串转换为字节数组
			msg.Value = sarama.StringEncoder("字符串2:" + strconv.Itoa(int(unixTime)))
		}

		// 使用通道发送
		client.Input() <- msg

	}
}
