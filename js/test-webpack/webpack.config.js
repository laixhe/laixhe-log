const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
  mode:'development',           // 开发模式 development production
  entry: './src/index.ts',      // 入口
  devtool: 'inline-source-map',
  module: {
    rules: [
      {
        test: /\.tsx?$/,          // 匹配哪些文件
        use: 'ts-loader',         // 使用哪些 loader 进行处理，多个用数组
        exclude: /node_modules/,  // 忽略需要编译的 ts 目录
      },
    ],
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.js'],
  },
  output: {
    filename: 'main.js',                   // 生成的 js 文件
    path: path.resolve(__dirname, 'dist'), // 打包生成的文件夹
    clean: true,
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: './public/index.html'
    })
  ],
  devServer: {
    contentBase: path.join(__dirname, 'dist'),
    compress: true,
    open: true,
    port: 8080,
  },
};