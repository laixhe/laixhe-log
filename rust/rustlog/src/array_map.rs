//! 集合类型示例：数组、元组、Vec、VecDeque、HashMap、BTreeMap、HashSet、BTreeSet、BinaryHeap。
//!
//! ## 选型速查（见 README 详细版）
//! - 默认存一组元素 → **Vec**
//! - 两端增删 → **VecDeque**
//! - 无序键值对 → **HashMap**；需要有序遍历 → **BTreeMap**
//! - 无序不重复值 → **HashSet**；有序不重复值 → **BTreeSet**
//! - 每次取最值 → **BinaryHeap**（最大堆，配合 Reverse 做最小堆）
//!
//! ## 为什么不推荐 LinkedList
//! 标准库虽然提供了双向链表 `LinkedList<T>`，但绝大多数场景性能不如 `Vec` / `VecDeque`。
//! 原因在于现代 CPU 极度依赖缓存命中率：`Vec`/`VecDeque` 是连续内存，CPU 预取器一次能拉一大段；
//! 而 `LinkedList` 节点分散在堆上，每次跳转都可能缓存未命中，慢几十到上百倍。
//! 结论：日常默认用 `Vec` / `VecDeque`，`LinkedList` 只在需要同时持有多个稳定节点指针时才考虑。
//!
//! ## 练习题
//! 1. 用 `BTreeMap` 插入 3 个键值对，观察遍历输出是否按 key 排序。
//! 2. 用 `BinaryHeap` 实现一个「任务优先级队列」，每次 pop 出优先级最高的任务。
//! 3. `Vec::dedup()` 可以去除相邻重复元素，但前提是先排序。对 `vec![3,1,4,1,5,9,2,6,5,3,5]` 先排序再 dedup。

use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::cmp::Reverse;

// ============ 数组 Array ============
// 数组中的每个元素类型必须相同，且长度**固定**（编译期确定，在栈上分配）
pub fn std_array() {
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

    // ⚠️ 直接用下标越界访问会 panic
    // 安全访问：用 get 方法（返回 Option<&T>）
    match arr.get(10) {
        Some(value) => println!("arr = {}", value),
        None => println!("arr = 索引越界了"),
    }
}

// ============ 元组 Tuple ============
// 元组可以将多个**不同类型**的值组合成一个固定大小的序列
// 只能修改元素的值，不能改变长度或类型（因为长度是类型的一部分）
pub fn std_tuple() {
    // 定义一个包含整数、浮点数和字符串的元组
    let tup: (i32, f64, &str) = (10, 3.14, "hello");
    println!("tup 元素：{}、{}、 {}", tup.0, tup.1, tup.2);

    let another_tup = (true, 'A', 42);
    // 解构（Destructuring）：一次性把元组每个字段绑定到变量
    let (a, b, c) = another_tup;
    println!("another_tup 元素：a = {}, b = {}, c = {}", a, b, c);
}

// ============ 动态数组(向量) Vec<T> ============
// 在堆上分配一块连续内存，并允许在运行时增长或收缩
// 尾部操作高效：push 和 pop 的**均摊**时间复杂度为 O(1)
// 中间插入/删除：O(n)，需要移动后续元素
pub fn std_vec() {
    // ---- 创建方式 ----
    let numbers1: Vec<i32> = Vec::new();
    println!("numbers1 = {:?}", numbers1); // 结果：[]

    // 预分配容量（避免频繁扩容，性能优化建议）
    let numbers2: Vec<i32> = Vec::with_capacity(10);
    println!(
        "numbers2 = {:?} 长度：{} 容量：{}",
        numbers2,
        numbers2.len(),
        numbers2.capacity()
    ); // 结果：[] 长度：0 容量：10

    let mut numbers3: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("numbers3 = {:?}", numbers3); // 结果：[1, 2, 3, 4, 5]

    // ---- 基础查询 ----
    println!(
        "contains 3? {}  contains 6? {}",
        numbers3.contains(&3),
        numbers3.contains(&6)
    ); // true false

    // 重复值初始化
    let mut numbers4: Vec<i32> = vec![0; 10];
    numbers4.push(1);
    numbers4.push(2);
    println!("numbers4 = {:?}", numbers4); // [0,0,...0,1,2]
    numbers4.pop(); // 删除末尾元素并返回（这里返回 Some(2)）
    println!("numbers4 = {:?}", numbers4); // [0,0,...0,1]

    // ---- 🌟 更多常用方法（补充）----

    // 1. insert / remove：在任意位置插入 / 删除（O(n)，因为要移动后续元素）
    let mut v = vec!['a', 'b', 'c', 'd'];
    v.insert(2, 'x');  // 在索引 2 插入 'x'，后面的元素后移
    println!("after insert: {:?}", v); // ['a', 'b', 'x', 'c', 'd']
    let removed = v.remove(1); // 删除索引 1 的元素 'b'，后面的元素前移
    println!("remove index 1 → got '{}', vec now: {:?}", removed, v); // 'b', ['a','x','c','d']

    // 2. swap_remove：快速删除（把最后一个元素移动到删除位置，O(1)，但**会打乱顺序**）
    let mut v2 = vec![10, 20, 30, 40, 50];
    let got = v2.swap_remove(1); // 删掉 20，把 50 搬过来
    println!("swap_remove index 1 → got {}, vec: {:?}", got, v2); // 20, [10,50,30,40]

    // 3. first / last / first_mut / last_mut：安全获取首尾元素（返回 Option）
    let v3 = vec![1, 2, 3];
    println!("first={:?} last={:?}", v3.first(), v3.last()); // Some(1) Some(3)

    // 4. sort / sort_by / dedup（相邻去重，需要先排序）
    let mut v4 = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    v4.sort();             // 升序排序
    println!("sorted: {:?}", v4);
    v4.dedup();            // 去掉相邻重复元素
    println!("deduped: {:?}", v4); // [1,2,3,4,5,6,9]

    // 5. extend：从另一个迭代器批量追加
    let mut v5 = vec![1, 2];
    v5.extend([3, 4, 5]); // 追加数组（也可接 Vec、range 等任何 IntoIterator）
    println!("extend: {:?}", v5); // [1,2,3,4,5]

    // 6. chunks / windows：按大小切片迭代（非常适合滑动窗口算法）
    let v6 = vec![10, 20, 30, 40, 50];
    println!("chunks(2):");
    for chunk in v6.chunks(2) {   // 不重叠分块：[10,20] [30,40] [50]
        println!("  {:?}", chunk);
    }
    println!("windows(3):");
    for win in v6.windows(3) {    // 滑动窗口：[10,20,30] [20,30,40] [30,40,50]
        println!("  {:?}", win);
    }

    // ---- 排序与查找 ----
    numbers3.sort_by(|a, b| b.cmp(a)); // 倒序排序
    for number in &numbers3 {
        println!("遍历1：{}", number);
    }
    for i in 0..numbers3.len() {
        println!("遍历2：{} = {}", i, numbers3[i]);
    }
    for (i, number) in numbers3.iter().enumerate() {
        println!("遍历3：{} = {}", i, number);
    }

    // position：找到第一个满足条件的元素的索引
    // iter() 返回 &&T，|&x| 解一层引用，等价于 |x: &&i32| *x == 3
    let index = numbers3.iter().position(|&x| x == 3);
    println!("查找元素 3 的位置：{:?}", index);

    // retain：**保留**符合条件的元素（原地修改）
    numbers3.retain(|number| number % 2 == 0);
    println!("保留偶数：{:?}", numbers3);
}

// ============ 双端队列 VecDeque<T> ============
// 支持在头部和尾部以 O(1) 时间复杂度高效地插入或删除元素
// 内部实现是环形缓冲区，非常适合实现队列（FIFO）或栈（LIFO）
pub fn std_vec_deque() {
    let deque1: VecDeque<i32> = VecDeque::new();
    let deque2: VecDeque<i32> = VecDeque::with_capacity(100);
    let deque3: VecDeque<i32> = VecDeque::from([1, 2, 3]);

    let vec4: Vec<i32> = vec![1, 2, 3];
    let deque4: VecDeque<i32> = vec4.into_iter().collect();

    println!(
        "创建方式: {:?} {:?} {:?} {:?}",
        deque1, deque2, deque3, deque4
    );

    // ---- 🌟 前后两端操作（VecDeque 核心价值）----
    let mut dq = VecDeque::from([2, 3, 4]);
    dq.push_front(1);  // 头部插入 O(1)
    dq.push_back(5);   // 尾部插入 O(1)
    println!("push 后: {:?}", dq); // [1,2,3,4,5]

    let front = dq.pop_front(); // 头部弹出 O(1)
    let back  = dq.pop_back();  // 尾部弹出 O(1)
    println!("pop_front={:?} pop_back={:?} 剩余: {:?}", front, back, dq); // 1,5,[2,3,4]
}

// ============ HashMap<K, V>（无序，哈希表实现）============
// 基于哈希表实现，提供平均 O(1) 的插入、查找和删除操作
// **遍历顺序不保证**（取决于哈希函数和插入历史）
pub fn std_hash_map() {
    // 显式创建空 HashMap
    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert(String::from("Blue"), 10);
    map.insert(String::from("Red"), 25);
    println!("初始: {:?}", map); // {"Blue": 10, "Red": 25}

    // 多种查询方式
    println!("get Red={:?}", map.get("Red")); // Some(25)

    match map.get("Red") {
        Some(v) => println!("match Red={}", v),
        None => println!("match Red 不存在"),
    }

    if let Some(v) = map.get("Red") {
        println!("if let Red={}", v);
    } else {
        println!("if let Red 不存在");
    }

    println!("Red 是否存在: {}", map.contains_key("Red")); // true

    // 删除数据
    map.remove("Red");
    println!("remove Red 后: {:?}", map); // {"Blue": 10}

    // entry API：不存在就插入，存在则不处理，并返回最新值的可变引用
    let or_default = map.entry("Red".to_string()).or_insert(20);
    println!("entry or_insert: Red={}", or_default); // 20

    // 遍历（默认不保证遍历顺序）
    println!("遍历 HashMap（顺序可能变）:");
    for (key, value) in &map {
        println!("  {} = {}", key, value);
    }
}

// ============ 🌟 BTreeMap<K, V>（有序，B 树实现）============
// 基于 B 树实现，键必须实现 Ord trait
// 插入/查找/删除：O(log n)
// 遍历**按 key 自然顺序**，支持范围查询（这是 HashMap 做不到的！）
pub fn std_btree_map() {
    let mut map: BTreeMap<&str, i32> = BTreeMap::new();
    map.insert("Charlie", 30);
    map.insert("Alice",   25);
    map.insert("Bob",     28);
    map.insert("David",   35);

    // ✨ 遍历是有序的！（按 key 字典序）
    println!("BTreeMap 顺序遍历（必定有序）:");
    for (name, age) in &map {
        println!("  {name}: {age}"); // Alice→Bob→Charlie→David
    }

    // ✨ 范围查询（HashMap 做不到）：查 "B" 到 "D" 之间的 entry
    println!("BTreeMap 范围查询 B..D:");
    for (name, age) in map.range("B".."D") {
        println!("  {name}: {age}"); // Bob, Charlie（不包含 David，因为是半开区间）
    }

    // 其他 API 与 HashMap 基本一致：get / insert / remove / entry / contains_key
}

// ============ 🌟 HashSet<T>（无序不重复集合）============
// 本质是 HashMap<T, ()> 的薄封装，只存键不存值
// 平均 O(1) 插入 / 查找 / 删除，适合去重、集合运算（交/并/差/对称差）
pub fn std_hash_set() {
    let mut a: HashSet<i32> = [1, 2, 3, 4].iter().copied().collect();
    let     b: HashSet<i32> = [3, 4, 5, 6].iter().copied().collect();

    // 插入重复元素会被自动忽略（返回 false 表示已存在）
    let inserted = a.insert(2); // 2 已存在
    println!("插入重复 2 成功？{}  集合 a={:?}", inserted, a); // false

    // 集合运算
    println!("交集 a∩b: {:?}", a.intersection(&b).collect::<Vec<_>>()); // [3,4]
    println!("并集 a∪b: {:?}", a.union(&b).collect::<Vec<_>>());        // [1,2,3,4,5,6]
    println!("差集 a-b: {:?}", a.difference(&b).collect::<Vec<_>>());   // [1,2]
    println!("对称差 (a-b)∪(b-a): {:?}", a.symmetric_difference(&b).collect::<Vec<_>>()); // [1,2,5,6]
}

// ============ 🌟 BTreeSet<T>（有序不重复集合）============
// 本质是 BTreeMap<T, ()> 的薄封装。遍历时按 Ord 排序。
// 同样支持范围查询（这是 HashSet 做不到的）。
pub fn std_btree_set() {
    let set: BTreeSet<i32> = [10, 20, 30, 40, 50].iter().copied().collect();

    println!("BTreeSet 有序遍历: {:?}", set.iter().collect::<Vec<_>>()); // [10,20,30,40,50]

    // 范围查询：取 [20, 40) 区间内的值
    println!("BTreeSet range 20..40: {:?}", set.range(20..40).collect::<Vec<_>>()); // [20,30]
}

// ============ 🌟 BinaryHeap<T>（优先级队列 / 最大堆）============
// 基于二叉堆实现，堆顶始终是集合中的最大值（最大堆）。
// 入堆 (push)、出堆 (pop) 都是 O(log n)。
// 需要最小堆？用 Reverse<T> 包一层即可。
pub fn std_binary_heap() {
    // ---- 最大堆（默认）----
    let mut max_heap: BinaryHeap<i32> = BinaryHeap::new();
    max_heap.push(30);
    max_heap.push(10);
    max_heap.push(50);
    max_heap.push(20);

    print!("最大堆依次 pop: ");
    while let Some(v) = max_heap.pop() {
        print!("{v} "); // 50 30 20 10
    }
    println!();

    // ---- 最小堆（用 Reverse 包装）----
    let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    min_heap.push(Reverse(30));
    min_heap.push(Reverse(10));
    min_heap.push(Reverse(50));
    min_heap.push(Reverse(20));

    print!("最小堆依次 pop: ");
    while let Some(Reverse(v)) = min_heap.pop() {
        print!("{v} "); // 10 20 30 50
    }
    println!();

    // ---- peek：查看堆顶（不弹出）----
    let h = BinaryHeap::from([3, 1, 4, 1, 5]);
    println!("peek 最大值 = {:?}", h.peek()); // Some(5)
}

// ============ 练习题参考答案（cargo test 可验证）============
#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BinaryHeap};

    // 练习 1：BTreeMap 按 key 有序遍历
    #[test]
    fn exercise_1_btree_map_sorted() {
        let mut map: BTreeMap<&str, i32> = BTreeMap::new();
        map.insert("b", 2);
        map.insert("a", 1);
        map.insert("c", 3);
        let keys: Vec<&str> = map.keys().copied().collect();
        assert_eq!(keys, vec!["a", "b", "c"]);
    }

    // 练习 2：BinaryHeap 任务优先级队列（数字越大优先级越高，最大堆先 pop 最大的）
    #[test]
    fn exercise_2_priority_queue() {
        let mut queue: BinaryHeap<(u8, &str)> = BinaryHeap::new();
        queue.push((3, "低优先级"));
        queue.push((1, "高优先级"));
        queue.push((2, "中优先级"));
        assert_eq!(queue.pop(), Some((3, "低优先级")));
        assert_eq!(queue.pop(), Some((2, "中优先级")));
        assert_eq!(queue.pop(), Some((1, "高优先级")));
    }

    // 练习 3：先排序再 dedup 去重
    #[test]
    fn exercise_3_sort_then_dedup() {
        let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        v.sort();
        v.dedup();
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6, 9]);
    }
}
