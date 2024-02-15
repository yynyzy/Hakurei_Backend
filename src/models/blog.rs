use crate::core::db_manager::mysql_manager;
use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};

// 对应表
#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Clone)]
pub struct BlogModel {
    pub id: i64,
    pub user_id: String,
    pub title: String,
    pub description: String,
    pub content: String,
    pub status: Option<i8>,
    #[sqlx(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[sqlx(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

impl BlogModel {
    pub async fn create_blog(user_id: &str, blog: BlogCreateRequestModel) -> Option<String> {
        let pool: sqlx::Pool<sqlx::MySql> = mysql_manager::get_db_conn_pool().await;
        let query =
            " INSERT INTO blog (user_id, title, description, content, status) VALUES (?, ?, ?, ?, ?)";

        let result = sqlx::query(query)
            .bind(user_id)
            .bind(&blog.title)
            .bind(&blog.description)
            .bind(&blog.content)
            .bind(&blog.status)
            .execute(&pool)
            .await
            .unwrap();
        if result.rows_affected() > 0 {
            Some(user_id.to_string())
        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlogCreateRequestModel {
    pub title: String,
    pub description: String,
    pub content: String,
    pub status: i8,
}
