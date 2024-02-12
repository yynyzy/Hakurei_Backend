use crate::core::db_manager::redis_manager::RedisManager;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    serde::{Deserialize, Serialize},
    Request,
};

pub const KEY: &[u8] = b"secret";

#[derive(Debug, Serialize, Deserialize)]

pub struct BasicAuth {
    sub: String,
    pub exp: i64,
}

impl BasicAuth {
    async fn from_request(header: &str) -> Result<BasicAuth, ()> {
        let split_vec = header.split_whitespace().collect::<Vec<_>>();
        if split_vec.len() != 2 {
            return Err(());
        }
        if split_vec[0] != "Bearer" {
            return Err(());
        }
        let token = split_vec[1];
        let jwt_decrypt = Self::from_jwt(token);
        match jwt_decrypt {
            Some(v) => {
                let redis_pool = RedisManager::redis_conn().await;
                let result = RedisManager::get_value(&v.sub, redis_pool).await;
                println!("id:{:?}", result);
                match result {
                    Ok(v2) => {
                        println!("a === {}", v2);
                        if v2 == token {
                            Ok(v)
                        } else {
                            Err(())
                        }
                    }
                    Err(_) => Err(()),
                }
            }
            None => Err(()),
        }
    }

    fn from_jwt(token_string: &str) -> Option<BasicAuth> {
        match decode::<BasicAuth>(
            token_string,
            &DecodingKey::from_secret(KEY),
            &Validation::default(),
        ) {
            Ok(c) => {
                return Some(c.claims);
            }
            Err(_) => None,
        }
    }

    // 根据 userId 生成 token
    pub fn get_token(user_id: &str) -> String {
        let exp = Utc::now() + Duration::hours(24);
        let basic_auth = BasicAuth {
            sub: user_id.to_string(),
            exp: exp.timestamp(),
        };
        let token: String = encode(
            &Header::default(),
            &basic_auth,
            &EncodingKey::from_secret(KEY),
        )
        .unwrap();
        token
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let base_auth = request.headers().get_one("Authorization");
        if let Some(base_auth) = base_auth {
            if let Ok(auth) = Self::from_request(base_auth).await {
                return Outcome::Success(auth);
            }
        }
        Outcome::Error((Status::Unauthorized, ()))
    }
}
