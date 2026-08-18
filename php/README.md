# PHP 学习笔记

本目录收集 PHP 的安装、配置与常用工具笔记，以及一个基于 Yaf 框架的示例项目。

## 目录导航

### 安装

| 文件 | 说明 |
| --- | --- |
| [php-ubuntu apt 安装.md](./php-ubuntu%20apt%20安装.md) | Ubuntu 通过 ondrej/php 仓库安装 PHP 8.5 |
| [php-编译.md](./php-编译.md) | 源码编译安装 PHP |
| [php-windows.md](./php-windows.md) | Windows 下启动 php-cgi |

### 配置

| 文件 | 说明 |
| --- | --- |
| [php-配置.md](./php-配置.md) | php.ini 常用配置（上传、时区、opcache、session） |
| [php-fpm 配置.md](./php-fpm%20配置.md) | php-fpm 进程池配置 |
| [php-安装扩展.md](./php-安装扩展.md) | 编译安装 redis / xdebug 扩展 |

### 工具

| 文件 | 说明 |
| --- | --- |
| [php-composer 基本.md](./php-composer%20基本.md) | Composer 安装与常用命令 |

### 示例项目

| 目录 | 说明 |
| --- | --- |
| [phplog/](./phplog/) | PHP 学习示例，含一个 Yaf 框架项目 |

## 推荐学习顺序

1. 先看 [php-ubuntu apt 安装.md](./php-ubuntu%20apt%20安装.md) 装好 PHP。
2. 看 [php-配置.md](./php-配置.md) 与 [php-fpm 配置.md](./php-fpm%20配置.md) 了解基础配置。
3. 看 [php-composer 基本.md](./php-composer%20基本.md) 掌握依赖管理。
4. 进入 [phplog/](./phplog/) 阅读 Yaf 示例代码。

## 环境约定

- 命令默认在 Linux（Ubuntu）下执行。
- 最低要求 PHP 8.5。
