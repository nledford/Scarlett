use crate::models::errors;
use crate::models::responses::ApiResponse;
use crate::schemas::directory_tree::get_directory_tree;
use actix_web::{get, web, HttpResponse};
use deadpool_postgres::Pool;

#[get("/directories")]
pub async fn get_tree(pool: web::Data<Pool>) -> Result<HttpResponse, errors::Error> {
    let res = get_directory_tree(&pool).await;

    match res {
        Ok(tree) => Ok(HttpResponse::Ok().json(ApiResponse::success(tree))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(err.to_string())),
    }
}
