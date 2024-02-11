use rocket::{catchers, routes};

mod auth;
mod core;
mod models;
mod routes;
mod services;
mod utils;

use routes::user_routes;

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
        .register("/", catchers![routes::error_routes::not_found_url])
        .launch()
        .await?;
    Ok(())
}
