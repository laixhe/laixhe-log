# Docker 学习笔记

本目录收集 Docker 与 Docker Compose 的入门笔记与常用服务编排示例。

## 目录导航

| 文件 / 目录 | 说明 |
| --- | --- |
| [docker-基本.md](./docker-基本.md) | 镜像、容器、网络等常用命令清单 |
| [docker-安装.md](./docker-安装.md) | Ubuntu 安装 Docker 与镜像加速配置 |
| [docker-compose 安装.md](./docker-compose%20安装.md) | 安装 Docker Compose v2 插件 |
| [dockerfile-基本.md](./dockerfile-基本.md) | Dockerfile 关键字说明与多阶段构建示例 |
| [common-docker/](./common-docker/) | 一套开箱即用的多服务编排（MySQL、Redis、Kafka 等） |

## 推荐学习顺序

1. 先看 [docker-安装.md](./docker-安装.md) 把 Docker 装好。
2. 看 [docker-基本.md](./docker-基本.md) 熟悉镜像与容器的常用命令。
3. 看 [dockerfile-基本.md](./dockerfile-基本.md) 学会自己构建镜像。
4. 看 [docker-compose 安装.md](./docker-compose%20安装.md) 装好 Compose，再进入 [common-docker/](./common-docker/) 体验多服务编排。

## 环境约定

- 命令默认在 Linux（Ubuntu）下执行。
- 新版 Docker 使用 `docker compose`（v2 插件，命令中间是空格），旧的 `docker-compose`（v1）已停止维护，不再推荐。
