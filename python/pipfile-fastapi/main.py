import uvicorn
from fastapi import FastAPI

from controllers.cuser import user_api


# 创建实例
app = FastAPI()

# 加载 user 路由
app.include_router(user_api, prefix='/user')

if __name__ == '__main__':
    uvicorn.run('main:app', host='0.0.0.0', port=8000, reload=True, debug=True)
