
#### 镜像基本操作

```
docker search  xxx   查找镜像
docker pull    xxx   获取镜像
docker images        查看镜像
docker rmi     xxx   删除镜像
docker commit        更新镜像
```

#### 容器基本操作

```
docker ps            查看容器  [-a 所有容器]
docker create  xxx   创建容器
docker start   xxx   启动容器
docker stop    xxx   停止容器
docker rm      xxx   删除容器
docker run     xxx   运行容器
docker logs    xxx   查看日志
```

#### 运行容器

```
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
# –restart=always         总是开机启动
# --dns
# --rm 自动清理容器内部的文件系统
```

#### 进入容器的终端

```
docker exec -i -t <容器名> /bin/bash
# -i                       交互
# -t                       终端
# /bin/bash                运行容器里的程序

docker run -itd --name redis -p 6379:6379 redis
docker exec -it redis /bin/bash
```

#### 容器转为镜像

- docker commit 容器ID 镜像名称:版本号
- - -m 提交的描述信息
- - -a 指定镜像作者

```
docker commit e218edb10161 laixhe/nginx:v1 -m="update info" -a="laixhe"
```

#### 网络

```
docker network ls
# 新建网络 [-d 指定类型，有 bridge、overlay]
docker network create xxx
# 查看网络的详细信息
docker network inspect xxx
```

#### 镜像压缩与还原

- docker save -o 压缩文件名称 镜像名称:版本号
- docker save -i 压缩文件名称

##### 镜像导入

```
docker load -i <image_file>
```

#### 查看日志

```
# 查看此容器 30分钟 之内的日志情况
docker logs --since 30m <容器名>

# 将日志不断输出到终端
docker logs <容器名> --follow
```
