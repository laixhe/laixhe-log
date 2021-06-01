use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: i32,
    shape: Shape
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="lowercase", tag="shape")]
enum Shape {
    Circle{
        radius: f64
    },
    Rectangle{
        length: f64,
        width: f64
    }
}

fn main() {
    let json_str = r#"
        {
            "name":"中Lai",
            "age":18,
            "shape":{
              "shape":"circle",
              "radius":11.01
            }
        }
    "#;
    // 反序列化
    let user: User = serde_json::from_str(json_str).unwrap();
    println!("反序列化：{:?}", user);
    // User { name: "中Lai", age: 18, shape: Circle { radius: 11.01 } }
    // 注意："shape":"circle" 是为了区分不同的数据

    // 序列化
    let user_1 = User{
        name:"laixhe".to_string(),
        age: 18,
        shape: Shape::Rectangle {
            length: 10.0,
            width: 20.0
        }
    };
    let user_1_str: String = serde_json::to_string(&user_1).unwrap();
    println!("序列化：{}", user_1_str)
    // {"name":"laixhe","age":18,"shape":{"shape":"rectangle","length":10.0,"width":20.0}}
    // 注意："shape":"rectangle"
}