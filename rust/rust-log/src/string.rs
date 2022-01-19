//
// 字符串通常指的的是：String 和 &str ( &'static str )
//
// String 类型实际上是 std::string::String 类型，且保证内部只保存标准的 UTF-8 文本，
//        在操作时又是以 char 类型进行操作的，char 类型大小为 4 个字节(32位)的 Unicode 码，
//        为了解决使用 Unicode 带来的麻烦，提供了将 utf-8 字节序列转化为类型 char 的 vector 的方法
//
// String 是一个结构体，由栈(24bytes)的空间保存指向堆空间的地址、容量、长度 ，
//        且在离开作用域的时候会调用 drop 方法来释放内存空间(类似析构函数)
//
// &str 是字符串片段(在运行时已知长度的文本)是无法修改的，一种比较特殊的数组片段(slice)，
//      由两个部分组成 内存地址和比特长度(多少bytes)

pub fn string_run() {

    // 通过字符串字面量构造 &str 类型的对象(不可变的)
    let s = "字符串片段";
    println!("s={}", s);
    
    // 创建字符串
    // let mut s1 = "字符串...".to_string(); // let mut s1 = "字符串...".to_owned()
    let mut s1 = String::from("字符串..."); // 在堆上分配内存
    println!("{:p}", s1.as_ptr()); // 打印堆内存的地址
    println!("{:p}", &s1);         // 打印变量 s1 在栈上的地址
    println!("{}", s1.len());      // 字符串在堆内存中的字节长度, 不是 字符长度

    // 追加字符串
    s1.push_str("追加...");
    // 拷贝(克隆)字符串（深拷贝）
    let s2 = s1.clone();
    println!("s1={}", s1);
    println!("s2={}", s2);
    // 浅拷贝（拷贝了指向堆空间的地址、长度、容量），并所有权转移到 s3，而 s2 的生命周期结束
    let s3 = s2;   // move
    //println!("{}", s2); // error
    println!("s3={}", s3);

    let mut s4 = String::from("操作其它的");
    let s4_1 = s4.pop().unwrap(); // 弹出(删除)最后一个字符
    println!("s4_1={}", s4_1);    // 的
    println!("s4={} [{}]", s4, s4.len()); // 操作其它

    println!("======================================");

    // 拼接字符串
    let hello = "Hello";
    let world = "World".to_string();
    let str_int = 100;

    let hello0 = hello.to_string() + "," + &world + "," + &str_int.to_string();
    let hello1 = format!("{},{},{}", hello, world, str_int);
    let hello2 = format!("{0},{1},{2}", hello, world, str_int);
    let hello3 = format!("{hello},{world},{str_int}");
    println!("{}", hello0);
    println!("{}", hello1);
    println!("{}", hello2);
    println!("{}", hello3);
    // 结果： Hello,World,100

    // 字符串转成数字
    let str_int: i64 = match String::from("a4").parse::<i64>() {
        Ok(num) => num,
        Err(e) => {
            println!("字符串转成数字失败：{}", &e);
            0
        },
    };
    println!("字符串转成数字 = {}", str_int);

    // 分隔字符串
    let str_split = "1,2,3,4,5".to_string();
    let str_split_array :Vec<&str> = str_split.split(",").collect();
    println!("分隔字符串1={:?}", str_split_array);
    // 结果：["1", "2", "3", "4", "5"]

    let str_split = "1 2    3\n4\n\r5\t6".to_string(); // 空格(空白)、换行符
    let str_split_array :Vec<&str> = str_split.split_whitespace().collect();
    println!("分隔字符串2={:?}", str_split_array);
    // 结果：["1", "2", "3", "4", "5", "6"]

    let str_split = "1 2    3\n4\n\r5\t6".to_string(); // 换行符
    let str_split_array :Vec<&str> = str_split.lines().collect();
    println!("分隔字符串3={:?}", str_split_array);
    // 结果：["1 2    3", "4", "\r5\t6"]

    // 拼接字符串(数组)
    let str_concat = ["你好", "，", "世界", "！"].concat();
    println!("拼接字符串1={}", str_concat);  // 结果：你好，世界！

    let str_join = ["a", "b", "c"].join(",");
    println!("拼接字符串2={}",str_join);     // 结果：a,b,c

    // trim_start() 去除字符串左边的空格
    // trim_end()   去除字符串末尾的空格
    // 去除字符串两侧的空白字符
    let str_trim = String::from(" EFG   \n\n\r\t");
    println!("去除字符串两侧的空白字符：[{}]", str_trim.trim()); // 结果：EFG

    // trim_start_matches() 去除字符串左边的指定字符
    // trim_end_matches()   去除字符串末尾的指定字符
    // 去除字符串两侧的指定字符
    let str_trim_matche = String::from(",去除字符串,");
    println!("去除字符串两侧的指定字符：[{}]", str_trim_matche.trim_matches(','));  // 结果：去除字符串

    // 转换字符串中的小写字母为大写
    let str_upper = "ab字母为大写cdEF".to_string();
    println!("转大写：{}", str_upper.to_uppercase()); // 结果：AB字母为大写CDEF

    // 转换字符串中所有大写字母为小写
    let str_lower = "AB字母为小写CDef";
    println!("转小写：{}", str_lower.to_lowercase()); // 结果：ab字母为小写cdef

    // 替换字符串
    let str_replace = String::from("aaa替换aaa字符串aaa");
    println!("替换字符串：{}", str_replace.replace("a", "")); // 结果：替换字符串

    // 判断字符串是否以 xxx 开头
    let str_starts_with = "开头abc开头123".to_string();
    println!("以什么开始 ：{}", str_starts_with.starts_with("开头")); // 结果：true
	
	// 判断字符串是否以 xxx 结尾
    let str_ends_with = "以abc结尾123结尾".to_string();
    println!("以什么结尾 ：{}", str_ends_with.ends_with("结尾")); // 结果：true

    // 查找到第一次出现的位置
    let str_find = "判断字符串是否包含".to_string();
    let str_find_option = str_find.find("包含");
    let mut str_find_i:i32 = -1;
    match str_find_option {
        Some(val) => {
            str_find_i = val as i32;
        },
        None => {
            println!("查找字符串失败");
        },
    }
    println!("查找字符串：{}", str_find_i);

    // 判断字符串是否包含 xxx
    println!("判断字符串是否包含：{}", str_find.contains("包含")); // 结果：true

    // 截取字符串
    let str_substr = String::from("0123456789");
    println!("截取字符串：{}", &str_substr[3..8]); // 结果：34567 (只适合 ASCII )

    // 生成重复字符串
    let str_repeat = "重复";
    println!("重复字符串:{}", str_repeat.repeat(3)); // 结果：重复重复重复

    // 是否为空字符串
    println!("是否为空字符串1：{} {}",  "".is_empty(), String::from("").is_empty());    // 结果：true true
    println!("是否为空字符串2：{} {}",  " ".is_empty(), String::from("  ").is_empty()); // 结果：false false

    // 将字符串转换为字节数组
    let str_data = String::from("将字符串转换为 字节数组");
    let str_bytes: &[u8] = str_data.as_bytes();
    // let str_bytes: Vec<u8> = str_data.as_bytes().to_owned();
    // let str_bytes: Vec<u8> = str_data.as_bytes().to_vec(); 
    // let str_bytes: Vec<u8> = str_data.bytes().collect::<Vec<u8>>();
    for (i, &item) in str_bytes.iter().enumerate() {
        if item == b' ' {
            println!("将字符串转换为字节数组:找到空格位置 {}，总长度 {}", i, str_bytes.len());
            // 结果：找到空格位置 21，总长度 34
        }
    }

    // 将字符串转换为类型为字符数组
    let str_data = String::from("将字符串转换为字符数组");
    let str_chars = str_data.chars().collect::<Vec<char>>();
    println!("将字符串转换为字符数组: 总长度 {}", str_chars.len()); // 结果：11
    // 由于 char 为 4 字节长，我们可以将其转化为 u32
    println!("将字符串转换为字符数组: {}", str_chars[0] as u32);

    // 将二进制字符串转字符串
    let str_byte_data = String::from("将二进制字符串转字符串").bytes().collect::<Vec<u8>>();
    let is_str = String::from_utf8(str_byte_data); // String::from_utf8_lossy(str_byte_data);
    println!("将二进制字符串转字符串: {:?}", is_str);
}
