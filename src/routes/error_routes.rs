use rocket::{catch, http::Status};

use crate::core::{common::constants, response::custom_response::CustomResponse};

#[catch(404)]
pub async fn not_found_url() -> CustomResponse<'static, ()> {
    CustomResponse::error(Status::NotFound, constants::URL_NOT_FOUND)
}
