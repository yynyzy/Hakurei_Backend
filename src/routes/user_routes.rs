use crate::{
    auth::auth,
    core::{
        common::constants::{self, USER_NOT_FOUND},
        db_manager::{mysql_conn, redis_manager::RedisManager},
        response::custom_response::CustomResponse,
    },
    models::user::{self, UserModel},
    services::user_services,
};
use r2d2::Pool;
use redis::Client;
use rocket::State;
use rocket::{get, http::Status, post, serde::json::Json};
use serde::Serialize;

#[get("/")]
pub async fn get_all_users(
    _auth_guard: auth::BasicAuth,
) -> CustomResponse<'static, Vec<UserModel>> {
    let pool = mysql_conn::get_db_conn_pool().await;
    let users = UserModel::find_all(&pool).await;
    CustomResponse::success(users)
}

#[post("/register", format = "application/json", data = "<register_params>")]
pub async fn register_user(
    register_params: Json<user::RegisterUserStruct>,
    redis_pool: &State<Pool<Client>>,
) -> Result<CustomResponse<'static, ResponseTokenStruct>, CustomResponse<'static, ()>> {
    let register_params = register_params.into_inner();
    let pool = mysql_conn::get_db_conn_pool().await;
    let user = UserModel::find_by_username(&pool, &register_params.username).await;
    if user.is_none() {
        let result = user_services::register_user(register_params).await;
        match result {
            Ok(v) => {
                let token = auth::BasicAuth::get_token(&v);
                RedisManager::set_value(&v, &token, redis_pool).await;
                Ok(CustomResponse::success(ResponseTokenStruct {
                    token: auth::BasicAuth::get_token(&v),
                }))
            }
            Err(_) => Err(CustomResponse::error(
                Status::InternalServerError,
                constants::CREATE_USER_EXITS,
            )),
        }
    } else {
        Err(CustomResponse::error(
            Status::Conflict,
            constants::USER_EXITS,
        ))
    }
}

#[post("/login", format = "application/json", data = "<login_params>")]
pub async fn login_user(
    login_params: Json<user::LoginUserStruct>,
    redis_pool: &State<Pool<Client>>,
) -> Result<CustomResponse<'static, ResponseTokenStruct>, CustomResponse<'static, ()>> {
    let pool = mysql_conn::get_db_conn_pool().await;
    let user = UserModel::find_by_username_and_password(
        &pool,
        &login_params.username,
        &login_params.password,
    )
    .await;
    match user {
        Some(v) => {
            let token = auth::BasicAuth::get_token(&v.id);
            RedisManager::set_value(&v.id, &token, redis_pool).await;
            Ok(CustomResponse::success(ResponseTokenStruct { token }))
        }
        None => Err(CustomResponse::error(Status::Unauthorized, USER_NOT_FOUND)),
    }
}

#[derive(Serialize)]
pub struct ResponseTokenStruct {
    pub token: String,
}
