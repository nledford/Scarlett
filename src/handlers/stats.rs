use actix_web::{get, HttpResponse, web};
use deadpool_postgres::Pool;

use crate::errors::ServiceError;
use crate::responses::api_response::ApiResponse;
use crate::stats::entities::EntityStats;
use crate::stats::photos::PhotosStats;
use crate::stats::tags::TagStats;

#[get("/stats/entities")]
pub async fn get_entity_stats(pool: web::Data<Pool>) -> Result<HttpResponse, ServiceError> {
    let res = EntityStats::get_entity_stats(&pool).await;

    match res {
        Ok(stats) => Ok(ApiResponse::success(stats)),
        Err(err) => Ok(ApiResponse::error(err.to_string())),
    }
}

#[get("/stats/photos")]
pub async fn get_photos_stats(pool: web::Data<Pool>) -> Result<HttpResponse, ServiceError> {
    let res = PhotosStats::get_photos_stats(pool.get_ref()).await;

    match res {
        Ok(stats) => Ok(ApiResponse::success(stats)),
        Err(err) => Ok(ApiResponse::error(err.to_string())),
    }
}

#[get("/stats/tags")]
pub async fn get_tag_stats(pool: web::Data<Pool>) -> Result<HttpResponse, ServiceError> {
    let res = TagStats::get_tag_stats(&pool).await;

    match res {
        Ok(stats) => Ok(ApiResponse::success(stats)),
        Err(err) => Ok(ApiResponse::error(err.to_string())),
    }
}
