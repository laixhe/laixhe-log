# Nginx WebSocket 代理

## WebSocket 建立过程

> 客户端发起升级协议的请求，采用标准的 HTTP 报文格式，在报文中添加头部信息。

### 客户端

表明连接需要升级：

```http
Connection: Upgrade
```

升级到 WebSocket 协议：

```http
Upgrade: websocket
```

协议版本为 13：

```http
Sec-WebSocket-Version: 13
```

客户端唯一的值：

> 生成一组 16 位的随机 base64 编码，是浏览器随机生成的。

> 与服务器响应的 `Sec-WebSocket-Accept` 对应。

```http
Sec-WebSocket-Key: I6qjdEaqYljv3+9x+GrhqA==
```

### 服务端

状态：

```http
Status Code: 101 Switching Protocols
```

已升级连接：

```http
Connection: Upgrade
```

已升级到 WebSocket 协议：

```http
Upgrade: websocket
```

响应的 `Sec-WebSocket-Accept`：

> 响应时，服务器必须将特殊 GUID 值 `258EAFA5-E914-47DA-95CA-C5AB0DC85B11` 附加到密钥，对结果字符串做 SHA-1 哈希，再输出其 base64 编码。

```http
Sec-WebSocket-Accept: base64( SHA-1( Sec-WebSocket-Key + 258EAFA5-E914-47DA-95CA-C5AB0DC85B11 ) )
```

### Nginx 配置

```nginx
location /ws {
    proxy_pass http://127.0.0.1/ws;

    proxy_redirect off;
    proxy_set_header Host $http_host;
    proxy_set_header X-Forwarded-For $remote_addr;
    proxy_set_header X-Real-IP $remote_addr;
    proxy_set_header http_user_agent $http_user_agent;

    # WebSocket 需要 HTTP/1.1 并显式声明升级头
    proxy_http_version 1.1;
    proxy_set_header Upgrade $http_upgrade;
    proxy_set_header Connection "Upgrade";
}
```
