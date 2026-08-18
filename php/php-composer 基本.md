# Composer 基本用法

Composer 是 PHP 的依赖管理工具。

## 安装 Composer

```bash
php -r "copy('https://install.phpcomposer.com/installer', 'composer-setup.php');"

php composer-setup.php
```

## Windows 系统

> 新建一个 `composer.bat` 文件，内容如下：

```bat
@php "%~dp0composer.phar" %*
```

## 国内镜像（阿里云）

```bash
composer config -g repo.packagist composer https://mirrors.aliyun.com/composer/
# 解除镜像并恢复到 packagist 官方源
composer config -g --unset repos.packagist
```

## 常用命令

```bash
composer show              # 查看已安装的包
composer require xxx       # 下载并安装指定包（并写入 composer.json）
composer install           # 根据 composer.lock 安装所有依赖
composer update [xxx]      # 更新依赖（可指定单个包）
composer remove xxx        # 移除包
composer dump-autoload     # 重新生成 autoload 自动加载文件
```
