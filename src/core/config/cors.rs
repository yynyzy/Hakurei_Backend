use rocket::options;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use std::path::PathBuf;

pub fn rocket_cors_config() -> Cors {
    CorsOptions {
        allowed_origins: AllowedOrigins::All,
        allowed_headers: AllowedHeaders::All,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("CORS CONFIG ERROR")
}

#[options("/<path..>")]
pub fn options_preflight(path: PathBuf) -> () {
    ()
}
