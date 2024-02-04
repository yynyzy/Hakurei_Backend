use rocket::{
    http::Status,
    response::{Responder, Response, Result},
    serde::{json::serde_json, Serialize},
    Request,
};
use std::io::Cursor;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ApiResponse<T> {
    status: u16,
    message: String,
    data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse {
            status: 200,
            message: "Success".to_string(),
            data: Some(data),
        }
    }

    pub fn error(status: u16, message: &str) -> Self {
        ApiResponse {
            status,
            message: message.to_string(),
            data: None,
        }
    }
}

impl<'r, T: Serialize> Responder<'r, 'static> for ApiResponse<T> {
    fn respond_to(self, _: &'r Request<'_>) -> Result<'static> {
        let json_str = serde_json::to_string(&self).map_err(|e| {
            eprintln!("Error serializing JSON: {:?}", e);
            Status::InternalServerError
        })?;
        Response::build()
            .header(rocket::http::ContentType::JSON)
            .status(Status::Ok)
            .sized_body(json_str.len(), Cursor::new(json_str))
            .ok()
    }
}
