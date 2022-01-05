use std::io;
use std::env;
use std::process;
use std::error;
use std::error::Error;
use std::fs;

pub struct CmdConfig {
    search: String,    // 要搜索的字符串
    file_name: String, // 要搜索的文件
}

impl CmdConfig {

    // 构造解析
    pub fn new(args: &[String]) ->  Result<CmdConfig, &str> {
        if args.len() < 3 {
            return Err("程序启动时参数错误...");
        }

        Ok(CmdConfig{
            search: String::from(&args[1]),
            file_name: String::from(&args[2]),
        })
    }

    // 获取内容
    pub fn run(&self) -> Result<(), Box<dyn Error>>{
        let contents = fs::read_to_string(&self.file_name)?;
        //println!("获取到的内容：{}", contents);

        // 查内容
        for line in self.search_data(&contents) {
            println!("查内容：{}", line)
        }

        Ok(())
    }

    // 查内容
    pub fn search_data<'a>(&self, contents: &'a str) -> Vec<&'a str>{
        let mut results = Vec::new();

        for line in contents.lines() {
            if line.contains(&self.search) {
                results.push(line)
            }
        }

        results
    }

}

//
pub fn cmd_file_run(){
    // 获取程序启动时参数
    let args : Vec<String> = env::args().collect::<Vec<String>>();

    // let cmd_config = match CmdConfig::new(&args) {
    //     Ok(t) => t,
    //     Err(e) => {
    //         println!("cmd config error:{}", e);
    //         process::exit(1);
    //     },
    // };

    // 下面的 比 上面的 简洁多了

    let cmd_config = CmdConfig::new(&args).unwrap_or_else(|e|{
        println!("cmd config error:{}", e);
        // 退出程序
        process::exit(1);
    });

    println!("要搜索的字符串={}", cmd_config.search);
    println!("要搜索的文件={}", cmd_config.file_name);

    if let Err(e) = cmd_config.run() {
        println!("run error={}", e);
        process::exit(1);
    }
}