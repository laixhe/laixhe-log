
# kafka 基本使用

> 安装
```
go get github.com/IBM/sarama
```

> 导入
```
import "github.com/IBM/sarama"
```

> consumer 消费者

> producer 生产者

```
kafka 对消息保存时根据 Topic 进行归类
每个 topic 将被分成多个 partition (区)
每个 partition 在存储层面是 append log 文件
任何发布到此 partition 的消息都会被直接追加到 log 文件的尾部
每条消息在文件中的位置称为 offset (偏移量)
offset 为一个 long 型数字，它是唯一标记一条消息
发送消息者成为 Producer (生产者)
消息接受者成为 Consumer (消费者)
kafka 集群几乎不需要维护任何 consumer 和 producer 状态信息,这些信息有 zookeeper 保存
```
