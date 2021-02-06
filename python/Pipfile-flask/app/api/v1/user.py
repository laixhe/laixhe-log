from flask import Blueprint

# 定义蓝图
user = Blueprint('user', __name__)


@user.route('/v1/user/get')
def get_user():
    return 'get user'
