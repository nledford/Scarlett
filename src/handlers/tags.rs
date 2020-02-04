use actix_web::{delete, get, patch, post, web, HttpResponse, Error};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};

use crate::requests::search_request::SearchRequest;
use crate::responses::api_response::ApiResponse;
use crate::schemas::tags::Tag;

// ALL TAGS ****************************************************************************************

#[get("/tags")]
pub async fn get_tags(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let tags = Tag::get_all(&pool).await?;

    Ok(ApiResponse::success(tags))
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
) -> Result<HttpResponse, Error> {
    let new_tag= Tag::create(params.into_inner().tag_name.as_str(), &pool).await?;

    Ok(ApiResponse::success(new_tag))
}

// UPDATE TAG **************************************************************************************

#[patch("/tags/{id}")]
pub async fn update_tag(
    _: web::Path<i32>,
    params: web::Json<Tag>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let updated_tag = Tag::update(params.into_inner(), &pool).await?;

    Ok(ApiResponse::success(updated_tag))
}

// DELETE TAG **************************************************************************************

#[delete("/tags/{id}")]
pub async fn delete_tag(
    info: web::Path<i32>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let message = Tag::delete(info.into_inner(), &pool).await?;

    Ok(ApiResponse::success(message))
}

// PERFORM SEARCH **********************************************************************************

#[get("/tags/search")]
pub async fn search_tags(
    params: web::Query<SearchRequest>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let res = Tag::perform_search(params.into_inner().q, &pool).await?;

    Ok(ApiResponse::success(res))
}
