mod utils;
mod models;
mod json;
mod mod_use;
mod date_time;

fn main() {
    json::deserialize();
    json::serialize();

    println!("--------------------------------------------");

    mod_use::log();

    println!("--------------------------------------------");

    date_time::std_time();

    println!("--------------------------------------------");
    println!("Hello, world!");
}
