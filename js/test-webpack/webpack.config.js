const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = (env, argv) => {
  const config = {
    mode:'development',           // 开发模式 development production
    entry: './src/index.ts',      // 入口文件
    devtool: 'inline-source-map',
    module: {
      rules: [
        {
          test: /\.tsx?$/,          // 匹配哪些 ts 文件
          use: 'ts-loader',         // 使用哪些 loader 进行处理，多个用数组
          exclude: /node_modules/,  // 忽略需要编译的 ts 目录
        },
        {
          // 将 ES6+ 转成 ES5
          test: /\.js$/,
          exclude: /node_modules/,
          use: {
            loader: 'babel-loader',
            options: {
              presets: [
                // @babel/preset-env 只能转译基本新的js语法，可用 core-js 按需加载 
                [
                  '@babel/preset-env', 
                  {
                    // 按需加载 core-js
                    useBuiltIns: "usage",
                    // 指定 core-js 版本
                    corejs: '3.14',
                    // 指定打包后是运行在 浏览器 还是 Nodejs
                    targets: "defaults",
                  },
                ],
              ]
            }
          }
        },
        {
          // 处理 css 文件
          test: /\.css$/i,
          use: [
            "style-loader", // 2.
            "css-loader",   // 1. 先第一个执行
          ]
        },
        {
          // 处理 scss 文件
          test: /\.s[ac]ss$/i,
          use: [
            "style-loader", // 3.
            "css-loader",   // 2.
            "sass-loader",  // 1. 先第一个执行
          ]
        },
        {
          // 处理图片打包 (可处理图片、字体、)
          test: /\.(png|jpe?g|gif)$/i,
          // asset 可以在 asset/resource 和 asset/inline 之间进行选择
          // 如果文件小于 8kb，则使用 asset/inline 否则使用 asset/resource
          type: 'asset',
        },
      ],
    },
    resolve: {
      // 配置模块解析的规则
      extensions: [ // 引入模块时，可以省略哪些后缀名
        '.tsx',
        '.ts',
        '.js',
        '.json'
      ]
    },
    output: {
      filename: 'main.js',                   // 打包生成的 js 文件
      path: path.resolve(__dirname, 'dist'), // 打包生成的文件夹(绝对路径)
      clean: true,
    },
    plugins: [
      new HtmlWebpackPlugin({
        // 指定生成 html 的模板
        template: './public/index.html',
        //filename: 'index.html', // 打包后文件名
      }),

    ],
    devServer: {
      contentBase: path.join(__dirname, 'dist'),
      compress: true,
      open: true,
      port: 5050,
    },
  }

  // 获取环境变量

  // 是否为 生产环境
  if(env.production){
    config.mode = 'production';
  }

  return config;
}