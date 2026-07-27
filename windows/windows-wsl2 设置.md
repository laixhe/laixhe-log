### 从局域网 (LAN) 访问 WSL
- powershell 下做端口转发代理
- 系统重启后，内部 ip 都会改变

### 在 powershell 中执行，获取虚拟机内 ubuntu 的 ip 地址
```
wsl -- ifconfig eth0
```

####  设置端口转发
```
netsh interface portproxy add v4tov4 listenport=[win10端口] listenaddress=0.0.0.0 connectport=[虚拟机的端口] connectaddress=[虚拟机的ip]
```

####   删除端口转发
```
netsh interface portproxy delete v4tov4 listenport=8088 listenaddress=0.0.0.0
```

####   查看端口转发
```
netsh interface portproxy show all
```

### 从 Windows 访问 WSL 应用( wslpp 端口转发软件 )
```
https://github.com/HobaiRiku/wsl2-auto-portproxy
```

### 从 WSL 访问 Windows 应用( 端口 )
```
cat /etc/resolv.conf
```
