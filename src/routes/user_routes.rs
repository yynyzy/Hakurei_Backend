use std::ops::Deref;

use crate::{
    auth::auth,
    core::{
        common::constants::{self, INTERNAL_SERVER_ERROR, USER_NOT_FOUND},
        db_manager::{mysql_manager, redis_manager::RedisManager},
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
) -> Result<CustomResponse<'static, Vec<UserModel>>, CustomResponse<'static, ()>> {
    let pool = mysql_manager::get_db_conn_pool().await;
    let users = UserModel::find_all(&pool).await;
    if let Some(users) = users {
        return Ok(CustomResponse::success(users));
    };
    Err(CustomResponse::error(
        Status::InternalServerError,
        INTERNAL_SERVER_ERROR,
    ))
}

#[post("/register", format = "application/json", data = "<register_params>")]
pub async fn register_user(
    register_params: Json<user::RegisterUserStruct>,
    redis_pool: &State<Pool<Client>>,
) -> Result<CustomResponse<'static, ResponseTokenStruct>, CustomResponse<'static, ()>> {
    let register_params = register_params.into_inner();
    let pool = mysql_manager::get_db_conn_pool().await;
    let user = UserModel::find_by_username(&pool, &register_params.username).await;
    if let Some(_) = user {
        return Err(CustomResponse::error(
            Status::Conflict,
            constants::USER_EXITS,
        ));
    } else {
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
            constants::CREATE_USER_FAILED,
        ))
    }
}

#[post("/login", format = "application/json", data = "<login_params>")]
pub async fn login_user(
    login_params: Json<user::LoginUserStruct>,
    redis_pool: &State<Pool<Client>>,
) -> Result<CustomResponse<'static, ResponseTokenStruct>, CustomResponse<'static, ()>> {
    let pool = mysql_manager::get_db_conn_pool().await;
    let user = UserModel::find_by_username_and_password(
        &pool,
        &login_params.username,
        &login_params.password,
    )
    .await;
    if let Some(user) = user {
        let token = auth::BasicAuth::get_token(&user.id);
        RedisManager::set_value(&user.id, &token, redis_pool.deref())
            .await
            .unwrap();
        return Ok(CustomResponse::success(ResponseTokenStruct { token }));
    }
    Err(CustomResponse::error(Status::Unauthorized, USER_NOT_FOUND))
}

#[derive(Serialize)]
pub struct ResponseTokenStruct {
    pub token: String,
}
