
> 在 HTTP 的 SERVER 中 添加

```
# 表示允许这个域跨域调用（客户端发送请求的域名和端口） 
# $http_origin 动态获取请求客户端请求的域
# 不用 * 的原因是带 cookie 的请求不支持 * 号
add_header Access-Control-Allow-Origin $http_origin;
# 带 cookie 请求需要加上这个字段，并设置为 true
add_header Access-Control-Allow-Credentials true;
# 预检命令的缓存，如果不缓存每次会发送两次请求
add_header Access-Control-Max-Age 3600;
# 指定允许跨域的方法，* 代表所有
add_header Access-Control-Allow-Methods 'GET, POST, OPTIONS';
# 指定请求头的字段
# 动态获取 $http_access_control_request_headers
add_header Access-Control-Allow-Headers 'DNT,X-Mx-ReqToken,Keep-Alive,User-Agent,X-Requested-With,If-Modified-Since,Cache-Control,Content-Type';

# OPTIONS 请求不转给后端，直接返回 200
# OPTIONS 预检命令，预检命令通过时才发送请求
# 检查请求的类型是不是预检命令
if ($request_method = 'OPTIONS') {
    return 200;
}

```