// use crate::auth::{Token, KEY};
// use crate::module::{Claims, UserAuth};
// use jsonwebtoken::{encode, EncodingKey, Header};
// use rocket::{
//     get, post,
//     serde::json::{serde_json::json, Json, Value},
// };
// use std::time::{SystemTime, UNIX_EPOCH};

// // get token
// #[post("/token", format = "json", data = "<user_auth>")]
// pub async fn get_token(user_auth: Json<UserAuth>) -> Value {
//     let user = user_auth.into_inner();
//     if user.id == 0 && user.key.eq("oR66T*W8y4VaXkh#rTjeZ$$Rby$NCy!nJX") {
//         let timestamp = SystemTime::now()
//             .duration_since(UNIX_EPOCH)
//             .unwrap()
//             .as_secs();
//         let token = match encode(
//             &Header::default(),
//             &Claims {
//                 sub: String::from("!Yg43#xQtBE357js"),
//                 exp: timestamp + 5,
//             },
//             &EncodingKey::from_secret(KEY),
//         ) {
//             Ok(t) => t,
//             Err(_) => panic!(),
//         };
//         json!({ "token": token })
//     } else {
//         json!({"token": "Auth Fail"})
//     }
// }

// // get token test
// #[get("/token/test")]
// pub async fn get_token_test(_auth: Token) -> Value {
//     print!("{:?}", _auth);
//     json!({"status":"Auth Success"})
// }
