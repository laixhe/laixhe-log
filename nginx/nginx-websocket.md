
### webSocket 建立过程
> 客户端发起升级协议的请求，采用标准的HTTP报文格式，在报文中添加头部信息

## 客户端

#### 表明连接需要升级
```
Connection: Upgrade
```

#### 升级到 websocket 协议
```
Upgrade: websocke
```

#### 协议的版本为13
```
Sec-WebSocket-Version: 13
```

#### 客户端唯一的值
> 生成一组16位的随机 base64 编码，是浏览器随机生成的

> 与服务器响应的 ```Sec-WebSocket-Accept``` 对应
```
Sec-WebSocket-Key: I6qjdEaqYljv3+9x+GrhqA==
```

## 服务端

#### 状态
```
Status Code: 101 Switching Protocols
```

#### 已升级连接
```
Connection: Upgrade
```

#### 已升级到 websocket协议
```
Upgrade: websocke
```

#### 响应的 ```Sec-WebSocket-Accept```
> 响应时，服务器必须将特殊 GUID 值 258EAFA5-E914-47DA-95CA-C5AB0DC85B11 附加到密钥，生成结果字符串的 SHA-1 哈希值，然后将其包含为Sec的 base-64 编码值
```
Sec-WebSocket-Accept: base-64( SHA-1( Sec-WebSocket-Key + 258EAFA5-E914-47DA-95CA-C5AB0DC85B11 ) )
```

#### Nginx 配置

```
location /ws {
    proxy_pass http://127.0.0.1/ws;
    
    proxy_redirect off;
    proxy_set_header Host $http_host;
    proxy_set_header X-Forwarded-For $remote_addr;
    proxy_set_header X-Real-IP $remote_addr;
    proxy_set_header http_user_agent $http_user_agent;
    
    proxy_http_version 1.1;
    proxy_set_header Upgrade $http_upgrade;
    proxy_set_header Connection "Upgrade";
}

```