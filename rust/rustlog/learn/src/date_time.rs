use std::thread;

// std::time::Duration 时间间隔计算
// let sec = Duration::from_secs(3);       // 3 秒
// let ms = Duration::from_millis(300);    // 300 毫秒
// let us = Duration::from_micros(500);    // 500 微秒
// let ns = Duration::from_nanos(1000);    // 1000 纳秒
//
// std::time::Instant    时间点（timestamp）
// std::time::SystemTime 系统时间

pub fn std_time(){
    let start = std::time::Instant::now();

    let now = std::time::SystemTime::now();
    println!("当前时间：{:?}", now);

    println!("计算耗时：{:?}", start.elapsed());
}

// 休眠
pub fn sleep(){
    thread::sleep(std::time::Duration::from_secs(1));
}