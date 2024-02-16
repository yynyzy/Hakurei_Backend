use crate::{
    auth::auth,
    core::{common::constants, config::custom_response::CustomResponse},
    models::blog::{self, BlogModel},
    services::blog_services,
};
use rocket::{get, http::Status, post, serde::json::Json};

#[post("/create", format = "application/json", data = "<blog>")]
pub async fn create_one(
    blog: Json<blog::BlogCreateRequestModel>,
    auth_guard: auth::BasicAuth,
) -> Result<CustomResponse<()>, CustomResponse<()>> {
    let is_succeed = blog_services::create_one(auth_guard.sub, blog.into_inner()).await;
    if let Ok(is_succeed) = is_succeed {
        return Ok(CustomResponse::success(is_succeed));
    }
    Err(CustomResponse::error(
        Status::InternalServerError,
        constants::BLOG_CREATED_FAILED.to_owned(),
    ))
}

#[get("/list")]
pub async fn get_blogs(
    _auth_guard: auth::BasicAuth,
) -> Result<CustomResponse<Vec<BlogModel>>, CustomResponse<()>> {
    let blogs = BlogModel::find_all().await;
    if let Some(blogs) = blogs {
        return Ok(CustomResponse::success(blogs));
    }
    Err(CustomResponse::error(
        Status::InternalServerError,
        constants::BLOGS_FIND_FAILED.to_owned(),
    ))
}
