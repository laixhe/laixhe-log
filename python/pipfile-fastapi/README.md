
#### 安装 python
> python 3.8.7

#### 安装 pipenv
```
pip install pipenv
```

#### 创建虚拟环境
```
pipenv install
```

#### 安装 fastapi
> 技术背景：python3.6+、Starlette、Pydantic

```
pipenv install fastapi
```

#### 安装 uvicorn
> 是一个闪电般快速的ASGI服务器

```
pipenv install uvicorn
```

#### 运行 fastapi
> main：表示项目根目录的 main.py 文件
> 
> app：表示 FastAPI 实例即是 main.py 内的 app = FastAPI()
> 
> --reload：表示 debug 模式，可以在更改代码后服务器重新启动
> 
> --host：地址，默认：127.0.0.1
> 
> --port：端口，默认：8000

```
uvicorn main:app --reload
```
