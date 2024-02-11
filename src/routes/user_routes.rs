use rocket::{get, http::Status, post, serde::json::Json};
use serde::Serialize;

use crate::{
    auth::auth,
    core::{common::constants, response::custom_response::CustomResponse},
    models::{
        self,
        user::{self, UserModel},
    },
    services::user_services,
};

#[derive(Serialize)]
pub struct ResponseTokenStruct {
    pub token: String,
}

#[get("/")]
pub async fn get_all_users(
    _auth_guard: auth::BasicAuth,
) -> CustomResponse<'static, Vec<UserModel>> {
    let pool = models::mysql_conn::get_db_conn_pool().await;
    let users = UserModel::find_all(&pool).await;
    CustomResponse::success(users)
}

#[post("/register", format = "application/json", data = "<register_params>")]
pub async fn register_user(
    register_params: Json<user::RegisterUserStruct>,
) -> Result<CustomResponse<'static, ResponseTokenStruct>, CustomResponse<'static, ()>> {
    // print!("{:?}", register_params);
    let pool = models::mysql_conn::get_db_conn_pool().await;
    let user = UserModel::find_by_username(&pool, &register_params.username).await;
    match user {
        Some(_) => Ok(CustomResponse::success(ResponseTokenStruct {
            token: auth::BasicAuth::get_token("67676916371637216371963"),
        })),
        None => Err(CustomResponse::error(
            Status::Conflict,
            constants::USER_EXITS,
        )),
    }
}

#[post("/login", format = "application/json", data = "<login_params>")]
pub async fn login_user(
    login_params: Json<user::LoginUserStruct>,
) -> CustomResponse<'static, ResponseTokenStruct> {
    print!("{:?}", login_params);
    if !user_services::find_user_by_basic_auth().await {
        // return json!({ "error": "没找到用户"});
    }
    // 在数据库中查询 用户是否存在，存在就返回她的 id， 将id 给生成 token；

    let token = auth::BasicAuth::get_token("67676916371637216371963");
    CustomResponse::success(ResponseTokenStruct { token })
}
