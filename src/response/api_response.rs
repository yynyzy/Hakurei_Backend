use rocket::{
    http::{ContentType, Status},
    response::{Responder, Response, Result},
    serde::{json::serde_json, Serialize},
    Request,
};
use std::io::Cursor;

#[derive(Debug, Serialize)]
pub struct ApiResponse<'r, T> {
    status: u16,
    message: &'r str,
    data: Option<T>,
}

impl<'r, T> ApiResponse<'r, T> {
    pub fn success(data: T) -> Self {
        ApiResponse {
            status: 200,
            message: "Success",
            data: Some(data),
        }
    }

    pub fn error(status: u16, message: &'r str) -> Self {
        ApiResponse {
            status,
            message,
            data: None,
        }
    }
}

impl<'r, T: Serialize> Responder<'r, 'static> for ApiResponse<'r, T> {
    fn respond_to(self, _: &'r Request<'_>) -> Result<'static> {
        let json_str = serde_json::to_string(&self).map_err(|e| {
            eprintln!("Error serializing JSON: {:?}", e);
            Status::InternalServerError
        })?;
        Response::build()
            .header(ContentType::JSON)
            .status(Status::Ok)
            .sized_body(json_str.len(), Cursor::new(json_str))
            .ok()
    }
}
