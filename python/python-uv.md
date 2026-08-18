# uv 包管理器

uv 是 Astral 出品的现代 Python 包管理器和项目管理工具，用于替代 pip + venv 的组合，速度更快、体验更统一。

##### 安装
```bash
# 选择 Windows PowerShell 安装
powershell -ExecutionPolicy ByPass -c "irm https://astral.sh/uv/install.ps1 | iex"

# 手动离线安装
https://github.com/astral-sh/uv/releases/latest
# 安装目录并添加到系统环境变量
推荐：$HOME\.local\bin
```

##### 命令

```bash
# 更新最新版本
uv self update
# 查看版本
uv --version
# 查看缓存目录
uv cache dir
# 查看 Python 安装目录
uv python dir
# 查看工具目录
uv tool dir

# 安装 Python 版本
uv python install 3.14
# 升级到最新 Python 版本
uv python upgrade [3.14]
# 查看已安装 Python 版本
uv python list [--only-installed 只列出已安装的版本]
# 在当前目录中使用特定的 Python 版本
uv python pin 3.14

# 安装全局 CLI 工具
uv tool install ruff
ruff --version

# 初始化项目
uv init [--no-package] xxx
# 运行
uv run [main.py]

# 下载依赖（从 uv.lock 文件同步）
uv sync [--frozen 手动同步（根据 uv.lock 精确安装）]
# 生成锁定文件
uv lock

# 安装依赖
uv add xxx [--dev 开发依赖] [numpy==1.26.0 指定版本]
# 卸载依赖
uv remove xxx
# 查看已安装依赖
uv list
# 查看已安装依赖树
uv tree
# 清理缓存
uv cache clean
# 删除未使用的包
uv cache prune

# 创建虚拟环境
uv venv [--python 3.14] [xxx]
# 进入虚拟环境命令
.venv\Scripts\activate
# 退出虚拟环境
deactivate

# 运行脚本
uv run python main.py
```

##### 体验 FastAPI
```bash
# 安装 FastAPI + uvicorn （全平台通用、零报错标准写法）
uv add "fastapi[standard]"
# 启动 FastAPI 项目
uv run fastapi dev --host 0.0.0.0 --port 8000 # 标准开发启动，自动识别 main.py + app 实例，自带热更新
uv run fastapi run --host 0.0.0.0 --port 8000 # 标准生产启动
# 可视化 Swagger UI 文档
http://127.0.0.1:8000/docs
# 简洁版 Redoc 文档
http://127.0.0.1:8000/redoc
```
