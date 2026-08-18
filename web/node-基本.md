# Node.js 与 npm 基本使用

## 环境变量配置

安装 Node.js 后，把可执行目录加入 PATH：

```bash
export PATH=$PATH:/usr/local/nodejs/bin
```

## 淘宝 npm 镜像

```bash
npm config set registry https://registry.npmmirror.com
```

## npm 基本使用

```bash
# 初始化，创建 package.json 文件
npm init

# 安装
npm install [package name]
    # 全局安装
    npm install -g [package name]
    # 开发依赖（--save-dev 的缩写，写入 devDependencies）
    npm install -D [package name]
    # 运行依赖（--save 的缩写，写入 dependencies）
    npm install -S [package name]

# 运行 package.json 中定义的脚本
npm run xxx

# 卸载
npm uninstall [package name]
    # 卸载全局模块
    npm uninstall -g [package name]

# 升级
npm update [package name]
    # 升级全局安装的模块
    npm update -g [package name]

# 列出已安装模块
npm list
npm list -g

# 配置
npm config list               # 查看配置
npm config set prefix xxx     # 设置 npm 安装目录
npm config set cache xxx      # 设置 npm 缓存目录

# 清理 npm 缓存
npm cache clean --force
```

## TypeScript 基本使用

```bash
npm install -g typescript

tsc -v       # 查看版本
tsc --init   # 创建 tsconfig.json 文件
tsc -w       # 开启监视模式，文件变化自动编译
```

## ts-node 基本使用

```bash
npm install -g ts-node

# --files 自动识别 tsconfig.json 文件
ts-node index.ts --files
```
