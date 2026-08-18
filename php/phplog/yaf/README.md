# Yaf 示例项目

一个基于 Yaf 框架的 PHP 示例，演示 MVC、多模块、数据库操作（PDO）与日志（Monolog）。

## 目录结构

```text
yaf/
├── application/
│   ├── common/        # 公共类（Response 统一响应、Log 日志封装）
│   ├── controllers/   # 默认模块控制器
│   ├── library/db/    # 本地类库（Sqldb 数据库封装）
│   ├── models/        # 数据模型
│   ├── modules/Admin/ # 后台模块（多模块示例）
│   ├── plugins/       # 插件（Yaf Hook）
│   └── views/         # 视图
├── conf/conf.ini      # 配置（路由、数据库）
├── composer.json      # Composer 依赖（Monolog）
└── public/
    ├── index.php      # 入口文件
    └── .htaccess      # Apache 重写规则
```

## 前置依赖

- PHP 8.5 + php-fpm
- Yaf 扩展（建议 ≥ 3.3.7，此版本起支持 PHP 8.5）
- PDO MySQL 扩展
- Composer（安装 Monolog）
- Nginx（或 Apache）

## 运行说明

1. 编译安装 Yaf 扩展（见下文）。
2. 安装 Monolog 依赖：

```bash
composer install
```

3. 修改 [conf/conf.ini](./conf/conf.ini) 中的数据库账号密码。
4. 将站点根目录指向 `public/`，入口为 `index.php`。
5. 配合 Nginx 把 `.php` 请求转发给 php-fpm（参考 [nginx-php.md](../../../nginx/nginx-php.md)）。

访问示例（路由定义在 conf.ini）：

```text
http://localhost/v1.0/show    -> Index 模块 Index 控制器 index 动作
http://localhost/v1.0/config  -> Index 模块 Index 控制器 config 动作
http://localhost/v1.0/test/1  -> Index 模块 Index 控制器 test 动作
```

## 编译 Yaf

> Yaf 是 C 扩展，需编译安装。**建议使用 3.3.7 及以上版本**（3.3.7 起支持 PHP 8.5）。

```bash
$PHP_BIN/phpize

./configure --with-php-config=$PHP_BIN/php-config
# 或指定绝对路径
# ./configure --with-php-config=/usr/local/php/bin/php-config

make

make install
```

### Yaf 相关配置

本地开发设置成 `develop`，测试环境配置成 `test`，生产环境配置成 `product`。

```ini
[yaf]
;运行环境，默认值：product
yaf.environ = "develop"
;开启命名空间，默认值：0
yaf.use_namespace = 1
;全局类库的目录路径，默认值：NULL
;yaf.library = "/.../"
```

## 日志（Monolog）

日志使用 Monolog（纯 PHP，兼容 PHP 8.5），封装在 [application/common/Log.php](./application/common/Log.php) 中。

```bash
composer require monolog/monolog
```

默认日志文件路径为 `/lai/logs/api.log`，如需修改请编辑 `Log.php` 中的 `StreamHandler` 路径。

调用示例：

```php
Log::info('message');
Log::error('message');
```
