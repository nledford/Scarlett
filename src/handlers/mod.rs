use actix_web::{get, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::models::errors;
use crate::models::responses::ApiResponse;

pub mod photos;
pub mod stats;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Route {
    pub method: String,
    pub path: String,
    pub description: String,
}

impl Route {
    pub fn new(method: &str, path: &str, description: &str) -> Self {
        Route {
            method: method.to_string(),
            path: path.to_string(),
            description: description.to_string(),
        }
    }
}

fn build_list_of_routes() -> Vec<Route> {
    let mut routes = Vec::new();

    routes.push(Route::new("GET", "/status", "check this service status"));

    routes
}

// INDEX *******************************************************************************************

#[get("/")]
pub async fn index() -> Result<HttpResponse, errors::Error> {
    let routes = build_list_of_routes();

    let res = ApiResponse::new("success", 200, "OK", routes);

    Ok(HttpResponse::Ok().json(res))
}

// STATUS ******************************************************************************************

#[derive(Serialize)]
pub struct AppStatus {
    pub duration: i64,
    pub message: String,
}

impl AppStatus {
    pub fn new(duration: i64, message: &str) -> Self {
        AppStatus {
            duration,
            message: message.to_string(),
        }
    }
}

#[get("/status")]
pub async fn status() -> Result<HttpResponse, errors::Error> {
    // TODO figure out a way to get application uptime
    let status = AppStatus::new(0, "The service is healthy");

    let res = ApiResponse::new("success", 200, "OK", status);

    Ok(HttpResponse::Ok().json(res))
}
