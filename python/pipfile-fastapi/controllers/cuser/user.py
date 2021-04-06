from typing import List
from fastapi import APIRouter, Path, Query
from pydantic import BaseModel

user_api = APIRouter()


# http://127.0.0.1:8000/user/1234
@user_api.get("/{uid}")
async def get_user(
        uid: int = Path(default=0, ge=1, le=10, description='用户ID')
):
    return {"uid": uid, "name": "get user laixhe"}


# http://127.0.0.1:8000/user?uid=123&name=laixhe
@user_api.get("/")
async def user_get(
        uid: int = 0,
        name: str = Query(default="", min_length=3, max_length=5, description='用户名')):
    return {"id": uid, "name": name}


@user_api.get("/users")
async def get_users(
        uids: List[int] = Query(default=[], description='多个用户ID')
):
    return uids


class PostUser(BaseModel):
    uid: int
    name: str
    age: int = 0


# http://127.0.0.1:8000/user/create
# Accept: application/json
# Content-Type: application/json
# Body: {"uid":1234,"name":"laixhe","age":18}
@user_api.post("/create")
async def post_user(user: PostUser):
    return {"user": user}

