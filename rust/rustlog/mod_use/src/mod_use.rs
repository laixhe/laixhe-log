use crate::utils;
use crate::models;

pub fn log(){
    utils::greet("laixhe");
    let user = models::User::new("laixhe", 18);
    user.print_info();
}