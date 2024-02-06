use rocket::{catchers, routes};

mod models;
mod response;
mod routes;
mod services;
mod utils;

use routes::user_routes;
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
    models::mysql_conn::get_db_conn();
    rocket::build()
        .mount(
            "/user",
            routes![user_routes::login_user, user_routes::register_user,],
        )
        .register("/", catchers![routes::error_routes::not_found_url])
        .launch()
        .await?;
    Ok(())
}
