use actix_web::{get, web, HttpResponse};
use deadpool_postgres::Pool;

use crate::models::errors;
use crate::models::responses::ApiResponse;
use crate::requests::get_photos_request::GetPhotosRequest;
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
        Ok(page) => Ok(HttpResponse::Ok().json(ApiResponse::success(page))),
        Err(err) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::error(err.to_string())))
        }
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
        Ok(photo) => Ok(HttpResponse::Ok().json(ApiResponse::success(photo))),
        Err(err) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::error(err.to_string())))
        }
    }
}

// UPDATE PHOTO ************************************************************************************

// RESET RANDOM SEED *******************************************************************************

#[get("/resetseed")]
pub async fn reset_seed(pool: web::Data<Pool>) -> Result<HttpResponse, errors::Error> {
    let res = schemas::reset_seed(&pool).await;

    match res {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success(
            "`photo_ordering` materialized view was refreshed successfully",
        ))),
        Err(err) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::error(err.to_string())))
        }
    }
}
