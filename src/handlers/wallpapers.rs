use actix_web::{delete, get, patch, post, web, HttpResponse};
use deadpool_postgres::Pool;

use crate::errors;
use crate::responses::api_response::ApiResponse;
use crate::schemas::wallpaper_sizes::WallpaperSize;

// ALL WALLPAPER SIZES *****************************************************************************

#[get("/wallpaper_sizes")]
pub async fn get_wallpaper_sizes(pool: web::Data<Pool>) -> Result<HttpResponse, errors::Error> {
    let res = WallpaperSize::get_all(&pool).await;

    match res {
        Ok(sizes) => Ok(ApiResponse::success(sizes)),
        Err(err) => Ok(ApiResponse::error(err.to_string())),
    }
}
