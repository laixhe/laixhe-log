# Windows WSL2 设置

## 从局域网 (LAN) 访问 WSL

- 在 PowerShell 下做端口转发代理。
- 系统重启后，WSL 内部 IP 会改变，需要重新设置。

## 获取 WSL 内部 IP

在 PowerShell 中执行，获取虚拟机内 Ubuntu 的 IP 地址：

```powershell
wsl -- ifconfig eth0
```

## 设置端口转发

```powershell
netsh interface portproxy add v4tov4 listenport=[win10端口] listenaddress=0.0.0.0 connectport=[虚拟机的端口] connectaddress=[虚拟机的ip]
```

## 删除端口转发

```powershell
netsh interface portproxy delete v4tov4 listenport=8088 listenaddress=0.0.0.0
```

## 查看端口转发

```powershell
netsh interface portproxy show all
```

## 从 Windows 访问 WSL 应用（端口转发软件）

```text
https://github.com/HobaiRiku/wsl2-auto-portproxy
```

## 从 WSL 访问 Windows 应用（端口）

```bash
cat /etc/resolv.conf
```
