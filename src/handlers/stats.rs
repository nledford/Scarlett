

use actix_web::{get, web, HttpResponse};
use deadpool_postgres::Pool;

use crate::models::errors;
use crate::models::responses::ApiResponse;
use crate::models::stats::PhotosStats;

#[get("/stats/photos")]
pub async fn get_photos_stats(pool: web::Data<Pool>) -> Result<HttpResponse, errors::Error> {
    let res = PhotosStats::get_photos_stats(pool.get_ref()).await;

    match res {
        Ok(stats) => Ok(HttpResponse::Ok().json(ApiResponse::new("success", 200, "ok", stats))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::new(
            "failure",
            500,
            "An error has occurred",
            err.to_string(),
        ))),
    }
}
