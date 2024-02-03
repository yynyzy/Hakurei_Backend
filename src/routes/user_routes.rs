use rocket::{
    post,
    serde::{json::Json, Deserialize, Serialize},
};

use crate::{handlers::api_response::ApiResponse, models::user, services::user_services, utils};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginResponse {
    pub token: String,
}
#[post("/login", format = "application/json", data = "<basic_auth>")]
pub async fn login_user(basic_auth: Json<user::BasicAuthStruct>) -> ApiResponse<LoginResponse> {
    print!("{:?}", basic_auth);
    if !user_services::find_user_by_basic_auth().await {
        // return json!({ "error": "没找到用户"});
    }
    // 在数据库中查询 用户是否存在，存在就返回她的 id， 将id 给生成 token；
    let user_id: String = "67676916371637216371963".to_string();

    let token = utils::auth::get_token(&user_id);
    ApiResponse::success(LoginResponse { token })
}
