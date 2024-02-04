use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct RegisterUserStruct {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct LoginUserStruct {
    pub username: String,
    pub password: String,
}
