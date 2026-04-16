mod utils;
mod models;
mod mod_use;

fn main() {
    mod_use::log();

    println!("--------------------------------------------");

    utils::greet("laixhe");
    let user = models::User::new("laixhe", 18);
    user.print_info();
}
