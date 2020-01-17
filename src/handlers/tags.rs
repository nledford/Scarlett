use actix_web::{get, web, HttpResponse};
use deadpool_postgres::Pool;

use crate::models::errors;
use crate::models::responses::ApiResponse;
use crate::schemas::tags::Tag;
use crate::schemas::DbTable;

// ALL TAGS ****************************************************************************************

#[get("/tags")]
pub async fn get_tags(pool: web::Data<Pool>) -> Result<HttpResponse, errors::Error> {
    let res = Tag::get_all(&pool).await;

    match res {
        Ok(tags) => Ok(HttpResponse::Ok().json(ApiResponse::new("success", 200, "Ok", tags))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::new(
            "error",
            500,
            "An error has occurred",
            err.to_string(),
        ))),
    }
}
