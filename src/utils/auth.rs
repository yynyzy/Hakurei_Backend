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
    fn from_request(header: &str) -> Option<BasicAuth> {
        let split_vec = header.split_whitespace().collect::<Vec<_>>();
        if split_vec.len() != 2 {
            return None;
        }
        if split_vec[0] != "Bearer" {
            return None;
        }
        Self::from_jwt(split_vec[1])
    }

    fn from_jwt(token_string: &str) -> Option<BasicAuth> {
        match decode::<BasicAuth>(
            token_string,
            &DecodingKey::from_secret(KEY),
            &Validation::default(),
        ) {
            Ok(c) => {
                println!("ExpTime:{:?}", c);
                return Some(c.claims);
            }
            Err(_) => None,
        }
    }

    pub fn get_token(user_id: &str) -> String {
        // let exp = Utc::now() + Duration::seconds(86400);
        let exp = Utc::now() + Duration::hours(24);
        let basic_auth = BasicAuth {
            sub: user_id.to_string(),
            exp: exp.timestamp(), // exp: Duration::seconds(86400),
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
            if let Some(auth) = Self::from_request(base_auth) {
                print!("base_auth {:?} ", auth);
                return Outcome::Success(auth);
            }
        }
        Outcome::Error((Status::Unauthorized, ()))
    }
}
