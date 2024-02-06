use rocket::{
    serde::{Deserialize, Serialize},
    FromForm,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterUserStruct {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginUserStruct {
    pub username: String,
    pub password: String,
}
