// 由关键字```const```声明常量
// 由关键字```static```声明全局变量
// 而枚举```enum```和传统上的枚举是不同的更像是结构体

// 系统的枚举有 Result<T, Error>、Option<T>
// Result -> Ok   | Err
// Option -> Some | None

// 定义枚举
#[derive(Debug)]
#[repr(i32)]
enum Color {
    Read  = 1,
    Green = 2,
    Blue  = 3,
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

pub fn const_enum_run() {

    println!("Color: {:?}", Color::Read);
    println!("Color: {:?}", Color::Green);
    println!("Color: {:?}", Color::Blue);

    let opt_jump = Operation::Jump(111);
    Operation::get(opt_jump);

    let opt_move = Operation::Move { x: 10, y: 20 };
    Operation::get(opt_move);

    println!("ip={},port={}", IP, PORT);
}
