
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct UserJson {
    name: String,
    age: i32,
    shape: ShapeJson
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="lowercase", tag="shape")]
enum ShapeJson {
    Circle{
        radius: f64
    },
    Rectangle{
        length: f64,
        width: f64
    }
}

pub fn json_code_run() {
	
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
    let user: UserJson = serde_json::from_str(json_str).unwrap();
    println!("反序列化：{:?}", user);
    // UserJson { name: "中Lai", age: 18, shape: Circle { radius: 11.01 } }
    // 注意："shape":"circle" 是为了区分不同的数据

    // 序列化
    let user_1 = UserJson {
        name:"laixhe".to_string(),
        age: 18,
        shape: ShapeJson::Rectangle {
            length: 10.0,
            width: 20.0
        }
    };
    let user_1_str: String = serde_json::to_string(&user_1).unwrap();
    println!("序列化：{}", user_1_str)
    // {"name":"laixhe","age":18,"shape":{"shape":"rectangle","length":10.0,"width":20.0}}
    // 注意："shape":"rectangle"
	
}