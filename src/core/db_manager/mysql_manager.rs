use dotenv::dotenv;

use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySql;
use std::env;

pub async fn get_db_conn_pool() -> sqlx::Pool<MySql> {
    dotenv().ok();
    let database_url: String = env::var("DATABASE_URL").expect("no MySql database url");
    let pool: sqlx::Pool<MySql> = MySqlPoolOptions::new()
        .connect(&database_url)
        .await
        .unwrap();
    pool
}
