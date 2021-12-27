
// 如果想把 函数 或 struct 等设为私有，可以将它放到某个模中块
// 父级模块无法访问子模块中的私有的
// 使用 super 来访问父级模块中的内容

// 定义模块(私有的)
pub mod lib_one {
    // 定义二级模块(公共的)
    pub mod lib_one_two {
        pub fn two_run(){
            println!("two_run...");

            crate::module_lib::lib_func_test();
            super::super::lib_func_test();
        }
    }
}

// 定义函数(公共的)
pub fn lib_func_run(){
    // 使用 绝对路径(推荐)
    crate::module_lib::lib_one::lib_one_two::two_run();

    println!("======================================");

    // 使用 相对路径
    lib_one::lib_one_two::two_run();
}

fn lib_func_test(){
    println!("lib_func_test...");
}