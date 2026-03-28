##### redis
```
phpize

./configure

make 

sudo make install


extension=redis

# 安装到其他 PHP 版本
/home/laixhe/software/php81/bin/phpize
./configure --with-php-config=/home/laixhe/software/php81/bin/php-config
```


```
[XDebug]
zend_extension=xdebug
xdebug.mode = debug
xdebug.client_host = 127.0.0.1
xdebug.client_port = 9003
xdebug.collect_return = On
xdebug.start_with_request = yes
xdebug.idekey = PHPSTORM
```
