#### 安装 Node
```
export PATH=$PATH:/usr/local/nodejs/bin
```

#### 淘宝npm镜像
```
npm config set registry https://registry.npmmirror.com
```

#### npm 基本使用
```
# 初始化 创建 package.json 文件
init

# 安装
install [package name]
    # 全局安装
    -g [-global]
    # 为npm install --save-dev的缩写(开发依赖-辅助) devDependencies
    -D
    # 为npm install --save的缩写(运行依赖-发布) dependencies
    -S

# 运行
run xxx

# 卸载
uninstall [package name]
    # 卸载全局模块
    -global

# 升级
update [package name]
    # 升级全局安装的模块
    -global

# 列出安装模块
list
list -global

# 配置
config
    # 
    list
    # npm 安装目录
    set prefix xxx
    # npm 缓存目录
    set cache xxx

# 清理npm缓存
npm cache clean --force
```

#### TypeScript 基本使用
```
npm install -g typescript

# 基本
tsc 
   -v
   --init  # 创建 tsconfig.json 文件
   -w      # 开启监视模式

```

#### ts-node 基本使用
```
npm install -g ts-node

# --files 自动识别 tsconfig.json 文件
ts-node index.ts --files
```