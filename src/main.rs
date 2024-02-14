mod auth;
mod core;
mod models;
mod routes;
mod services;
mod utils;

use core::{config::cors, db_manager::redis_manager::RedisManager};
use rocket::{catchers, routes};
use routes::{error_routes, user_routes};

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = RedisManager::redis_conn().await;

    rocket::build()
        .attach(cors::rocket_cors_config())
        .manage(pool)
        .mount(
            "/user",
            routes![
                cors::options_preflight,
                user_routes::get_all_users,
                user_routes::login_user,
                user_routes::register_user,
            ],
        )
        .register(
            "/",
            catchers![error_routes::not_found, error_routes::internal_server_error],
        )
        .launch()
        .await?;
    Ok(())
}
