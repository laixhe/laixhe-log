# Nginx 日志切割

按天切割 access.log，避免日志文件无限增长。核心思路：把当天的日志改名备份，再让 Nginx 重新打开新的日志文件。

## 简单日志切割

```bash
#!/bin/bash

# 日志保存目录（按需修改）
base_path=/usr/local/nginx/logs

# 获取昨天的完整日期，例如 2026-08-12（用 %F 避免跨月覆盖）
nginx_yesterday=$(date -d yesterday +%F)

# 备份昨天的日志
mv $base_path/access.log $base_path/access_$nginx_yesterday.log

# 让 nginx 重新打开日志文件，后续日志写入新的 access.log
/usr/local/nginx/sbin/nginx -s reopen
```

## 配合定时任务

把脚本保存为 `cut_nginx_log.sh`，加入 crontab 每天凌晨执行：

```bash
chmod +x /usr/local/nginx/cut_nginx_log.sh

# 编辑定时任务
crontab -e
```

添加一行（每天 0 点执行）：

```bash
0 0 * * * /usr/local/nginx/cut_nginx_log.sh
```
