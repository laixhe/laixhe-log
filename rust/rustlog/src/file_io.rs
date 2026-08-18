//! 文件读写：std::fs 文件操作（对应 Python 文件操作 / Go os 包 / C# File）。
//!
//! ## 前置知识
//! - 读写函数都返回 `Result`，用 `?` 或 `match` 处理错误
//! - `Path` / `PathBuf` 处理路径（对应 Python os.path / Go filepath）
//! - 演示统一使用 `std::env::temp_dir()` 下的临时目录，避免污染项目
//!
//! ## 练习题
//! 1. 写入文件后追加一行，再读出来验证内容。
//! 2. 遍历一个目录，打印所有文件名。

use std::fs;
use std::io::{BufRead, Write};
use std::path::PathBuf;

// 生成测试用的临时文件路径（每个函数独立目录，避免冲突）
fn temp_path(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("rustlog_demo_{}", name));
    let _ = fs::create_dir_all(&dir);
    dir.join("data.txt")
}

// ============ 整体读写 ============
pub fn read_write() {
    let path = temp_path("read_write");

    // 写文件：fs::write（对应 Python open().write()）
    fs::write(&path, "hello\nworld\n").expect("写入失败");
    println!("写入成功: {}", path.display());

    // 读文件：fs::read_to_string（对应 Python open().read()）
    let content = fs::read_to_string(&path).expect("读取失败");
    println!("读出内容: {:?}", content); // "hello\nworld\n"

    // 判断文件是否存在（对应 Python os.path.exists）
    println!("文件存在: {}", path.exists()); // true

    // 追加写入：OpenOptions（对应 Python open(..., "a")）
    let mut f = fs::OpenOptions::new()
        .append(true)
        .open(&path)
        .expect("打开失败");
    f.write_all(b"appended\n").expect("追加失败");
    let after = fs::read_to_string(&path).unwrap();
    println!("追加后: {:?}", after); // "hello\nworld\nappended\n"

    // 删除文件（对应 Python os.remove）
    fs::remove_file(&path).expect("删除失败");
    println!("删除后存在: {}", path.exists()); // false
}

// ============ 逐行读取 ============
pub fn read_lines() {
    let path = temp_path("read_lines");
    fs::write(&path, "go\nrust\njava\n").unwrap();

    // BufRead 逐行读取（对应 Go bufio.Scanner / Python for line in f）
    let f = fs::File::open(&path).unwrap();
    let reader = std::io::BufReader::new(f);

    print!("逐行读取: ");
    for line in reader.lines() {
        print!("{} ", line.unwrap()); // go rust java
    }
    println!();

    let _ = fs::remove_file(&path);
}

// ============ 目录操作 ============
pub fn dir_ops() {
    let base = std::env::temp_dir().join("rustlog_demo_dir_ops");
    let _ = fs::remove_dir_all(&base); // 清理上一次残留
    fs::create_dir_all(&base.join("sub")).unwrap(); // 递归创建

    // 写入几个文件
    fs::write(base.join("1.txt"), "").unwrap();
    fs::write(base.join("2.txt"), "").unwrap();

    // 遍历目录（对应 Python os.listdir / Go os.ReadDir）
    print!("目录内容: ");
    for entry in fs::read_dir(&base).unwrap() {
        let entry = entry.unwrap();
        print!("{} ", entry.file_name().to_string_lossy());
    }
    println!();

    // 重命名（对应 Python os.rename）
    fs::rename(base.join("1.txt"), base.join("renamed.txt")).unwrap();
    println!("重命名后 1.txt 存在: {}", base.join("1.txt").exists()); // false

    let _ = fs::remove_dir_all(&base);
}

// ============ 练习题参考答案 ============
#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Write;

    // 练习 1：写入 → 追加 → 读出验证
    #[test]
    fn exercise_1_write_append_read() {
        let dir = std::env::temp_dir().join("rustlog_exercise_1");
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("data.txt");

        fs::write(&path, "line1\n").unwrap();
        let mut f = fs::OpenOptions::new().append(true).open(&path).unwrap();
        f.write_all(b"line2\n").unwrap();

        assert_eq!(fs::read_to_string(&path).unwrap(), "line1\nline2\n");
        let _ = fs::remove_dir_all(&dir);
    }

    // 练习 2：遍历目录打印文件名
    #[test]
    fn exercise_2_list_dir() {
        let dir = std::env::temp_dir().join("rustlog_exercise_2");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("a.txt"), "").unwrap();
        fs::write(dir.join("b.txt"), "").unwrap();

        let names: Vec<String> = fs::read_dir(&dir)
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        assert_eq!(names.len(), 2);
        let _ = fs::remove_dir_all(&dir);
    }
}
