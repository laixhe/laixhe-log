
##### 创建 mysql 用户， 组及目录
```
groupadd -r mysql
useradd -r -g mysql mysql
```

#####  作相关的配置如下：
```
vim /usr/local/mysql/my.cnf
# ======== 请查看 Mysql配置文件.md =========

rm /etc/my.cnf
cp  /usr/local/mysql/my.cnf /etc/my.cnf
```

#####  设置权限并初始化MySQL系统授权表
```
cd /usr/local/mysql
mkdir data
mkdir logs
chown -R root:root .      // 改所有者，注意是root
chown -R mysql .          // 改所有者，注意是 mysql .
chgrp -R mysql .          // 改所属的用户组，注意是 mysql .
```

#####  以 root 初始化操作时要加 –user=mysql 参数，生成一个随机密码（注意保存登录时用）
```
/usr/local/mysql/bin/mysqld --initialize --user=mysql --basedir=/usr/local/mysql --datadir=/usr/local/mysql/data
```

#####  再设置目录权限
```
cd /usr/local/mysql
chown -R root .              // 改所有者，注意是 root
chown -R mysql data          // 更改 data 目录所有者为 mysql
chown -R mysql logs          // 更改 logs 目录所有者为 mysql
```

##### mysql 启动
```
/usr/local/mysql/support-files/mysql.server start启动 stop停止 restart重启

// 配置mysql启动文件(可选) 创建软连接
ln -s /usr/local/mysql/support-files/mysql.server /usr/local/mysql/bin/mysql.server
```


##### 添加 mysql 的环境变量
```
vim /etc/profile
在文件末尾加上
PATH=$PATH:/usr/local/mysql/bin
使其修改生效
source /etc/profile
```

#### 修改 mysql 初始密码
##### 用户配置
```
# ======== 请查看 Mysql用户.md =========
```
