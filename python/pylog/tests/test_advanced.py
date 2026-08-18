"""进阶主题的核心断言测试（对应 Go *_test.go / Rust #[cfg(test)] 练习题）。"""

import json
from functools import reduce
from http.client import HTTPConnection

from pylog import containers, http_demo, iterators


# ---------- 数值：格式化与转换 ----------
def test_number_hex_leading_zeros() -> None:
    assert f"{666:08X}" == "0000029A"


def test_number_python_int_no_overflow() -> None:
    # Python int 任意精度，255 + 1 不回绕
    assert 255 + 1 == 256
    assert 2**100 > 0  # 任意大整数


def test_number_parse_errors() -> None:
    try:
        int("not_a_number")
        raise AssertionError("应抛 ValueError")
    except ValueError:
        pass


def test_number_truncate_and_round() -> None:
    assert int(3.99) == 3  # 向零截断
    assert round(3.99) == 4  # 四舍五入
    assert int("29A", 16) == 666  # 进制解析


# ---------- 迭代器：练习题 ----------
def test_iterators_even_squares_sum() -> None:
    # 1..5 中偶数平方和：2^2 + 4^2 = 20
    total = sum(x * x for x in range(1, 6) if x % 2 == 0)
    assert total == 20


def test_iterators_practice_avg_salary() -> None:
    query = [e["salary"] for e in iterators.STAFF if e["dept"] == "R&D" and e["age"] >= 30]
    avg = sum(query) / len(query)
    assert avg == 52500.0  # (45000+60000)/2


def test_iterators_zip_chain() -> None:
    assert list(zip(["Alice", "Bob"], [95, 87])) == [("Alice", 95), ("Bob", 87)]
    assert reduce(lambda a, b: a + b, range(1, 11), 0) == 55  # fold 累加 1..10


# ---------- 容器：LRU / 环 / 堆 / 去重 ----------
def test_containers_lru_eviction() -> None:
    cache = containers.LRUCache(3)
    cache.put("key1", "value1")
    cache.put("key2", "value2")
    cache.put("key3", "value3")
    cache.get("key2")  # 访问 key2，使其变为最近使用

    cache.put("key4", "value4")  # 容量 3，应淘汰 key1
    assert [k for k, _ in cache.entries()] == ["key3", "key2", "key4"]
    assert cache.get("key1") == ""  # key1 已被淘汰
    assert cache.get("key2") == "value2"


def test_containers_ring_buffer() -> None:
    ring = containers.RingBuffer(3)
    ring.add(1)
    ring.add(2)
    ring.add(3)
    ring.add(4)  # 覆盖最旧的 1
    assert ring.snapshot() == [2, 3, 4]


def test_containers_heap_order() -> None:
    import heapq

    h = [2, 1, 5, 3, 4]
    heapq.heapify(h)
    assert [heapq.heappop(h) for _ in range(len(h))] == [1, 2, 3, 4, 5]


def test_containers_unique_and_grouping() -> None:
    nums = [3, 1, 2, 1, 3, 2, 4, 5, 4]
    assert list(dict.fromkeys(nums)) == [3, 1, 2, 4, 5]  # 去重保持顺序

    by_dept = {}
    for e in [{"dept": "R&D"}, {"dept": "R&D"}, {"dept": "HR"}]:
        by_dept.setdefault(e["dept"], 0)
        by_dept[e["dept"]] += 1
    assert by_dept == {"R&D": 2, "HR": 1}


# ---------- JSON ----------
def test_json_omitempty() -> None:
    from pylog import json_demo

    filtered = json_demo._omitempty({"time1": None, "array1": [], "map1": {}, "name": "ok"})
    assert json.dumps(filtered, ensure_ascii=False) == '{"name": "ok"}'


def test_json_roundtrip() -> None:
    data = {"name": "laixhe", "age": 18}
    assert json.loads(json.dumps(data)) == data


# ---------- HTTP：启动服务器后请求断言 ----------
def test_http_get_root() -> None:
    server = http_demo.start_server()
    base = f"http://127.0.0.1:{server.server_address[1]}"
    try:
        assert http_demo.client_get(base + "/") == "Hello Python HTTP"
    finally:
        server.shutdown()
        server.server_close()


def test_http_get_query_and_post() -> None:
    server = http_demo.start_server()
    base = f"http://127.0.0.1:{server.server_address[1]}"
    try:
        assert http_demo.client_get(base + "/get?name=laixhe") == "http get name=laixhe"
        form = {"name": "laixhe"}
        assert http_demo.client_post_form(base + "/post", form) == "http post name=laixhe"
        body = '{"name":"laixhe"}'
        assert http_demo.client_post_json(base + "/post", body) == "http post name=laixhe"
    finally:
        server.shutdown()
        server.server_close()


def test_http_raw_client() -> None:
    # 对应 Go 的 http.Get / net/http 底层请求
    server = http_demo.start_server()
    port = server.server_address[1]
    conn = HTTPConnection("127.0.0.1", port, timeout=10)
    try:
        conn.request("GET", "/")
        resp = conn.getresponse()
        assert resp.status == 200
        assert resp.read().decode("utf-8") == "Hello Python HTTP"
    finally:
        conn.close()
        server.shutdown()
        server.server_close()
