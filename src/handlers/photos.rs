use actix_web::{get, web, HttpResponse};
use deadpool_postgres::Pool;

use crate::models::db::PhotosAll;
use crate::models::errors;

#[get("/photos")]
pub async fn get_photos(pool: web::Data<Pool>) -> Result<HttpResponse, errors::Error> {
    let res = PhotosAll::all_photos(&pool).await;

    match res {
        Ok(photos) => Ok(HttpResponse::Ok().json(photos)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    }
}
