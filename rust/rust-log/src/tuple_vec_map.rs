use std::collections::HashMap;

// tuple(元组)、Vec(向量[列表])、Map(hash 哈希)
// 元组是不可变的，可以将多个类型的值放到一起
// 访问超出数组的范围索引，编译可能会通过，运行时会报错(runtime panic)

pub fn tuple_vec_map_run() {
    // 定义数组并初始化
    let array: [i32; 5] = [1,2,3,4,5];
    for arr in &array {
        println!("array={}", arr); // 结果： 1 2 3 4 5
    }
    // 定义数组并初始化数据都为 0
    let array_data: [i32; 5] = [0; 5];
    for arr in array_data.iter() {
        println!("array_data={}", arr); // 结果： 0 0 0 0 0
    }

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
    //let mut vectors: Vec<i32> = Vec::new(); // 与下面效果一样
    let mut vectors = vec![1, 2, 3];
    vectors.push(4); // 追加元素
    println!("vector = {:?}", vectors); // 结果： [1, 2, 3, 4]
    for i in &vectors {
        println!("vector i={}", i)
    }
    // 获取某个值(没有这个 索引 时会报错)
    println!("vector get 3 = {}", vectors[3]);
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
    for (key, value) in &maps {
        println!("HashMap key={} value={}", key, value);
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
    // 结果：{"red": 30, "yellow": 20, "blue": 50}
}
