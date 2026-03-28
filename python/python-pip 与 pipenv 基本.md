##### 安装
```
# 安装最新版本
sudo add-apt-repository ppa:deadsnakes/ppa
sudo apt update
sudo apt install python3.11
sudo apt install python3.11-full  # 一次性安装所有附加项
sudo apt install python3-pip

python3.11 --version
pip --version

# 使用 Python 3.11 作为默认的 Python3
# 创建 python3 的符号链接
sudo update-alternatives --install /usr/bin/python3 python3 /usr/bin/python3.10 1
sudo update-alternatives --install /usr/bin/python3 python3 /usr/bin/python3.11 2
# 切换 python3 默认版本
sudo update-alternatives --config python3

python3 --version
```

##### pip

```

# 配置文件
C:\Users\laiki\AppData\Roaming\pip\pip.ini
~/.pip/pip.conf

[global]
index-url = https://mirrors.aliyun.com/pypi/simple/

[install]
trusted-host=mirrors.aliyun.com


# 阿里云开源软件镜像站
pip config set global.index-url https://mirrors.aliyun.com/pypi/simple/
# 清华大学开源软件镜像站
pip config set global.index-url https://pypi.tuna.tsinghua.edu.cn/simple/

# 临时使用
pip install -i https://mirrors.aliyun.com/pypi/simple/ 包名flask

# 安装
install [package name]
    # 升级包
    --upgrade[-U]

# 卸载
uninstall [package name]

# 配置
config
    list

# 列出所有要更新的包
pip list --outdate

# 升级所有包
pip install pip-review
pip-review --local --interactive

# pip 找不到修复
python -m ensurepip
# 升级 pip
python -m pip install --upgrade pip
```

```
基本用法：pip install pipenv 等效于 python -m pip install pipenv
指定版本：pip install pipenv==1.0.0
最小版本：pip install pipenv>=1.0.0

```

#### poetry

    https://python-poetry.org/docs
    pip install poetry           安装 poetry
    poetry --version

    poetry init                  已有项目
    poetry new xxx               创建项目
    poetry install               创建虚拟环境(确保当前目录有 pyproject.toml 文件)
    poetry add     [moduel]      安装包 [--dev|-D]
    poetry show                  查看依赖 [--tree]  [--outdated查看更新]
    poetry update  [moduel]      更新依赖
    poetry remove  [module]      卸载包
    poetry env                   虚拟环境 [list] [--full-path]
    poery  shell                 进入虚拟机环境
    poetry config                配置 [--list]

    # pyproject.toml 设置 PyPI 镜像源 (默认使用 pip.ini 的设置)
    [[tool.poetry.source]]
    name = "aliyun"
    url = "https://mirrors.aliyun.com/pypi/simple/"

##### pipenv

    pip install pipenv             安装 pipenv
    pipenv --version

    pipenv --where                 列出本地工程路径
    pipenv --venv                  列出虚拟环境路径
    pipenv --py                    列出虚拟环境的 Python 可执行文件
    pipenv install                 创建虚拟环境
    pipenv isntall   [moduel]      安装包 --dev
    pipenv update    [moduel]      更新包
    pipenv uninstall [module]      卸载包
    pipenv uninstall --all         卸载所有包
    pipenv graph                   查看包依赖
    pipenv lock                    生成 lockfile
    pipenv run python [pyfile]     运行 py 文件
    pipenv --rm                    删除虚拟环境

<!---->

    pipenv install -r requirements.txt  通过 requirements.txt 安装包
    pipenv lock -r > requirements.txt   生成 requirements.txt 文件

