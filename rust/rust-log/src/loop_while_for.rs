
// 有三种循环语句：loop、while、for
// loop 代表一个无限循环，能够对循环体添加命名标签，
//      经常用于多个循环存在，而只想用 break 退出其中一个的情形

pub fn loop_while_for_run(){
    let a = 10;
    let b = 4;
    let result = loop_run(a, b);
    println!("{} minus {} is {}", a, b, result);

    for_run();

    match_run();

}

fn loop_run(a: i32, b: i32) -> i32 {
    let mut result = 0;
    'increment: loop {
        if result == a {
            let mut dec = b;
            loop {
                if dec == 0 {
                    break 'increment;
                } else {
                    result -= 1;
                    dec -= 1;
                }
            }
        } else {
            result += 1;
        }
    }
    result
}

fn for_run(){
    // for i=0;i<10;i++
    for i in 0..10 { // 不包含结束数字， 从 0 到 9
        print!("for_run i={},", i);
    }
    println!();
    for i in (0..10).rev() { // 从 9 到 0
        print!("for_run i={},", i);
    }
    println!();
}

fn match_run(){

    // switch-case-default
    let status = 10;
    match status {
        0 => {
            println!("match 0 ={}", status);
        },
        4 | 5 | 6 => {
            println!("match 4 or 5 or 6 ={}", status);
        },
        7..=9 => {
            println!("match 7 or 8 or 9 ={}", status);
        }
        10 => {
            println!("match 10 ={}", status);
        },

        _ => { // 最后匹配不到的 (就像 switch 中的 default 一样)，必须放到最后
            println!("match _ ={}", status);
        }
    }

}
