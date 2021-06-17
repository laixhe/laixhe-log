use std::collections::HashMap;

// tuple(元组)、Vec(向量[列表])、Map(hash 哈希)
// 元组是不可变的

fn main() {
    // 定义数组并初始化
    let array: [i32; 5] = [1,2,3,4,5];
    for arr in &array {
        println!("array={}", arr); // 结果： 1 2 3 4 5
    }
    // 定义数组并初始化数据都为 0
    let array_data: [i32; 5] = [0; 5];
    for arr in &array_data {
        println!("array_data={}", arr); // 结果： 0 0 0 0 0
    }
    
    // 定义元组
    let tuple:(i32,f64,u8) = (-325,4.9,22);
    // 结果： (-325, 4.9, 22) -325 22
    println!("{:?} {} {}", tuple, tuple.0, tuple.2);
    // 元组解构赋值
    let (age,is) = (30,true);
    println!("{} {}",age ,is); // 结果： 30 true
    
    // Vec(向量)(Vector)类似于列表(List)
    //let mut vector: Vec<i32> = Vec::new(); // 与下面效果一样
    let mut vector = vec![1, 2, 3];
    vector.push(4); // 追加元素
    println!("{:?}", vector); // 结果： [1, 2, 3, 4]
    for i in &vector {
        println!("i={}", i)
    }

    // 哈希 map (hash map)
    let mut scores:HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("blue"), 50);
    scores.insert(String::from("yellow"), 20);
    for (key, value) in &scores {
        println!("key={} value={}", key, value);
    }
}
