use actix_web::{get, web};
use deadpool_postgres::Pool;

use crate::responses::api_response::ApiResponse;
use crate::schemas::directory_tree::get_directory_tree;
use crate::types::HandlerResult;

#[get("/directories")]
pub async fn get_tree(pool: web::Data<Pool>) -> HandlerResult {
    let tree = get_directory_tree(&pool).await?;

    Ok(ApiResponse::success(tree))
}
