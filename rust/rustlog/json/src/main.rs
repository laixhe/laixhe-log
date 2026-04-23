use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Json {
    pub name: String,
    pub age: u32,
}

pub fn serialize(){
    let json = Json{
        name: "laixhe".to_string(),
        age: 18,
    };
    let json_string = serde_json::to_string(&json).unwrap();
    println!("序列化结果: {}", json_string);
}

pub fn deserialize(){
    let user: Json = serde_json::from_str(r#"{"name":"laixhe","age":18}"#).unwrap();
    println!("反序列化结果: {:?}", user);
}

fn main() {
    deserialize();
    serialize();
}