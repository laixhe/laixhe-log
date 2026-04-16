use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
struct AuthClaims {
    sub: String,
    role: String,
    exp: usize,
}

fn main() {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    let claims = AuthClaims {
        sub: "user_123".to_owned(),
        role: "admin".to_owned(),
        exp: (now + 3600) as usize, // 1小时有效期
    };

    let secret = b"super_secret_key";
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret)
    ).unwrap();

    println!("Generated JWT: {}", token);
}
