use rocket::{
    http::{ContentType, Status},
    response::{Responder, Response, Result},
    serde::{json::serde_json, Serialize},
    Request,
};
use std::io::Cursor;

#[derive(Debug, Serialize)]
pub struct ApiResponse<'a, T> {
    status: Status,
    message: &'a str,
    data: Option<T>,
}

impl<'a, T> ApiResponse<'a, T> {
    pub fn success(data: T) -> Self {
        ApiResponse {
            status: Status::Ok,
            message: "Success",
            data: Some(data),
        }
    }
}

impl<'a> ApiResponse<'a, ()> {
    pub fn error(status: Status, message: &'a str) -> Self {
        ApiResponse {
            status,
            message,
            data: None,
        }
    }
}

impl<'a, T: Serialize> Responder<'a, 'static> for ApiResponse<'a, T> {
    fn respond_to(self, _: &'a Request<'_>) -> Result<'static> {
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
