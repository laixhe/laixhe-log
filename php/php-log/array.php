<?php
// Array(数组) (可以分为：数值数组[类似列表] 和 关联数组[类似字典])

// 不需要具有相同的类型(在项目中最好使用相同类型)

// 数值数组[类似列表]
$list = array('a', 'b', 'c');
var_dump($list);
$list[] = 'd'; // 追加元素
var_dump($list);

// 关联数组[类似字典]
$map = array('a'=>1, 'b'=>2, 'c'=>3);
var_dump($map);
$map['d'] = 4; // 不存在元素就新增元素，存在则修改
var_dump($map);
