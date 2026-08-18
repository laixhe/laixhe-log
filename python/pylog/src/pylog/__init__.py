"""pylog —— Python 学习示例项目：按主题拆分模块，逐个演示基础语法。"""

from . import (
    basic_types,
    classes,
    collections,
    concurrency,
    containers,
    control_flow,
    date_time,
    exceptions,
    functions,
    http_demo,
    iterators,
    json_demo,
    number,
    regex,
    strings,
)


def main() -> None:
    """入口：按学习顺序串联所有主题。"""
    basic_types.run()
    control_flow.run()
    functions.run()
    collections.run()
    strings.run()
    classes.run()
    exceptions.run()
    date_time.run()
    regex.run()
    # ===== 进阶主题（参考 Go golog / Rust rustlog）=====
    number.run()
    iterators.run()
    containers.run()
    concurrency.run()
    json_demo.run()
    http_demo.run()
