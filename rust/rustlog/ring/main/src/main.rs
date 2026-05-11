use ring::digest;

fn sha256() {
    let password = "123456";
    let sha256_hash = digest::digest(&digest::SHA256, password.as_bytes());
    println!("sha256 -> 123456 -> : {:?}", sha256_hash);
}

fn main() {
    sha256()
}
