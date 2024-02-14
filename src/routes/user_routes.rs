use std::ops::Deref;

use crate::{
    auth::auth,
    core::{
        common::constants::{self},
        config::custom_response::CustomResponse,
        db_manager::redis_manager::RedisManager,
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
) -> Result<CustomResponse<Vec<UserModel>>, CustomResponse<()>> {
    let users = UserModel::find_all().await;
    if let Some(users) = users {
        return Ok(CustomResponse::success(users));
    };
    Err(CustomResponse::error(
        Status::InternalServerError,
        constants::INTERNAL_SERVER_ERROR.to_owned(),
    ))
}

#[post("/register", format = "application/json", data = "<register_params>")]
pub async fn register_user(
    register_params: Json<user::RegisterUserStruct>,
    redis_pool: &State<Pool<Client>>,
) -> Result<CustomResponse<ResponseTokenStruct>, CustomResponse<()>> {
    let register_params = register_params.into_inner();
    let user = UserModel::find_by_username(&register_params.username).await;
    if let Some(_) = user {
        return Err(CustomResponse::error(
            Status::Conflict,
            constants::USER_EXITS.to_owned(),
        ));
    }
    let v = user_services::register_user(register_params).await;
    if let Ok(v) = v {
        let token = auth::BasicAuth::get_token(&v);
        RedisManager::set_value(&v, &token, redis_pool.deref())
            .await
            .unwrap();
        return Ok(CustomResponse::success(ResponseTokenStruct {
            token: auth::BasicAuth::get_token(&v),
        }));
    }
    Err(CustomResponse::error(
        Status::InternalServerError,
        constants::CREATE_USER_FAILED.to_owned(),
    ))
}

#[post("/login", format = "application/json", data = "<login_params>")]
pub async fn login_user(
    login_params: Json<user::LoginUserStruct>,
    redis_pool: &State<Pool<Client>>,
) -> Result<CustomResponse<ResponseTokenStruct>, CustomResponse<()>> {
    let user =
        UserModel::find_by_username_and_password(&login_params.username, &login_params.password)
            .await;
    if let Some(user) = user {
        let token = auth::BasicAuth::get_token(&user.id);
        RedisManager::set_value(&user.id, &token, redis_pool.deref())
            .await
            .unwrap();
        return Ok(CustomResponse::success(ResponseTokenStruct { token }));
    }
    Err(CustomResponse::error(
        Status::Unauthorized,
        constants::USER_NOT_FOUND.to_owned(),
    ))
}

#[derive(Serialize)]
pub struct ResponseTokenStruct {
    pub token: String,
}
