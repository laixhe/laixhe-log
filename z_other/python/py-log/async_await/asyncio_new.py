import time
import asyncio


def asyncio_run():
    print('asyncio_run start: %s' % time.time())

    # 创建事件循环
    event_loop = asyncio.get_event_loop()

    tasks = []
    for i in range(5):
        tasks.append(asyncio_hello(i))

    # 把异步的任务丢给事件循环
    event_loop.run_until_complete(asyncio.wait(tasks))

    print('asyncio_run end: %s' % time.time())

    event_loop.close()
    pass


# 定义异步函数
async def asyncio_hello(i: int):
    # 等待睡眠时间，允许其他任务运行
    await asyncio.sleep(1)
    print('asyncio_hello : %d : %s' % (i, time.time()))
