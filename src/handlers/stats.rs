use actix_web::{get, web, HttpResponse};
use deadpool_postgres::Pool;

use crate::models::errors;
use crate::models::responses::ApiResponse;
use crate::models::stats::PhotosStats;

#[get("/stats/photos")]
pub async fn get_photos_stats(pool: web::Data<Pool>) -> Result<HttpResponse, errors::Error> {
    let res = PhotosStats::get_photos_stats(pool.get_ref()).await;

    match res {
        Ok(stats) => Ok(ApiResponse::success(stats)),
        Err(err) => Ok(ApiResponse::error(err.to_string())),
    }
}
