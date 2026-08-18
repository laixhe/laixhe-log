#### 构建 Golang 打包环境
- COPY 命令通过指定 --from=0 参数，把前一阶段构建的产物拷贝到了当前的镜像中
- 也可以 (修改 FROM golang:alpine AS build ) (修改 --from=build)
```
# FROM golang:1.20-alpine
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
