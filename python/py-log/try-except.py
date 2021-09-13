# 异常捕获机制
"""
语法格式
try:
  可能出现的代码块
except:
  出错之后执行的代码块
else:
  没有出错的代码块
finally:
  不管有没有出错都执行的代码块

raise Exception("主动抛出异常")
"""


if __name__ == '__main__':
    try:
        li = [1, 2, 3]
        print(li[5])
    except Exception as e:
        # Exception 可以捕获所有异常
        print(e)

    pass
