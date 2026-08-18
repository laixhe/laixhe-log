# PHP Windows 启动

Windows 下通过 `php-cgi` 启动 PHP，供 Nginx 等 Web 服务器转发请求。

> 如果 `php.ini` 不在 php 根目录下，则需明确指定 `php.ini` 的路径。

## phpstart.bat

```bat
D:
cd D:\software\php
php-cgi -b 127.0.0.1:9000 -c php.ini
```
