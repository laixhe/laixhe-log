package main

import (
	"fmt"
	"sync"

	"github.com/IBM/sarama"
)

// 用于管理分区 协程
var wgGet sync.WaitGroup

func main() {

	//config := sarama.NewConfig()

	// 根据给定的代理地址和配置创建一个消费者
	consumer, err := sarama.NewConsumer([]string{"127.0.0.1:9092"}, nil)
	if err != nil {
		fmt.Println("Failed to consumer:", err)
		return
	}
	defer consumer.Close()

	// 获取 topic 的 partition 数量(返回了该 topic 的所有分区 id)
	partitionList, err := consumer.Partitions("test_data")
	if err != nil {
		fmt.Println("Failed to get the list of Partitions:", err)
		return
	}

	// 按每个分区进行消费
	for partition := range partitionList {

		// 获取每个分区
		// ConsumePartition 方法根据主题，分区和给定的偏移量创建创建了相应的分区消费者
		// 如果该分区消费者已经消费了该信息将会返回 error
		// sarama.OffsetNewest 表明了为最新消息
		pc, err := consumer.ConsumePartition("test_data", int32(partition), sarama.OffsetNewest)
		if err != nil {
			fmt.Printf("Failed to start consumer for partition %d: %s\n", partition, err)
			return
		}
		defer pc.AsyncClose()

		wgGet.Add(1)

		// 每个分区消费(返回一个消费消息类型的只读通道，由代理产生)
		go func(partitionConsumer sarama.PartitionConsumer) {

			for msg := range pc.Messages() {
				fmt.Printf("Partition:%d, Offset:%d, Key:%s, Value:%s\n", msg.Partition, msg.Offset, string(msg.Key), string(msg.Value))
			}

			wgGet.Done()

		}(pc)

	}

	wgGet.Wait()

}
