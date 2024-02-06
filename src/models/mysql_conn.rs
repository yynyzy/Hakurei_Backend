use super::user;
use dotenv::dotenv;
use mysql::prelude::*;
use mysql::*;
use std::env;

pub fn get_db_conn() {
    dotenv().ok();
    let database_url: String = env::var("DATABASE_URL").unwrap();
    let pool = Pool::new(Opts::from_url(&database_url).unwrap()).unwrap(); // 获取连接池
    let mut conn = pool.get_conn().unwrap(); // 获取链接

    let res = conn
        .query_map(
            "select * from users",
            |(
                id,
                username,
                password,
                salt,
                email,
                phone,
                status,
                avatar,
                deleted,
                createdAt,
                updatedAt,
            )| user::User {
                id: id,
                username: username,
                password: password,
                salt: salt,
                email: email,
                phone: phone,
                status: status,
                avatar: avatar,
                deleted: deleted,
                createdAt: createdAt,
                updatedAt: updatedAt,
            },
        )
        .expect("Query failed.");

    for i in res {
        println!("{:?}", i)
    }

    // conn.query_("SELECT * FROM users")
    //     .unwrap()
    //     .for_each(|row| {
    //         let r: (u128, String) = from_row(row.unwrap());
    //         println!("{}", r.0);
    //     });
    // for row in result {
    //     let username: String = row.get("username").unwrap();
    //     let password: i32 = row.get("password").unwrap();
    //     println!("{} is {} haha", username, password);
    // }
}
