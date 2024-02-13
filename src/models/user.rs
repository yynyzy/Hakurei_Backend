use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool};

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Clone)]
pub struct UserModel {
    pub id: String,
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub salt: String,
    pub status: i8,
    pub avatar: Option<String>,
    pub deleted: i8,
    #[sqlx(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[sqlx(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

impl UserModel {
    pub async fn create_one_user(pool: &Pool<MySql>, user: UserModel) -> Result<String, ()> {
        let query = "
 INSERT INTO users (id, username, password, email, phone, salt, status, avatar, deleted)
 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)";

        let result = sqlx::query(query)
            .bind(&user.id)
            .bind(&user.username)
            .bind(&user.password)
            .bind(&user.email)
            .bind(&user.phone)
            .bind(&user.salt)
            .bind(&user.status)
            .bind(&user.avatar)
            .bind(&user.deleted)
            .execute(pool)
            .await
            .unwrap();
        if result.rows_affected() > 0 {
            Ok(user.id.clone())
        } else {
            Err(())
        }
    }

    pub async fn find_all(pool: &Pool<MySql>) -> Option<Vec<UserModel>> {
        let users = sqlx::query_as::<_, UserModel>("SELECT * FROM users")
            .fetch_all(pool)
            .await
            .ok();
        users
    }

    pub async fn find_by_username(pool: &Pool<MySql>, username: &str) -> Option<UserModel> {
        let user: Option<UserModel> =
            sqlx::query_as::<_, UserModel>("SELECT * FROM users WHERE username = ?")
                .bind(username)
                .fetch_optional(pool)
                .await
                .unwrap();
        user
    }

    pub async fn find_by_username_and_password(
        pool: &Pool<MySql>,
        username: &str,
        password: &str,
    ) -> Option<UserModel> {
        let user = sqlx::query_as::<_, UserModel>(
            "SELECT * FROM users WHERE username = ? AND password = ?",
        )
        .bind(username)
        .bind(password)
        .fetch_optional(pool)
        .await
        .unwrap();
        user
    }
}

#[derive(Deserialize, Debug)]
pub struct RegisterUserStruct {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginUserStruct {
    pub username: String,
    pub password: String,
}
