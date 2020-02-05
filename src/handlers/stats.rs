use actix_web::{get, web};
use deadpool_postgres::Pool;

use crate::responses::api_response::ApiResponse;
use crate::stats::entities::EntityStats;
use crate::stats::photos::PhotosStats;
use crate::stats::tags::TagStats;
use crate::types::HandlerResult;

#[get("/stats/entities")]
pub async fn get_entity_stats(pool: web::Data<Pool>) -> HandlerResult {
    let stats = EntityStats::get_entity_stats(&pool).await?;

    Ok(ApiResponse::success(stats))
}

#[get("/stats/photos")]
pub async fn get_photos_stats(pool: web::Data<Pool>) -> HandlerResult {
    let stats = PhotosStats::get_photos_stats(&pool).await?;

    Ok(ApiResponse::success(stats))
}

#[get("/stats/tags")]
pub async fn get_tag_stats(pool: web::Data<Pool>) -> HandlerResult {
    let stats = TagStats::get_tag_stats(&pool).await?;

    Ok(ApiResponse::success(stats))
}
