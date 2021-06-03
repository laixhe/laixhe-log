import time
import asyncio


# 定义异步函数
async def hello(i: int):
    await asyncio.sleep(1)
    print('%d----%s' % (i, time.time()))


# 创建事件循环
loop = asyncio.get_event_loop()

if __name__ == '__main__':
    # 创建事件循环
    loop = asyncio.get_event_loop()

    tasks = []
    for i in range(5):
        tasks.append(hello(i))

    # 把异步的任务丢给事件循环
    loop.run_until_complete(asyncio.wait(tasks))
    print("run...")
    loop.close()
    pass
