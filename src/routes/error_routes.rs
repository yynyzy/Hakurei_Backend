use rocket::{catch, http::Status};

use crate::response::api_response::ApiResponse;

#[catch(404)]
pub async fn not_found_url() -> ApiResponse<'static, ()> {
    ApiResponse::error(Status::NotFound, "url not found")
}
