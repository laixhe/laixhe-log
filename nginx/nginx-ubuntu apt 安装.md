# Nginx Ubuntu 安装（官方仓库）

> 参考：https://nginx.org/en/linux_packages.html#Ubuntu

## 安装依赖

```bash
sudo apt install curl gnupg2 ca-certificates lsb-release ubuntu-keyring
```

## 添加官方签名密钥

```bash
curl https://nginx.org/keys/nginx_signing.key | gpg --dearmor | sudo tee /usr/share/keyrings/nginx-archive-keyring.gpg >/dev/null

# 校验密钥指纹（可选）
gpg --dry-run --quiet --no-keyring --import --import-options import-show /usr/share/keyrings/nginx-archive-keyring.gpg
```

## 添加软件源

```bash
echo "deb [signed-by=/usr/share/keyrings/nginx-archive-keyring.gpg] http://nginx.org/packages/mainline/ubuntu $(lsb_release -cs) nginx" | sudo tee /etc/apt/sources.list.d/nginx.list
```

## 安装并启动

```bash
sudo apt update
sudo apt install nginx
```

配置文件目录为 `/etc/nginx`。

```bash
sudo systemctl start nginx
sudo systemctl enable nginx
```

> 修改配置后重载：`sudo systemctl reload nginx` 或 `sudo nginx -s reload`。
