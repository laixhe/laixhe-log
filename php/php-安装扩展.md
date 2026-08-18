# PHP 安装扩展

源码方式编译安装 PHP 扩展（以 redis、xdebug 为例）。

## redis

```bash
phpize

./configure

make

sudo make install
```

编译完成后，在 `php.ini` 中启用：

```ini
extension=redis
```

### 安装到其他 PHP 版本

```bash
/home/laixhe/software/php85/bin/phpize
./configure --with-php-config=/home/laixhe/software/php85/bin/php-config
```

## Xdebug

编译安装后，在 `php.ini` 中配置（Xdebug 3 写法）：

```ini
[XDebug]
zend_extension=xdebug
xdebug.mode = debug
xdebug.client_host = 127.0.0.1
xdebug.client_port = 9003
xdebug.collect_return = On
xdebug.start_with_request = yes
xdebug.idekey = PHPSTORM
```
