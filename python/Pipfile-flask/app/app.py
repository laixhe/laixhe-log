from flask import Flask


# 注册蓝图
def register_blueprints(app: Flask):
    from app.api.v1.user import user
    from app.api.v1.book import book
    app.register_blueprint(user)
    app.register_blueprint(book)


# 创建 flask
def create_app():
    app = Flask(__name__)
    app.config.from_object('app.config.secure')
    app.config.from_object('app.config.setting')

    register_blueprints(app)

    return app
