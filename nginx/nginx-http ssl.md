# Nginx HTTPS / SSL 配置

> 网站启用 HTTPS 后，会加重服务器负担。传统 HTTP 用 TCP 三次握手建立连接，而 SSL/TLS 在此基础上还需要额外的握手包，所以负担更明显。但现代浏览器普遍强制 HTTPS，安全收益远大于这点开销。

## HTTP 跳转到 HTTPS

```nginx
server {
    listen 80;
    return 301 https://$host$request_uri;
}
```

## 配置 HTTPS

```nginx
server {

    listen 443 ssl;
    server_name xxxx;

    ssl_certificate 证书.crt;
    ssl_certificate_key 证书.key;

    # 仅保留安全协议，TLSv1 / TLSv1.1 已被浏览器弃用
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers ECDHE-RSA-AES128-GCM-SHA256:HIGH:!aNULL:!MD5:!RC4:!DHE;
    ssl_prefer_server_ciphers on;
    # 会话过期时间，单位分钟
    ssl_session_timeout 5m;
    # 会话缓存的类型和大小
    ssl_session_cache shared:SSL:50m;
}
```

## 开启 HTTP/2

> 尽管 HTTP/2 协议本身不要求一定开启 SSL，但浏览器要求必须启用 SSL 才能使用 HTTP/2。

Nginx 1.25.1+ 已弃用 `listen ... http2` 写法，改为独立的 `http2 on;`：

```nginx
server {
    listen 443 ssl;
    http2 on;
    ...
}
```
