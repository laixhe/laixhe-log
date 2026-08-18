"""主题：JSON 序列化（json 标准库）。

对应 Go golog/json_test.go 与 Rust serde_json。

常用参数与 Go Tag 对应关系：
- ensure_ascii=False → 中文不转义（Go 默认把非 ASCII 转义为 \\uXXXX 形式）
- indent=N           → 美化输出（对应 json.MarshalIndent）
- sort_keys=True     → 按 key 排序输出（对应 maps.Keys 排序）
- skipkeys           → 跳过非字符串 key
- omitempty 语义     → 序列化前手动过滤 None / 空集合（Python 无内置 tag）
"""

import json
from datetime import datetime


def _omitempty(data: dict) -> dict:
    """手动实现 omitempty：过滤掉 None / 空字符串 / 空集合（对应 Go omitempty tag）。"""
    return {k: v for k, v in data.items() if v is not None and v != "" and v != [] and v != {}}


def run() -> None:
    print("========== JSON 序列化 ==========")

    # ---------- 1. 基础序列化 / 反序列化 ----------
    data = {"name": "laixhe", "age": 18, "tags": ["go", "rust", "python"]}
    s = json.dumps(data, ensure_ascii=False)
    print(s)  # {"name": "laixhe", "age": 18, "tags": ["go", "rust", "python"]}

    # 反序列化（对应 json.Unmarshal）
    parsed = json.loads(s)
    print(parsed["name"], parsed["age"])  # laixhe 18

    # ---------- 2. omitempty：空值忽略（对应 Go TestJson）----------
    t_json = {"time1": None, "array1": [], "map1": {}, "name": "ok"}
    print(json.dumps(_omitempty(t_json), ensure_ascii=False))  # {"name": "ok"}

    # 非空值正常序列化
    full = {"time1": "2025-06-21T09:18:39Z", "array1": [1, 2], "map1": {"a": "1"}}
    print(json.dumps(full, ensure_ascii=False))
    # {"time1": "2025-06-21T09:18:39Z", "array1": [1, 2], "map1": {"a": "1"}}

    # ---------- 3. 数值以字符串形式序列化（对应 Go string tag）----------
    query = {
        "path": "/index/index",
        "query": "name=laixhe&age=18",
        "age": "18",       # 数字转字符串形式
        "score": "88.8",
        "is_pass": "false",
    }
    print(json.dumps(query, ensure_ascii=False))
    # 结果：{"path": "/index/index", "query": "name=laixhe&age=18", "age": "18",
    #        "score": "88.8", "is_pass": "false"}

    # 反序列化：字符串数字可再转回数值
    query2 = json.loads('{"age":"19","score":"99.99"}')
    print(int(query2["age"]), float(query2["score"]))  # 19 99.99

    # ---------- 4. 美化输出（对应 json.MarshalIndent）----------
    pretty = json.dumps(query, ensure_ascii=False, indent=2, sort_keys=True)
    print(pretty)

    # ---------- 5. 特殊类型与错误处理 ----------
    # datetime 需要 default 处理（对应 Go 中 time.Time 的自定义 MarshalJSON）
    now = datetime.now()
    print(json.dumps({"time": now}, ensure_ascii=False, default=str))
    # {"time": "2026-08-15 17:04:50.123456"}

    # 解析失败抛 JSONDecodeError（对应 json.Unmarshal 返回 error）
    try:
        json.loads("{invalid json")
    except json.JSONDecodeError as e:
        print("解析失败:", e.msg)  # Expecting property name enclosed in double quotes
