
### 简单日志切割

```
日志保存位置
nginx_pth=xxx

获取昨天的日信息
nginx_yesterday=$(date -d yesterday +"%d")

备份昨天的日志
mv $base_path/access.log $base_path/$nginx_yesterday.log

重读日志
/usr/local/nginx/sbin/nginx -s reopen
```