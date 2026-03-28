
##### 由于 Nginx 要求 cgi 方式的 php，还要修改以下配置
```
cgi.force_redirect = 0
cgi.fix_pathinfo = 1
fastcgi.impersonate = 1
cgi.rfc2616_headers = 1
```

##### 可能需要将php脚本的内存限制调大，否则某些项目启动将会提示内存不足
```
memory_limit = 128M
```

##### 文件上传
```
file_uploads = On
# 限制 PHP 处理上传文件的最大值，此值必须小于 post_max_size 值，Nginx 的 client_max_body_size 一样
upload_max_filesize = 2M
# 限制通过 POST 方法可以接受的信息最大量
post_max_size = 2M
```

##### 设置PHP的时区
```
date.timezone = Asia/Shanghai
```

##### 添加新的模块
> extension_dir = "ext"
```
extension=yaf.so
zend_extension=opcache.so
```

##### 处理会话 session tcp://
```
session.save_handle = "memcached"
session.save_path = "127.0.0.1:11211"
# 有效期,客户端超过120分钟没有刷新，当前 session 就会失效
session.gc_maxlifetime=7200
```

##### 操作码缓存 opcache
```
# 开启操作码缓存
opcache.enable=1
# 仅针对 CLI 版本的 PHP 启用操作码缓存(不用)
opcache.enable_cli=1
# 控制优化级别的二进制位掩码
opcache.optimization_level=1

# 为操作码缓存分配的共享内存量(单位 MB),默认64M
opcache.memory_consumption=512
# 用来存储临时驻留字符串的内存量(单位 MB),默认8M
opcache.interned_strings_buffer=64
# 操作码缓存中最多能存储多少个 php 脚本
# 一定要比 php 应用中的文件数量大,默认2000
opcache.max_accelerated_files=4000

# 这个值设为 1 时，经过一段时间后 php 会检查 php 脚本的内容是否变化
# 检查的时间间隔由 opcache.revalidate_freq 设置指定。如果这个设置的值为 0,
# php 不会检查 php 脚本的内容是否变化，我们必须手动清除缓存的操作码。
# 建议开发环境中设为 1，生产环境设为 0,默认1
opcache.validate_timestamps=1
# 设置多久 (单位是秒) 检查一次 php 脚本的内容是否变化,默认2
opcache.revalidate_freq=60
```
