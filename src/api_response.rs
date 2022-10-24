use serde::{Serialize, Deserialize};
use rocket::http::{ContentType, Status};
use rocket::{Request};
use rocket::response::{self, Response, Responder};
use rocket::serde::json::{serde_json, Value};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    #[serde(skip_serializing)]
    pub status: u16,
    pub success: bool,
    pub payload: Value,
    pub error: Option<String>,
}

impl ApiResponse {
    pub fn new(success: bool, payload: Option<Value>, error: Option<&str>, status: Option<Status>) -> ApiResponse {
        let mut payload_val: Value = json!({});
        let mut error_msg: Option<String> = None;
        let mut status_code = Status::BadRequest;

        match status {
            Some(status) => { status_code = status; }
            _ => {}
        }

        match payload {
            Some(p_val) => { payload_val = p_val; }
            _ => {}
        }

        match error {
            Some(err) => { error_msg = Option::from(String::from(err)); }
            _ => {}
        }

        ApiResponse {
            status: status_code.code,
            success,
            payload: payload_val,
            error: error_msg
        }
    }
    pub fn app_api_success(payload: Option<Value>) -> ApiResponse {
        ApiResponse::new(true, payload, None, Some(Status::Ok))
    }
    pub fn app_api_some_error(err: &str) -> ApiResponse {
        ApiResponse::new(false, None, Some(err), Some(Status::BadRequest))
    }
    pub fn app_api_unauthorized_error() -> ApiResponse {
        ApiResponse::new(false, None, Some("Unauthorized"), Some(Status::Unauthorized))
    }
    pub fn app_api_forbidden_error() -> ApiResponse {
        ApiResponse::new(false, None, Some("Forbidden"), Some(Status::Forbidden))
    }
    pub fn app_api_not_found_error() -> ApiResponse {
        ApiResponse::new(false, None, Some("Not found"), Some(Status::NotFound))
    }
    pub fn app_api_internal_server_error() -> ApiResponse {
        ApiResponse::new(false, None, Some("InternalServerError"), Some(Status::InternalServerError))
    }
}

impl<'r> Responder<'r, 'static> for ApiResponse {
    fn respond_to<'json>(self, request: &'r Request<'_>) -> rocket::response::Result<'static> {
        let serialized = serde_json::to_string(&self).unwrap();
        Response::build_from(serialized.respond_to(request)?)
            .header(ContentType::new("application", "json"))
            .status(Status::from_code(self.status).unwrap())
            .ok()
    }
}