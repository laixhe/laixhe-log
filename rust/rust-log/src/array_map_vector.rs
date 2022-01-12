use std::collections::HashMap;

// tuple(元组)、Vec(向量[列表])、Map(hash 哈希)
// 元组是不可变的，可以将多个类型的值放到一起
// 访问超出数组的范围索引，编译可能会通过，运行时会报错(runtime panic)

pub fn tuple_vec_map_run() {
	
	// 数组
    // 索引从 0 开始

    // 定义数组并初始化
    let array: [i32; 5] = [1,2,3,4,5];
    println!("array={:?}, len={}", array, array.len()); // 结果： [1, 2, 3, 4, 5]
    for arr in &array {
        println!("array arr={}", arr);
    }

    // 定义数组并初始化数据都为 0
    let array_data: [i32; 5] = [0; 5];
    println!("array_data={:?}, len={}", array_data, array_data.len()); // 结果： [0, 0, 0, 0, 0]
    for arr in array_data.iter() {
        println!("array_data arr={}", arr);
    }

    // 获取某个值(没有这个 索引 时会报错)
    println!("array get = {}", array[2]);

    println!("=================================");
    
    // 定义元组
    let tuple:(i32, f64, u8) = (-325, 4.9, 22);
    // 结果： (-325, 4.9, 22) -325 22
    println!("tuple={:?} {} {}", tuple, tuple.0, tuple.2);
    // 元组解构赋值
    let (age, is) = (30, true);
    println!("tuple={} {}", age, is); // 结果： 30 true
    
    println!("=================================");

    // Vec(向量)(Vector)类似于列表(List)
    //let mut vectors: Vec<i32> = Vec::new();             // 初始化
    //let mut vectors: Vec<i32> = Vec::with_capacity(10); // 初始化并设置容量
    let mut vectors = vec![1, 2, 3];           // 初始化并追加元素

    // 追加元素
    vectors.push(4);
    println!("vector={:?}, len={}", vectors, vectors.len()); // 结果： [1, 2, 3, 4]
    for i in &vectors {
        println!("vector i={}", i)
    }

    // 删除某个下标索引元素
    vectors.remove(1); // 相当删除了 2

    // 判断是否包含某个值
    if vectors.contains(&3) {
    }

    // 获取某个值(没有这个 索引 时会报错)
    println!("vector get = {}", vectors[2]);
    // 判断key是否存在
    let is_i: bool = match vectors.get(3) {
        Some(_i) => true,
        None => false,
    };
    println!("vector is 3 = {}", is_i); // 结果: true

    println!("=================================");
    
    // 哈希Map (hash map)
    let mut maps:HashMap<String, i32> = HashMap::new();

    maps.insert(String::from("blue"), 50); // 新增或修改元素
    maps.insert(String::from("yellow"), 20);
    maps.insert(String::from("green"), 40);
    println!("maps={:?}, len={}", maps, maps.len()); // 结果: {"yellow": 20, "blue": 50, "green": 40}
    for (key, value) in &maps {
        println!("HashMap key={} value={}", key, value);
    }

    // 删除某个键元素
    maps.remove("green");

    // 判断是否包含某个键
    if maps.contains_key("green"){
    }

    // 获取某个值(没有这个 Key 时会报错)
    println!("HashMap blue={}", maps["blue"]);
    // 判断key是否存在
    let is_key: bool = match maps.get("blue") {
        Some(_i) => true,
        None => false,
    };
    println!("HashMap is blue = {}", is_key); // 结果: true

    // 当这个 key 存在时返回其 value，否则 新增 并返回其 value
    maps.entry("blue".to_string()).or_insert(111);
    maps.entry("red".to_string()).or_insert(30);   // 新增
    println!("HashMap = {:?}", maps);
    // 结果：{"blue": 50, "red": 30, "yellow": 20}

    to_array();
}

// 其他转换
pub fn to_array(){
    // 分隔字符串  (在 string.rs 中)
	// 拼接字符串  (在 string.rs 中)

	// 分隔字符串 (从 字符串 分隔为 数字类型数组) (需要自己实现)
	// 拼接字符串 (从 数字类型数组 拼接为 字符串) (需要自己实现)
}
