//! JWT（JSON Web Token）签发与验证示例。
//!
//! ## 前置知识
//! JWT 由三部分组成（用 `.` 连接）：
//! ```
//! Header.Payload.Signature
//! ```
//! - **Header**   ：声明签名算法（如 HS256 / RS256）和 token 类型
//! - **Payload**  ：业务数据（Claims），标准字段有：
//!     - `sub`(subject)       主题，通常是用户 ID
//!     - `exp`(expiration)    过期时间（Unix 时间戳）
//!     - `iat`(issued at)     签发时间
//!     - `nbf`(not before)    在此时间前无效
//!     - `iss`(issuer)        签发者
//!     - `aud`(audience)      接收方
//! - **Signature**：Header + Payload 用密钥做 HMAC/RSA 签名，防篡改
//!
//! ## 练习题
//! 1. 在 Claims 中加 `iat` 和 `nbf` 字段，观察 Token 验证逻辑。
//! 2. 把 `secret` 改成错误的，观察 `decode` 报什么错误。
//! 3. 把系统时间调快 2 小时（或把 exp 设为过去），观察过期验证结果。

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Claims（载荷）：存储在 JWT 中的业务数据
#[derive(Debug, Serialize, Deserialize)]
struct AuthClaims {
    /// 主题 subject：用户名 / 用户 ID
    sub:  String,
    /// 自定义字段：角色
    role: String,
    /// 过期时间 expiration（Unix 时间戳，秒）
    exp:  usize,
    /// 签发时间 issued at
    iat:  usize,
}

fn main() {
    // ========== 1. 签发 Token (Encode) ==========
    let now_secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        // 免责：unwrap() 仅示例方便。生产中可：
        // - unwrap_or_default() 给默认 0；或 match 出 SystemTimeError 分支
        .unwrap()
        .as_secs();

    let claims = AuthClaims {
        sub:  "user_123".to_owned(),
        role: "admin".to_owned(),
        exp:  (now_secs + 3600) as usize, // 1 小时有效期
        iat:  now_secs as usize,
    };

    // ⚠️ 生产中密钥不要写在代码里！请从环境变量 / 配置文件加载
    //      对称加密（HS256）用同一个密钥签发 + 验证；非对称（RS256/ES256）用私钥签发公钥验证
    let secret = b"super_secret_key_please_change_in_production";

    // 默认 Header 用 HS256 算法
    let header = Header::default();
    let token = encode(&header, &claims, &EncodingKey::from_secret(secret))
        // 免责：unwrap() 仅示例方便；生产中这里可能因序列化失败返回 Err
        .expect("JWT 签发失败");
    println!("✅ 签发成功 JWT: {}\n", token);

    // ========== 2. 验证 Token (Decode) ==========

    // ---- 正常验证：正确的密钥 + 未过期 ----
    println!("======= 正常验证（正确密钥）=======");
    match decode::<AuthClaims>(
        &token,
        &DecodingKey::from_secret(secret),
        &Validation::new(Algorithm::HS256), // 显式指定算法，避免算法混淆攻击
    ) {
        Ok(data) => {
            // data.header  是解析后的 Header
            // data.claims  是我们自定义的业务数据
            println!(
                "🎉 Token 验证通过: 用户名={}, 角色={}, 签发时间戳={}",
                data.claims.sub, data.claims.role, data.claims.iat
            );
        }
        Err(e) => {
            // jsonwebtoken::errors::Error 有明确的错误分类：
            // - ExpiredSignature → 过期
            // - InvalidSignature → 签名错误（密钥不对/被篡改）
            // - InvalidToken     → 格式错误
            // - ImmatureSignature → nbf 未到生效时间
            println!("❌ Token 验证失败: {}", e);
        }
    }

    // ---- 错误演示 1：用错误密钥验证（模拟被篡改或用错环境）----
    println!("\n======= 错误密钥验证（演示错误分支）=======");
    let wrong_key = b"wrong_secret_!!!".as_slice();
    match decode::<AuthClaims>(
        &token,
        &DecodingKey::from_secret(wrong_key),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(_) => println!("❓ 理论上不会通过"),
        Err(e) => println!("✅ 如预期失败（密钥错误）: {}", e),
    }

    // ---- 错误演示 2：已过期 Token ----
    println!("\n======= 过期 Token 验证（演示错误分支）=======");
    let expired_claims = AuthClaims {
        sub:  "user_expired".to_owned(),
        role: "guest".to_owned(),
        exp:  (now_secs - 3600) as usize, // 1 小时前过期
        iat:  (now_secs - 7200) as usize,
    };
    let expired_token = encode(
        &Header::default(),
        &expired_claims,
        &EncodingKey::from_secret(secret),
    )
    .unwrap(); // 免责：同上

    match decode::<AuthClaims>(
        &expired_token,
        &DecodingKey::from_secret(secret),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(_) => println!("❓ 理论上不会通过"),
        Err(e) => println!("✅ 如预期失败（Token 过期）: {}", e),
    }
}
