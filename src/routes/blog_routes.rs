use crate::{
    auth::auth,
    models::blog::{self, BlogModel},
};
use rocket::{post, serde::json::Json};

#[post("/create", format = "application/json", data = "<blog>")]
pub async fn create_blog(blog: Json<blog::BlogCreateRequestModel>, auth_guard: auth::BasicAuth) {
    println!("auth_guard={:?}", auth_guard);
    println!("data={:?}", blog);
    BlogModel::create_blog(auth_guard.sub.as_str(), blog.into_inner()).await;
    // CustomResponse::error(Status::NotFound, constants::NOT_FOUND.to_owned())
}
