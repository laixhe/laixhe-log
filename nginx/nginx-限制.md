
### 限制请求速度,每秒只能处理每个 IP 地址的 N 个请求求
```
limit_req_zone $binary_remote_addr zone=perip:10m rate=5r/s;
 key : 定义需要限流的对象
 zone: 定义共享内存区来存储访问信息
 rate: 用于设置最大访问速率
 
 上面 binary_remote_addr 就是 key，表示基于客户端 ip(remote_addr) 进行限流
 定义了一个大小为10M，名称为 perip 的内存区，用于存储IP地址访问信息
 rate=5r/s 表示每秒只能处理每个IP地址的5个请求
 Nginx限流是按照毫秒级为单位的
```

##### 例
```
http {
    limit_req_zone $binary_remote_addr zone=perip:10m rate=5r/s;
}

server {
    location / {
        limit_req zone=perip;
        .....
    }
}
```

##### 限制请求速度,并建立5缓冲队列和不延迟
```
limit_req zone=one burst=5 nodelay;
```

### 限制并发连接数 - 单个IP
```
# 用于开辟一个共享内存空间保存客户端 IP,空间名称为 perip ,空间大小为 10M
limit_conn_zone $binary_remote_addr zone=perip:10m;

# 限制连接数量
limit_conn perip 5;
```

##### 例
```
http {
    limit_conn_zone $binary_remote_addr zone=perip:10m;
}

server {
    location / {
        limit_conn perip 5;
        .....
    }
}
```

### 限制并发连接数 - 整个虚拟服务器
```
# 用于开辟一个共享内存空间保存虚拟服务器,空间名称为 perserver ,空间大小为 100M
limit_conn_zone $server_name zone=perserver:100m;

# 限制连接数量
limit_conn perserver 500;
```

##### 例
```
http {
    limit_conn_zone $server_name zone=perserver:100m;
}

server {
    location / {
        limit_conn perserver 500;
        .....
    }
}
```

### 其它
	
##### 对每个连接限速100k/秒
```
limit_rate 100k;
```
##### 设置不限速传输的响应大小,前3M的下载内容不进行限速
```
limit_rate_after 3m;
```

##### 简单的限制下载速度
```
location / {
    limit_rate 256K;
}
```

