use crate::models::blog::{BlogCreateRequestModel, BlogModel};

pub async fn create_one(user_id: String, mut blog: BlogCreateRequestModel) -> Result<(), ()> {
    blog.user_id = Some(user_id);
    let is_succeed = BlogModel::create_one(blog).await;
    if let Some(is_succeed) = is_succeed {
        return Ok(is_succeed);
    }
    Err(())
}
