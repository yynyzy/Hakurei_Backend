use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// 我们的声言结构型, 需要由`Serialize` 或 `Deserialize`派生
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,
    exp: usize,  // 必须。 (validate_exp 在验证中默认为真值)。截止时间 (UTC 时间戳)
    iat: usize,  // 可选。 发布时间 (UTC 时间戳)
    iss: String, // 可选。 发布者
    nbf: usize,  // 可选。 不早于 (UTC 时间戳)
    sub: String, // 可选。 标题 (令牌指向的人)
}
