from fastapi import APIRouter

# create router
router = APIRouter(
    prefix='/user'
)


@router.get('/add')
def user_add():
    return {"message": "add use"}


@router.get('/get')
def user_get():
    return {"message": "get user"}
