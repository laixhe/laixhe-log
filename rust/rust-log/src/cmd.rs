use std::io;

// 获取程序启动时参数 std::env::args
pub fn cmd_args(){
    let mut args = std::env::args();
    println!("{:?}", args);

    // 第一个值一般来说就是 程序本身的名字
    println!("{:?}", args.nth(0));
}


// 交互模式-读取用户的输入
pub fn cmd_stdin_run(){
    // 用户
    let mut username = String::new();
    // 密码
    let mut password  = String::new();

    println!("请输入用户名:");
    io::stdin().read_line(&mut username).expect("读取用户失败");
    // io::Result Ok Err

    println!("请输入密码:");
    io::stdin().read_line(&mut password).expect("读取密码失败");
    
    username = username.trim().to_string();
    password = password.trim().to_string();
    println!("读取用户的输入:[username={}] [password={}]", username, password);
}