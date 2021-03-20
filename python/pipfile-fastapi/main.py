from fastapi import FastAPI

# 创建实例
app = FastAPI()


# http://127.0.0.1:8000/items/123
@app.get("/items/{item_id}")
async def items(item_id: int):
    return {"item_id": item_id}


# http://127.0.0.1:8000/get?uid=123&name=laixhe
@app.get("/get")
async def get(uid: int = 0, name: str = ""):
    return {"id": uid, "name": name}


# http://127.0.0.1:8000/
@app.get("/")
async def index():
    return {"Hello": "World"}
