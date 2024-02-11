use rocket::{catch, http::Status};

use crate::core::{common::constants, response::custom_response::CustomResponse};

#[catch(404)]
pub async fn not_found() -> CustomResponse<'static, ()> {
    CustomResponse::error(Status::NotFound, constants::NOT_FOUND)
}

#[catch(500)]
pub async fn internal_server_error() -> CustomResponse<'static, ()> {
    CustomResponse::error(
        Status::InternalServerError,
        constants::INTERNAL_SERVER_ERROR,
    )
}
