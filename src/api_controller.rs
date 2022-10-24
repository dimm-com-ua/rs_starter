use rocket::{Build, Rocket};
use serde_json::json;
use crate::api_response::ApiResponse;

const API_ROOT: &str = "/api/v1";

#[inline]
pub fn routes(rocket_build: Rocket<Build>) -> Rocket<Build> {
    rocket_build.mount(API_ROOT.to_string(), routes![
        hello,
        health
    ])
}

#[get("/hello")]
fn hello() -> ApiResponse {
    ApiResponse::app_api_success(Some(json!({"hello": "World!"})))
}

#[get("/health")]
fn health() -> String {
    "I'm ok".to_string()
}