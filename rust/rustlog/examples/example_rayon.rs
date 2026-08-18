//! 并行计算示例（rayon）。
//!
//! ## 前置知识
//! - **rayon**：Rust 的数据并行库，把顺序迭代器变成并行迭代器
//! - `par_iter()`：并行遍历（只读），`par_iter_mut()`：并行遍历（可变）
//! - 工作窃取 (work-stealing)：线程空闲时从其他线程"偷"任务，自动负载均衡
//!
//! ## 何时用 rayon
//! | 场景 | 推荐 | 原因 |
//! |---|---|---|
//! | CPU 密集型 + 大数据集 | ✅ rayon | 多核并行，自动负载均衡 |
//! | I/O 密集型 | ❌ tokio 异步 | rayon 会阻塞线程，浪费 CPU |
//! | 小数据集 (< 1万) | ❌ 顺序 | 线程调度开销 > 并行收益 |
//!
//! ## 练习题
//! 1. 用 `par_iter().filter().count()` 统计 1~1百万中能被 7 整除的数的个数。
//! 2. 对比 `iter().map().collect()` 和 `par_iter().map().collect()` 的性能差异。
//! 3. 用 `par_chunks(1000)` 分块并行处理一个大数组。

use rayon::prelude::*;
use std::time::Instant;

// ============ 并行求和 ============

pub fn parallel_sum() {
    let nums: Vec<u64> = (1..=1_000_000).collect();

    // 顺序求和
    let seq_sum: u64 = nums.iter().sum();
    println!("✅ 顺序求和: {seq_sum}");

    // 并行求和——只需把 iter() 换成 par_iter()
    let par_sum: u64 = nums.par_iter().sum();
    println!("✅ 并行求和: {par_sum}");

    assert_eq!(seq_sum, par_sum);
    println!("✅ 两者结果一致");
}

// ============ 并行 map ============

pub fn parallel_map() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8];

    // par_iter().map()：并行 map
    // 注意：collect() 会保持顺序（和顺序迭代器一致），只是内部并行计算
    let squared: Vec<i32> = nums.par_iter().map(|&x| x * x).collect();
    println!("✅ 并行 map (平方): {squared:?}");

    // par_iter().filter()：并行 filter
    let evens: Vec<&i32> = nums.par_iter().filter(|&&x| x % 2 == 0).collect();
    println!("✅ 并行 filter (偶数): {evens:?}");

    // 链式调用：map → filter → collect
    let result: Vec<i32> = nums
        .par_iter()
        .map(|&x| x * 3)
        .filter(|&x| x > 10)
        .collect();
    println!("✅ 并行 map+filter (x*3 >10): {result:?}");
}

// ============ 并行排序 ============

pub fn parallel_sort() {
    let mut nums: Vec<u32> = (0..1000).rev().collect();

    // par_sort_unstable：并行排序（不稳定但更快）
    // "不稳定"指相等元素的前后顺序可能变，对纯数字无所谓
    nums.par_sort_unstable();
    println!("✅ 并行排序: 前 5 = {:?} ... 后 5 = {:?}", &nums[..5], &nums[995..]);

    // 并行排序字符串
    let mut words = vec!["banana", "apple", "cherry", "date", "elderberry"];
    words.par_sort_unstable();
    println!("✅ 并行排序字符串: {words:?}");
}

// ============ 并行 reduce (fold) ============

pub fn parallel_reduce() {
    let nums: Vec<i32> = (1..=100).collect();

    // 顺序 reduce：sum
    let seq_sum: i64 = nums.iter().map(|&x| x as i64).sum();

    // 并行 reduce：par_iter().reduce()
    // reduce 要求操作符满足结合律（如加法、乘法），因为并行执行顺序不固定
    let par_sum: i64 = nums.par_iter().map(|&x| x as i64).reduce(|| 0, |a, b| a + b);

    println!("✅ 顺序 sum (1..=100): {seq_sum}");
    println!("✅ 并行 reduce (1..=100): {par_sum}");
    assert_eq!(seq_sum, par_sum);
}

// ============ 性能对比 ============

pub fn performance_compare() {
    let nums: Vec<u64> = (1..=1_000_000).collect();

    // 顺序：map(x^2) + sum
    // 注意：范围不能太大，否则 x^2 求和会溢出 u64
    let start = Instant::now();
    let seq: u64 = nums.iter().map(|x| x * x).sum();
    let seq_time = start.elapsed();

    // 并行：map(x^2) + sum
    let start = Instant::now();
    let par: u64 = nums.par_iter().map(|x| x * x).sum();
    let par_time = start.elapsed();

    println!("✅ 性能对比 (1百万次 x^2 求和):");
    println!("   顺序: {seq_time:?}");
    println!("   并行: {par_time:?}");
    println!("   加速比: {:.1}x", seq_time.as_secs_f64() / par_time.as_secs_f64());
    assert_eq!(seq, par);
}

// ============ 并行 any / all ============

pub fn parallel_any_all() {
    let nums: Vec<i32> = (1..=1_000_000).collect();

    // par_iter().any()：并行查找，找到一个 true 就提前返回
    let has_large = nums.par_iter().any(|&x| x > 999_990);
    println!("✅ 并行 any (>999990): {has_large}");

    // par_iter().all()：并行检查，遇到一个 false 就提前返回
    let all_positive = nums.par_iter().all(|&x| x > 0);
    println!("✅ 并行 all (>0): {all_positive}");
}

fn main() {
    println!("============= parallel_sum =============");
    parallel_sum();
    println!("\n============= parallel_map =============");
    parallel_map();
    println!("\n============= parallel_sort =============");
    parallel_sort();
    println!("\n============= parallel_reduce =============");
    parallel_reduce();
    println!("\n============= performance_compare =============");
    performance_compare();
    println!("\n============= parallel_any_all =============");
    parallel_any_all();
}
