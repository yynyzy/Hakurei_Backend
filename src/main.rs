use rocket::{
    catch, catchers, get, post, routes,
    serde::{
        json::{serde_json::json, Value},
        Deserialize, Serialize,
    },
};

mod auth;
mod router;

// use crate::module::Claims;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};

pub const KEY: &[u8] = b"secret";

// #[derive(Debug)]
// pub struct Token;
// // Bearer Token
// impl Token {
//     fn from_header(header: &str) -> Option<Token> {
//         let split_vec = header.split_whitespace().collect::<Vec<_>>();
//         if split_vec.len() != 2 {
//             return None;
//         }
//         if split_vec[0] != "Bearer" {
//             return None;
//         }
//         Self::from_jwt(split_vec[1])
//     }
//     fn from_jwt(token_string: &str) -> Option<Token> {
//         let mut val = Validation::default();
//         val.sub = Some("!Yg43#xQtBE357js".to_string());
//         match decode::<Claims>(token_string, &DecodingKey::from_secret(KEY), &val) {
//             Ok(c) => {
//                 println!("ExpTime:{:?}", c.claims.exp);
//                 return Some(Token);
//             }
//             Err(_) => None,
//         }
//     }
// }

// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for Token {
//     type Error = ();
//     async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
//         let header_auth = request.headers().get_one("Authorization");
//         if let Some(header_auth) = header_auth {
//             if let Some(auth) = Self::from_header(header_auth) {
//                 return Outcome::Success(auth);
//             }
//         }
//         Outcome::Error((Status::Unauthorized, ()))
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

fn get_token() -> String {
    let claims = Claims {
        sub: "990902".to_string(),
        company: "HAKUREI".to_string(),
        exp: 86400,
    };
    let token: String = match encode(&Header::default(), &claims, &EncodingKey::from_secret(KEY)) {
        Ok(t) => t,
        Err(_) => panic!(),
    };
    token
}
// struct BasicAuthStruct {
//     username: String,
//     password: String,
// }

// impl BasicAuthStruct {
//     fn from_header(header: &str) -> Option<BasicAuthStruct> {
//         let split_vec = header.split_whitespace().collect::<Vec<_>>();
//     }
// }

#[post("/login", data = "<a>")]
async fn login_user(a: String) -> Value {
    let token = get_token();
    json!({ "token": token})
}

#[catch(404)]
async fn not_found_url() -> Value {
    json!("url not found")
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rocket::build()
        .mount("/user", routes![login_user])
        // .mount("/", routes![router::get_token, router::get_token_test])
        .register("/", catchers![not_found_url])
        .launch()
        .await?;
    Ok(())
}
