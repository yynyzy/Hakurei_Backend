use crate::core::db_manager::mysql_manager;
use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Clone)]
pub struct UserModel {
    pub id: String,
    pub username: String,
    pub password: String,
    pub role: Option<i8>, // 1 管理员， 2 普通用户， 3 游客
    pub email: Option<String>,
    pub phone: Option<String>,
    pub status: i8,
    pub avatar: Option<String>,
    pub deleted: i8,
    #[sqlx(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[sqlx(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

impl UserModel {
    pub async fn create_user(user: UserModel) -> Option<String> {
        let pool: sqlx::Pool<sqlx::MySql> = mysql_manager::get_db_conn_pool().await;
        let query = "
 INSERT INTO users (id, username, password, role, email, phone, status, avatar, deleted)
 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)";

        let result = sqlx::query(query)
            .bind(&user.id)
            .bind(&user.username)
            .bind(&user.password)
            .bind(&user.role)
            .bind(&user.email)
            .bind(&user.phone)
            .bind(&user.status)
            .bind(&user.avatar)
            .bind(&user.deleted)
            .execute(&pool)
            .await
            .unwrap();
        if result.rows_affected() > 0 {
            Some(user.id.clone())
        } else {
            None
        }
    }

    pub async fn find_all() -> Option<Vec<UserModel>> {
        let pool: sqlx::Pool<sqlx::MySql> = mysql_manager::get_db_conn_pool().await;
        let users = sqlx::query_as::<_, UserModel>("SELECT * FROM users")
            .fetch_all(&pool)
            .await
            .ok();
        users
    }

    pub async fn find_by_username(username: &str) -> Option<UserModel> {
        let pool: sqlx::Pool<sqlx::MySql> = mysql_manager::get_db_conn_pool().await;
        let user: Option<UserModel> =
            sqlx::query_as::<_, UserModel>("SELECT * FROM users WHERE username = ?")
                .bind(username)
                .fetch_optional(&pool)
                .await
                .unwrap();
        user
    }

    pub async fn find_by_username_and_password(
        username: &str,
        password: &str,
    ) -> Option<UserModel> {
        let pool: sqlx::Pool<sqlx::MySql> = mysql_manager::get_db_conn_pool().await;

        let user = sqlx::query_as::<_, UserModel>(
            "SELECT * FROM users WHERE username = ? AND password = ?",
        )
        .bind(username)
        .bind(password)
        .fetch_optional(&pool)
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
