use actix_web::{get, web, HttpResponse};
use deadpool_postgres::Pool;

use crate::models::errors;
use crate::models::responses::ApiResponse;
use crate::requests::get_photos_request::GetPhotosRequest;
use crate::schemas;
use crate::schemas::photo::Photo;
use crate::schemas::photo_full::PhotoFull;

// ALL PHOTOS **************************************************************************************

/* Possible candidate for pagination query
* SELECT *
  FROM (SELECT row_number() over () AS position, p.*
        FROM photos p
                 INNER JOIN photo_ordering po ON p.id = po.photo_id
        ORDER BY po.position) t
  WHERE t.position > 0
  LIMIT 100;
*/

#[get("/photos")]
//pub async fn get_photos(pool: web::Data<Pool>) -> Result<HttpResponse, errors::Error> {
//    let res = PhotoFull::all_photos(&pool).await;
//
//    match res {
//        Ok(photos) => Ok(HttpResponse::Ok().json(photos)),
//        Err(err) => Ok(HttpResponse::InternalServerError().json(err.to_string())),
//    }
//}
pub async fn get_photos(
    info: web::Query<GetPhotosRequest>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::Error> {
    let res = PhotoFull::get_page(info.into_inner(), &pool).await;

    match res {
        Ok(page) => Ok(HttpResponse::Ok().json(ApiResponse::new("success", 200, "ok", page))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::new(
            "error",
            500,
            "An error has occurred",
            err.to_string(),
        ))),
    }
}

// SINGLE PHOTO ************************************************************************************

#[get("/photos/{photo_id}")]
pub async fn get_photo(
    info: web::Path<i64>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::Error> {
    let res = Photo::get_photo_by_id(info.into_inner(), &pool).await;

    match res {
        Ok(photo) => Ok(HttpResponse::Ok().json(photo)),
        Err(err) => Ok(HttpResponse::InternalServerError().json(err.to_string())),
    }
}

// UPDATE PHOTO ************************************************************************************

// RESET RANDOM SEED *******************************************************************************

#[get("/resetseed")]
pub async fn reset_seed(pool: web::Data<Pool>) -> Result<HttpResponse, errors::Error> {
    let res = schemas::reset_seed(&pool).await;

    match res {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::new(
            "success",
            200,
            "ok",
            "`photo_ordering` materialized view was refreshed successfully",
        ))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::new(
            "error",
            500,
            "An error has occurred",
            err.to_string(),
        ))),
    }
}
