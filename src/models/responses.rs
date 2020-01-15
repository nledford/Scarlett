/*
 * SEE:
 *   - https://technick.net/guides/software/software_json_api_format/
 *   - https://jsonapi.org/
 */

use std::env;

use chrono::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

// STANDARD RESPONSE *******************************************************************************

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResponse<T> {
    pub program: String,
    pub version: String,
    pub release: i64,
    pub datetime: NaiveDateTime,
    pub timestamp: i64,
    pub status: String,
    pub code: i64,
    pub message: String,
    pub data: T,
}

impl<T> ApiResponse<T> {
    pub fn new(status: &str, code: i64, message: &str, data: T) -> Self {
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
