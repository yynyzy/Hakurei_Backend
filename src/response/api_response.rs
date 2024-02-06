use rocket::{
    http::{ContentType, Status},
    response::{Responder, Response, Result},
    serde::{json::serde_json, Serialize},
    Request,
};
use std::io::Cursor;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    status: Status,
    message: String,
    data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse {
            status: Status::Ok,
            message: "Success".to_owned(),
            data: Some(data),
        }
    }

    pub fn error(status: Status, message: String) -> Self {
        ApiResponse {
            status,
            message,
            data: None,
        }
    }
}

impl<'a, T: Serialize> Responder<'a, 'static> for ApiResponse<T> {
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
