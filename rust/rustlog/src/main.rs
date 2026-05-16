use std::collections::VecDeque;
use std::collections::HashMap;

// 字符类型 Character
// Rust 的 char 类型代表一个 Unicode 标量值，
// 占用 4 个字节。它可以表示中文、表情符号等。
fn std_char(){
    let c1: char = 'A'; // 英文字母
    let c2: char = '中'; // 中文字符
    let c3: char = '😀'; // 表情符号（Emoji）
    let c4: char = '1'; // 数字字符
    let c5: char = '!'; // 标点符号

    println!("c1 = {} c2 = {} c3 = {} c4 = {} c5 = {}", c1, c2, c3, c4, c5);
}

// 数组 Array
// 数组中的每个元素类型必须相同，且长度固定
// 数组分配在栈上，而不是堆上
fn std_array(){
    // 声明一个包含 5 个 i32 类型元素的数组
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("arr = {:?}", arr);

    for element in &arr {
        println!("for遍历元素：{}", element);
    }

    // 声明一个包含 10 个元素的数组，每个元素值为 0
    let arr1: [i32; 10] = [0; 10];
    println!("arr1 = {:?}", arr1);

    for i in 0..arr1.len() {
        println!("索引遍历 {}: {}", i, arr1[i]);
    }
}

// 元组 Tuple
// 元组可以将多个不同类型的值组合成一个固定大小的序列
// 只能修改元素的值，不能改变长度或类型
// 创建元组使用圆括号()
fn std_tuple(){
    // 定义一个包含整数、浮点数和字符串的元组
    let tup: (i32, f64, &str) = (10, 3.14, "hello");

    // 类型推断：Rust 会自动推断元组类型
    let another_tup: (bool, char, i32) = (true, 'A', 42);

    println!("tup 元素：{}、{}、 {}", tup.0, tup.1, tup.2);

    let (a, b, c) = another_tup; // 解构
    println!("another_tup 元素：a = {}, b = {}, c = {}", a, b, c);
}

// 动态数组(向量) Vec<T>
// 在堆上分配一块连续内存，并允许在运行时增长或收缩
// 尾部操作高效：push 和 pop 的均摊时间复杂度为 O(1)
// 中间插入/删除：时间复杂度 O(n)，因为需要移动后续元素
fn std_vec(){
    let numbers1: Vec<i32> = Vec::new();
    println!("numbers1 = {:?}", numbers1);

    let numbers2: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("numbers2 = {:?}", numbers2);
    println!("numbers2 = {} {}", numbers2.contains(&3), numbers2.contains(&6));
    // contains 判断 Vector 是否包含指定元素

    let mut numbers3: Vec<i32> = vec![0; 10]; // 包含 10 个 0 的 Vec<i32>
    numbers3.push(1); // 向末尾添加一个元素
    println!("numbers3 = {:?}", numbers3);
}

// 双端队列 VecDeque<T>
// 支持在头部和尾部以 O(1) 时间复杂度高效地插入或删除元素
// 非常适合实现队列（先进先出）或双端队列（两端操作）场景
fn std_vec_deque(){
    // 空队列
    let deque1: VecDeque<i32> = VecDeque::new();

    // 预分配容量（避免频繁扩容）
    let deque2: VecDeque<i32> = VecDeque::with_capacity(100);

    // 从数组创建
    let deque3: VecDeque<i32> = VecDeque::from([1, 2, 3]);

    // 从 Vec 转换
    let vec4: Vec<i32>  = vec![1, 2, 3];
    let deque4: VecDeque<i32> = vec4.into_iter().collect();

    println!("std_vec_deque = {:?} {:?} {:?} {:?}", deque1,deque2,deque3,deque4);
}

// 集合 HashMap<K, V>
// 基于哈希表实现，提供平均 O(1) 的插入、查找和删除操作
fn std_hash_map(){
    // 显式创建空 HashMap
    let mut map: HashMap<String, u32> = HashMap::new();
    // 插入键值对
    map.insert(String::from("Blue"), 10);
    map.insert(String::from("Red"), 25);

    println!("{:?}", map);

    let value = map.get("Red"); // Option<&V>
    println!("{:?}", value);

    let or_default = map.entry("Blue".to_string()).or_insert(0);
    println!("{:?}", or_default);
}

// 有序集合 BTreeMap<K, V>
// 基于 B 树的有序键值映射
// 根据键的顺序（通过 Ord trait）存储元素，支持高效的范围查询、有序遍历和日志时间复杂度的插入、删除、查找

// 无序集合 HashSet<T>
// 基于哈希表的无序集合
// 存储不重复的值，并支持平均 O(1) 的插入、删除和查找操作

// 有序集合 BTreeSet<T>
// 基于 B 树的有序集合
// 存储不重复的值，并且始终按照元素的顺序（通过 Ord trait）进行排序)

// 优先级队列 BinaryHeap<T>
// 基于二叉堆的优先级队列
// 是一个最大堆（max-heap），意味着堆顶始终是集合中的最大值

// 双向链表 LinkedList<T>
// 是一个线性集合，每个元素都包含指向前一个和后一个元素的指针

/*
重要提示：
在绝大多数 Rust 使用场景中，LinkedList 性能不佳，通常应优先使用 Vec<T> 或 VecDeque<T>
只有在需要频繁的中间插入/删除且同时需要在链表两端进行高效操作时，才考虑使用链表。
即便如此，现代 CPU 缓存特性使得连续内存的 VecDeque 往往更快。
*/

fn main() {
    println!("============= std_char =============");
    std_char();

    println!("============= std_array =============");
    std_array();

    println!("============= std_tuple =============");
    std_tuple();

    println!("============= std_vec =============");
    std_vec();

    println!("============= std_vec_deque =============");
    std_vec_deque();

    println!("============= std_hash_map =============");
    std_hash_map();
}
