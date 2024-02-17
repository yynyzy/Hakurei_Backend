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
    pub async fn create_one(blog: BlogCreateRequestModel) -> Option<()> {
        let pool: sqlx::Pool<sqlx::MySql> = mysql_manager::get_db_conn_pool().await;
        let query =
            "INSERT INTO blog (user_id, title, description, content, status) VALUES (?, ?, ?, ?, ?)";

        let result = sqlx::query(query)
            .bind(&blog.user_id)
            .bind(&blog.title)
            .bind(&blog.description)
            .bind(&blog.content)
            .bind(&blog.status)
            .execute(&pool)
            .await
            .unwrap();
        if result.rows_affected() > 0 {
            Some(())
        } else {
            None
        }
    }

    pub async fn find_all() -> Option<Vec<BlogModel>> {
        let pool: sqlx::Pool<sqlx::MySql> = mysql_manager::get_db_conn_pool().await;
        let query = "SELECT * FROM blog";

        let blogs = sqlx::query_as::<_, BlogModel>(query)
            .fetch_all(&pool)
            .await
            .ok();
        blogs
    }

    pub async fn find_by_user_id(user_id: &str) -> Option<Vec<BlogModel>> {
        let pool: sqlx::Pool<sqlx::MySql> = mysql_manager::get_db_conn_pool().await;
        let query = "SELECT * FROM blog WHERE user_id = ?";
        let blogs = sqlx::query_as::<_, BlogModel>(query)
            .bind(user_id)
            .fetch_all(&pool)
            .await
            .ok();
        blogs
    }

    pub async fn find_by_id(article_id: &str) -> Option<Vec<BlogModel>> {
        let pool: sqlx::Pool<sqlx::MySql> = mysql_manager::get_db_conn_pool().await;
        let query = "SELECT * FROM blog WHERE id = ?";
        let blog = sqlx::query_as::<_, BlogModel>(query)
            .bind(article_id)
            .fetch_all(&pool)
            .await
            .ok();
        blog
    }

    pub async fn delete_by_id(article_id: &str) -> Option<()> {
        let pool: sqlx::Pool<sqlx::MySql> = mysql_manager::get_db_conn_pool().await;
        let query = "DELETE FROM blog WHERE id = ?";
        let result = sqlx::query(query)
            .bind(article_id)
            .execute(&pool)
            .await
            .unwrap();
        if result.rows_affected() > 0 {
            Some(())
        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlogCreateRequestModel {
    pub user_id: Option<String>,
    pub title: String,
    pub description: String,
    pub content: String,
    pub status: i8,
}
