#### 建立 nginx 组

    sudo groupadd -r nginx
    sudo useradd -r -g nginx nginx

#### 下载

    http://php.net/downloads.php

#### 解压

    .tar.gz    格式解压为   tar -zxvf xx.tar.gz
    .tar.bz2   格式解压为   tar -jxvf xx.tar.bz2

####

    /usr/local/php
    
#### 设置配置项

    ./configure \
    --prefix=/usr/local/php \
    --with-config-file-path=/usr/local/php/etc \
    --with-mysqli --with-pdo-mysql --with-pdo-sqlite \
    --with-mhash --with-iconv --with-zlib --with-zip \
    --with-curl --with-openssl --with-sodium \
    --with-jpeg --with-freetype --with-webp \
    --enable-gd --enable-gd-jis-conv \
    --enable-pdo --enable-mysqlnd \
    --enable-mbregex --enable-mbstring \
    --enable-sysvmsg --enable-sysvsem --enable-sysvshm \
    --enable-shmop --enable-soap --enable-sockets \
    --enable-xml --enable-bcmath --enable-calendar --enable-exif \
    --enable-opcache --enable-cgi --enable-pcntl \
    --enable-fpm \
    --with-fpm-user=nginx --with-fpm-group=nginx
    
    # 自 PHP 7.4.0 起，--with-gd 变为 --enable-gd
    # 自 PHP 7.4.0 起，--with-png 已经被移除(内置了)，需要 --with-jpeg --with-freetype 启用
    # 自 PHP 8.0.0 起，JSON 扩展是核心扩展了，所以始终启用，不需要 --enable-json
    
    
#### 编译和安装

    make
    sudo make install

#### 编译出错后清除

    make clean

#### 复制配置文件

```
sudo cp 解压目录下/php.ini-development /usr/local/php/etc/php.ini

sudo cp /usr/local/php/etc/php-fpm.conf.default /usr/local/php/etc/php-fpm.conf

sudo cp /usr/local/php/etc/php-fpm.d/www.conf.default /usr/local/php/etc/php-fpm.d/www.conf

sudo cp 解压目录下/sapi/fpm/php-fpm.service /etc/systemd/system/php-fpm.service

sudo vim /etc/systemd/system/php-fpm.service (改成 ProtectSystem=false )
```

#### 配置 php 启动文件(可选)

    sudo  ln -s /usr/local/php/sbin/php-fpm /usr/local/php/bin/php-fpm    ---创建软连接

#### 添加 php 的环境变量

    sudo vim /etc/profile
    # 在文件末尾加上
    export PATH=$PATH:/usr/local/php/bin
    # 使其修改生效
    source /etc/profile

#### php-fpm 的执行文件

    /usr/local/php/sbin/php-fpm

####

    killall php-fpm 杀死进程
    kill -INT      立即终止 SIGINT
    kill -QUIT   平滑终止 SIGQUIT
    kill -USR1  重新打开日志文件 SIGUSR1
    kill -USR2  重启 SIGUSR2
    
    sudo systemctl start php-fpm
    sudo systemctl restart php-fpm
    sudo systemctl enable php-fpm

#### 设置时区

    date.timezone = Asia/Shanghai

#### 关闭 SELINUX

    vim /etc/selinux/config
    SELINUX=disabled        #修改为 disabled
    #SELINUXTYPE=targeted   #注释掉

