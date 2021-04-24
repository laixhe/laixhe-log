import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// 如果编辑器提示 path 模块找不到，则可以安装一下 npm install --save-dev @types/node
import { resolve } from 'path'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      '@': resolve(__dirname, 'src'), // 设置 @ 指向 src 目录
      "comps": resolve(__dirname, "src/components"),
    }
  },
  base: './', // 设置打包路径，会生成 dist 目录
  server: {
    port: 5050, // 设置服务启动端口号
    open: true, // 设置服务启动时是否自动打开浏览器
    cors: true  // 允许跨域
  }
})
