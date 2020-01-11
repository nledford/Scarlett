use actix_web::{get, Responder};

pub mod photos;
pub mod stats;

#[get("/")]
pub async fn index() -> impl Responder {
    "Welcome to Scarlett!"
}
