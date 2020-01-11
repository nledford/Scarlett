use actix_web::{get, Responder};

pub mod photos;

#[get("/")]
pub async fn index() -> impl Responder {
    "Welcome to Scarlett!"
}
