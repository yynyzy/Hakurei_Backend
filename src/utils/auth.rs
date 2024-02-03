use jsonwebtoken::{encode, EncodingKey, Header};
use rocket::serde::{Deserialize, Serialize};

pub const KEY: &[u8] = b"secret";

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

pub fn get_token(user_id: &str) -> String {
    let claims = Claims {
        sub: user_id.to_string(),
        company: "HAKUREI".to_string(),
        exp: 86400,
    };
    let token: String = match encode(&Header::default(), &claims, &EncodingKey::from_secret(KEY)) {
        Ok(t) => t,
        Err(_) => panic!(),
    };
    token
}
