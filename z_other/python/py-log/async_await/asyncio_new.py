import time
import asyncio


def asyncio_new_run():
    # 创建事件循环
    # event_loop = asyncio.get_event_loop()
    #
    # tasks = []
    # for i in range(5):
    #     tasks.append(asyncio_hello(i, True))

    # 把异步的任务丢给事件循环
    # event_loop.run_until_complete(asyncio.wait(tasks))
    #
    # print('asyncio_run end: %s' % time.time())
    #
    # event_loop.close()

    asyncio_run()
    pass


# 执行异步函数
def asyncio_run():

    # asyncio.run 是 python 3.7 之后版本添加

    # 执行异步函数
    asyncio.run(asyncio_hello(1, False))
    asyncio.run(asyncio_task())
    asyncio.run(asyncio_future())

    pass


# 执行 Task 异步任务
async def asyncio_task():

    # asyncio.create_task 是 python 3.7 之后版本添加
    #
    # 创建 Task 对象，将当前要执行的任务添加到事件循环中
    #
    # task1 = asyncio.create_task(asyncio_hello(6, False))
    # task2 = asyncio.create_task(asyncio_hello(7, False))
    # # 执行 Task 异步任务
    # await task1
    # await task2
    #
    # Task 任务数组
    task_list = [
        asyncio.create_task(asyncio_hello(6, False)),
        asyncio.create_task(asyncio_hello(7, False))
    ]
    # 执行 Task 异步任务数组，如果有返回值，就 任务数组 顺序的元组
    await asyncio.wait(task_list)

    pass


# 将一个普通的函数 转换为 异步
async def asyncio_future():
    # 创建事件循环
    event_loop = asyncio.get_event_loop()

    # 转换为 异步
    hello_future = event_loop.run_in_executor(None, hello_func, 111)

    hello = await hello_future

    print("asyncio_future hello", hello)

    pass


# 定义异步函数
async def asyncio_hello(i: int, is_sleep: bool):
    if is_sleep:
        print('asyncio_hello 1 : %d : %s' % (i, time.time()))
        # 等待睡眠时间，允许其他任务运行
        await asyncio.sleep(1)  # 睡眠 1 秒

    print('asyncio_hello 2 : %d : %s' % (i, time.time()))


# 定义普通函数
def hello_func(i: int):
    print("hello_func 一个普通的函数", i)
    return "普通函数返回"
