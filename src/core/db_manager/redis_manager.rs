use dotenv::dotenv;
use r2d2::Pool;
use redis::Client;
use rocket::State;

use std::{
    env,
    ops::{Deref, DerefMut},
};

pub struct RedisManager;

impl RedisManager {
    pub async fn redis_conn() -> Pool<Client> {
        dotenv().ok();
        let url: String = env::var("REDIS").expect("no redis url");
        // 创建连接池
        let manager = redis::Client::open(url).unwrap();
        let pool = r2d2::Pool::builder().max_size(15).build(manager).unwrap();
        pool
    }

    pub async fn set_value(
        key: &String,
        value: &String,
        redis_pool: &State<Pool<Client>>,
    ) -> String {
        let pool = redis_pool.deref();
        let mut pconn = pool.get().unwrap();
        let conn = pconn.deref_mut();
        redis::Cmd::set(key, value).query::<String>(conn).unwrap()
    }
}
