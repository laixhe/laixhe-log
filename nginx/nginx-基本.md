
### nginx.conf 文件的结构

- Global: nginx 运行相关
- events: 与用户的网络连接相关
- http
- - http Global: 代理，缓存，日志，以及第三方模块的配置
- - server
- - - server Global: 虚拟主机相关
- - - location: 地址定向，数据缓存，应答控制，以及第三方模块的配置
> 所有的所有的所有的指令，都要以;结尾


#### 定义Nginx运行的用户和用户组
```
user  nginx;
```

#### nginx进程数，建议设置为等于CPU总核心数
```
worker_processes  4;
```

#### nginx 默认没有开启利用多核 CPU
> 通过增加 worker_cpu_affinity 配置参数来充分利用多核 CPU 以下是 8 核的配置参数
```
worker_cpu_affinity 00000001 00000010 00000100 00001000 00010000 00100000 01000000 10000000;
```

#### 毎个进程的最大文件打开数，最好与 Linux 的 ulimit -n 的值保持一致
```
worker_rlimit_nofile 65535;
```

#### 全局错误日志定义类型
> [ debug | info | notice | warn | error | crit ]
```
error_log  /usr/local/nginx/logs/error.log;
```

#### 进程文件
```
pid  /usr/local/nginx/logs/nginx.pid;
```

#### 工作模式与连接数上限
```
events {
    # 使用epoll的I/O模型，用这个模型来高效处理异步事件
    use epoll;
    # 单个进程最大连接数
    worker_connections  10240;
}
```

#### 设定 http 服务器
```
http {

    # 文件扩展名与文件类型映射表
    include         mime.types;
    # 默认文件类型
    default_type  application/octet-stream;
    # 默认编码
    #charset utf-8;

    # 开启高效文件传输模式
    sendfile on;
    # 防止网络阻塞,让在一个数据包中发送所有的头文件，而不是一个一个单独发,默认是关闭
    tcp_nopush on;
    # 防止网络阻塞,将小的数据包(缓存)打成一个大的包发送,默认是开启(禁用nagle算法，也即不缓存数据)
    tcp_nodelay on;

    # 长连接超时时间，单位是秒
    keepalive_timeout 65;
    # 设置客户端请求头读取超时时间
    client_header_timeout 10;
    # 设置客户端请求主体读取超时时间
    client_body_timeout 10;
    # 指定响应客户端的超时时间
    send_timeout 60;

    # 服务器名字的hash表大小
    server_names_hash_bucket_size 128;
    # 设置允许客户端请求的最大的单个文件字节数
    client_max_body_size 1024m; 
    # 指定来自客户端请求头的缓冲区大小(一般一个请求的头部大小不会超过1k)
    client_header_buffer_size 16k;
    # 指定客户端请求中较大的消息头的缓存最大数量和大小
    large_client_header_buffers 4 32k;
    # 缓冲区代理缓冲用户端请求的最大字节数
    client_body_buffer_size 256k;

    # gzip模块设置

    # 开启gzip压缩输出
    gzip on;
    # 不压缩临界值，大于1K的才压缩
    gzip_min_length 1k;
    # 压缩缓冲区 [4 16k 代表以16k为单位，安装原始数据大小以16k为单位的4倍申请内存]
    gzip_buffers 4 16k;
    # 压缩版本，识别 http 的协议版本
    gzip_http_version 1.1;
    # 压缩等级
    gzip_comp_level 6;
    # 压缩类型，默认就已经包含text/html
    gzip_types text/plain text/css text/javascript text/xml text/x-json text/x-component application/javascript application/x-javascript text/javascript application/json application/xml font/opentype font/ttf image/x-icon;
    # 在 http header 中添加 Vary:Accept-Encoding 给代理服务器用的
    gzip_vary on;
    # 这里设置无论 header 头是怎么样，都是无条件启用压缩
    gzip_proxied any;
    
    autoindex on;            # 显示目录
    autoindex_exact_size on; # 显示文件大小
    autoindex_localtime  on; # 显示文件时间

    #禁止IP防问和非法域名
    server {
        listen 80 default;
        listen 88 default;
        listen 8080 default;
        server_name _;
        return 500;
    }

    include /usr/local/nginx/conf/conf.d/*.conf;
}
```
