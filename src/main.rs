use rocket::{catchers, routes};

mod auth;
mod core;
mod models;
mod routes;
mod services;
mod utils;

use routes::{error_routes, user_routes};

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rocket::build()
        .mount(
            "/user",
            routes![
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
