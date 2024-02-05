use rocket::{post, serde::json::Json};

use crate::{
    models::user,
    response::{api_response::ApiResponse, response_obj},
    services::user_services,
    utils::auth,
};

#[post("/register", format = "application/json", data = "<register_params>")]
pub async fn register_user(
    register_params: Json<user::RegisterUserStruct>,
    _auth_guard: auth::BasicAuth,
) -> ApiResponse<'static, response_obj::ResponseTokenStruct> {
    print!("{:?}", register_params);
    print!("auth_guard {:?}", _auth_guard);
    // if !user_services::find_user_by_basic_auth().await {
    //     // return json!({ "error": "没找到用户"});
    // }
    let token = auth::BasicAuth::get_token("67676916371637216371963");
    ApiResponse::success(response_obj::ResponseTokenStruct { token })
}

#[post("/login", format = "application/json", data = "<login_params>")]
pub async fn login_user(
    login_params: Json<user::LoginUserStruct>,
) -> ApiResponse<'static, response_obj::ResponseTokenStruct> {
    // print!("{:?}", login_params);
    if !user_services::find_user_by_basic_auth().await {
        // return json!({ "error": "没找到用户"});
    }
    // 在数据库中查询 用户是否存在，存在就返回她的 id， 将id 给生成 token；

    let token = auth::BasicAuth::get_token("67676916371637216371963");
    ApiResponse::success(response_obj::ResponseTokenStruct { token })
}
