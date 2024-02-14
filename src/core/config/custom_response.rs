use rocket::{
    http::{ContentType, Status},
    response::{Responder, Response, Result},
    serde::{json::serde_json, Serialize},
    Request,
};
use std::io::Cursor;

#[derive(Debug, Serialize)]
pub struct CustomResponse<T> {
    status: Status,
    message: String,
    data: Option<T>,
}

impl<T> CustomResponse<T> {
    pub fn success(data: T) -> Self {
        CustomResponse {
            status: Status::Ok,
            message: "Success".to_owned(),
            data: Some(data),
        }
    }
}

impl CustomResponse<()> {
    pub fn error(status: Status, message: String) -> Self {
        CustomResponse {
            status,
            message,
            data: None,
        }
    }
}

impl<'r, T: Serialize> Responder<'r, 'static> for CustomResponse<T> {
    fn respond_to(self, _: &'r Request<'_>) -> Result<'static> {
        let json_str = serde_json::to_string(&self).map_err(|_e| Status::InternalServerError)?;
        Response::build()
            .header(ContentType::JSON)
            .status(Status::Ok)
            .sized_body(json_str.len(), Cursor::new(json_str))
            .ok()
    }
}
