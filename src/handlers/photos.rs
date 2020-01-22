use actix_web::{delete, get, post, web, HttpResponse};
use deadpool_postgres::Pool;

use crate::errors::errors;
use crate::requests::get_photos_request::GetPhotosRequest;
use crate::responses::api_response::ApiResponse;
use crate::schemas;
use crate::schemas::photo::Photo;
use crate::schemas::photo_full::PhotoFull;
use crate::schemas::DbTable;

// ALL PHOTOS **************************************************************************************

#[get("/photos")]
pub async fn get_photos(
    info: web::Query<GetPhotosRequest>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::Error> {
    let res = PhotoFull::get_page(info.into_inner(), &pool).await;

    match res {
        Ok(page) => Ok(ApiResponse::success(page)),
        Err(err) => Ok(ApiResponse::error(err.to_string())),
    }
}

// SINGLE PHOTO ************************************************************************************

#[get("/photos/{photo_id}")]
pub async fn get_photo(
    info: web::Path<i64>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::Error> {
    let res = Photo::get_by_id(info.into_inner(), &pool).await;

    match res {
        Ok(photo) => Ok(ApiResponse::success(photo)),
        Err(err) => Ok(ApiResponse::error(err.to_string())),
    }
}

// UPDATE PHOTO ************************************************************************************

// DELETE PHOTO ************************************************************************************

// PHOTO TAGS **************************************************************************************

#[post("/photos/{photo_id}/tags/{tag_id}")]
pub async fn add_tag_to_photo(
    info: web::Path<(i64, i64)>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::Error> {
    let (photo_id, tag_id) = info.into_inner();

    let res = Photo::add_tag_to_photo(photo_id, tag_id, &pool).await;

    match res {
        Ok(message) => Ok(ApiResponse::success(message)),
        Err(err) => Ok(ApiResponse::error(err.to_string())),
    }
}

#[delete("/photos/{photo_id}/tags/{tag_id}")]
pub async fn remove_tag_from_photo(
    info: web::Path<(i64, i64)>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::Error> {
    let (photo_id, tag_id) = info.into_inner();

    let res = Photo::remove_tag_from_photo(photo_id, tag_id, &pool).await;

    match res {
        Ok(message) => Ok(ApiResponse::success(message)),
        Err(err) => Ok(ApiResponse::error(err.to_string())),
    }
}

// RESET RANDOM SEED *******************************************************************************

#[get("/resetseed")]
pub async fn reset_seed(pool: web::Data<Pool>) -> Result<HttpResponse, errors::Error> {
    let res = schemas::reset_seed(&pool).await;

    match res {
        Ok(_) => Ok(ApiResponse::success(
            "`photo_ordering` materialized view was refreshed successfully",
        )),
        Err(err) => Ok(ApiResponse::error(err.to_string())),
    }
}
