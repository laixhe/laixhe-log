"""pylog 各主题模块的冒烟测试：验证 run() 能正常执行并输出对应主题标题。"""

from pylog import (
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


def test_basic_types(capsys) -> None:
    basic_types.run()
    assert "基本数据类型" in capsys.readouterr().out


def test_control_flow(capsys) -> None:
    control_flow.run()
    assert "控制流" in capsys.readouterr().out


def test_functions(capsys) -> None:
    functions.run()
    assert "函数" in capsys.readouterr().out


def test_collections(capsys) -> None:
    collections.run()
    assert "集合类型" in capsys.readouterr().out


def test_strings(capsys) -> None:
    strings.run()
    assert "字符串" in capsys.readouterr().out


def test_classes(capsys) -> None:
    classes.run()
    assert "类与对象" in capsys.readouterr().out


def test_exceptions(capsys) -> None:
    exceptions.run()
    assert "异常处理" in capsys.readouterr().out


def test_date_time(capsys) -> None:
    date_time.run()
    assert "时间与日期" in capsys.readouterr().out


def test_regex(capsys) -> None:
    regex.run()
    assert "正则" in capsys.readouterr().out


def test_number(capsys) -> None:
    number.run()
    assert "数值类型进阶" in capsys.readouterr().out


def test_iterators(capsys) -> None:
    iterators.run()
    assert "迭代器与推导式" in capsys.readouterr().out


def test_containers(capsys) -> None:
    containers.run()
    assert "容器进阶" in capsys.readouterr().out


def test_concurrency(capsys) -> None:
    concurrency.run()
    assert "并发同步" in capsys.readouterr().out


def test_json_demo(capsys) -> None:
    json_demo.run()
    assert "JSON 序列化" in capsys.readouterr().out


def test_http_demo(capsys) -> None:
    http_demo.run()
    assert "HTTP 服务与客户端" in capsys.readouterr().out
