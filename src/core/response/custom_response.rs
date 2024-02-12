use rocket::{
    http::{ContentType, Status},
    response::{Responder, Response, Result},
    serde::{json::serde_json, Serialize},
    Request,
};
use std::io::Cursor;

#[derive(Debug, Serialize)]
pub struct CustomResponse<'a, T> {
    status: Status,
    message: &'a str,
    data: Option<T>,
}

impl<'a, T> CustomResponse<'a, T> {
    pub fn success(data: T) -> Self {
        CustomResponse {
            status: Status::Ok,
            message: "Success",
            data: Some(data),
        }
    }
}

impl<'a> CustomResponse<'a, ()> {
    pub fn error(status: Status, message: &'a str) -> Self {
        CustomResponse {
            status,
            message,
            data: None,
        }
    }
}

impl<'a, T: Serialize> Responder<'a, 'static> for CustomResponse<'a, T> {
    fn respond_to(self, _: &'a Request<'_>) -> Result<'static> {
        let json_str = serde_json::to_string(&self).map_err(|e| Status::InternalServerError)?;
        Response::build()
            .header(ContentType::JSON)
            .status(Status::Ok)
            .sized_body(json_str.len(), Cursor::new(json_str))
            .ok()
    }
}
