### 参考官方文档
- https://docs.docker.com/engine/install/ubuntu/

#### 使用内置仓库安装
```
sudo apt install docker.io

# 查看是否开机启动
sudo systemctl list-units --type=service | grep docker
sudo systemctl enable docker.service

# 查看是否启动
ps -ef | grep docker
sudo systemctl start docker.service
sudo systemctl restart docker.service

sudo docker version
```

#### 镜像加速
```
sudo vim /etc/docker/daemon.json

{
  "registry-mirrors": [
    "https://docker.mirrors.ustc.edu.cn",
    "https://hub-mirror.c.163.com",
    "https://registry.docker-cn.com"
  ],
  "exec-opts": ["native.cgroupdriver=systemd"]
}

# native.cgroupdriver=systemd 是给k8s v1.22之后的版本设置

sudo systemctl daemon-reload
```
