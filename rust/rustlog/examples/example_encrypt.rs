//! 加密与哈希示例：SHA256/384/512（ring）、密码哈希（bcrypt）、BLAKE3。
//!
//! ## 前置知识
//!
//! ### 什么是「哈希」（Hash）？
//! 把任意长度输入通过**单向**算法，变成**固定长度**的「摘要」。
//! 核心性质：
//! 1. 确定性 —— 同一个输入永远得到同一个哈希
//! 2. 不可逆 —— 几乎不可能从哈希反推原文（因此适合存密码）
//! 3. 雪崩效应 —— 输入改 1 个比特，哈希面目全非
//!
//! ### 常见算法选择
//! | 场景 | 推荐算法 | 说明 |
//! |---|---|---|
//! | 文件校验 / 数据完整性 | **BLAKE3**（最快）或 SHA256 | BLAKE3 性能远超 SHA2，但 SHA2 生态更通用 |
//! | **用户密码存储** | **bcrypt / Argon2** / scrypt | ❌ 绝对不能用 SHA/MD5！太容易被彩虹表/GPU 暴力破解 |
//! | 数字签名 / 证书 | SHA256 / SHA384 | 配合 RSA/ECDSA 使用 |
//!
//! ## 练习题
//! 1. 把 `password_input` 改成 `12345678`，哈希值发生了多大变化？（雪崩效应）
//! 2. 把 bcrypt 的 cost 调到 14，观察耗时大约翻了几倍。
//! 3. 用增量方式（start + update + finish）计算一个大文件的 SHA256（此处用拼接多段字符串模拟）。

use ring::digest::{self, Context};
use data_encoding::HEXUPPER;

fn main() {
    sha_family();
    sha_incremental();
    hash_password();
    blake3_demo();
}

// ============ SHA256 / SHA384 / SHA512（ring）============
fn sha_family() {
    let password = "123456".as_bytes();

    // ring::digest::digest(算法, 数据) —— 一次性哈希适合小数据
    let sha256 = digest::digest(&digest::SHA256, password);
    println!("SHA256('123456') -> {}", HEXUPPER.encode(sha256.as_ref()));

    let sha384 = digest::digest(&digest::SHA384, password);
    println!("SHA384('123456') -> {}", HEXUPPER.encode(sha384.as_ref()));

    let sha512 = digest::digest(&digest::SHA512, password);
    println!("SHA512('123456') -> {}", HEXUPPER.encode(sha512.as_ref()));
}

// ============ 增量 SHA（大文件 / 流式数据场景）============
//
// 很多时候数据没法一次读到内存里（比如几 GB 的大文件），
// 这时应该用 Context 做「增量哈希」：start → 多次 update → finish
fn sha_incremental() {
    println!("\n==== 增量 SHA256（模拟大文件分片）====");

    let mut ctx = Context::new(&digest::SHA256);
    // 模拟数据分块到来（比如 TCP 读缓冲区、文件分块）
    ctx.update(b"Hello, ");
    ctx.update(b"Rust ");
    ctx.update(b"World!");

    let result = ctx.finish();
    let inc_hash = HEXUPPER.encode(result.as_ref());
    println!("增量 update 结果: {}", inc_hash);

    // 校验：一次性哈希相同数据，结果必须一致
    let once = digest::digest(&digest::SHA256, b"Hello, Rust World!");
    let once_hash = HEXUPPER.encode(once.as_ref());
    assert_eq!(inc_hash, once_hash, "增量哈希必须等价于一次性哈希");
    println!("✅ 增量哈希 vs 一次性哈希，两者一致（断言通过）");
}

// ============ bcrypt 密码哈希 & 验证 ============
//
// ❌ 划重点：普通哈希（SHA/BLAKE）不适合存密码！
// 原因 1：太快了（每秒几十亿次），GPU 暴力破解很轻松
// 原因 2：相同密码 → 相同哈希，容易被彩虹表比对
// ✅ bcrypt / Argon2 是**慢哈希 + 自带随机盐（Salt）**：
//  - 每一次 hash 都会随机生成新的 salt，因此同一密码每次出来的 hash 都不一样
//  - cost 参数控制慢的程度：cost 每 +1，耗时约翻倍（抗 GPU/ASIC 暴力）
fn hash_password() {
    println!("\n==== bcrypt 密码哈希 & 验证 ====");

    let password_input = "123456";

    // cost 参数：迭代轮数 = 2^cost
    // - 10 左右：一般 Web 后端体验友好（几十 ms）
    // - 12 以上：更安全但更慢，移动端 / 高并发服务要权衡
    // 生产建议 >= 10，并根据机器性能动态调整到单次验证 100~500ms
    const COST: u32 = 10;

    // ---- 1. 注册 / 改密码时：生成 hash 存进数据库 ----
    let password_hash = bcrypt::hash(password_input, COST)
        // 免责：unwrap() 仅示例方便；bcrypt 可能因为 cost 非法等返回 Err
        .expect("bcrypt 哈希失败");
    println!("bcrypt(cost={}) hash = \"{}\"", COST, password_hash);

    // 注意：相同 password，每次 hash 都不同！（因为内置随机 salt）
    let password_hash_2 = bcrypt::hash(password_input, COST).unwrap(); // 免责：同上
    assert_ne!(password_hash, password_hash_2, "两次 hash 应该不一样（salt 不同）");
    println!("✅ 同一密码两次哈希结果不同（自动加盐，演示完毕）");

    // ---- 2. 登录时：验证用户输入 vs 数据库里的 hash ----
    // 这里用**刚才生成的 hash**做闭环验证，而不是硬编码——这样演示更直观
    let verified = bcrypt::verify(password_input, &password_hash)
        // 免责：unwrap() 仅示例方便；verify 可能因 hash 格式错误返回 Err
        .expect("bcrypt 验证时发生错误");
    println!("✅ 使用正确密码验证: verified = {}", verified); // true

    // 错误密码演示
    let wrong = bcrypt::verify("wrong_password", &password_hash).unwrap(); // 免责：同上
    println!("❌ 使用错误密码验证: verified = {}", wrong); // false
}

// ============ BLAKE3 高速哈希 ============
//
// BLAKE3 是目前主流密码学哈希里速度最快的之一，比 SHA2/3 快很多倍。
// 适合文件校验、内容寻址（IPFS）、去重等不需要兼容旧协议的场景。
fn blake3_demo() {
    println!("\n==== BLAKE3 ====");

    let data = b"123456";
    let hash = blake3::hash(data);
    println!("BLAKE3('123456') -> {}", hash.to_hex());

    // BLAKE3 也支持增量哈希（API 更简洁）：Hasher::new() + update + finalize
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"piece 1; ");
    hasher.update(b"piece 2; ");
    hasher.update(b"piece 3");
    let inc = hasher.finalize();
    println!("BLAKE3 增量模式 -> {}", inc.to_hex());
}
