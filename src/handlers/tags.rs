use actix_web::{get, post, web, HttpResponse};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};

use crate::models::errors;
use crate::models::responses::ApiResponse;
use crate::schemas::tags::Tag;
use crate::schemas::DbTable;

// ALL TAGS ****************************************************************************************

#[get("/tags")]
pub async fn get_tags(pool: web::Data<Pool>) -> Result<HttpResponse, errors::Error> {
    let res = Tag::get_all(&pool).await;

    match res {
        Ok(tags) => Ok(HttpResponse::Ok().json(ApiResponse::success(tags))),
        Err(err) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::error(err.to_string())))
        }
    }
}

// CREATE TAG **************************************************************************************

#[derive(Serialize, Deserialize)]
pub struct NewTag {
    #[serde(alias = "tagName")]
    pub tag_name: String,
}

#[post("/tags")]
pub async fn create_tag(
    params: web::Json<NewTag>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::Error> {
    let res = Tag::create(params.into_inner().tag_name.as_str(), &pool).await;

    match res {
        Ok(new_tag) => Ok(HttpResponse::Ok().json(ApiResponse::success(tag))),
        Err(err) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::error(err.to_string())))
        }
    }
}
