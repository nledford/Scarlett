use actix_web::{delete, get, patch, post, web};
use deadpool_postgres::Pool;

use crate::responses::api_response::ApiResponse;
use crate::schemas::collections::Collection;
use crate::types::HandlerResult;

// ALL COLLECTIONS *********************************************************************************

#[get("/collections")]
pub async fn get_collections(pool: web::Data<Pool>) -> HandlerResult {
    let collections = Collection::get_all(&pool).await?;

    Ok(ApiResponse::success(collections))
}

// SINGLE COLLECTION *******************************************************************************

#[get("/collections/{id}")]
pub async fn get_collection(info: web::Path<i32>, pool: web::Data<Pool>) -> HandlerResult {
    let collection = Collection::get(info.into_inner(), &pool).await?;

    Ok(ApiResponse::success(collection))
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
) -> HandlerResult {
    let collection = params.into_inner();

    let new_collection = Collection::create(&collection.name, &collection.query, &pool).await?;

    Ok(ApiResponse::success(new_collection))
}

// UPDATE COLLECTION *******************************************************************************

#[patch("/collections/{id}")]
pub async fn update_collection(
    _: web::Path<i32>,
    params: web::Json<Collection>,
    pool: web::Data<Pool>,
) -> HandlerResult {
    let updated_collection = Collection::update(params.into_inner(), &pool).await?;

    Ok(ApiResponse::success(updated_collection))
}

// DELETE COLLECTION *******************************************************************************

#[delete("/collections/{id}")]
pub async fn delete_collection(info: web::Path<i32>, pool: web::Data<Pool>) -> HandlerResult {
    let message = Collection::delete(info.into_inner(), &pool).await?;

    Ok(ApiResponse::success(message))
}
