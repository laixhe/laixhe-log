// 定义模块(公共的)
pub mod rwfile {
    use std::fs;
    use std::fs::File;
    use std::fs::OpenOptions;
    use std::io;
    use std::io::Read;
    use std::io::Write;
    use std::io::ErrorKind;

    // 定义模块的函数(公共的)

    // 打开文件 没有该文件就 创建文件
    pub fn open_create(){
        let file_txt = "./FileWriteRead.txt";

        // File::open   只读 打开文件
        // File::create 可写 打开文件 - 如果文件存在则清空旧内容 - 如果文件不存在则新建

        // let f = match File::open(file_txt) {
        //     Ok(file_open) => file_open,
        //     Err(err) => match err.kind(){
        //         ErrorKind::NotFound => match File::create(file_txt) {
        //             Ok(file_create) => file_create,
        //             Err(err_create) => panic!("Error creating file: {:?}", err_create),
        //         },
        //         other_error => panic!("Error opening the file: {:?}", other_error),
        //     }
        // };

        // 下面的 比 上面的 简洁多了

        let file = File::open(file_txt).unwrap_or_else(|error|{
            if error.kind() == ErrorKind::NotFound {
                File::create(file_txt).unwrap_or_else(|error|{
                    panic!("Error creating file: {:?}", error);
                })
            } else {
                panic!("Error opening the file: {:?}", error);
            }
        });

    }

    // 一次性读取文件全部数据
    pub fn fs_read_to() -> Result<String, io::Error> {
        let file_txt = "./FileWriteRead.txt";

        // let open_file_txt = File::open(file_txt);
        // let mut f = match open_file_txt {
        //     Ok(file) => file,
        //     Err(err) => return Err(err),
        // };

        // let mut s = String::new();
        // match f.read_to_string(&mut s) {
        //     Ok(_) => Ok(s),
        //     Err(err) => Err(err),
        // }

        // 下面的 比 上面的 简洁多了

        // 使用了 ? 运算符，只能用于返回 Result 的函数

        // let mut f = File::open(file_txt)?;
        // let mut s = String::new();
        // f.read_to_string(&mut s)?;
        // Ok(s)

        // 下面的 比 上面的 更简洁多了
        
        // let mut s = String::new();
        // File::open(file_txt)?.read_to_string(&mut s)?;
        // Ok(s)

        // 下面的 比 上面的 更简洁多了

        fs::read_to_string(file_txt)
        // fs::read("D:\\text.txt") // 读取的文件是二进制文件
    }

    // 一次性写入文件全部数据 - 是以覆盖形式 - 如果文件不存在就会创建文件
    pub fn fs_write_to(){
        let file_txt = "./FileWriteRead.txt";

        let is_write = fs::write(file_txt, "//1234567890//文件,");
        match is_write {
            Ok(v) => {
                println!("fs_write_to = {:?}", v);
            },
            Err(e) => {
                println!("fs_write_to error = {}", &e);
            },
        }
        
    }

    // OpenOptions 是一个灵活的打开文件的方法，它可以设置打开权限，除 append 权限以外还有 read 权限和 write 权限
    pub fn fs_read_write() -> io::Result<()> {
        let file_txt = "./FileWriteRead.txt";

        let mut file = OpenOptions::new().read(true).write(true).append(true).open(file_txt)?;

        let mut buffer = [0u8; 5];
        loop {
            match file.read(&mut buffer) {
                Ok(v) => {
                    println!("fs_read_write read len={}", v);
                    if v > 0 && v <= 5 {
                        //println!("fs_read_write read = {:?}", buffer[0..v]);
                    }
                    if v < 5 {
                        break;
                    }
                    
                    
                },
                Err(e) => {
                    return Err(e);
                },
            }
        }

        file.write(b"//1234567890//,")?; // b"" 只支持 ASCII
        file.write("//1234567890//文件,".as_bytes())?;
        file.write_all("//1234567890//文件,".as_bytes())?;

        Ok(())
        
    }

    // 从文件系统中删除某个文件
    pub fn fs_remove(){
        let file_txt = "./FileWriteRead.txt";

        let is_remove = std::fs::remove_file(file_txt);
        match is_remove {
            Ok(v) => {
                println!("fs_remove = {:?}", v);
            },
            Err(e) => {
                println!("fs_remove error = {}", &e);
            },
        }
    }
}