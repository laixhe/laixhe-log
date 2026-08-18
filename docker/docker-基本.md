# Docker 基本命令

> 说明：命令行 `#` 后是参数含义或预期输出，方便对照验证。

## 镜像基本操作

```bash
docker search xxx      # 查找镜像
docker pull xxx        # 获取镜像
docker images          # 查看镜像列表
docker rmi xxx         # 删除镜像
docker commit          # 更新镜像（把容器改动保存为新镜像）
docker image prune     # 清理悬空镜像（无标签、无容器使用的镜像）
docker system df       # 查看镜像/容器/卷占用的磁盘空间
```

## 容器基本操作

```bash
docker ps              # 查看运行中的容器（加 -a 查看所有容器）
docker create xxx      # 创建容器（不启动）
docker start xxx       # 启动容器
docker stop xxx        # 停止容器
docker rm xxx          # 删除容器（运行中的要加 -f）
docker run xxx         # 运行容器（创建并启动，最常用）
docker logs xxx        # 查看日志（加 -f 实时滚动）
docker inspect xxx     # 查看容器/镜像的详细信息（JSON）
```

## 运行容器

```bash
docker run --name nginx-001 -p 80:80 -d nginx
# -p 系统端口:容器端口     端口映射(容器内部端口映射外部)
# -v 系统目录:容器目录     目录映射
# -e 设置任意环境变量
# -d 容器后台运行
# -i 交互
# -t 终端
# -w 指定工作目录
# --name nginx-001        自定义容器名称
# --network xxx-net       加入(指定)网络组
# --hostname              修改主机名hostname
# --restart=always        总是开机启动
# --dns
# --rm 自动清理容器内部的文件系统（容器退出后自动删除）

# 常见结果示意：
docker ps
# CONTAINER ID   IMAGE     COMMAND   ...   STATUS   PORTS         NAMES
# abc123def456   nginx     "nginx"   ...   Up 5s    0.0.0.0:80->80/tcp  nginx-001
```

## 进入容器的终端

```bash
docker exec -i -t <容器名> /bin/bash
# -i                       交互
# -t                       终端
# /bin/bash                运行容器里的程序

docker run -itd --name redis -p 6379:6379 redis
docker exec -it redis /bin/bash
```

## 容器转为镜像

```bash
docker commit 容器ID 镜像名称:版本号
# -m 提交的描述信息
# -a 指定镜像作者

docker commit e218edb10161 laixhe/nginx:v1 -m="update info" -a="laixhe"
```

## 网络

```bash
docker network ls
# 新建网络 [-d 指定类型，有 bridge、overlay]
docker network create xxx
# 查看网络的详细信息
docker network inspect xxx
```

## 镜像压缩与还原

导出镜像为压缩包用 `save`，导入压缩包用 `load`，注意不要混淆。

```bash
# 导出：把镜像保存为 tar 压缩文件
docker save -o 压缩文件名称 镜像名称:版本号

# 导入：从 tar 压缩文件还原镜像
docker load -i 压缩文件名称
```

## 查看日志

```bash
# 查看此容器 30分钟 之内的日志情况
docker logs --since 30m <容器名>

# 将日志不断输出到终端（等同于 -f / --follow）
docker logs <容器名> --follow
```
