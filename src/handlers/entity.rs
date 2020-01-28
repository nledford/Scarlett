use actix_web::{delete, get, patch, post, web, HttpResponse};
use deadpool_postgres::Pool;

use crate::errors;
use crate::responses::api_response::ApiResponse;
use crate::schemas::entity::Entity;

// ALL ENTITIES ************************************************************************************

#[get("/entities")]
pub async fn get_entities(pool: web::Data<Pool>) -> Result<HttpResponse, errors::Error> {
    let res = Entity::get_all(&pool).await;

    match res {
        Ok(entities) => Ok(ApiResponse::success(entities)),
        Err(err) => Ok(ApiResponse::error(err.to_string())),
    }
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
) -> Result<HttpResponse, errors::Error> {
    let res = Entity::create_simple(params.into_inner().entity_name.as_str(), &pool).await;

    match res {
        Ok(new_entity) => Ok(ApiResponse::success(new_entity)),
        Err(err) => Ok(ApiResponse::error(err.to_string())),
    }
}

// UPDATE ENTITY ***********************************************************************************

#[patch("/entities/{id}")]
pub async fn update_entity(
    _: web::Path<i32>,
    params: web::Json<Entity>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::Error> {
    let res = Entity::update(params.into_inner(), &pool).await;

    match res {
        Ok(updated_entity) => Ok(ApiResponse::success(updated_entity)),
        Err(err) => Ok(ApiResponse::error(err.to_string())),
    }
}

// DELETE ENTITY ***********************************************************************************

#[delete("/entities/{id}")]
pub async fn delete_entity(
    info: web::Path<i32>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::Error> {
    let res = Entity::delete(info.into_inner(), &pool).await;

    match res {
        Ok(message) => Ok(ApiResponse::success(message)),
        Err(err) => Ok(ApiResponse::error(err.to_string())),
    }
}
