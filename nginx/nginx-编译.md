# Nginx 源码编译安装

## 建立 nginx 用户和组

```bash
groupadd -r nginx
useradd -r -g nginx nginx
```

## 依赖说明

```text
zlib   : nginx 提供 gzip 模块，需要 zlib 库支持
openssl: nginx 提供 ssl 功能
pcre   : 支持地址重写 rewrite 功能
```

## 设置配置项

```bash
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

## 编译和安装

```bash
make
make install
```

## 编译出错后清除

```bash
make clean
```

## 创建临时目录

```bash
mkdir /usr/local/nginx/temp
```

## 添加 nginx 环境变量

```bash
vim /etc/profile
# 在文件末尾加上：
PATH=$PATH:/usr/local/nginx/sbin
# 使其修改生效
source /etc/profile
```

## 常用命令

```bash
nginx -s stop       快速关闭Nginx，可能不保存相关信息，并迅速终止web服务
nginx -s quit       平稳关闭Nginx，保存相关信息，有安排的结束web服务
nginx -s reload     因改变了Nginx相关配置，需要重新加载配置而重载
nginx -s reopen     重新打开日志文件
nginx -c filename   为 Nginx 指定一个配置文件，来代替缺省的
nginx -t            不运行，而仅仅测试配置文件 将检查配置文件的语法的正确性，并尝试打开配置文件中所引用到的文件
nginx -v            显示 nginx 的版本
nginx -V            显示 nginx 的版本，编译器版本和配置参数
```

## 关闭 SELinux

```bash
vim /etc/selinux/config
SELINUX=disabled       # 修改为 disabled
#SELINUXTYPE=targeted  # 注释掉
```
