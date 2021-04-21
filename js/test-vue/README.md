#### 安装 vite
```
npm install -g vite
```

#### 创建项目
```
npm init @vitejs/app
```

#### 安装依赖
```
npm install
```

#### 安装在 Node 下的 TypeScript 的类型声明
```
npm install --save-dev @types/node
```

#### 安装路由工具
```
npm install vue-router@next
```

#### 安装状态管理
```
npm install vuex@next
```

#### 安装UI框架
```
npm install element-plus
```

#### 安装Http客户端
```
npm install axios
```

#### 安装CSS预编译器 sass less stylus 
```
npm install --save-dev sass
```

#### 运行
```
npm run dev
```

#### 项目打包
```
npm run build
```

#### VSCode 使用 EditorConfig 需要去插件市场下载插件 EditorConfig for VS Code
> 编码风格

#### VSCode 编辑器使用 Prettier 配置需要下载插件 Prettier - Code formatter
> 代码格式化工具

```
npm install --save-dev prettier
```

---

#### 规范目录结构
```
└── src/
    ├── router/                    // 路由配置目录
    ├── store/                     // 状态管理目录
    ├── utils/                     // 工具函数目录
    ├── views/                     // 页面组件目录
    ├── App.vue
    ├── main.ts
    ├── shims-vue.d.ts
├── index.html
├── tsconfig.json                  // TypeScript 配置文件
├── vite.config.ts                 // Vite 配置文件
└── package.json
```

---

#### 安装 vue 的手脚架
```
npm install -g @vue/cli
```

#### 确保 vue cli 版本在 4.5.0 以上
```
vue --version
```

#### 创建项目
```
vue create test-vue
```

#### 运行
```
npm run serve
```

#### 打包生产
```
npm run build
```