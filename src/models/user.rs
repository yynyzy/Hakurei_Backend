use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub salt: String,
    pub status: u8,
    pub avatar: Option<String>,
    pub deleted: u8,
    pub createdAt: String,
    pub updatedAt: String,
}

#[derive(Deserialize, Debug)]
pub struct RegisterUserStruct {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginUserStruct {
    pub username: String,
    pub password: String,
}
