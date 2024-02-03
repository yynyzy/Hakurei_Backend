use rocket::{
    post,
    request::{FromRequest, Outcome},
    serde::json::{serde_json::json, Json, Value},
};

use crate::{models, services::user_services, utils};

#[post("/login", format = "application/json", data = "<basic_auth>")]
pub async fn login_user(basic_auth: Json<models::user::BasicAuthStruct>) -> Value {
    print!("{:?}", basic_auth);
    if !user_services::find_user_by_basic_auth().await {
        return json!({ "error": "没找到用户"});
    }
    // 在数据库中查询 用户是否存在，存在就返回她的 id， 将id 给生成 token；
    let user_id: String = "67676916371637216371963".to_string();

    let token = utils::auth::get_token(&user_id);
    json!({ "token": token})
}
