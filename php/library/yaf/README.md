
### 编译 Yaf
```
$PHP_BIN/phpize

./configure --with-php-config=$PHP_BIN/php-config
./configure --with-php-config=/usr/local/php/bin/php-config

make

make install
```

#### Yaf 相关配置
```
本地开发设置成 develop
测试环境配置成 test
生产环境配置成 product
```

```
[yaf]
;运行环境，默认值：product
yaf.environ = "develop"
;开启命名空间，默认值：0
yaf.use_namespace = 1
;全局类库的目录路径，默认值：NULL
;yaf.library = "/.../"
```

### 编译 SeasLog
```
$PHP_BIN/phpize

./configure --with-php-config=$PHP_BIN/php-config

make

make install
```

#### SeasLog 相关配置
```
[SeasLog]
;日志格式模板
seaslog.default_template = "%L | %R | %m | %I | %T | %Q | %F | %M"
```
