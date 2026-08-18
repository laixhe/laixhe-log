# Windows 常用命令

## 清空屏幕命令

```powershell
cls
```

## 端口占用

```batch
# 查看所有端口占用
netstat -ano
# 查看指定端口占用（例如 80 端口）
netstat -ano | findstr 80
```

## 结束进程

```batch
taskkill /f /t /im xxx.exe
```

## 参考

- [电脑怎么彻底退出微软 Windows10/11 账户账号](https://www.zhihu.com/question/8239136687)
