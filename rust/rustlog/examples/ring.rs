use ring::digest;
use data_encoding::HEXUPPER;

// 加密
// ring SHA256 SHA384 SHA512

fn main() {
    sha256_384_512();
    blake3();
}

fn sha256_384_512() {
    let password = "123456".as_bytes();
    let sha256_hash = digest::digest(&digest::SHA256, password);
    println!("sha256 -> 123456 -> : {:?}", HEXUPPER.encode(sha256_hash.as_ref()));

    let sha384_hash = digest::digest(&digest::SHA384, password);
    println!("sha384 -> 123456 -> : {:?}", HEXUPPER.encode(sha384_hash.as_ref()));

    let sha512_hash = digest::digest(&digest::SHA512, password);
    println!("sha512 -> 123456 -> : {:?}", HEXUPPER.encode(sha512_hash.as_ref()));
}

fn blake3() {
    let password = b"123456";
    let blake3_hash = blake3::hash(password);
    println!("blake3 -> 123456 -> : {:?}", blake3_hash.to_hex());
}


