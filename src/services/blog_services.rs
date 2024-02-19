use crate::models::{
    blog::{BlogCreateRequestModel, BlogModel},
    user::UserModel,
};

pub async fn create_one(user_id: &str, mut blog: BlogCreateRequestModel) -> Result<(), ()> {
    blog.user_id = Some(user_id.to_owned());
    let user = UserModel::find_by_user_id(user_id).await;
    if let Some(user) = user {
        blog.user_name = Some(user.username);
    }
    let is_succeed = BlogModel::create_one(blog).await;
    if let Some(is_succeed) = is_succeed {
        return Ok(is_succeed);
    }
    Err(())
}

// 删除文章（用户自己的文章）
pub async fn delete_owner_article(id: i64, user_id: &str) -> Result<u8, ()> {
    // 先查询 是否有权限删除这个 article
    //  is_user_article_existed 如果为空数组表示用户没有权限删除不是他的文章，返回 2
    // 删除成功返回 1
    let is_user_article_existed = BlogModel::find_by_uid_and_id(id, user_id).await;
    if let Some(is_user_article_existed) = is_user_article_existed {
        if is_user_article_existed.len() > 0 {
            let is_succeed = BlogModel::delete_by_id(id).await;
            if let Some(_) = is_succeed {
                return Ok(1);
            }
        } else {
            return Ok(2);
        }
    }
    Err(())
}
