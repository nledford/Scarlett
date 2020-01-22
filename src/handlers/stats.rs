use actix_web::{get, web, HttpResponse};
use deadpool_postgres::Pool;

use crate::errors::errors;
use crate::responses::api_response::ApiResponse;
use crate::stats::photos::PhotosStats;
use crate::stats::tags::EntityStats;
use crate::stats::entities::TagStats;

#[get("/stats/entities")]
pub async fn get_entity_stats(pool: web::Data<Pool>) -> Result<HttpResponse, errors::Error> {
    let res = EntityStats::get_entity_stats(&pool).await;

    match res {
        Ok(stats) => Ok(ApiResponse::success(stats)),
        Err(err) => Ok(ApiResponse::error(err.to_string()))
    }
}

#[get("/stats/photos")]
pub async fn get_photos_stats(pool: web::Data<Pool>) -> Result<HttpResponse, errors::Error> {
    let res = PhotosStats::get_photos_stats(pool.get_ref()).await;

    match res {
        Ok(stats) => Ok(ApiResponse::success(stats)),
        Err(err) => Ok(ApiResponse::error(err.to_string())),
    }
}

#[get("/stats/tags")]
pub async fn get_tag_stats(pool: web::Data<Pool>) -> Result<HttpResponse, errors::Error> {
    let res = TagStats::get_tag_stats(&pool).await;

    match res {
        Ok(stats) => Ok(ApiResponse::success(stats)),
        Err(err) => Ok(ApiResponse::error(err.to_string()))
    }
}
