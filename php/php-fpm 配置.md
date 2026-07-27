```
[www]
# 用户和组
user = nginx
group = nginx
# ip端口
listen = 127.0.0.1:9000
# 进程管理方式 dynamic 动态 static 静态
pm = dynamic
# 同一时刻允许最大进程数(static 静态)
pm.max_children = 300
# 动态方式下的起始进程数
pm.start_servers = 20
# 动态方式下空闲最小进程数
pm.min_spare_servers = 5
# 动态方式下空闲最大进程数
pm.max_spare_servers = 50
# 进程处理的请求数累积到 10000 个后，自动重启该进程
pm.max_requests = 10000
# 打开文件描述符限制
rlimit_files = 1024000

# 错误级别,默认: notice,可用级别为: alert（必须立即处理）,error（错误情况）,warning（警告情况）,
# notice（一般重要信息）,debug（调试信息）
log_level = notice
# 错误日志，默认在安装目录中的 var/log/php-fpm.log
error_log = log/php-fpm.log
```
