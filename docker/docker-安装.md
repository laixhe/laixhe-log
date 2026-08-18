# Docker 安装（Ubuntu）

> 参考官方文档：https://docs.docker.com/engine/install/ubuntu/

## 使用内置仓库安装

```bash
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

> 提示：内置仓库（docker.io）版本通常较旧。如需最新版，建议按官方文档添加 Docker 官方 apt 仓库后安装 `docker-ce`。

## 镜像加速

编辑 `/etc/docker/daemon.json`，加入国内镜像源：

```bash
sudo vim /etc/docker/daemon.json
```

文件内容示例：

```json
{
  "registry-mirrors": [
    "https://<你的加速器地址>"
  ],
  "exec-opts": ["native.cgroupdriver=systemd"]
}
```

> **镜像源说明**：早年流传的公共加速源（`registry.docker-cn.com`、`docker.mirrors.ustc.edu.cn`、`hub-mirror.c.163.com` 等）已陆续停止服务或改为需认证访问（2024 年起尤其明显），不要再照抄。
>
> 推荐做法：注册[阿里云容器镜像服务](https://www.aliyun.com/product/acr)或腾讯云容器镜像服务，在控制台获取个人专属加速地址（形如 `https://xxxx.mirror.aliyuncs.com`），填入上面的 `registry-mirrors`。不配置加速时默认从 Docker Hub 拉取，国内网络可能较慢。

> `native.cgroupdriver=systemd` 是给 k8s v1.22 之后的版本设置，非 k8s 场景可不加。

保存后重载配置并重启 Docker：

```bash
sudo systemctl daemon-reload
sudo systemctl restart docker.service
```
