# Nginx 学习笔记

本目录收集 Nginx 的安装、配置与常用场景示例。

## 目录导航

### 安装

| 文件 | 说明 |
| --- | --- |
| [nginx-ubuntu apt 安装.md](./nginx-ubuntu%20apt%20安装.md) | Ubuntu 通过官方仓库安装 |
| [nginx-编译.md](./nginx-编译.md) | 源码编译安装 |
| [nginx-windows.md](./nginx-windows.md) | Windows 下的启动/停止脚本 |

### 基础配置

| 文件 | 说明 |
| --- | --- |
| [nginx-基本.md](./nginx-基本.md) | nginx.conf 结构与常用指令 |
| [nginx-配置.md](./nginx-配置.md) | 完整 nginx.conf 示例 |
| [nginx-匹配.md](./nginx-匹配.md) | location 正则与条件语法 |

### 常用场景

| 文件 | 说明 |
| --- | --- |
| [nginx-http server.md](./nginx-http%20server.md) | 虚拟主机、静态资源缓存 |
| [nginx-http ssl.md](./nginx-http%20ssl.md) | HTTPS / SSL / HTTP2 配置 |
| [nginx-http-proxy 代理.md](./nginx-http-proxy%20代理.md) | 反向代理与缓存 |
| [nginx-php.md](./nginx-php.md) | 配合 PHP-FPM |
| [nginx-tcp.md](./nginx-tcp.md) | TCP / UDP 四层代理 |
| [nginx-websocket.md](./nginx-websocket.md) | WebSocket 代理 |
| [nginx-允许跨域.md](./nginx-允许跨域.md) | CORS 跨域配置 |

### 运维

| 文件 | 说明 |
| --- | --- |
| [nginx-限制.md](./nginx-限制.md) | 限流、限速、并发限制 |
| [nginx-日志切割.md](./nginx-日志切割.md) | 日志按天切割 |

## 推荐学习顺序

1. 先看 [nginx-ubuntu apt 安装.md](./nginx-ubuntu%20apt%20安装.md) 装好 Nginx。
2. 看 [nginx-基本.md](./nginx-基本.md) 理解配置文件结构。
3. 按需查阅 [常用场景](#常用场景) 中的各示例。

## 环境约定

- 配置文件默认使用 `nginx` 语言标注。
- 现代 Nginx 推荐使用 TLSv1.2 / TLSv1.3，`listen ... http2` 旧写法已改用 `http2 on;`（Nginx 1.25.1+）。
