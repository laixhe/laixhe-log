# PHP Ubuntu 安装（ondrej/php）

使用 ondrej/php 仓库安装 PHP 8.5 及常用扩展。

## 添加仓库并安装

```bash
# 添加 ondrej/php 存储库
sudo add-apt-repository ppa:ondrej/php
sudo apt update

# 安装基础包（大括号是 bash 展开，等价于逐个列出包名）
sudo apt install php8.5 php8.5-common php8.5-cli php8.5-fpm php8.5-{curl,bz2,mbstring,intl}

# 安装 php8.5-common 大致相当于安装所有扩展，如下所示
sudo apt install php8.5-{calendar,ctype,exif,ffi,fileinfo,ftp,gettext,iconv,pdo,phar,posix,shmop,sockets,sysvmsg,sysvsem,sysvshm,tokenizer}

# 其他扩展
sudo apt install php8.5-dev
sudo apt install php8.5-redis
```

## 配置

配置文件目录：`/etc/php/8.5`。

修改 FPM 监听方式为 TCP：

```bash
vim /etc/php/8.5/fpm/pool.d/www.conf
# 将
listen = /run/php/php8.5-fpm.sock
# 修改为
listen = 127.0.0.1:9000
```

## 启动

```bash
sudo systemctl start php8.5-fpm
sudo systemctl enable php8.5-fpm
```

> 修改配置后重启：`sudo systemctl restart php8.5-fpm`。
