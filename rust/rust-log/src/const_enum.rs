// 由关键字```const```声明常量
// 由关键字```static```声明全局变量
// 而枚举```enum```和传统上的枚举是不同的更像是结构体

// 系统的枚举有 Result、Option<T>
// Result -> Ok   | Err
// Option -> Some | None

// 定义枚举
enum Color {
    Read,
    Green,
    Blue
}
// 定义枚举方法
impl Color {
    pub fn get(c: Color) -> u8 {
        match c {
            Color::Read => 1,
            Color::Green => 2,
            Color::Blue => {
                3
            }
        }
    }
}
// 定义枚举
// 可以给枚举的每一个成员，指定一个或多个数据类型
enum Operation {
    Jump(u32),
    Move {x: i32, y:i32},
}
impl Operation {
    pub fn get(o: Operation){
        match o {
            Operation::Jump(value) => {
                println!("Operation Jump: {}", value);
            },
            Operation::Move {x,y} => {
                println!("Operation Move, x: {}, y: {}", x, y);
            }
        }
    }
}

// 全局变量
static IP: &'static str = "127.0.0.1";
// 常量
const PORT: i32 = 5500;

fn constEnumRun() {
    let color_blue = Color::Blue;
    println!("Color: {}", Color::get(color_blue));

    let opt_move = Operation::Move { x: 10, y: 20 };
    Operation::get(opt_move);

    println!("ip={},port={}", IP, PORT);
}
