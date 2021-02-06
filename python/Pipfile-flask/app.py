from app.app import create_app

app = create_app()


# @app.route('/')
# def index():
#     return 'index....'


# http://127.0.0.1:5000/test/laixhe?age=20
# name=laixhe, age=20
# @app.route("/test/<name>")
# def test(name):
#     # age = request.args.get("age", 18)
#     age = 10
#     return "name=%s, age=%s" % (name, age)


if __name__ == '__main__':
    app.run()
