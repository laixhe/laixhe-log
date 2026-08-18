# Docker Compose 安装

> 参考官方文档：https://docs.docker.com/compose/install/

现代 Docker 推荐使用 **Compose v2** 插件（命令为 `docker compose`，中间是空格），旧的 `docker-compose`（v1）已停止维护，请勿再安装。

## 方式一：Docker Desktop（推荐，Windows / macOS）

Docker Desktop 自带 Compose v2 插件，安装后直接可用：

```bash
docker compose version
```

## 方式二：Linux 通过包管理器安装（推荐，Ubuntu）

使用 `apt` 安装 Docker 官方仓库的 Compose 插件：

```bash
sudo apt-get update
sudo apt-get install docker-compose-plugin
```

## 方式三：手动下载二进制（无包管理器时）

将 Compose 二进制放到用户级插件目录 `~/.docker/cli-plugins/`：

```bash
mkdir -p ~/.docker/cli-plugins
curl -SL "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" -o ~/.docker/cli-plugins/docker-compose
chmod +x ~/.docker/cli-plugins/docker-compose
```

## 验证

```bash
docker compose version
```

> 旧版 `docker-compose` 与新版 `docker compose` 用法基本一致，但新版去掉了连字符 `-`。本仓库所有示例均使用 `docker compose`。
