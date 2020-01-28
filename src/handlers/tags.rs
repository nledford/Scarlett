use actix_web::{delete, get, patch, post, web, HttpResponse};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};

use crate::errors;
use crate::responses::api_response::ApiResponse;
use crate::schemas::tags::Tag;

// ALL TAGS ****************************************************************************************

#[get("/tags")]
pub async fn get_tags(pool: web::Data<Pool>) -> Result<HttpResponse, errors::Error> {
    let res = Tag::get_all(&pool).await;

    match res {
        Ok(tags) => Ok(ApiResponse::success(tags)),
        Err(err) => Ok(ApiResponse::error(err.to_string())),
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
        Ok(new_tag) => Ok(ApiResponse::success(new_tag)),
        Err(err) => Ok(ApiResponse::error(err.to_string())),
    }
}

// UPDATE TAG **************************************************************************************

#[patch("/tags/{id}")]
pub async fn update_tag(
    _: web::Path<i32>,
    params: web::Json<Tag>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::Error> {
    let res = Tag::update(params.into_inner(), &pool).await;

    match res {
        Ok(updated_tag) => Ok(ApiResponse::success(updated_tag)),
        Err(err) => Ok(ApiResponse::error(err.to_string())),
    }
}

// DELETE TAG **************************************************************************************

#[delete("/tags/{id}")]
pub async fn delete_tag(
    info: web::Path<i32>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::Error> {
    let res = Tag::delete(info.into_inner(), &pool).await;

    match res {
        Ok(message) => Ok(ApiResponse::success(message)),
        Err(err) => Ok(ApiResponse::error(err.to_string())),
    }
}
