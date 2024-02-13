use crate::{
    models::user::{self, UserModel},
    utils::uuid::UUID,
};

pub async fn register_user(params: user::RegisterUserStruct) -> Result<String, ()> {
    let user = UserModel {
        id: UUID::generate_uuid(),
        username: params.username,
        password: params.password,
        email: None,
        phone: None,
        salt: 1.to_string(),
        status: 1,
        avatar: None,
        deleted: 0,
        created_at: None,
        updated_at: None,
    };
    UserModel::create_user(user).await.ok_or(())
}
