mod array_map;
mod char_string;
mod number;

fn main() {
    println!("============= number::convert =============");
    number::convert();

    println!("============= number::overflow =============");
    number::overflow();

    println!("============= char_string::std_char =============");
    char_string::std_char();

    println!("============= char_string::convert =============");
    char_string::convert();

    println!("============= array_map::std_array =============");
    array_map::std_array();

    println!("============= array_map::std_tuple =============");
    array_map::std_tuple();

    println!("============= array_map::std_vec =============");
    array_map::std_vec();

    println!("============= array_map::std_vec_deque =============");
    array_map::std_vec_deque();

    println!("============= array_map::std_hash_map =============");
    array_map::std_hash_map();
}
