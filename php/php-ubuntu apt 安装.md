### 更新apt源文件

    # 添加 ondrej/php 存储库
    sudo add-apt-repository ppa:ondrej/php
    sudo apt update

    sudo apt install php8.3 php8.3-common php8.3-cli php8.3-fpm php8.3-{curl,bz2,mbstring,intl}
    # 安装 php8.3-common 大致相当于安装所有扩展，如下所示
    sudo apt install php8.3-{calendar,ctype,exif,ffi,fileinfo,ftp,gettext,iconv,pdo,phar,posix,shmop,sockets,sysvmsg,sysvsem,sysvshm,tokenizer}

    # 其他扩展
    sudo apt install php8.3-dev
    sudo apt install php8.3-redis

    # 目录
    /etc/php/8.3

    # 修改配置
    vim /etc/php/8.3/fpm/pool.d/www.conf
    listen = /run/php/php8.3-fpm.sock
    修改为
    listen = 127.0.0.1:9000

    sudo systemctl start php8.3-fpm
    sudo systemctl restart php8.3-fpm

