use actix_web::{delete, get, patch, post, web};
use deadpool_postgres::Pool;

use crate::requests::search_request::SearchRequest;
use crate::responses::api_response::ApiResponse;
use crate::schemas::entity::Entity;
use crate::types::HandlerResult;

// ALL ENTITIES ************************************************************************************

#[get("/entities")]
pub async fn get_entities(pool: web::Data<Pool>) -> HandlerResult {
    let entities = Entity::get_all(&pool).await?;

    Ok(ApiResponse::success(entities))
}

// CREATE ENTITY ***********************************************************************************

#[derive(serde::Serialize, serde::Deserialize)]
pub struct NewEntitySimple {
    #[serde(alias = "entityName")]
    pub entity_name: String,
}

#[post("/entities")]
pub async fn create_entity_simple(
    params: web::Json<NewEntitySimple>,
    pool: web::Data<Pool>,
) -> HandlerResult {
    let new_entity = Entity::create_simple(params.into_inner().entity_name.as_str(), &pool).await?;

    Ok(ApiResponse::success(new_entity))
}

// UPDATE ENTITY ***********************************************************************************

#[patch("/entities/{id}")]
pub async fn update_entity(
    _: web::Path<i32>,
    params: web::Json<Entity>,
    pool: web::Data<Pool>,
) -> HandlerResult {
    let updated_entity = Entity::update(params.into_inner(), &pool).await?;

    Ok(ApiResponse::success(updated_entity))
}

// DELETE ENTITY ***********************************************************************************

#[delete("/entities/{id}")]
pub async fn delete_entity(info: web::Path<i32>, pool: web::Data<Pool>) -> HandlerResult {
    let message = Entity::delete(info.into_inner(), &pool).await?;

    Ok(ApiResponse::success(message))
}

// SEARCH ******************************************************************************************

#[get("/entities/search")]
pub async fn search_entities(
    params: web::Query<SearchRequest>,
    pool: web::Data<Pool>,
) -> HandlerResult {
    let res = Entity::perform_search(params.into_inner().q, &pool).await?;

    Ok(ApiResponse::success(res))
}
