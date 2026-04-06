mod utils;
mod models;
mod json;
mod mod_use;

fn main() {
    json::deserialize();
    json::serialize();

    println!("--------------------------------------------");

    mod_use::log();

    println!("--------------------------------------------");
    println!("Hello, world!");
}
