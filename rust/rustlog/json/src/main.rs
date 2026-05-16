use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Json {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "age")]
    pub age: u32,
    // 忽略空字段，保持输出整洁
    #[serde(rename = "admin", skip_serializing_if = "Option::is_none")]
    admin: Option<String>,
}

pub fn serialize(){
    let json = Json{
        name: "laixhe".to_string(),
        age: 18,
        admin: Some(String::from("admin")),
    };
    let json_string = serde_json::to_string(&json).unwrap();
    println!("序列化结果: {}", json_string);
}

pub fn deserialize(){
    let user: Json = serde_json::from_str(r#"{"name":"laixhe","age":18}"#).unwrap();
    println!("反序列化结果: {:?}", user);
}

fn main() {
    serialize();
    deserialize();
}