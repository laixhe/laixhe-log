
### Redis安装

#### make 进行编译

> make PREFIX=/usr/local/redis install

> 参考编译目录下的 redis.conf

#### 设置环境变量
```
vim /etc/profile

export REDIS_HOME=/usr/local/redis
export PATH=$PATH:$REDIS_HOME/bin

source /etc/profile
```

> Go 安装
```
go get github.com/go-redis/redis/v8
```

> Go 导入
```
import "github.com/go-redis/redis/v8"
```
