### 搭建 Webpack5 与 Vue3

#### 初始化项目
> 在根目录会生成 ```package.json``` 文件

```
# 创建文件夹
mkdir webpack-vue && cd webpack-vue
# 初始化项目
npm init -y
```

#### 配置 Webpack 环境
> webpack-cli 是执行 webpack 的工具 webpack 4.x 版本以后，剥离出了 webpack-cli 所以这里我们需要单独下载它

```
npm install webpack webpack-cli -D
```

> 注意:
> > -D 等价于 --save-dev 开发环境时所需依赖
> >
> > -S 等价于 --save 生产环境时所需依赖

在根目录创建 ```webpack.config.js``` 文件用于编写 webpack 配置

```
// webpack.config.js
const path = require('path');
  
module.exports = {
    mode: 'development',                             // 环境模式
    entry: path.resolve(__dirname, './src/main.js'), // 打包入口
    output: {
        path: path.resolve(__dirname, 'dist'),       // 打包出口
        filename: 'js/[name].js'                     // 打包完的静态资源文件名
    }
}
```

修改 ```package.json``` 的 ```scripts``` 属性

```
{
  "name": "webpack-vue",
  "version": "1.0.0",
  "description": "",
  "main": "index.js",
  "scripts": {
    "beta": "webpack --config ./webpack.config.js"
  },
  "keywords": [],
  "author": "",
  "license": "ISC",
  "dependencies": {},
  "devDependencies": {
    "webpack": "^5.19.0",
    "webpack-cli": "^4.4.0"
  }
}

```

在根目录添加 src 文件夹，并且在 src 文件夹内添加 ```main.js``` 文件

```
// src/main.js
console.log('test webpack')
```

用命令行测试下(打包)

```
npm run beta


# webpack --config ./webpack.config.js
# asset js/main.js 798 bytes [emitted] (name: main)
# ./src/main.js 43 bytes [built] [code generated]  
# webpack 5.18.0 compiled successfully in 83 ms 
```

打包成功后，会在项目的根目录自动创建一个 dist 文件夹，里面的 main.js 文件就是我们打包后的文件


#### 处理 HTML 模板

```
npm install html-webpack-plugin -D
```

在 ```webpack.config.js``` 下引入并使用
```
const path = require('path')
const HtmlWebpackPlugin = require('html-webpack-plugin')

module.exports = {
  mode: 'development',
  entry: path.resolve(__dirname, './src/main.js'),
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'js/[name].js'
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, './index.html'), // 我们要使用的 html 模板地址
      filename: 'index.html',   // 打包后输出的文件名
      title: '处理 HTML 模板' // index.html 模板内，通过 <%= htmlWebpackPlugin.options.title %> 拿到的变量
    })
  ]
}
```

在根目录手工创建 ```index.html``` 文件

```
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title><%= htmlWebpackPlugin.options.title %></title>
</head>
<body>
  <div id="root"></div>
</body>
</html>
```

修改 src/main.js

```
const root = document.getElementById('root')
root.textContent = '处理 HTML 模板...'
```

#### 开发服务器
> 注意 webpack-cli 升级到 4.x 的时候，就不能用 webpack-dev-server 跑脚本了，而是改为 webpack serve 去跑

```
npm install webpack-dev-server -D
```

修改 ```webpack.config.js``` 配置的 ```devServer``` 属性

```
const path = require('path')
const HtmlWebpackPlugin = require('html-webpack-plugin')

module.exports = {
  mode: 'development',                             // 环境模式
  entry: path.resolve(__dirname, './src/main.js'), // 打包入口
  output: {
    path: path.resolve(__dirname, 'dist'),         // 打包出口
    filename: 'js/[name].js'                       // 打包完的静态资源文件名
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, './index.html'), // 我们要使用的 html 模板地址
      filename: 'index.html',   // 打包后输出的文件名
      title: '处理 HTML 模板' // index.html 模板内，通过 <%= htmlWebpackPlugin.options.title %> 拿到的变量
    })
  ],
  devServer: {
    contentBase: path.resolve(__dirname, './dist'),
    compress: true,
    port: 8080
  }
}
```

修改 ```package.json``` 的 ```scripts``` 属性

```
{
  "name": "webpack-vue",
  "version": "1.0.0",
  "description": "",
  "main": "index.js",
  "scripts": {
    "beta": "webpack --config ./webpack.config.js",
    "dev": "webpack serve --progress --config ./webpack.config.js"
  },
  "keywords": [],
  "author": "",
  "license": "ISC",
  "dependencies": {},
  "devDependencies": {
    "html-webpack-plugin": "^4.5.1",
    "webpack": "^5.19.0",
    "webpack-cli": "^4.4.0",
    "webpack-dev-server": "^3.11.2"
  }
}
```

用命令行测试下(http)

```
npm run dev
```

# 清除打包文件
> 每次打包生成的文件都会 dist 目录保留，使用插件每次打包前先清除以前的打包文件

```
npm install clean-webpack-plugin -D
```

修改 ```webpack.config.js``` 配置的 ```plugins``` 属性

```
const path = require('path')
const HtmlWebpackPlugin = require('html-webpack-plugin')
const { CleanWebpackPlugin } = require('clean-webpack-plugin')

module.exports = {
  mode: 'development',                             // 环境模式
  entry: path.resolve(__dirname, './src/main.js'), // 打包入口
  output: {
    path: path.resolve(__dirname, 'dist'),         // 打包出口
    filename: 'js/[name].js'                       // 打包完的静态资源文件名
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, './index.html'), // 我们要使用的 html 模板地址
      filename: 'index.html',   // 打包后输出的文件名
      title: '处理 HTML 模板' // index.html 模板内，通过 <%= htmlWebpackPlugin.options.title %> 拿到的变量
    }),
    new CleanWebpackPlugin()
  ],
  devServer: {
    contentBase: path.resolve(__dirname, './dist'),
    compress: true,
    port: 8080
  }
}
```

#### 集成 TypeScript

```
npm install typescript ts-loader -D
```

初始化 tsconfig.json 文件

```
tsc --init
```

修改 ```webpack.config.js``` 文件的 ```module.rules``` 属性

```
const path = require('path')
const HtmlWebpackPlugin = require('html-webpack-plugin')
const { CleanWebpackPlugin } = require('clean-webpack-plugin')

module.exports = {
  mode: 'development',                             // 环境模式
  entry: path.resolve(__dirname, './src/main.js'), // 打包入口
  output: {
    path: path.resolve(__dirname, 'dist'),         // 打包出口
    filename: 'js/[name].js'                       // 打包完的静态资源文件名
  },
  module: {
    rules: [
        {
            test: /\.ts$/,
            use: [
                'ts-loader'
            ]
        },
    ]
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, './index.html'), // 我们要使用的 html 模板地址
      filename: 'index.html',   // 打包后输出的文件名
      title: '处理 HTML 模板' // index.html 模板内，通过 <%= htmlWebpackPlugin.options.title %> 拿到的变量
    }),
    new CleanWebpackPlugin()
  ],
  devServer: {
    contentBase: path.resolve(__dirname, './dist'),
    compress: true,
    port: 8080
  }
}
```

##### 引入 Vue 3.x
> 注意：要使用 vue@next 才能成功引入 Vue 3.x，否则就会引入 2.x 的最高版本

```
npm install vue@next -S
npm install vue-loader@next @vue/compiler-sfc
```

修改 ```main.js``` 文件

```
import { createApp } from 'vue' // Vue 3.x 引入 vue 的形式
import App from './App.vue'     // 引入 APP 页面组建

const app = createApp(App)     // 通过 createApp 初始化 app
app.mount('#root')             // 将页面挂载到 root 节点
```

新增 ```src/App.vue``` 文件

```
<template>
    <div>
        <div>解析Vue文件了哟~</div>
    </div>
</template>
<script>
export default {
  
}
</script>
```

修改 ```webpack.config.js``` 文件的 ```module.rules``` 属性

```
const path = require('path')
const HtmlWebpackPlugin = require('html-webpack-plugin')
const { CleanWebpackPlugin } = require('clean-webpack-plugin')
// 最新的 vue-loader 中，VueLoaderPlugin 插件的位置有所改变
const { VueLoaderPlugin } = require('vue-loader/dist/index')

module.exports = {
  mode: 'development',                             // 环境模式
  entry: path.resolve(__dirname, './src/main.js'), // 打包入口
  output: {
    path: path.resolve(__dirname, 'dist'),         // 打包出口
    filename: 'js/[name].js'                       // 打包完的静态资源文件名
  },
  module: {
    rules: [
        {
          test: /\.ts$/,
          use: [
            'ts-loader'
          ]
        },
        {
          test: /\.vue$/,
          use: [
            'vue-loader'
          ]
        }
    ]
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, './index.html'), // 我们要使用的 html 模板地址
      filename: 'index.html',   // 打包后输出的文件名
      title: '处理 HTML 模板' // index.html 模板内，通过 <%= htmlWebpackPlugin.options.title %> 拿到的变量
    }),
    new CleanWebpackPlugin(),
    new VueLoaderPlugin()
  ],
  devServer: {
    contentBase: path.resolve(__dirname, './dist'),
    compress: true,
    port: 8080
  }
}
```


#### 处理 CSS 样式

```
npm install style-loader css-loader less less-loader -D
```

修改 ```webpack.config.js``` 文件的 ```module.rules``` 属性

```
const path = require('path')
const HtmlWebpackPlugin = require('html-webpack-plugin')
const { CleanWebpackPlugin } = require('clean-webpack-plugin')
// 最新的 vue-loader 中，VueLoaderPlugin 插件的位置有所改变
const { VueLoaderPlugin } = require('vue-loader/dist/index')

module.exports = {
  mode: 'development',                             // 环境模式
  entry: path.resolve(__dirname, './src/main.js'), // 打包入口
  output: {
    path: path.resolve(__dirname, 'dist'),         // 打包出口
    filename: 'js/[name].js'                       // 打包完的静态资源文件名
  },
  module: {
    rules: [
        {
          test: /\.ts$/,
          use: [
            'ts-loader'
          ]
        },
        {
          test: /\.vue$/,
          use: [
            'vue-loader'
          ]
        },
        {
          test: /\.css$/,
          use: [
              'style-loader',
              'css-loader'
          ]
      },
      {
          test: /\.less$/,
          use: [
              'style-loader',
              'css-loader',
              'less-loader'
          ]
      }
    ]
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, './index.html'), // 我们要使用的 html 模板地址
      filename: 'index.html',   // 打包后输出的文件名
      title: '处理 HTML 模板' // index.html 模板内，通过 <%= htmlWebpackPlugin.options.title %> 拿到的变量
    }),
    new CleanWebpackPlugin(),
    new VueLoaderPlugin()
  ],
  devServer: {
    contentBase: path.resolve(__dirname, './dist'),
    compress: true,
    port: 8080
  }
}
```

修改 ```src/App.vue``` 文件

```
<template>
    <div>
        <div>解析Vue文件了哟~</div>
    </div>
</template>
<script>
export default {
  
}
</script>
<style>
  div {
    color: yellowgreen;
  }
</style>
```

#### 将 ES6+ 转 ES5
> 由于有些浏览器无法解析 ES6+ 等高级语法，故需要将其转化为浏览器能够解析的低级语法

```
npm install @babel/core babel-loader @babel/preset-env -D
```

在根目录手工创建 ```babel.config.js``` 文件

```
module.exports = {
    presets: [
      ["@babel/preset-env", {
        "targets": {
          "browsers": ["last 2 chrome version"] // 最近 2 个版本的浏览器
        }
      }]
    ]
}
```

修改 ```webpack.config.js``` 配置

```
const path = require('path')
const HtmlWebpackPlugin = require('html-webpack-plugin')
const { CleanWebpackPlugin } = require('clean-webpack-plugin')
// 最新的 vue-loader 中，VueLoaderPlugin 插件的位置有所改变
const { VueLoaderPlugin } = require('vue-loader/dist/index')

module.exports = {
  mode: 'development',                             // 环境模式
  entry: path.resolve(__dirname, './src/main.js'), // 打包入口
  output: {
    path: path.resolve(__dirname, 'dist'),         // 打包出口
    filename: 'js/[name].js'                       // 打包完的静态资源文件名
  },
  module: {
    rules: [
        {
          test: /\.ts$/,
          use: [
            'ts-loader'
          ]
        },
        {
          test: /\.vue$/,
          use: [
            'vue-loader'
          ]
        },
        {
          test: /\.css$/,
          use: [
              'style-loader',
              'css-loader'
          ]
      },
      {
          test: /\.less$/,
          use: [
              'style-loader',
              'css-loader',
              'less-loader'
          ]
      },
      {
        test: /\.js$/,
        exclude: /node_modules/, // 不编译node_modules下的文件
        use: {
            loader: 'babel-loader',
            options: {
                presets: ['@babel/preset-env']
            }
        }
      }
    ]
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, './index.html'), // 我们要使用的 html 模板地址
      filename: 'index.html',   // 打包后输出的文件名
      title: '处理 HTML 模板' // index.html 模板内，通过 <%= htmlWebpackPlugin.options.title %> 拿到的变量
    }),
    new CleanWebpackPlugin(),
    new VueLoaderPlugin()
  ],
  devServer: {
    contentBase: path.resolve(__dirname, './dist'),
    compress: true,
    port: 8080
  }
}
```
