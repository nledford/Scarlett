use chrono::NaiveDateTime;
use chrono::prelude::*;
use actix_web::http::StatusCode;
use serde::{Serialize, Deserialize};

const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResponse<T> {
    pub program: String,
    pub version: String,
    pub release: i32,
    pub datetime: NaiveDateTime,
    pub timestamp: i64,
    pub status: String,
    pub code: i32,
    pub message: String,
    pub data: T,
}

impl<T> ApiResponse<T> {
    pub fn new(status: &str, code: i32, message: &str, data: T) -> Self {
        ApiResponse {
            program: APP_NAME.to_string(),
            version: VERSION.to_string(),
            release: 0, // TODO get a release number
            datetime: Utc::now().naive_utc(),
            timestamp: Utc::now().timestamp_nanos(),
            status: status.to_string(),
            code,
            message: message.to_string(),
            data,
        }
    }
}