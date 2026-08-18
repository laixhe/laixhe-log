# Bun 基本使用

[Bun](https://bun.sh) 是一个高性能的 JavaScript / TypeScript 运行时，集成了运行时、打包器、测试运行器和包管理器，可替代 Node.js + npm，启动更快。

## 常用命令

```bash
bun upgrade                                       # 升级 Bun 本身
bun update                                        # 更新项目依赖

bun install                                       # 安装依赖（替代 npm install）
bun run <script>                                  # 运行 package.json 中的脚本（替代 npm run）

bun build ./index.ts --outdir ./dist              # 打包
bun build --compile --target=browser ./index.html # 编译打包到浏览器目标
```
