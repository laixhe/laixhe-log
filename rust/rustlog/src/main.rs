//! RustLog 主入口：串联所有基础类型、集合与迭代器专题的示例模块。

mod array_map;
mod basic;
mod char_string;
mod concurrency;
mod control_flow;
mod error;
mod file_io;
mod function;
mod generic_trait;
mod iterators;
mod number;
mod ownership;
mod pattern;
mod struct_enum;

fn main() {
    // ===== 基础语法 =====
    println!("============= basic::basic_types =============");
    basic::basic_types();

    println!("============= basic::variables =============");
    basic::variables();

    println!("============= basic::consts =============");
    basic::consts();

    println!("============= basic::shadowing =============");
    basic::shadowing();

    println!("============= control_flow::if_else =============");
    control_flow::if_else();

    println!("============= control_flow::loops =============");
    control_flow::loops();

    println!("============= control_flow::match_pattern =============");
    control_flow::match_pattern();

    println!("============= function::basics =============");
    function::basics();

    println!("============= function::multi_return =============");
    function::multi_return();

    println!("============= function::closures =============");
    function::closures();

    println!("============= struct_enum::struct_basic =============");
    struct_enum::struct_basic();

    println!("============= struct_enum::struct_method =============");
    struct_enum::struct_method();

    println!("============= struct_enum::enums =============");
    struct_enum::enums();

    println!("============= struct_enum::option_enum =============");
    struct_enum::option_enum();

    println!("============= ownership::move_semantics =============");
    ownership::move_semantics();

    println!("============= ownership::borrow =============");
    ownership::borrow();

    println!("============= ownership::borrow_compare =============");
    ownership::borrow_compare();

    println!("============= pattern::destructure =============");
    pattern::destructure();

    println!("============= pattern::at_binding =============");
    pattern::at_binding();

    println!("============= pattern::match_guard =============");
    pattern::match_guard();

    println!("============= pattern::matches_macro =============");
    pattern::matches_macro();

    println!("============= pattern::if_let_destructure =============");
    pattern::if_let_destructure();

    println!("============= generic_trait::generic_function =============");
    generic_trait::generic_function();

    println!("============= generic_trait::generic_struct =============");
    generic_trait::generic_struct();

    println!("============= generic_trait::traits =============");
    generic_trait::traits();

    println!("============= generic_trait::trait_bounds =============");
    generic_trait::trait_bounds();

    println!("============= error::result_basic =============");
    error::result_basic();

    println!("============= error::question_mark =============");
    error::question_mark();

    println!("============= error::custom_error =============");
    error::custom_error();

    println!("============= error::panic_demo =============");
    error::panic_demo();

    println!("============= file_io::read_write =============");
    file_io::read_write();

    println!("============= file_io::read_lines =============");
    file_io::read_lines();

    println!("============= file_io::dir_ops =============");
    file_io::dir_ops();

    println!("============= concurrency::threads =============");
    concurrency::threads();

    println!("============= concurrency::thread_capture =============");
    concurrency::thread_capture();

    println!("============= concurrency::mutex_demo =============");
    concurrency::mutex_demo();

    println!("============= concurrency::channel_demo =============");
    concurrency::channel_demo();

    // ===== number：数值类型 =====
    println!("============= number::number_to_string =============");
    number::number_to_string();

    println!("============= number::overflow =============");
    number::overflow();

    println!("============= number::type_conversion =============");
    number::type_conversion();

    // ===== char_string：字符与字符串 =====
    println!("============= char_string::std_char =============");
    char_string::std_char();

    println!("============= char_string::std_string =============");
    char_string::std_string();

    println!("============= char_string::string_vs_str =============");
    char_string::string_vs_str();

    println!("============= char_string::string_parse =============");
    char_string::string_parse();

    // ===== array_map：集合类型 =====
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

    println!("============= array_map::std_btree_map =============");
    array_map::std_btree_map();

    println!("============= array_map::std_hash_set =============");
    array_map::std_hash_set();

    println!("============= array_map::std_btree_set =============");
    array_map::std_btree_set();

    println!("============= array_map::std_binary_heap =============");
    array_map::std_binary_heap();

    // ===== iterators：⭐ 迭代器专题 =====
    println!("============= iterators::basics =============");
    iterators::basics();

    println!("============= iterators::adapters =============");
    iterators::adapters();

    println!("============= iterators::consumers =============");
    iterators::consumers();

    println!("============= iterators::practice (综合实战) =============");
    iterators::practice();
}
