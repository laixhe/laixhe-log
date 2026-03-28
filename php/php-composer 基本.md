#### 安装 Composer

    php -r "copy('https://install.phpcomposer.com/installer', 'composer-setup.php');"

    php composer-setup.php

#### Windows 系统

> 新建一个 `composer.bat` 文件

    @php "%~dp0composer.phar" %*

#### 国内镜像(阿里云)

    composer config -g repo.packagist composer https://mirrors.aliyun.com/composer/
    # 解除镜像并恢复到 packagist 官方源
    composer config -g --unset repos.packagist

####
    show -all        # 查看已安装的包
    require xxx      # 下载包
    install xxx      # 安装包
    update [xxx]     # 更新包
    remove xxx       # 移除包
    dumpautoload     # 将 autoload 字段声明注册到 composer 中