
### 代理配置参数
```
proxy_pass http://xx;                          # 请求转发给真正的后端服务器
proxy_redirect off;
proxy_set_header Host $http_host;              # 把 host 头传过去
proxy_set_header X-Forwarded-For $remote_addr; # 设置用户真实ip否则获取到的都是 nginx 服务器的ip
proxy_set_header X-Real-IP $remote_addr;
proxy_set_header http_user_agent $http_user_agent;

proxy_connect_timeout 60;          # 后端服务器连接超时时间 (代理连接超时)
proxy_send_timeout 60;             # 后端服务器数据回传时间 (代理发送超时)
proxy_read_timeout 60;             # 连接成功后，后端服务器响应时间 (代理接收超时)
proxy_buffering on;                # 启用缓冲,默认开启(on),此参数开启后 proxy_buffers 和 proxy_busy_buffers_size 参数才会起作用
proxy_buffer_size 4k;              # 设置代理服务器 (nginx) 保存用户头信息的缓冲区大小
proxy_buffers 16 16k;              # 缓冲区,如果页面大小为256k，则为其分配16个16k的缓冲区来缓存
proxy_busy_buffers_size 32k;       # 高负荷下缓冲大小 (proxy_buffers*2)
proxy_temp_file_write_size 64k;    # 限制一次性写入临时文件的数据大小
#proxy_ignore_client_abort on;     # 表示代理服务端不要主动关闭客户端连接，默认 off(不建议)

#proxy_max_temp_file_size 1024m;    # 默认 1024m, 该指令用于设置当网页内容大于 proxy_buffers 时，临时文件大小的最大值。如果文件大于这个值，它将从 upstream 服务器同步地传递请求，而不是缓冲到磁盘
#proxy_temp_path  /vxxx/proxy_temp; # 定义缓冲存储目录，之前必须要先手动创建此目录
#proxy_headers_hash_max_size 51200;
#proxy_headers_hash_bucket_size 6400;


{
proxy_cache_path /lai/cache levels=1:2 keys_zone=my_cache:10m max_size=100m inactive=60m use_temp_path=off;
# 要放在 http 配置段下
# levels 在 /lai/cache 设置了一个两级层次结构的目录,如果 levels 参数没有配置，则将所有的文件放到同一个目录中
# keys_zone 设置一个共享内存区,名为 my_cache ,并 10M 大小
# max_size 设置了缓存的上限100M
# inactive 指定了项目在不被访问的情况下能够在内存中保持的时间,默认值为10分钟(10m)
}

proxy_cache my_cache;
proxy_cache_valid 200 302 10m;    # 为响应码是200和302的资源，设置缓存时长为10分钟
proxy_cache_lock on;              # 缓存并发锁
procy_cache_lock_timeout 1s;      # 缓存并发锁等待时间
# 收到服务器返回的 error，timeout 或者其他指定的5xx错误，其缓存信息发送给客户端
proxy_cache_use_stale error timeout invalid_header updating http_500 http_502 http_503 http_504 http_403 http_404;
add_header Cache-Status "$upstream_cache_status";
proxy_cache_bypass $cookie_nocache $arg_nocache$arg_comment; # 绕开缓存,如果任何一个参数值不为空，或者不等于0，nginx就不会查找缓存，直接进行代理转发
```
