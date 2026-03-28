
####  建立nginx 组
```
groupadd -r nginx
useradd -r -g nginx nginx
```
	
####  注意点
```
zlib:nginx提供gzip模块，需要zlib库支持
openssl:nginx提供ssl功能
pcre:支持地址重写rewrite功能
```

#### 设置配置项
```
./configure \
--prefix=/usr/local/nginx \
--user=nginx \
--group=nginx \
--with-pcre \
--with-http_ssl_module \
--with-http_v2_module \
--with-http_realip_module \
--with-http_addition_module \
--with-http_sub_module \
--with-http_dav_module \
--with-http_flv_module \
--with-http_mp4_module \
--with-http_gunzip_module \
--with-http_gzip_static_module \
--with-http_random_index_module \
--with-http_secure_link_module \
--with-http_stub_status_module \
--with-http_auth_request_module \
--with-http_image_filter_module \
--with-http_slice_module \
--with-mail \
--with-threads \
--with-file-aio \
--with-stream \
--with-mail_ssl_module \
--with-stream_ssl_module \
--http-client-body-temp-path=/usr/local/nginx/temp/client \
--http-proxy-temp-path=/usr/local/nginx/temp/proxy \
--http-fastcgi-temp-path=/usr/local/nginx/temp/fcgi \
--http-uwsgi-temp-path=/usr/local/nginx/temp/uwsgi \
--http-scgi-temp-path=/usr/local/nginx/temp/scgi
```

#### 编译 和 安装
```
make
make install
```

#### 编译出错后清除
```
make clean
```

#### 创建以下目录
```
mkdir /usr/local/nginx/temp
```

#### 添加 nginx 的环境变量
```
vim /etc/profile
在文件末尾加上
PATH=$PATH:/usr/local/nginx/sbin
使其修改生效
source /etc/profile
```

#### 可以使用的命今
```
nginx -s stop       快速关闭Nginx，可能不保存相关信息，并迅速终止web服务
nginx -s quit       平稳关闭Nginx，保存相关信息，有安排的结束web服务
nginx -s reload     因改变了Nginx相关配置，需要重新加载配置而重载
nginx -s reopen     重新打开日志文件
nginx -c filename   为 Nginx 指定一个配置文件，来代替缺省的
nginx -t            不运行，而仅仅测试配置文件 将检查配置文件的语法的正确性，并尝试打开配置文件中所引用到的文件
nginx -v            显示 nginx 的版本
nginx -V            显示 nginx 的版本，编译器版本和配置参数
```


#### 关闭SELINUX
```
vim /etc/selinux/config
SELINUX=disabled       # 修改为 disabled
#SELINUXTYPE=targeted  # 注释掉
```
