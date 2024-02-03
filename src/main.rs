use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rocket::{
    catch, catchers, get,
    http::Status,
    post,
    request::{FromRequest, Outcome},
    routes,
    serde::{
        json::{serde_json::json, Json, Value},
        Deserialize, Serialize,
    },
};

mod handlers;
mod models;
mod routes;
mod services;
mod utils;
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

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rocket::build()
        .mount("/user", routes![routes::user_routes::login_user])
        // .mount("/", routes![router::get_token, router::get_token_test])
        .register("/", catchers![routes::error_routes::not_found_url])
        .launch()
        .await?;
    Ok(())
}
