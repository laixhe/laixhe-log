module.exports = {
    presets: [
      ["@babel/preset-env", {
        "targets": {
          "browsers": ["last 2 chrome version"] // 最近 2 个版本的浏览器
        }
      }]
    ]
}