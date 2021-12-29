mod rwfile;
mod string;

use rwfile::write_read;

fn main(){
    write_read::rwfile::fs_write_to();
    let result = write_read::rwfile::fs_read_write();
    println!("{:?}", result);

    string::string_run();
}
