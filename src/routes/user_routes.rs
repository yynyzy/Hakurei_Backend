use rocket::{get, http::Status, post, serde::json::Json};
use serde::Serialize;

use crate::{
    models::{
        self,
        user::{self, UserModel},
    },
    response::api_response::ApiResponse,
    services::user_services,
    utils::auth,
};

#[derive(Serialize)]
pub struct ResponseTokenStruct {
    pub token: String,
}

#[get("/")]
pub async fn get_all_users(_auth_guard: auth::BasicAuth) -> ApiResponse<'static, Vec<UserModel>> {
    let pool = models::mysql_conn::get_db_conn_pool().await;
    let users = UserModel::find_all(&pool).await;
    ApiResponse::success(users)
}

#[post("/register", format = "application/json", data = "<register_params>")]
pub async fn register_user(
    register_params: Json<user::RegisterUserStruct>,
) -> Result<ApiResponse<'static, ResponseTokenStruct>, ApiResponse<'static, ()>> {
    // print!("{:?}", register_params);
    let pool = models::mysql_conn::get_db_conn_pool().await;
    let user = UserModel::find_by_username(&pool, &register_params.username).await;
    match user {
        Some(_) => Ok(ApiResponse::success(ResponseTokenStruct {
            token: auth::BasicAuth::get_token("67676916371637216371963"),
        })),
        None => Err(ApiResponse::error(Status::Conflict, "user already exits")),
    }
}

#[post("/login", format = "application/json", data = "<login_params>")]
pub async fn login_user(
    login_params: Json<user::LoginUserStruct>,
) -> ApiResponse<'static, ResponseTokenStruct> {
    print!("{:?}", login_params);
    if !user_services::find_user_by_basic_auth().await {
        // return json!({ "error": "没找到用户"});
    }
    // 在数据库中查询 用户是否存在，存在就返回她的 id， 将id 给生成 token；

    let token = auth::BasicAuth::get_token("67676916371637216371963");
    ApiResponse::success(ResponseTokenStruct { token })
}
