#### 混合持久化模式(在redis 4.0 之后，官方提供了混合持久化模式)
```
aof-use-rdb-preamble yes
```

#### 相关配置
```
# 开启aof机制 - 默认redis使用的是rdb方式持久化
appendonly yes

# aof文件名
appendfilename "appendonly.aof"

# 写入策略,默认 everysec
# always   每个写操作都保存到 aof 文件中
# everysec 每秒写入一次aof文件，因此，最多可能会丢失1s的数据
# no
appendfsync everysec

# 设置为 yes 表示 rewrite 期间对新写操作不 fsync,暂时存在内存中,等 rewrite 完成后再写入,默认no
no-appendfsync-on-rewrite no

# 保存目录
dir ~/redis/

```

#### 修复 aof 日志文件
```
redis-check-aof -fix file.aof
```
