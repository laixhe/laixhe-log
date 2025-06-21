
#### save 命令
```
# 同步数据到磁盘上
> save

# 异步保存数据集到磁盘上
> bgsave

```

#### 服务器配置自动触发 redis.conf
```
# 900s 内至少达到一条写命令
save 900 1
# 300s 内至少达至 10 条写命令
save 300 10
# 60s 内至少达到 10000 条写命令
save 60 10000
```
#### 相关配置 redis.conf
```
# 是否压缩rdb文件
rdbcompression yes

# 默认值 yes，即当 bgsave 快照操作出错时停止写数据到磁盘
# yes：不能进行工作，no：可以继续进行工作
stop-writes-on-bgsave-error no

# 是否校验rdb文件，有利于文件的容错性，会有大概10%的性能损耗
rdbchecksum yes

# rdb文件的名称
dbfilename redis-6379.rdb

# rdb文件保存目录
dir ~/redis/

save 900 1
save 300 10
save 60 10000

```
