import uvicorn
from fastapi import FastAPI

import router_user

app = FastAPI()


@app.get('/')
async def root():
    return {'message': 'Hello World!'}

app.include_router(router_user.router)

if __name__ == '__main__':
    uvicorn.run(app, host='0.0.0.0', port=5080)
