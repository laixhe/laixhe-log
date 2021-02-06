from flask import Blueprint

# 定义蓝图
book = Blueprint('book', __name__)


@book.route('/v1/book/get')
def get_book():
    return 'get book'
