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