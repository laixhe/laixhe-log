
> 众周所知网站启用https后，会加剧服务器的负担。传统的http使用TCP三次握手建立连接，而SSL和TLS在这个基础上还需要9个握手包，所以这个负担显而易见。

#### HTTP 跳转到 HTTPS
```
server {
    listen 80;
    return 301 https://$host$request_uri;
}
```

#### 配置 HTTPS
```
server {

    listen 443 ssl;
    server_name xxxx;
	
    ssl_certificate 证书.crt;
    ssl_certificate_key 证书.key;

    ssl_protocols TLSv1 TLSv1.1 TLSv1.2;
    ssl_ciphers ECDHE-RSA-AES128-GCM-SHA256:HIGH:!aNULL:!MD5:!RC4:!DHE;
    ssl_prefer_server_ciphers on;
    # 过期时间，分钟
    ssl_session_timeout 5m;
    # 设置缓存的类型和大小
    ssl_session_cache shared:SSL:50m;
}
```

#### 开启 HTTP/2
> 尽管 HTTP/2 协议本身并不要求一定开启 SSL，但浏览器要求一定要启用 SSL 才能使用 HTTP/2。
```
server {
    listen 443 ssl http2;
    ...
}
```
