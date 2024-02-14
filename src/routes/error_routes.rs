use rocket::{catch, http::Status};

use crate::core::{common::constants, response::custom_response::CustomResponse};

#[catch(404)]
pub async fn not_found() -> CustomResponse<()> {
    CustomResponse::error(Status::NotFound, constants::NOT_FOUND.to_owned())
}

#[catch(500)]
pub async fn internal_server_error() -> CustomResponse<()> {
    CustomResponse::error(
        Status::InternalServerError,
        constants::INTERNAL_SERVER_ERROR.to_owned(),
    )
}
