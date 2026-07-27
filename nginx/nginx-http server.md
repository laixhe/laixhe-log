
#### 要放在 http 配置段下
> 虚拟主机的配置

```
server {

    # 监听端口
    listen              80;
    # 域名可以有多个，用空格隔开
    server_name  localhost;

    # 错误日志
    error_log  log/error_80.log;
    # 常规日志
    access_log  log/access_80.log;
    # 默认编码
    charset utf-8;

    # 定义服务器的默认网站根目录位置
    root   html;
    # 定义首页索引文件的名称
    index  index.html index.htm index.php;

    # 开启目录列表访问
    #autoindex on;

    # 图片缓存时间设置
    location ~ .*.(gif|jpg|jpeg|png|bmp|swf)$ {
        # 缓存 7 天
        expires 7d;
    }
    
    # JS 和 CSS 缓存时间设置
    location ~ .*.(js|css)?$ {
        # 缓存 1 个小时
        expires 1h;
    }

}
```
