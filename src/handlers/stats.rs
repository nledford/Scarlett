use crate::models::db::PhotosStats;
use crate::models::errors;
use actix_web::{get, web, HttpResponse};
use deadpool_postgres::Pool;

#[get("/stats/photos")]
pub async fn get_photos_stats(pool: web::Data<Pool>) -> Result<HttpResponse, errors::Error> {
    let res = PhotosStats::get_photos_stats(pool.get_ref()).await;

    match res {
        Ok(stats) => Ok(HttpResponse::Ok().json(stats)),
        Err(err) => Ok(HttpResponse::InternalServerError().json(err.to_string())),
    }
}
