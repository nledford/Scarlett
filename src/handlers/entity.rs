use crate::errors::errors;
use crate::responses::api_response::ApiResponse;
use crate::schemas::entity::Entity;
use crate::schemas::DbTable;
use actix_web::{get, web, HttpResponse};
use deadpool_postgres::Pool;

// ALL ENTITIES ************************************************************************************

#[get("/entities")]
pub async fn get_entities(pool: web::Data<Pool>) -> Result<HttpResponse, errors::Error> {
    let res = Entity::get_all(&pool).await;

    match res {
        Ok(entities) => Ok(ApiResponse::success(entities)),
        Err(err) => Ok(ApiResponse::error(err.to_string())),
    }
}
