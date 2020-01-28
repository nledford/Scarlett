use actix_web::{get, HttpResponse, web};
use deadpool_postgres::Pool;

use crate::errors::ServiceError;
use crate::responses::api_response::ApiResponse;
use crate::schemas::directory_tree::get_directory_tree;

#[get("/directories")]
pub async fn get_tree(pool: web::Data<Pool>) -> Result<HttpResponse, ServiceError> {
    let res = get_directory_tree(&pool).await;

    match res {
        Ok(tree) => Ok(ApiResponse::success(tree)),
        Err(err) => Ok(ApiResponse::error(err.to_string())),
    }
}
