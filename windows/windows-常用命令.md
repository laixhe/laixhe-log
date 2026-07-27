电脑怎么彻底退出微软 Windows10/11 账户账号
https://www.zhihu.com/question/8239136687

#### 清空屏幕命令
```
clr
```

#### 端口占用
```
# 查看端口
netstat -ano
# 指定端口查看
netstat -ano|findstr 80
```

#### 结束进程
```
taskkill /f /t /im xxx.exe
```