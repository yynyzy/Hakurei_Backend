use crate::{
    auth::auth,
    core::{common::constants, config::custom_response::CustomResponse},
    models::blog::{self, BlogModel},
    services::blog_services,
};
use rocket::{delete, get, http::Status, post, serde::json::Json};

#[post("/create", format = "application/json", data = "<blog>")]
pub async fn create_one(
    blog: Json<blog::BlogCreateRequestModel>,
    auth_guard: auth::BasicAuth,
) -> Result<CustomResponse<()>, CustomResponse<()>> {
    let is_succeed = blog_services::create_one(auth_guard.sub.as_str(), blog.into_inner()).await;
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

#[get("/list/my")]
pub async fn get_blogs_by_myself(
    auth_guard: auth::BasicAuth,
) -> Result<CustomResponse<Vec<BlogModel>>, CustomResponse<()>> {
    let blogs = BlogModel::find_by_user_id(auth_guard.sub.as_str()).await;
    if let Some(blogs) = blogs {
        return Ok(CustomResponse::success(blogs));
    }
    Err(CustomResponse::error(
        Status::InternalServerError,
        constants::BLOGS_FIND_FAILED.to_owned(),
    ))
}

#[get("/<id>")]
pub async fn get_blog_by_id(
    id: String,
    _auth_guard: auth::BasicAuth,
) -> Result<CustomResponse<Vec<BlogModel>>, CustomResponse<()>> {
    let blog = BlogModel::find_by_id(id.as_str()).await;
    if let Some(blog) = blog {
        return Ok(CustomResponse::success(blog));
    }
    Err(CustomResponse::error(
        Status::InternalServerError,
        constants::BLOGS_FIND_FAILED.to_owned(),
    ))
}

#[delete("/<id>")]
pub async fn delete_blog_by_id(
    id: i64,
    auth_guard: auth::BasicAuth,
) -> Result<CustomResponse<()>, CustomResponse<()>> {
    let is_succeed = blog_services::delete_owner_article(id, auth_guard.sub.as_str()).await;
    if let Ok(is_succeed) = is_succeed {
        if is_succeed == 1 {
            return Ok(CustomResponse::success(()));
        }
        if is_succeed == 2 {
            return Err(CustomResponse::error(
                Status::Forbidden,
                constants::BLOGS_DELETE_FAILED_WITH_NO_PERMISSION.to_owned(),
            ));
        }
    }
    Err(CustomResponse::error(
        Status::InternalServerError,
        constants::BLOGS_DELETE_FAILED.to_owned(),
    ))
}
