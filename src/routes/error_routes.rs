use rocket::{catch, http::Status};

use crate::response::{api_response::ApiResponse, response_obj};

#[catch(404)]
pub async fn not_found_url() -> ApiResponse<response_obj::ResponseTokenStruct> {
    ApiResponse::error(Status::NotFound, "url not found".to_owned())
}
