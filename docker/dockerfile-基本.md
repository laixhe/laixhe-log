| 关键字 | 作用 | 备注 |
| --- | --- | --- |
| FROM | 指定父镜像 | 指定 Dockerfile 基于哪个 image 构建 |
| MAINTAINER | 作者信息（已废弃） | 早期用来标明作者，现已废弃，请改用 LABEL |
| LABEL | 标签 | 用来标明Dockerfile的标签，可以使用Label代替Maintainer，最终都是在docker image基本信息中可以查看 |
| RUN | 执行命令 | 执行一段命令，默认是/bin/sh 格式: RUN command 或者 RUN ["command" , "param1","param2"] |
| CMD | 容器启动命令 | 提供启动容器时候的默认命令 和ENTRYPOINT配合使用.格式 CMD command param1 param2 或者 CMD ["command" , "param1","param2"] |
| ENTRYPOINT | 入口 | 一般在制作一些执行就关闭的容器中会使用 |
| COPY | 复制文件 | build的时候复制文件到image中 |
| ADD | 添加文件 | build的时候添加文件到image中 不仅仅局限于当前build上下文 可以来源于远程服务 |
| ENV | 环境变量 | 指定build时候的环境变量 可以在启动的容器的时候 通过-e覆盖 格式ENV name=value |
| ARG | 构建参数 | 构建参数 只在构建的时候使用的参数 如果有ENV 那么ENV的相同名字的值始终覆盖arg的参数 |
| VOLUME | 挂载目录(目录映射) | -v 绑定 格式 VOLUME ["目录"] |
| EXPOSE | 暴露端口 | 定义容器运行的时候监听的端口 启动容器的使用-p来绑定暴露端口 格式: EXPOSE 8080 或者 EXPOSE 8080/udp |
| WORKDIR | 工作目录 | 指定容器内部的工作目录 如果没有创建则自动创建 如果指定/ 使用的是绝对地址 如果不是/开头那么是在上一条workdir的路径的相对路径 |
| USER | 指定执行用户 | 指定 build 或者启动的时候用户，在 RUN / CMD / ENTRYPOINT 执行的时候生效 |
| HEALTHCHECK | 健康检查 | 定义容器健康检查命令，可在 docker ps 看到 HEALTHY/UNHEALTHY 状态，也供编排工具（k8s、compose）探针使用，推荐为常驻服务配置 |
| ONBUILD | 触发器 | 当本镜像被作为基础镜像 FROM 时执行其中命令，用于制作“基础镜像模板”，不影响当前镜像自身的构建 |
| STOPSIGNAL | 发送信号量到宿主机 | 设置容器停止时向主进程发送的系统调用信号（默认 SIGTERM） |
| SHELL | 指定执行脚本的shell | 指定RUN CMD ENTRYPOINT 执行命令的时候 使用的shell |


## 构建 Golang 打包环境
- COPY 命令通过指定 --from=0 参数，把前一阶段构建的产物拷贝到了当前的镜像中
- 也可以 (修改 FROM golang:alpine AS build ) (修改 --from=build)

```dockerfile
FROM golang:alpine

ENV GOPROXY https://proxy.golang.com.cn,direct
ENV CGO_ENABLED 0
WORKDIR /code
COPY main.go .
RUN go build -o hello main.go
 
FROM alpine
ENV TZ Asia/Shanghai
RUN apk add --update curl bash && rm -rf /var/cache/apk/*
WORKDIR /code
COPY --from=0 /code/hello .
EXPOSE 80
CMD ["./hello", "-f", "x.yaml"]
```

## 构建 docker 镜像

- docker build [-f Dockerfile文件路径(默认当前目录)] -t 镜像名称:版本号 镜像存放的绝对路径
