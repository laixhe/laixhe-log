//! JSON 序列化 / 反序列化示例（serde + serde_json）。
//!
//! ## 前置知识
//! - **serde**        ：Rust 生态通用的序列化框架，本身不绑定具体格式
//! - **serde_json**   ：serde 的 JSON 格式实现（对应还有 serde_yaml / toml / ron 等）
//! - `#[derive(Serialize, Deserialize)]`：过程宏，自动给结构体生成序列化/反序列化代码
//!
//! ## 练习题
//! 1. 给结构体加 `#[serde(rename_all = "camelCase")]`，观察 JSON 字段名变成什么样。
//! 2. 用 `serde_json::Value` 手写一个嵌套 JSON（包含数组、对象）。
//! 3. 故意构造一个非法 JSON 字符串，观察 `from_str` 返回的错误类型长什么样。

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

// ============ 结构体序列化 / 反序列化（最常用）============

/// 演示 serde 的常用 attribute
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    // 字段名默认原样匹配 JSON key（大小写敏感）
    pub name:  String,
    pub age:   u32,

    // #[serde(skip_serializing_if = "path")]
    // 序列化时若值满足条件则跳过该字段——这里 None 就不输出，保持 JSON 简洁
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin: Option<String>,

    // #[serde(default)]
    // 反序列化时如果 JSON 里没有这个字段，则用该类型的 Default::default()（bool 默认 false）
    #[serde(default)]
    pub active: bool,

    // #[serde(rename = "...")]
    // 把 Rust 字段名和 JSON key 做映射——当两边命名风格不一致时非常有用
    // 例如 Rust 蛇形命名 user_role，但 JSON 方要求驼峰 userRole
    #[serde(rename = "userRole", default = "default_role")]
    pub user_role: String,
}

/// serde(rename, default = "...") 里的 default 必须是一个「函数路径」，不能是字面量
fn default_role() -> String {
    "guest".to_string()
}

// ============ 序列化 / 反序列化：基础 ============

pub fn serialize_basic() {
    let user = User {
        name:      "laixhe".to_string(),
        age:       18,
        admin:     Some("super_admin".to_string()),
        active:    true,
        user_role: "editor".to_string(),
    };
    // to_string：把结构体序列化成 JSON 字符串（一行）
    let json_str = serde_json::to_string(&user)
        // 免责：unwrap() 仅示例方便；实际可能因为字段序列化失败返回 Err
        .expect("序列化失败");
    println!("✅ 序列化 (紧凑): {}", json_str);
    // 期望：{"name":"laixhe","age":18,"admin":"super_admin","active":true,"userRole":"editor"}

    // to_string_pretty：美化输出（带缩进换行），适合配置文件 / 调试
    let pretty = serde_json::to_string_pretty(&user).unwrap(); // 免责：同上
    println!("✅ 序列化 (美化):\n{pretty}");
}

pub fn deserialize_basic() {
    // 情况 1：完整 JSON，所有字段都在
    let full_json = r#"{"name":"laixhe","age":18,"admin":"admin","active":true,"userRole":"owner"}"#;
    let user1: User = serde_json::from_str(full_json)
        .expect("反序列化失败（完整 JSON）");
    println!("✅ 反序列化（完整）: {:?}", user1);

    // 情况 2：缺省字段 —— 演示 #[serde(default)] 和 default_role() 的效果
    // 这里缺了 "active"（会取 bool::default() = false）和 "userRole"（会取 default_role() = "guest"）
    // 还缺了 "admin"（因为是 Option，缺省为 None）
    let partial_json = r#"{"name":"new_user","age":25}"#;
    let user2: User = serde_json::from_str(partial_json)
        .expect("反序列化失败（缺省字段 JSON）");
    println!(
        "✅ 反序列化（缺省字段演示 default）: name={}, active={}, role={}, admin={:?}",
        user2.name, user2.active, user2.user_role, user2.admin
    );

    // 情况 3：非法 JSON —— 演示错误分支（生产代码不要 unwrap！）
    let bad_json = r#"{"name":"broken","age":not_a_number}"#;
    match serde_json::from_str::<User>(bad_json) {
        Ok(_) => println!("❓ 理论上不会成功"),
        Err(e) => println!("✅ 如预期失败（非法 JSON）: 错误种类=[{:?}] 详情={e}", e.classify()),
    }
}

// ============ 动态 JSON：serde_json::Value ============
//
// 当你不知道 JSON 具体结构（比如上游返回格式不固定），或需要临时构造 JSON 时使用。
// Value 是一个枚举：Null / Bool / Number / String / Array / Object
pub fn dynamic_value() {
    // 方式 1：json! 宏（最方便，像写 JSON 字面量一样）
    let v: Value = json!({
        "code": 0,
        "msg":  "ok",
        "data": {
            "user":   "张三",
            "tags":   ["rust", "go", "python"],
            "scores": [95.5, 88.0, 92.3],
            "vip":    true,
            "note":   null
        }
    });
    println!("\n✅ json! 宏构造:");
    println!("{}", serde_json::to_string_pretty(&v).unwrap()); // 免责：示例方便

    // 取值：用 [] 索引（越界 / 类型不对都返回 Value::Null，不会 panic）
    // 也可以用 .get() 返回 Option<&Value>
    let username = &v["data"]["user"];
    let first_tag = &v["data"]["tags"][0];
    let nonexistent = &v["data"]["this_key_does_not_exist"][999]; // → Null
    println!(
        "取值: user={username}, first_tag={first_tag}, 不存在的字段={nonexistent:?}"
    );

    // 把 Value 转回具体结构体（只要结构匹配就行）
    #[derive(Debug, Deserialize)]
    struct ApiResp {
        code: i32,
        msg:  String,
    }
    let resp: ApiResp = serde_json::from_value(v.clone())
        .expect("Value → 结构体失败");
    println!("Value → 结构体 ApiResp: {:?}", resp);
}

// ============ 数组 / 嵌套 ============

pub fn arrays_and_nested() {
    // 结构体数组
    let users = vec![
        User { name: "A".into(), age: 20, admin: None,          active: true,  user_role: "guest".into()  },
        User { name: "B".into(), age: 30, admin: Some("op".into()), active: false, user_role: "admin".into() },
    ];
    let json = serde_json::to_string_pretty(&users).unwrap(); // 免责：示例方便
    println!("\n✅ 结构体数组 → JSON:\n{json}");

    // JSON 数组 → Vec<User>
    let restored: Vec<User> = serde_json::from_str(&json).unwrap(); // 免责：示例方便
    println!("✅ JSON 数组 → Vec<User>，人数={}", restored.len());
}

fn main() {
    println!("============= serialize_basic =============");
    serialize_basic();
    println!("\n============= deserialize_basic =============");
    deserialize_basic();
    println!("\n============= dynamic_value =============");
    dynamic_value();
    println!("\n============= arrays_and_nested =============");
    arrays_and_nested();
}
