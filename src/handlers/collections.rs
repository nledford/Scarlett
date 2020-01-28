use actix_web::{delete, get, HttpResponse, patch, post, web};
use deadpool_postgres::Pool;

use crate::errors::ServiceError;
use crate::responses::api_response::ApiResponse;
use crate::schemas::collections::Collection;

// ALL COLLECTIONS *********************************************************************************

#[get("/collections")]
pub async fn get_collections(pool: web::Data<Pool>) -> Result<HttpResponse, ServiceError> {
    let res = Collection::get_all(&pool).await;

    match res {
        Ok(collections) => Ok(ApiResponse::success(collections)),
        Err(err) => Ok(ApiResponse::error(err.to_string())),
    }
}

// SINGLE COLLECTION *******************************************************************************

#[get("/collections/{id}")]
pub async fn get_collection(
    info: web::Path<i32>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = Collection::get_by_id(info.into_inner(), &pool).await;

    match res {
        Ok(collection) => Ok(ApiResponse::success(collection)),
        Err(err) => Ok(ApiResponse::error(err.to_string())),
    }
}

// CREATE COLLECTION *******************************************************************************

#[derive(serde::Deserialize)]
pub struct NewCollection {
    pub name: String,
    pub query: String,
}

#[post("/collections")]
pub async fn create_collection(
    params: web::Json<NewCollection>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let collection = params.into_inner();

    let res = Collection::create(&collection.name, &collection.query, &pool).await;

    match res {
        Ok(new_collection) => Ok(ApiResponse::success(new_collection)),
        Err(err) => Ok(ApiResponse::error(err.to_string())),
    }
}

// UPDATE COLLECTION *******************************************************************************

#[patch("/collections/{id}")]
pub async fn update_collection(
    _: web::Path<i32>,
    params: web::Json<Collection>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = Collection::update(params.into_inner(), &pool).await;

    match res {
        Ok(updated_collection) => Ok(ApiResponse::success(updated_collection)),
        Err(err) => Ok(ApiResponse::error(err.to_string())),
    }
}

// DELETE COLLECTION *******************************************************************************

#[delete("/collections/{id}")]
pub async fn delete_collection(
    info: web::Path<i32>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = Collection::delete(info.into_inner(), &pool).await;

    match res {
        Ok(message) => Ok(ApiResponse::success(message)),
        Err(err) => Ok(ApiResponse::error(err.to_string())),
    }
}
