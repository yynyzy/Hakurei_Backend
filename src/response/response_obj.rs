use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct ResponseTokenStruct {
    pub token: String,
}
