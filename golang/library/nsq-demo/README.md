
> consumer 消费者

> producer 生产者


### Nsq 简单使用

> 安装
```
go get github.com/nsqio/go-nsq
```

> 导入
```
import "github.com/nsqio/go-nsq"
```

#### nsqlookupd
```
-tcp-address  = 127.0.0.1:4160 是 nsqd 用的端口 - 广播
-http-address = 127.0.0.1:4161 是 http 消费监听端口 - 客户端用它来发现和管理
```

#### nsqd
```
--lookupd-tcp-address=127.0.0.1:4160 解析 TCP 地址名字-为上面 nsqlookupd 的 tcp
-tcp-address=127.0.0.1:4150          是 tcp 生产监听接口
-http-address=127.0.0.1:4151         是 HTTP 客户端监听
-broadcast-address=127.0.0.1         地址注册到 nsqlookupd (默认名是 OS Name)
-data-path=/xx/xx                    为消息持久化的位置
-mem-queue-size=0                    内存里的消息数，设置为 0，所有的消息将会存储到磁盘
```

#### nsqadmin
```
--lookupd-http-address=127.0.0.1:4161  为上面 nsqlookupd 的 http
```

#### 简单使用
```
nohup ./nsqlookupd > /dev/null 2>&1 &
nohup ./nsqd --lookupd-tcp-address=127.0.0.1:4160 > /dev/null 2>&1 &
nohup ./nsqadmin --lookupd-http-address=127.0.0.1:4161 > /dev/null 2>&1 &
```