### 更新apt源文件
```
# 所在的网址
https://nginx.org/en/linux_packages.html#Ubuntu

#
sudo apt install curl gnupg2 ca-certificates lsb-release ubuntu-keyring
#
sudo curl https://nginx.org/keys/nginx_signing.key | gpg --dearmor | sudo tee /usr/share/keyrings/nginx-archive-keyring.gpg >/dev/null
#
gpg --dry-run --quiet --no-keyring --import --import-options import-show /usr/share/keyrings/nginx-archive-keyring.gpg
#
sudo echo "deb [signed-by=/usr/share/keyrings/nginx-archive-keyring.gpg] http://nginx.org/packages/mainline/ubuntu `lsb_release -cs` nginx" | sudo tee /etc/apt/sources.list.d/nginx.list

# 安装目录
/etc/nginx
#
sudo apt update
sudo apt install nginx
#
sudo systemctl start nginx
sudo systemctl restart nginx
sudo systemctl enable nginx
```
