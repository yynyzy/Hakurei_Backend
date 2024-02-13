use dotenv::dotenv;
use r2d2::Pool;
use redis::{Client, Commands, RedisError};

use std::{env, ops::DerefMut};

pub struct RedisManager;

impl RedisManager {
    pub async fn redis_conn() -> Pool<Client> {
        dotenv().ok();
        let url: String = env::var("REDIS").expect("no Redis database url");
        // 创建连接池
        let manager = redis::Client::open(url).unwrap();
        let pool = r2d2::Pool::builder().max_size(15).build(manager).unwrap();
        pool
    }

    pub async fn set_value(
        key: &String,
        value: &String,
        pool: &Pool<Client>,
    ) -> Result<String, RedisError> {
        let mut pconn = pool.get().unwrap();
        let conn = pconn.deref_mut();
        conn.set(key, value)
    }

    pub async fn get_value(key: &str, pool: &Pool<Client>) -> Result<String, RedisError> {
        let mut pconn = pool.get().unwrap();
        let conn = pconn.deref_mut();
        conn.get(key)
    }
}
