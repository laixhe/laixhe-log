#### json 相关函数
```
json_type          # 判断类型 ARRAY

json_array         # 创建数组
json_object        # 创建对象
json_quote         # 将 json 转成 json 字符串

json_contains      # 判断是否包含某个 json 值 (一般用于查数组)
json_extract       # 提取 json 值 (一般用于查某个值)
json_search        # 查找符合的值 

json_set           # 更新(插入)
json_replace       # 更新字段的值 (必须字段存在)
json_insert        # 插入字段与值 (必须字段不存在)
json_remove        # 删除字段

json_array_append  # 追加数组 (必须字段存在)
json_array_insert  # 插入数组 (必须字段存在)

json_keys          # 提取 json 中的键值为数组
json_search        # 按给定字符串关键字搜索

```

#### 创建表和数据
```
CREATE TABLE `users` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `data` json NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

INSERT INTO `users` (`id`, `data`) VALUES (1,'{"name":"laixhe","email":"laixhe@laixhe.com","address":[{"line":"abc","name":"123"},{"line":"def","name":"123"}],"code":[1,2,3],"tags":["go","php"]}');
INSERT INTO `users` (`id`, `data`) VALUES (2,'{"name":"laiki","email":"laiki@laixhe.com","address":[{"line":"def","name":"456"}],"code":[4,5,3],"tags":["c++","php"]}');
INSERT INTO `users` (`id`, `data`) VALUES (3, CAST('{"name":"lai","email":"lai@laixhe.com","address":[{"line":"abc","name":"789"}],"code":[1,3,6],"tags":["java","php"]}' AS JSON));

```

#### 查询
- 使用 字段->'$.json属性' 进行查询条件
- 使用 JSON_EXTRACT 函数查询，JSON_EXTRACT(字段, "$.json属性")
- 根据 json 数组查询，用 JSON_CONTAINS(字段, JSON_OBJECT('json属性', "内容"))
- 一般使用 JSON_EXTRACT 函数，不使用 -> 符号

```
# 查找 data JSON字段(对象类型)中的 email 字段
SELECT * FROM `users` WHERE `data` -> '$.email'='laixhe@laixhe.com';
# 或
SELECT * FROM `users` WHERE JSON_EXTRACT(`data`,'$.email')='laixhe@laixhe.com';

# 提取 JSON 字段中 address 的数据
SELECT JSON_EXTRACT(`data`,'$.address') AS `address` FROM `users`;
# 从 address 数组中查找 name 的值是否等于查找的值，返回 1 或 0
SELECT `id`,JSON_CONTAINS(JSON_EXTRACT(`data`,'$.address'), JSON_OBJECT('name', '456')) AS `is_addr` FROM `users`;

# 查找 data JSON字段(对象类型)中的 address(数组对象类型) 数组字段中 name 值等于 456 的记录
SELECT * FROM `users` WHERE JSON_CONTAINS(JSON_EXTRACT(`data`,'$.address'), JSON_OBJECT('name', '456')) > 0;

# 查找 data JSON字段(对象类型)中的 code(数组类型)中的值必须有 1 和 3
SELECT * FROM `users` WHERE JSON_CONTAINS(JSON_EXTRACT(`data`,'$.code'), JSON_ARRAY(1,3)) > 0;

```

#### 操作
```
# 更新(插入) 当 email 存在就修改否则就插入
UPDATE `users` SET `data`=JSON_SET(`data`,"$.email", "laiki@laixhe.com") WHERE `id`=2;

# 插入 email 字段与值 (必须 email 字段不存在)
UPDATE `users` SET `data`=JSON_INSERT(`data`,"$.email", "laiki@laixhe.com") WHERE `id`=2;

# 更新 email 字段的值 (必须 email 字段存在)
UPDATE `users` SET `data`=JSON_REPLACE(`data`,"$.email", "laiki@laixhe.com") WHERE id=2;

# 删除 data JSON字段(对象类型)中的 email 字段
UPDATE `users` SET `data`=JSON_REMOVE(`data`,"$.email") WHERE `id`=2;

# 追加数组 (必须 code 字段存在)
UPDATE `users` SET `data`=JSON_ARRAY_APPEND(`data`,"$.code", 7) WHERE `id`=2;
```
